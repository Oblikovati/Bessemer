use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/display-notes/cases.bin"),
        b"NTE1",
        720,
    )
}

#[test]
fn complete_note_and_text_graphs_match_independent_primitives_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn notes_and_text_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn display_note_tail_fields_follow_four_independent_version_boundaries() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for (label, gate) in [
            ("raw_788", 0x2bbb),
            ("raw_78c", 0x2f77),
            ("thread_790", 0x2f9a),
            ("integers_6c0", 0x2fb7),
        ] {
            let path = format!("root.object.note.{label}");
            assert_eq!(
                fields.iter().any(|f| f.path.as_ref() == path),
                version > gate,
                "{kind}/{version:#x}/{label}"
            );
        }
        assert!(
            !fields
                .iter()
                .any(|f| f.path.starts_with("root.object.dimension"))
        );
    }
}

#[test]
fn character_records_retain_short_flags_and_surrogate_units() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 / 6 == 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for (label, expected) in [("raw_30", 65535), ("raw_10", 32768), ("raw_40", 0xd800)] {
            assert_eq!(
                fields
                    .iter()
                    .find(|f| f.path.as_ref() == format!("root.object.{label}"))
                    .unwrap()
                    .value,
                ModelFieldValue::U16(expected)
            );
        }
        assert_eq!(super::encode_fields(&fields[1..], 30000).unwrap().len(), 38);
    }
}

#[test]
fn invalid_note_chunk_counts_do_not_publish_partial_fields() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for count in [i32::MIN, -1, i32::MAX] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.note.flag_chunks_768.count",
            &count.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, false).is_err());
    }
}

#[test]
fn nonnull_note_components_reject_the_unverified_external_predicate() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 6 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.component_reference.component",
        &2_u16.to_le_bytes(),
    );
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("expected unsupported component branch")
    };
    assert_eq!(error.class, crate::FailureClass::Unsupported);
    assert!(error.cause.contains("expected null"));
}

#[test]
fn text_display_pointer_rejects_an_unrelated_cached_text_object() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 18 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.display_item_170",
        &2_u16.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, false).is_err());
}
