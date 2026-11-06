use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "edit.rs"]
mod edit;
#[path = "replay.rs"]
mod replay;
fn options() -> FlowWideVectorOptions {
    FlowWideVectorOptions {
        maximum_payload_bytes: 2_000_000,
        maximum_vectors: 100_000,
        maximum_values: 1_000_000,
    }
}
fn open(raw: &[u8], options: FlowWideVectorOptions) -> bessemer::Result<FlowWideVectors> {
    let source = state_carrier::carrier(148, raw);
    let id = source.records().nth(1).unwrap().id;
    source.record_148_wide_vectors(&id, options)
}
fn check_case(row: &VectorCase) -> FlowWideVectors {
    let view = open(&row.raw, options()).unwrap();
    assert_eq!(view.version(), row.version);
    assert_eq!(view.decoded_bytes(), row.raw);
    assert_eq!(view.stored_count(), row.arrays.len());
    assert_eq!(view.options(), options());
    assert_eq!(view.record(), &view.source().records().nth(1).unwrap().id);
    assert_eq!(view.length_table_interval(), (8, 8 + row.arrays.len() * 4));
    for (ordinal, array) in row.arrays.iter().enumerate() {
        check_array(&view, ordinal, array);
    }
    assert!(view.vector(row.arrays.len()).is_err() && view.vector(usize::MAX).is_err());
    check_coverage(&view, row);
    view
}
fn check_array(view: &FlowWideVectors, ordinal: usize, expected: &VectorProjection) {
    let array = view.vector(ordinal).unwrap();
    assert_eq!(array.as_bytes(), expected.raw);
    assert_eq!(array.word_count(), expected.length as usize);
    assert_eq!(
        array.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    let (values, remainder) = expected.raw.as_chunks::<8>();
    assert!(remainder.is_empty());
    for (index, raw) in values.iter().enumerate() {
        assert_eq!(array.raw_bits(index), Some(u64::from_le_bytes(*raw)));
    }
    assert!(array.raw_bits(array.word_count()).is_none() && array.raw_bits(usize::MAX).is_none());
    assert!(
        view.replace_pattern(ordinal, array.word_count(), 0)
            .is_err()
    );
}
fn check_coverage(view: &FlowWideVectors, row: &VectorCase) {
    let proof = view.coverage();
    assert_eq!(proof.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(proof.payload_bytes, row.raw.len());
    assert_eq!(proof.stored_vectors, row.arrays.len());
    assert_eq!(
        proof.stored_values,
        row.arrays.iter().map(|array| array.length as usize).sum()
    );
    assert_eq!(
        proof.empty_vectors,
        row.arrays.iter().filter(|array| array.length == 0).count()
    );
    assert_eq!(proof.achieved_error, 0);
    check_diagnostics(view);
}
fn check_diagnostics(view: &FlowWideVectors) {
    use FlowSimpleTableDiagnostic::*;
    assert_eq!(
        view.diagnostics(),
        &[
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn every_owned_vector_length_raw_pattern_and_optional_empty_presence_matches_the_source() {
    for row in cases() {
        check_case(&row);
    }
}
