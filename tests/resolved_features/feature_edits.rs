use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/feature-edits/cases.bin"),
        b"FED1",
        162,
    )
}

#[test]
fn complete_feature_edits_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn feature_edits_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn bending_requires_nonnull_data_even_when_the_matrix_helper_is_null() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 6 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.data_810",
        &0_u16.to_le_bytes(),
    );
    let Err(error) = decode_graph(version, &changed, true) else {
        panic!("null bending data was accepted")
    };
    assert!(error.to_string().contains("nonnull"));
}

#[test]
fn move_face_typed_slots_reject_an_unrelated_cached_root() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 1 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    for label in [
        "direction_818",
        "line_830",
        "list_808",
        "dimension_840",
        "reference_ad0",
        "point_reference_ad8",
        "direction_af8",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{label}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
}

#[test]
fn bending_packed_word_retains_every_signed_value_without_migration() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 >= 12) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let value = fields
            .iter()
            .find(|f| f.path.ends_with("packed_80_e8_ec"))
            .unwrap();
        assert_eq!(
            value.value,
            super::ModelFieldValue::I32(
                [i32::MIN, i32::MAX, -1, 0, -32769, 65535][usize::from(kind % 6)]
            )
        );
    }
}

#[test]
fn bending_direct_array_rejects_an_impossible_extended_count() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 12 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.array_a0.count.count_word",
        &[255; 6],
    );
    assert!(decode_graph(version, &changed, true).is_err());
}

#[test]
fn move_face_transform_retains_compact_presence_flags_and_negative_zero() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let field = fields
            .iter()
            .find(|f| f.path.ends_with("payload.transform_928"))
            .unwrap();
        let super::ModelFieldValue::Transform(transform) = &field.value else {
            panic!("move-face transform did not retain compact framing")
        };
        let style = usize::from(kind);
        assert_eq!(transform.has_matrix, [0, 0, 255, 255, 0, 255][style]);
        assert_eq!(transform.flags, [0, 7, 0x1a, 0x2b, 0x4c, 0x8d][style]);
        assert_eq!(transform.translation[0].bits(), (-0_f64).to_bits());
    }
}

#[test]
fn bending_stock_slot_reads_complete_reviewed_subtypes() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 >= 12 && r.1 == 19000)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let field = fields
            .iter()
            .find(|f| f.path.ends_with("stock_128"))
            .unwrap();
        let super::ModelFieldValue::ObjectTag(tag) = &field.value else {
            panic!("stock slot lost tagged framing")
        };
        let expected = [
            None,
            Some("moStock_c"),
            Some("moXformStock_c"),
            Some("moMirrorStock_c"),
            None,
            Some("moStock_c"),
        ][usize::from(kind % 6)];
        match (&tag.record, expected) {
            (crate::archive_codec::ObjectRecord::Null, None) => {}
            (crate::archive_codec::ObjectRecord::New { class, .. }, Some(name)) => {
                assert_eq!(class.name, name)
            }
            _ => panic!("stock slot did not retain its reviewed concrete subtype"),
        }
    }
}
