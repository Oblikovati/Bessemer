use bessemer::simulation::*;

pub(super) fn words(output: &mut Vec<u8>, values: &[u32]) {
    output.extend(values.iter().flat_map(|value| value.to_le_bytes()));
}

pub(super) fn span(output: &mut Vec<u8>, bytes: &[u8]) {
    words(output, &[bytes.len().try_into().unwrap()]);
    output.extend_from_slice(bytes);
}

pub(super) fn coverage(output: &mut Vec<u8>, scope: SimulationLbcPropertyCoverage) {
    assert_eq!(
        scope.completion,
        SimulationLbcReadCompletion::CompletedFraming
    );
    let diagnostics = diagnostic_bits(&scope.diagnostics);
    let edited =
        scope.check_mode == SimulationLbcPropertyCheckMode::ExactSelectedProfileAndPreservation;
    words(
        output,
        &[
            scope.checked_bytes as u32,
            scope.checked_fields as u32,
            1,
            scope.requested_pattern_error_bits,
            diagnostics,
            u32::from(edited),
        ],
    );
}

fn diagnostic_bits(diagnostics: &[SimulationLbcPropertyDiagnostic]) -> u32 {
    diagnostics.iter().fold(0, |bits, value| {
        bits | match value {
            SimulationLbcPropertyDiagnostic::NativeApplicationNotExecuted => 1,
            SimulationLbcPropertyDiagnostic::FieldMeaningsUnassigned => 2,
            SimulationLbcPropertyDiagnostic::RuntimeVersionNotInferred => 4,
        }
    })
}

pub(super) fn image(record: &SimulationLbcProperty) -> Vec<u8> {
    let mut output = Vec::new();
    let (fixed, version) = match record.profile() {
        SimulationLbcPropertyProfile::VersionSelected(version) => (0, version.value() as u32),
        SimulationLbcPropertyProfile::InstalledFixedWriter => (1, 0),
    };
    words(&mut output, &[1, fixed, version]);
    span(&mut output, &record.to_bytes());
    words(&mut output, &[record.fields().len() as u32]);
    for (index, field) in record.fields().iter().enumerate() {
        words(&mut output, &field_words(record, index, field));
    }
    assert_eq!(
        record.coverage().check_mode,
        SimulationLbcPropertyCheckMode::ExactSelectedProfile
    );
    coverage(&mut output, record.coverage());
    output
}

fn field_words(
    record: &SimulationLbcProperty,
    index: usize,
    field: &SimulationLbcPropertyField,
) -> [u32; 3] {
    assert_eq!(
        record
            .field(&record.field_id(index).unwrap())
            .unwrap()
            .value,
        field.value
    );
    [
        field.stream_offset as u32,
        u32::from(field.member.runtime_offset()),
        field.value as u32,
    ]
}

pub(super) fn edit(result: &SimulationLbcFieldEdit) -> Vec<u8> {
    let mut output = Vec::new();
    let changed = result.completion == SimulationLbcEditCompletion::Changed;
    assert_eq!(
        changed,
        result.source.revision() != result.archive.revision()
    );
    edit_header(&mut output, result, changed);
    for (index, entry) in result.history.iter().enumerate() {
        history(&mut output, result, index, entry, changed);
    }
    assert_eq!(result.coverage.profile, result.source.profile());
    assert_eq!(
        result.coverage.check_mode,
        SimulationLbcPropertyCheckMode::ExactSelectedProfileAndPreservation
    );
    coverage(&mut output, result.coverage);
    output
}

fn edit_header(output: &mut Vec<u8>, result: &SimulationLbcFieldEdit, changed: bool) {
    span(output, &result.source.to_bytes());
    span(output, &image(&result.archive));
    let parent = result.source.field(&result.parent).unwrap();
    assert_eq!(
        result.archive.field(&result.result).unwrap().stream_offset,
        parent.stream_offset
    );
    words(
        output,
        &[
            parent.stream_offset as u32 / 4,
            result.replacement_value as u32,
            u32::from(changed),
            result.history.len() as u32,
        ],
    );
}

fn history(
    output: &mut Vec<u8>,
    edit: &SimulationLbcFieldEdit,
    index: usize,
    entry: &SimulationLbcFieldHistory,
    changed: bool,
) {
    let parent = edit.source.field(&entry.parent).unwrap();
    let result = edit.archive.field(&entry.result).unwrap();
    assert_eq!(parent.stream_offset, index * 4);
    assert_eq!(parent.member, result.member);
    if !changed {
        assert_eq!(entry.parent, entry.result);
    }
    words(
        output,
        &[
            parent.stream_offset as u32,
            result.stream_offset as u32,
            u32::from(parent.member.runtime_offset()),
            parent.value as u32,
            result.value as u32,
            u32::from(entry.relation == SimulationLbcFieldRelation::Derived),
        ],
    );
}
