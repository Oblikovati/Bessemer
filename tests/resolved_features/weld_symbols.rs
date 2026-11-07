use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue, ModelObjectRole};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/weld-symbols/cases.bin"),
        b"WSY1",
        216,
    )
}

#[test]
fn native_weld_symbol_subtypes_accept_their_complete_typed_parents() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 % 6 == 1 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn every_symbol_graph_matches_independent_primitives_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn symbol_graphs_reject_truncated_prefixes_extra_bytes_and_old_contexts() {
    assert_rejected_boundaries(records().into_iter().filter(|r| r.1 == 19000).collect());
}

#[test]
fn empty_symbol_extents_match_native_closed_forms_without_direct_text_map_slots() {
    for (kind, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let expected = match kind / 6 {
            0 => 503,
            1 => 543,
            2 => 190 + usize::from(version > 0x42de) * 12 + usize::from(version > 0x46bf) * 10,
            3 => 113,
            _ => panic!("expected symbol kind"),
        };
        assert_eq!(
            super::encode_fields(&fields[1..], 30000).unwrap().len(),
            expected
        );
        assert_eq!(slots, 3);
    }
}

fn object_tag<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected object tag")
    };
    &tag.record
}

#[test]
fn new_and_cached_native_subtypes_keep_exact_detail_and_attachment_families() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 % 6 == 2 && r.1 == 19000)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let (expected, wrong) = if kind / 6 == 3 {
            ("moAttachPoint_c", "moDetailItem_c")
        } else {
            ("moDetailItem_c", "moAttachPoint_c")
        };
        for path in ["root.object.items[0]", "root.object.items[2]"] {
            let tag = object_tag(&fields, path);
            assert!(accepts(tag, &[expected]));
            assert!(!accepts(tag, &[wrong]));
            assert!(!accepts(tag, super::display_dimension::CLASSES));
        }
    }
}

fn accepts(tag: &ObjectRecord, names: &[&'static str]) -> bool {
    super::objects::require_role(tag, ModelObjectRole::Declared(names), "symbol role").is_ok()
}

#[test]
fn surface_finish_pointers_reject_their_unrelated_containing_array() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].surface_finish_178",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated finish rejection")
        };
        assert!(error.to_string().contains("surface_finish_178"));
    }
}

#[test]
fn cosmetic_attachment_feature_roles_reject_unrelated_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 21) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].feature_108",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated feature rejection")
        };
        assert!(error.to_string().contains("feature_108"));
    }
}

fn first_font_identity(fields: &[ModelField]) -> crate::archive_codec::ArchiveMapIndex {
    fields
        .iter()
        .find_map(|field| {
            let ModelFieldValue::ObjectTag(tag) = &field.value else {
                return None;
            };
            let ObjectRecord::New { index, class } = &tag.record else {
                return None;
            };
            (class.name == "utCharFormat_c").then_some(*index)
        })
        .unwrap()
}

#[test]
fn three_direct_weld_texts_share_actual_seeded_fonts_without_outer_object_tags() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| [3, 9].contains(&r.0) && r.1 == 19000)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let font = first_font_identity(&fields);
        let root = if kind / 6 == 1 {
            "root.object.items[1].weld"
        } else {
            "root.object.items[1]"
        };
        assert_direct_font(&fields, root, font);
    }
}

#[test]
fn weld_raw_scalar_bits_keep_signaling_nan_payloads_and_signed_zero() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let root = "root.object.items[1]";
        for (suffix, bits) in RAW_BITS {
            assert_double(&fields, &format!("{root}.{suffix}"), bits);
        }
    }
}

fn assert_direct_font(
    fields: &[ModelField],
    root: &str,
    font: crate::archive_codec::ArchiveMapIndex,
) {
    for offset in ["188", "2e0", "438"] {
        let path = format!("{root}.text_{offset}");
        assert!(!fields.iter().any(|f| f.path.as_ref() == path));
        let ObjectRecord::Reference { index, .. } =
            object_tag(fields, &format!("{path}.character_format"))
        else {
            panic!("expected cached font")
        };
        assert_eq!(*index, font);
    }
}

fn assert_double(fields: &[ModelField], path: &str, bits: u64) {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::F64(value) = field.value else {
        panic!("expected raw symbol double")
    };
    assert_eq!(value.bits(), bits);
}

const RAW_BITS: [(&str, u64); 4] = [
    ("raw_130", 0xfff0000000000000),
    ("vector_138[0]", 0x8000000000000000),
    ("vector_138[1]", 0x7ff0000000000042),
    ("vector_138[2]", 0x7ff0000000000000),
];
