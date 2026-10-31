use bessemer::simulation::*;

pub(super) fn words(output: &mut Vec<u8>, values: &[u32]) {
    output.extend(values.iter().flat_map(|value| value.to_le_bytes()));
}

pub(super) fn span(output: &mut Vec<u8>, raw: &[u8]) {
    words(output, &[raw.len().try_into().unwrap()]);
    output.extend_from_slice(raw);
}

pub(super) fn context(
    output: &mut Vec<u8>,
    version: SimulationPersistVersion,
    limits: SimulationParameterLimits,
) {
    words(
        output,
        &[
            version.value() as u32,
            limits.maximum_records as u32,
            limits.maximum_table_items as u32,
            limits.maximum_text_bytes as u32,
        ],
    );
}

pub(super) fn kind(value: &SimulationParameterValue) -> u32 {
    match value {
        SimulationParameterValue::Signed(_) => 0,
        SimulationParameterValue::Unsigned(_) => 1,
        SimulationParameterValue::DoubleBits(_) => 2,
        SimulationParameterValue::Text(_) => 3,
    }
}

fn role_words(role: SimulationParameterRole) -> [u32; 3] {
    use SimulationParameterRole::*;
    match role {
        HeaderWord(index) => [0, index as u32, 0],
        Gate700013 => [1, 0, 0],
        Gate700018 => [2, 0, 0],
        RecordCount => [3, 0, 0],
        TableWord(index) => [4, index as u32, 0],
        Tail => [11, 0, 0],
        Gate820015 => [12, 0, 0],
        Gate830009 => [13, 0, 0],
        other => record_role(other),
    }
}

fn record_role(role: SimulationParameterRole) -> [u32; 3] {
    use SimulationParameterRole::*;
    match role {
        RecordText(index) => [5, index as u32, 0],
        RecordSignedWord(index) => [6, index as u32, 0],
        RecordUnsignedWord { record, component } => [7, record as u32, component as u32],
        RecordConditionalWord(index) => [8, index as u32, 0],
        RecordConditionalDouble(index) => [9, index as u32, 0],
        RecordValue { record, column } => [10, record as u32, column as u32],
        other => panic!("{other:?}; expected a complete record primitive role"),
    }
}

fn completion(value: SimulationParameterCompletion) -> u32 {
    match value {
        SimulationParameterCompletion::CompletedEmptyRecords => 0,
        SimulationParameterCompletion::CompletedRecords => 1,
        SimulationParameterCompletion::Unchanged => 2,
        SimulationParameterCompletion::Changed => 3,
    }
}

fn diagnostics(entries: &[SimulationParameterDiagnostic]) -> u32 {
    entries.iter().fold(0, |bits, entry| {
        bits | match entry {
            SimulationParameterDiagnostic::NativeApplicationNotExecuted => 1,
            SimulationParameterDiagnostic::RuntimeVersionNotInferred => 2,
            SimulationParameterDiagnostic::LegacyFallbackNotUsed => 4,
            SimulationParameterDiagnostic::NativeRetentionNotApplied => 8,
            SimulationParameterDiagnostic::FieldMeaningsUnassigned => 16,
        }
    })
}

fn coverage(output: &mut Vec<u8>, scope: SimulationParameterCoverage) {
    context(output, scope.version, scope.limits);
    words(
        output,
        &[
            completion(scope.completion),
            u32::from(
                scope.check_mode
                    == SimulationParameterCheckMode::ExactSelectedProfileAndPreservation,
            ),
            scope.checked_bytes as u32,
            scope.checked_fields as u32,
            scope.table_count as u32,
            scope.record_count as u32,
            scope.checked_text_frames as u32,
            diagnostics(&scope.diagnostics),
            scope.requested_pattern_error_bits,
        ],
    );
}

pub(super) fn image(source: &SimulationParameterSets) -> Vec<u8> {
    let mut output = Vec::new();
    image_header(&mut output, source);
    for (ordinal, field) in source.fields().iter().enumerate() {
        assert_eq!(field.id, source.field_id(ordinal).unwrap());
        words(&mut output, &role_words(field.role));
        words(
            &mut output,
            &[field.stream_offset as u32, kind(&field.value)],
        );
        span(
            &mut output,
            &source.field(&field.id).unwrap().value.to_bytes(),
        );
    }
    coverage(&mut output, source.coverage());
    output
}

fn image_header(output: &mut Vec<u8>, source: &SimulationParameterSets) {
    context(output, source.version(), source.limits());
    span(output, &source.to_bytes());
    words(output, &source.parts().header.words.map(|word| word as u32));
    words(
        output,
        &[
            source.parts().table.len() as u32,
            source.parts().records.len() as u32,
            source.fields().len() as u32,
        ],
    );
}

pub(super) fn edit(change: &SimulationParameterEdit, ordinal: usize) -> Vec<u8> {
    let mut output = Vec::new();
    span(&mut output, &change.source.to_bytes());
    span(&mut output, &image(&change.archive));
    words(&mut output, &[ordinal as u32, kind(&change.replacement)]);
    span(&mut output, &change.replacement.to_bytes());
    words(
        &mut output,
        &[
            u32::from(change.coverage.completion == SimulationParameterCompletion::Changed),
            change.history.len() as u32,
        ],
    );
    for (index, relation) in change.history.iter().enumerate() {
        history(&mut output, change, index, relation);
    }
    coverage(&mut output, change.coverage);
    output
}

fn history(
    output: &mut Vec<u8>,
    change: &SimulationParameterEdit,
    ordinal: usize,
    relation: &SimulationParameterHistory,
) {
    assert_eq!(relation.parent, change.source.field_id(ordinal).unwrap());
    assert_eq!(relation.result, change.archive.field_id(ordinal).unwrap());
    let parent = change.source.field(&relation.parent).unwrap();
    let result = change.archive.field(&relation.result).unwrap();
    assert_eq!(parent.role, result.role);
    history_positions(output, ordinal, parent, result);
    span(output, &parent.value.to_bytes());
    span(output, &result.value.to_bytes());
    words(
        output,
        &[u32::from(
            relation.relation == SimulationParameterRelation::Derived,
        )],
    );
}

fn history_positions(
    output: &mut Vec<u8>,
    ordinal: usize,
    parent: &SimulationParameterField,
    result: &SimulationParameterField,
) {
    words(
        output,
        &[
            ordinal as u32,
            parent.stream_offset as u32,
            result.stream_offset as u32,
        ],
    );
    words(output, &role_words(parent.role));
    words(output, &[kind(&parent.value)]);
}
