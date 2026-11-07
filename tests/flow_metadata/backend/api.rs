use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "edit.rs"]
mod edit;
fn options() -> FlowMetadataPrefixOptions {
    FlowMetadataPrefixOptions {
        string_mode: FlowMetadataStringMode::NativeUtf16,
        maximum_payload_bytes: 1_000_000,
        maximum_vectors: 1024,
        maximum_objects: 1024,
        maximum_values: 100_000,
        maximum_string_units: 100_000,
    }
}
fn prefix(raw: &[u8]) -> FlowMetadataPrefix {
    let mut payload = vec![3, 0, 0, 0, 0, 0, 0, 0, 0];
    payload.extend_from_slice(raw);
    let carrier = carrier(&payload);
    carrier
        .record_424_metadata_prefix(&carrier.latest(424).unwrap().unwrap().id, options())
        .unwrap()
}
fn open(raw: &[u8]) -> bessemer::Result<FlowMediumBackendPrefix> {
    prefix(raw).medium_backend_prefix()
}
#[test]
fn every_frozen_backend_header_kind_conditional_key_boundary_and_tail_matches_the_typed_reader() {
    for case in cases() {
        let view = open(&case.raw).unwrap();
        check_read(&view, &case);
    }
}
fn check_read(view: &FlowMediumBackendPrefix, case: &BackendCase) {
    assert_eq!(view.backend_bytes(), case.raw);
    assert_eq!(view.opaque_tail(), case.tail);
    assert_eq!(view.tables().len(), case.tables.len());
    assert_eq!(view.metadata_prefix().record().ordinal(), 1);
    assert_eq!(view.diagnostics(), view.metadata_prefix().diagnostics());
    for (table, expected) in view.tables().zip(&case.tables) {
        check_table(table, expected);
    }
    assert!(view.table(case.tables.len()).is_none());
    check_coverage(view, case);
}
fn check_table(table: FlowMediumBackendTable<'_>, expected: &BackendTable) {
    assert_eq!(table.ordinal(), expected.ordinal as usize);
    assert_eq!(table.backend_interval().0, expected.start as usize);
    assert_eq!(table.declared_count(), expected.count);
    assert_eq!(table.first_kind(), expected.kind);
    assert_eq!(table.first_key_bytes().unwrap_or(&[]), expected.key);
    assert_eq!(table.first_key_bytes().is_some(), !expected.key.is_empty());
    let size = 5 + if expected.kind.is_some() { 4 } else { 0 } + expected.key.len();
    assert_eq!(table.backend_interval().1, expected.start as usize + size);
}
fn check_coverage(view: &FlowMediumBackendPrefix, case: &BackendCase) {
    let coverage = view.coverage();
    assert_eq!(coverage.backend_bytes, case.raw.len());
    assert_eq!(coverage.known_backend_bytes, case.end as usize);
    assert_eq!(coverage.unknown_bytes, case.tail.len());
    assert_eq!(coverage.backend_framing_complete, case.complete);
    assert!(!coverage.full_record_schema);
    assert_eq!(coverage.achieved_error, 0);
    let stop = match case.stop.as_slice() {
        b"backend_end" => FlowMediumBackendStop::EmptyTablesEnd,
        b"first_table_object_body" => FlowMediumBackendStop::FirstTableObjectBody,
        b"second_table_object_body" => FlowMediumBackendStop::SecondTableObjectBody,
        _ => panic!("unexpected frozen stop"),
    };
    assert_eq!(view.stop(), stop);
}

fn carrier(payload: &[u8]) -> FlowCarrier {
    let mut bytes = 8u16.to_le_bytes().to_vec();
    for (identifier, payload) in [
        (424u32, &b"decoy"[..]),
        (424, payload),
        (999, &b"opaque"[..]),
    ] {
        bytes.extend_from_slice(&identifier.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(payload);
    }
    FlowCarrier::from_bytes(
        &bytes,
        FlowLimits {
            maximum_wire_bytes: 2_000_000,
            maximum_payload_bytes: 1_000_000,
            maximum_records: 3,
        },
    )
    .unwrap()
}

#[test]
fn complete_native_outer_metadata_composes_with_backend_fields_and_exact_whole_document_reversals()
{
    let native = crate::fixture::prefix_cases().remove(0);
    let source = carrier(&native.raw);
    let id = source.latest(424).unwrap().unwrap().id;
    let prefix = source.record_424_metadata_prefix(&id, options()).unwrap();
    let backend = prefix.medium_backend_prefix().unwrap();
    assert_eq!(prefix.coverage().known_prefix_bytes, 112);
    assert_eq!(backend.coverage().known_backend_bytes, 26);
    assert_eq!(backend.backend_bytes().len(), 13079);
    check_read(&backend, &cases().remove(0));
    for cell in [
        edit::Cell::Count(0),
        edit::Cell::Kind(0),
        edit::Cell::Key(0),
    ] {
        edit::check_edit(&backend, cell);
    }
}
