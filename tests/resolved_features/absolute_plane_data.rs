use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/absolute-plane-data/cases.bin"),
        b"APL1",
        108,
    )
}

pub(super) fn containing_plane_data_class() -> String {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 8 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let pointer = fields
        .iter()
        .find(|f| f.path.as_ref() == "root.object.items[0].plane_data")
        .unwrap();
    let super::ModelFieldValue::ObjectTag(tag) = &pointer.value else {
        panic!("expected complete containing-plane data tag")
    };
    let crate::archive_codec::ObjectRecord::New { class, .. } = &tag.record else {
        panic!("expected fresh absolute plane-data class")
    };
    class.name.clone()
}

#[test]
fn absolute_planes_and_typed_parents_match_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}

#[test]
fn absolute_planes_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn absolute_plane_data_uses_the_nondefault_base_without_a_suffix() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let expected = 259 + usize::from(version > 0x2b83) * 2 + usize::from(version > 0x2ba5) * 4;
        assert_eq!(
            super::encode_fields(&fields[1..], 30000).unwrap().len(),
            expected
        );
        assert!(!fields.iter().any(|f| f.path.contains("display_root_point")));
        assert_eq!(slots, 3);
    }
}

#[test]
fn typed_plane_data_pointers_still_reject_unrelated_cached_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 8) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].plane_data",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated cached plane-data rejection")
        };
        assert!(error.to_string().contains("plane_data"));
    }
}
