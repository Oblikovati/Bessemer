use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "edit.rs"]
mod edit;
#[path = "replay.rs"]
mod replay;
fn options() -> FlowAuxWideOptions {
    FlowAuxWideOptions {
        maximum_decoded_bytes: 2_000_000,
        maximum_values: 100_000,
    }
}
fn open(stored: &[u8], options: FlowAuxWideOptions) -> bessemer::Result<FlowAuxWideValues> {
    let source = state_carrier::carrier(444, stored);
    let id = source.records().nth(1).unwrap().id;
    source.record_444_aux_wides(&id, options)
}
fn check_case(row: &AuxWideCase) -> FlowAuxWideValues {
    let view = open(&row.stored, options()).unwrap();
    assert_eq!(view.version(), row.version);
    assert_eq!(view.discarded_legacy(), row.legacy);
    assert_eq!(view.decoded_bytes(), row.decoded);
    assert_eq!(view.stored_count(), row.count as usize);
    assert_eq!(view.options(), options());
    assert!(view.native_values_discarded());
    assert_eq!(view.record(), &view.source().records().nth(1).unwrap().id);
    check_values(&view, row);
    check_coverage(&view, row);
    view
}
fn check_values(view: &FlowAuxWideValues, row: &AuxWideCase) {
    let array = view.values();
    let start = if row.version == 0 { 12 } else { 8 };
    assert_eq!(array.as_bytes(), row.values);
    assert_eq!(array.word_count(), row.count as usize);
    assert_eq!(array.payload_interval(), (start, row.end as usize));
    let (chunks, remainder) = row.values.as_chunks::<8>();
    assert!(remainder.is_empty());
    for (index, expected) in chunks.iter().enumerate() {
        assert_eq!(array.raw_bits(index), Some(u64::from_le_bytes(*expected)));
    }
    assert_eq!(array.raw_bits(array.word_count()), None);
    assert_eq!(array.raw_bits(usize::MAX), None);
}
fn check_coverage(view: &FlowAuxWideValues, row: &AuxWideCase) {
    let proof = view.coverage();
    assert_eq!(proof.mode, FlowLegacyTableCheckMode::ExactZlibTable);
    assert_eq!(proof.stored_bytes, row.stored.len());
    assert_eq!(proof.decoded_bytes, row.decoded.len());
    assert_eq!(proof.wide_words, row.count as usize);
    assert_eq!(
        (
            proof.wire_rows,
            proof.raw_words,
            proof.raw_bytes,
            proof.achieved_error
        ),
        (0, 0, 0, 0)
    );
    check_diagnostics(view);
}
fn check_diagnostics(view: &FlowAuxWideValues) {
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
fn every_owned_decoded_field_and_raw_wide_pattern_matches_the_source_projection() {
    for row in cases() {
        check_case(&row);
    }
}
