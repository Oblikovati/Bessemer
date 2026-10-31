use super::pattern;
use bessemer::simulation::*;

pub(super) fn words(output: &mut Vec<u8>, values: &[u32]) {
    output.extend(values.iter().flat_map(|value| value.to_le_bytes()));
}

pub(super) fn span(output: &mut Vec<u8>, bytes: &[u8]) {
    words(output, &[bytes.len().try_into().unwrap()]);
    output.extend_from_slice(bytes);
}

pub(super) fn profile_tag(profile: SimulationPersistScalarProfile) -> u32 {
    match profile {
        SimulationPersistScalarProfile::CountedI32 => 0,
        SimulationPersistScalarProfile::UniqueIdsI64 => 1,
    }
}

fn role_words(role: SimulationPersistScalarRole) -> [u32; 2] {
    match role {
        SimulationPersistScalarRole::UsedCount => [0, 0],
        SimulationPersistScalarRole::AllocationCapacity => [1, 0],
        SimulationPersistScalarRole::Entry(index) => [2, index as u32],
    }
}

pub(super) fn coverage(output: &mut Vec<u8>, scope: SimulationPersistScalarCoverage) {
    let populated = scope.completion == SimulationPersistScalarCompletion::CompletedValues;
    let edited =
        scope.check_mode == SimulationPersistScalarCheckMode::ExactCompleteFrameAndPreservation;
    words(
        output,
        &[
            profile_tag(scope.profile),
            scope.checked_bytes as u32,
            scope.checked_header_bytes as u32,
            scope.checked_entries as u32,
            scope.maximum_entries as u32,
            u32::from(populated),
            u32::from(edited),
            diagnostics(&scope.diagnostics),
            scope.requested_pattern_error_bits,
        ],
    );
}

fn diagnostics(values: &[SimulationPersistScalarDiagnostic]) -> u32 {
    values.iter().fold(0, |bits, value| {
        bits | match value {
            SimulationPersistScalarDiagnostic::NativeApplicationNotExecuted => 1,
            SimulationPersistScalarDiagnostic::SemanticMappingNotInferred => 2,
            SimulationPersistScalarDiagnostic::CapacityNotAllocated => 4,
        }
    })
}

pub(super) fn image(record: &SimulationPersistScalarArchive) -> Vec<u8> {
    let mut output = Vec::new();
    image_header(&mut output, record);
    for field in record.fields() {
        assert_eq!(record.field(&field.id).unwrap().value, field.value);
        words(&mut output, &[field.stream_offset as u32]);
        words(&mut output, &role_words(field.role));
        span(&mut output, &pattern(field.value));
    }
    assert_eq!(
        record.coverage().check_mode,
        SimulationPersistScalarCheckMode::ExactCompleteFrame
    );
    coverage(&mut output, record.coverage());
    output
}

fn image_header(output: &mut Vec<u8>, record: &SimulationPersistScalarArchive) {
    words(output, &[1, profile_tag(record.profile())]);
    for bytes in [
        record.to_bytes().as_slice(),
        record.header(),
        record.payload(),
    ] {
        span(output, bytes);
    }
    words(
        output,
        &[
            record.used_count() as u32,
            u32::from(record.allocation_count().is_some()),
            record.allocation_count().unwrap_or(0) as u32,
            record.fields().len() as u32,
        ],
    );
}

pub(super) fn edit(result: &SimulationPersistScalarEdit) -> Vec<u8> {
    let mut output = Vec::new();
    span(&mut output, &result.source.to_bytes());
    span(&mut output, &image(&result.archive));
    let changed = result.completion == SimulationPersistScalarEditCompletion::Changed;
    assert_eq!(
        changed,
        result.source.revision() != result.archive.revision()
    );
    edit_header(&mut output, result, changed);
    for (index, entry) in result.history.iter().enumerate() {
        history(&mut output, result, index, entry, changed);
    }
    assert_eq!(
        result.coverage.check_mode,
        SimulationPersistScalarCheckMode::ExactCompleteFrameAndPreservation
    );
    coverage(&mut output, result.coverage);
    output
}

fn edit_header(output: &mut Vec<u8>, result: &SimulationPersistScalarEdit, changed: bool) {
    let ordinal = result
        .source
        .fields()
        .iter()
        .position(|field| field.id == result.parent)
        .unwrap();
    words(output, &[ordinal as u32]);
    span(output, &pattern(result.replacement));
    words(output, &[u32::from(changed), result.history.len() as u32]);
    assert_eq!(
        result.archive.field(&result.result).unwrap().value,
        result.replacement
    );
}

fn history(
    output: &mut Vec<u8>,
    edit: &SimulationPersistScalarEdit,
    index: usize,
    entry: &SimulationPersistScalarHistory,
    changed: bool,
) {
    let parent = edit.source.field(&entry.parent).unwrap();
    let child = edit.archive.field(&entry.result).unwrap();
    assert_eq!(entry.parent, edit.source.fields()[index].id);
    assert_eq!(entry.result, edit.archive.fields()[index].id);
    assert_eq!(parent.role, child.role);
    if !changed {
        assert_eq!(entry.parent, entry.result);
    }
    history_values(output, index, parent, child, entry.relation);
}

fn history_values(
    output: &mut Vec<u8>,
    index: usize,
    parent: &SimulationPersistScalarField,
    child: &SimulationPersistScalarField,
    relation: SimulationPersistScalarRelation,
) {
    words(
        output,
        &[
            index as u32,
            parent.stream_offset as u32,
            child.stream_offset as u32,
        ],
    );
    words(output, &role_words(parent.role));
    span(output, &pattern(parent.value));
    span(output, &pattern(child.value));
    words(
        output,
        &[u32::from(
            relation == SimulationPersistScalarRelation::Derived,
        )],
    );
}
