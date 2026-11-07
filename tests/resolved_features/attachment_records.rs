use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelFieldValue, ModelObjectRole};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/attachment-records/cases.bin"),
        b"ATR1",
        60,
    )
}

#[test]
fn native_parametric_point_accepts_complete_typed_detail_attachment_parent() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 == 7 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn all_attachment_graphs_match_independent_fields_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn attachment_graphs_reject_all_truncations_extra_bytes_and_old_contexts() {
    assert_rejected_boundaries(records());
}

#[test]
fn data_is_sixty_six_bytes_and_parametric_is_one_hundred_twenty_seven_without_new_slots() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            super::encode_fields(&fields[1..], 30000).unwrap().len(),
            if kind / 6 == 0 { 66 } else { 127 }
        );
        assert_eq!(slots, 3);
        assert_eq!(fields.len(), if kind / 6 == 0 { 10 } else { 21 });
    }
}

#[test]
fn only_parametric_points_belong_to_the_attachment_point_family() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for path in ["root.object.items[0]", "root.object.items[2]"] {
            let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
            let ModelFieldValue::ObjectTag(tag) = &field.value else {
                panic!("expected attachment tag")
            };
            let valid = accepts_attachment(&tag.record);
            assert_eq!(valid, kind / 6 == 1);
        }
    }
}

fn accepts_attachment(record: &ObjectRecord) -> bool {
    super::objects::require_role(
        record,
        ModelObjectRole::Declared(&["moAttachPoint_c"]),
        "attachment",
    )
    .is_ok()
}

#[test]
fn drawing_view_data_pointers_reject_unrelated_cached_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].view_38",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated view rejection")
        };
        assert!(error.to_string().contains("view_38"));
    }
}

#[test]
fn fresh_and_cached_generic_parametric_references_keep_the_actual_seed() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 9) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let tag = |path: &str| fields.iter().find(|f| f.path.as_ref() == path).unwrap();
        let ModelFieldValue::ObjectTag(seed) = &tag("root.object.items[0]").value else {
            panic!("expected seed")
        };
        let ModelFieldValue::ObjectTag(reference) =
            &tag("root.object.items[1].reference_118").value
        else {
            panic!("expected reference")
        };
        assert_seed_identity(&seed.record, &reference.record);
    }
}

fn assert_seed_identity(seed: &ObjectRecord, reference: &ObjectRecord) {
    let (ObjectRecord::New { index: left, .. }, ObjectRecord::Reference { index: right, .. }) =
        (seed, reference)
    else {
        panic!("expected actual cached seed")
    };
    assert_eq!(left, right);
}
