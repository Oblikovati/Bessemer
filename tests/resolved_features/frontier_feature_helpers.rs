use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/frontier-feature-helpers/cases.bin"),
        b"FFH1",
        390,
    )
}

#[test]
fn frontier_helpers_match_all_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}

#[test]
fn frontier_helpers_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn frontier_helper_minima_have_native_byte_widths_without_fabricated_suffixes() {
    for (kind, _, bytes, _, slots) in records()
        .into_iter()
        .filter(|row| row.0 >= 12 && row.0 % 6 == 0)
    {
        let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        let expected = [17, 12, 4][usize::from(kind / 6 - 2)];
        assert_eq!(bytes.len() - 6 - name_length, expected);
        assert_eq!(slots, 3);
    }
}

#[test]
fn suffix_free_features_and_helpers_reject_unbound_class_spellings() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 % 6 == 0) {
        let end = 6 + usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        let mut changed = bytes.to_vec();
        changed[end - 1] ^= 1;
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("{kind}: expected unsupported near-match class spelling");
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}

const ITEM_BITS: [u64; 6] = [
    0,
    0x8000_0000_0000_0000,
    0x7ff0_0000_0000_0000,
    0xfff0_0000_0000_0000,
    0x7ff8_0000_0000_002a,
    0x7ff0_0000_0000_002a,
];

fn stored_field<'a>(fields: &'a [super::ModelField], suffix: &str) -> &'a ModelFieldValue {
    &fields
        .iter()
        .find(|field| field.path.ends_with(suffix))
        .unwrap()
        .value
}

#[test]
fn pattern_items_preserve_exact_float_bits_and_single_byte_flags() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|row| row.0 / 6 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let style = usize::from(kind % 6);
        let value = stored_field(&fields, ".raw_10");
        assert!(matches!(value, ModelFieldValue::F64(value) if value.bits() == ITEM_BITS[style]));
        assert_eq!(
            stored_field(&fields, ".raw_08"),
            &ModelFieldValue::U8([0, 1, 2, 128, 254, 255][style])
        );
    }
}

fn assert_invalid_pointer(version: u32, bytes: &[u8], path: &str) {
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let changed = super::manager_context_tests::replace_field(&fields, path, &2u16.to_le_bytes());
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("{path}: expected rejected cached array at typed helper pointer");
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
}

#[test]
fn edge_helper_dimension_and_edge_roles_reject_cached_unrelated_array() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 20) {
        for label in [
            "edge_radius.dimension",
            "edge_radius.edge",
            "dimension_28",
            "dimension_30",
        ] {
            assert_invalid_pointer(version, bytes, &format!("root.object.items[1].{label}"));
        }
    }
}

#[test]
fn radius_dimension_is_typed_while_object_pointer_accepts_containing_array() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|row| row.0 == 28) {
        assert_invalid_pointer(version, bytes, "root.object.items[1].dimension_08");
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].object_10",
            &2u16.to_le_bytes(),
        );
        let rebuilt = decode_graph(version, &changed, false).unwrap().0;
        assert_eq!(super::encode_fields(&rebuilt, 30000).unwrap(), changed);
    }
}
