use super::encode_fields;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/ui-bends/cases.bin"),
        b"UIB1",
        126,
    )
}

#[test]
fn complete_ui_bends_and_line_records_match_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}

#[test]
fn ui_bends_and_lines_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn ruled_line_bodies_have_twelve_bytes_without_a_runtime_handle() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 == 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(encode_fields(&fields[1..], 30000).unwrap().len(), 12);
        assert_eq!(fields.len(), 3);
        assert_eq!(slots, 3);
    }
}

#[test]
fn sketch_line_bodies_cross_the_strict_late_word_gate() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 == 12) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            encode_fields(&fields[1..], 30000).unwrap().len(),
            if version > 0x42cb { 12 } else { 8 }
        );
        assert_eq!(
            fields.iter().any(|f| f.path.ends_with("raw_10")),
            version > 0x42cb
        );
        assert_eq!(slots, 3);
    }
}

#[test]
fn ordinary_ui_suffix_has_only_the_single_bend_pointer() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let payload: Vec<_> = fields
            .iter()
            .filter(|f| f.path.starts_with("root.object.payload."))
            .cloned()
            .collect();
        assert_eq!(payload.len(), 1);
        assert!(payload[0].path.ends_with("bend_808"));
        assert_eq!(encode_fields(&payload, 30000).unwrap().len(), 2);
    }
}

#[test]
fn ordinary_single_bend_role_rejects_an_unrelated_cached_array() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].payload.bend_808",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected typed single-bend reference rejection")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}
