use bessemer::simulation::*;

pub(super) fn word(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_le_bytes());
}

pub(super) fn span(output: &mut Vec<u8>, bytes: &[u8]) {
    word(output, bytes.len().try_into().unwrap());
    output.extend_from_slice(bytes);
}

pub(super) fn coverage(output: &mut Vec<u8>, scope: SimulationPersistCoverage) {
    let incomplete = scope.completion == SimulationPersistCompletion::IncompleteUnconsumedTail;
    let diagnostics = scope.diagnostics.iter().fold(0, |bits, value| {
        bits | match value {
            SimulationPersistDiagnostic::NativeApplicationNotExecuted => 1,
            SimulationPersistDiagnostic::FieldSemanticsUnassigned => 2,
            SimulationPersistDiagnostic::NumericConversionNotInferred => 4,
        }
    });
    for value in [
        scope.source_bytes as u32,
        scope.consumed_bytes as u32,
        scope.opaque_bytes as u32,
        u32::from(incomplete),
        scope.requested_pattern_error_bits,
        diagnostics,
    ] {
        word(output, value);
    }
}

pub(super) fn text(record: &SimulationPersistText) -> Vec<u8> {
    let mut output = Vec::new();
    for bytes in [record.source_parent(), record.header(), record.payload()] {
        span(&mut output, bytes);
    }
    let units = record.utf16_units();
    text_fields(&mut output, record, units.is_some());
    let encoded: Vec<_> = units
        .into_iter()
        .flatten()
        .flat_map(|unit| unit.to_le_bytes())
        .collect();
    span(&mut output, &encoded);
    assert_eq!(
        record.coverage().check_mode,
        SimulationPersistCheckMode::ExactTextFrame
    );
    coverage(&mut output, record.coverage());
    output
}

fn text_fields(output: &mut Vec<u8>, record: &SimulationPersistText, has_units: bool) {
    let (start, payload, end) = record.source_offsets();
    for value in [
        start as u32,
        payload as u32,
        end as u32,
        record.declared_bytes(),
        u32::from(record.is_null()),
        u32::from(has_units),
    ] {
        word(output, value);
    }
}

pub(super) fn study(record: &SimulationStudyOrder) -> Vec<u8> {
    let mut output = Vec::new();
    span(&mut output, &record.to_bytes());
    word(&mut output, record.count_offset() as u32);
    word(&mut output, record.items().len() as u32);
    span(&mut output, record.count_header());
    for item in std::iter::once(record.leading()).chain(record.items()) {
        span(&mut output, &text(item));
    }
    assert_eq!(
        record.coverage().check_mode,
        SimulationPersistCheckMode::ExactStudyOrder
    );
    coverage(&mut output, record.coverage());
    output
}

pub(super) fn primary(record: &SimulationPrimaryVersion) -> Vec<u8> {
    let mut output = Vec::new();
    span(&mut output, &record.to_bytes());
    span(&mut output, &text(record.text()));
    primary_fields(&mut output, record);
    primary_words(&mut output, record);
    primary_updates(&mut output, record);
    assert_eq!(
        record.coverage().check_mode,
        SimulationPersistCheckMode::ExactPrimaryVersionMode
    );
    coverage(&mut output, record.coverage());
    output
}

fn primary_fields(output: &mut Vec<u8>, record: &SimulationPrimaryVersion) {
    for value in [
        record.numeric_version().value() as u32,
        u32::from(record.read_mode() == SimulationPersistReadMode::Full),
        record.consumed_end() as u32,
        record.words().len() as u32,
    ] {
        word(output, value);
    }
}

fn primary_words(output: &mut Vec<u8>, record: &SimulationPrimaryVersion) {
    span(output, record.word_payload());
    span(
        output,
        &record
            .words()
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect::<Vec<_>>(),
    );
    span(output, record.remainder());
}

fn primary_updates(output: &mut Vec<u8>, record: &SimulationPrimaryVersion) {
    word(output, record.owner_updates().len() as u32);
    for update in record.owner_updates() {
        word(
            output,
            match update.destination {
                SimulationPersistOwnerSlot::Owner1A8 => 0x1a8,
                SimulationPersistOwnerSlot::Owner1C8 => 0x1c8,
            },
        );
        word(output, update.value as u32);
    }
}

pub(super) fn legacy(record: &SimulationLegacyVersion) -> Vec<u8> {
    let mut output = Vec::new();
    span(&mut output, &record.to_bytes());
    span(&mut output, record.prefix());
    span(&mut output, record.opaque_suffix());
    word(&mut output, record.numeric_version().value() as u32);
    span(
        &mut output,
        &record
            .text_units()
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        record.coverage().check_mode,
        SimulationPersistCheckMode::ExactLegacyVersionProfile
    );
    coverage(&mut output, record.coverage());
    output
}
