use super::*;
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/edit.rs"]
mod edit;
#[path = "api/replay.rs"]
mod replay;
fn options() -> FlowControlOptions {
    FlowControlOptions {
        maximum_decoded_bytes: 2_000_000,
        maximum_entries: 1_000_000,
    }
}
fn width(row: &ControlCase) -> FlowControlLengthWidth {
    match row.width {
        4 => FlowControlLengthWidth::FourBytes,
        8 => FlowControlLengthWidth::EightBytes,
        value => panic!("fixture width {value}; expected four or eight"),
    }
}
fn open_saved(raw: &[u8], options: FlowControlOptions) -> bessemer::Result<FlowSavedSpans> {
    let source = state_carrier::carrier(SAVED, raw);
    let id = source.records().nth(1).unwrap().id;
    source.saved_span_writer(&id, options)
}
fn open_index(
    raw: &[u8],
    width: FlowControlLengthWidth,
    options: FlowControlOptions,
) -> bessemer::Result<FlowTerminalIndex> {
    let source = state_carrier::carrier(INDEX, raw);
    let id = source.records().nth(1).unwrap().id;
    source.terminal_index_writer(&id, width, options)
}
fn check_saved(row: &ControlCase) -> FlowSavedSpans {
    let view = open_saved(&row.stored, options()).unwrap();
    assert_eq!(view.options(), options());
    assert_eq!(view.record(), &view.source().records().nth(1).unwrap().id);
    assert_eq!(view.decoded_bytes(), row.decoded);
    assert_eq!(view.writer_marker(), row.marker);
    assert_eq!(view.stored_count(), row.count as usize);
    assert_eq!(view.compressed_interval(), (0, row.stored.len()));
    for (ordinal, entry) in words(&row.entries).as_chunks::<2>().0.iter().enumerate() {
        check_saved_entry(view.entry(ordinal).unwrap(), entry, ordinal);
    }
    check_saved_coverage(&view, row);
    assert_eq!(view.diagnostics(), SAVED_DIAGNOSTICS);
    view
}
fn check_saved_entry(actual: FlowSavedSpan<'_>, entry: &[u64; 2], ordinal: usize) {
    assert_eq!(actual.ordinal(), ordinal);
    assert_eq!(
        (actual.key_bits(), actual.value_bits()),
        (entry[0], entry[1])
    );
    assert_eq!(
        actual.payload_interval(),
        (16 + ordinal * 16, 32 + ordinal * 16)
    );
}
fn check_index(row: &ControlCase) -> FlowTerminalIndex {
    let view = open_index(&row.stored, width(row), options()).unwrap();
    assert_eq!(view.options(), options());
    assert_eq!(view.record(), &view.source().records().nth(1).unwrap().id);
    assert_eq!(view.decoded_bytes(), row.decoded);
    assert_eq!(view.writer_marker(), row.marker);
    assert_eq!(view.stored_count(), row.count as usize);
    assert_eq!(view.length_width(), width(row));
    assert_eq!(view.length_width().byte_width(), row.width as usize);
    check_index_transport(&view, row);
    for (ordinal, entry) in index_entries(&row.entries).into_iter().enumerate() {
        check_index_entry(view.entry(ordinal).unwrap(), entry, ordinal);
    }
    check_index_coverage(&view, row);
    check_index_diagnostics(&view);
    view
}
fn check_index_entry(actual: FlowTerminalIndexEntry<'_>, entry: (u32, u64, u64), ordinal: usize) {
    assert_eq!(actual.ordinal(), ordinal);
    assert_eq!(
        (
            actual.identifier(),
            actual.record_start(),
            actual.record_span()
        ),
        entry
    );
    assert_eq!(
        actual.payload_interval(),
        (4 + ordinal * 20, 24 + ordinal * 20)
    );
}
fn check_index_transport(view: &FlowTerminalIndex, row: &ControlCase) {
    let footer = row.footer.as_ref().unwrap();
    assert_eq!(view.compressed_interval(), (8, footer.start as usize));
    assert_eq!(
        view.footer_interval(),
        (footer.start as usize, row.stored.len())
    );
    assert_eq!(view.backward_distance(), footer.distance);
    assert_eq!(view.footer_identifier(), footer.identifier);
}
fn check_saved_coverage(view: &FlowSavedSpans, row: &ControlCase) {
    let proof = view.coverage();
    assert_eq!(proof.mode, FlowControlCheckMode::ExactSavedSpanWriter);
    assert_eq!(proof.length_width, None);
    assert_eq!(proof.decoded_prefix_bytes, 16);
    assert_eq!(proof.entry_bytes, row.count as usize * 16);
    assert_eq!(proof.raw_prefix_bytes, 0);
    assert_eq!(proof.footer_bytes, 0);
    check_common_coverage(proof, row);
}
fn check_index_coverage(view: &FlowTerminalIndex, row: &ControlCase) {
    let proof = view.coverage();
    assert_eq!(proof.mode, FlowControlCheckMode::ExactTerminalIndexWriter);
    assert_eq!(proof.length_width, Some(width(row)));
    assert_eq!(proof.decoded_prefix_bytes, 4);
    assert_eq!(proof.entry_bytes, row.count as usize * 20);
    assert_eq!(proof.raw_prefix_bytes, 8);
    assert_eq!(proof.footer_bytes, row.width as usize + 4);
    check_common_coverage(proof, row);
}
fn check_common_coverage(proof: FlowControlCoverage, row: &ControlCase) {
    assert_eq!(proof.stored_bytes, row.stored.len());
    assert_eq!(proof.decoded_bytes, row.decoded.len());
    assert_eq!(proof.stored_entries, row.count as usize);
    assert_eq!(proof.compressed_bytes, row.compressed as usize);
    assert_eq!(proof.achieved_error, 0);
}
const SAVED_DIAGNOSTICS: &[FlowControlDiagnostic] = &[
    FlowControlDiagnostic::WriterProfileAssumed,
    FlowControlDiagnostic::InstalledNativePayloadNotVerified,
    FlowControlDiagnostic::NativeRuntimeNotReproduced,
    FlowControlDiagnostic::SavedSpanMeaningsNotAssigned,
    FlowControlDiagnostic::OtherControlRecordsNotRewritten,
    FlowControlDiagnostic::NativeApplicationNotChecked,
];
fn check_index_diagnostics(view: &FlowTerminalIndex) {
    use FlowControlDiagnostic::*;
    let mut expected = vec![
        WriterProfileAssumed,
        InstalledNativePayloadNotVerified,
        NativeRuntimeNotReproduced,
        IndexTargetsNotValidated,
        IndexAdoptionNotPerformed,
        OtherControlRecordsNotRewritten,
        NativeApplicationNotChecked,
    ];
    if view.length_width() == FlowControlLengthWidth::FourBytes {
        expected.push(FourByteCarrierFormatNotEstablished);
    }
    assert_eq!(view.diagnostics(), expected);
}
#[test]
fn every_owned_entry_field_interval_footer_and_coverage_matches_the_strict_reference() {
    for row in cases() {
        if row.identifier == SAVED {
            check_saved(&row);
        } else {
            check_index(&row);
        }
    }
}
