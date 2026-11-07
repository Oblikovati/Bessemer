use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

const BITS: [[u64; 4]; 3] = [
    [
        0x8000000000000000,
        0x3ff4000000000000,
        0xc004000000000000,
        0x8000000000000000,
    ],
    [
        0x7ff0000000000042,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000001234,
    ],
    [
        1,
        0x7fefffffffffffff,
        0x8000000000000001,
        0x3fc5555555555555,
    ],
];
fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/display-welds/cases.bin"),
        b"DWL1",
        162,
    )
}

#[test]
fn native_display_weld_layout_accepts_complete_inherited_and_derived_fields() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 == 0 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn every_display_weld_graph_matches_independent_primitives_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn display_welds_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn weld_payload_retains_exact_ansi_unicode_and_long_cstring_extents() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let suffix = &fields[fields.len() - 7..];
        assert!(
            suffix
                .iter()
                .all(|f| f.path.starts_with("root.object.payload."))
        );
        let encoded = super::encode_fields(suffix, 30000).unwrap();
        let length = 36 + [4, 24, 526][usize::from(kind / 6)] + 2;
        assert_eq!(encoded.len(), length);
        assert_eq!(encoded, &bytes[bytes.len() - length..]);
    }
}

#[test]
fn weld_numeric_suffix_retains_signed_zero_nan_infinity_subnormals_and_signed_words() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let suffix = &fields[fields.len() - 7..];
        for (index, field) in suffix[..4].iter().enumerate() {
            let ModelFieldValue::F64(value) = field.value else {
                panic!("expected raw weld double")
            };
            assert_eq!(value.bits(), BITS[usize::from(kind / 6)][index]);
        }
        assert_eq!(
            suffix[4].value,
            ModelFieldValue::I32([i32::MIN, i32::MAX, -1][usize::from(kind / 6)])
        );
    }
}

#[test]
fn display_welds_remain_distinct_from_display_dimension_layouts_and_roles() {
    assert!(super::display_dimension::DisplayDimensionKind::parse("moDisplayWeld_c").is_none());
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert!(
            !fields
                .iter()
                .any(|f| f.path.starts_with("root.object.dimension"))
        );
        let ModelFieldValue::ObjectTag(tag) = &fields[0].value else {
            panic!("expected weld tag")
        };
        let role = super::ModelObjectRole::Declared(super::display_dimension::CLASSES);
        assert!(super::objects::require_role(&tag.record, role, "dimension").is_err());
    }
}

#[test]
fn inherited_component_roles_reject_the_unrelated_shared_payload_object() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].item.component_feature",
            &4u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated component rejection")
        };
        assert!(error.to_string().contains("component_feature"));
    }
}
