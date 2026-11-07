use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/weldment-finishing/cases.bin"),
        b"WFF1",
        396,
    )
}

fn root_prefix(style: u8) -> &'static str {
    match style {
        2 => "root.object.items[0]",
        3 | 5 => "root.object.items[1]",
        _ => "root.object",
    }
}

#[test]
fn complete_finishing_graphs_match_all_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}

#[test]
fn finishing_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn gussets_include_the_complete_fourteen_byte_extrusion_suffix() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let suffix: Vec<_> = fields
            .iter()
            .filter(|f| {
                f.path
                    .starts_with("root.object.extrusion.extrusion_suffix.")
            })
            .cloned()
            .collect();
        assert_eq!(suffix.len(), 3);
        assert_eq!(super::encode_fields(&suffix, 30000).unwrap().len(), 14);
        assert_eq!(
            field(&fields, "root.object.extrusion.extrusion_suffix.word_808").value,
            ModelFieldValue::I32(i32::MIN)
        );
    }
}

#[test]
fn null_pointer_gusset_and_bead_payloads_have_exact_native_widths() {
    for (kind, version, bytes, _, slots) in records()
        .into_iter()
        .filter(|r| r.0 / 6 <= 1 && r.0 % 6 == 0)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let suffix: Vec<_> = fields
            .into_iter()
            .filter(|f| f.path.starts_with("root.object.payload."))
            .collect();
        assert_eq!(
            super::encode_fields(&suffix, 30000).unwrap().len(),
            if kind / 6 == 0 { 76 } else { 67 }
        );
        assert_eq!(slots, 5);
    }
}

#[test]
fn independent_side_data_has_twenty_two_bytes_and_no_inherited_fields() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 == 12) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(super::encode_fields(&fields[1..], 30000).unwrap().len(), 22);
        assert_eq!(fields.len(), 7);
        assert_eq!(slots, 3);
    }
}

fn field<'a>(fields: &'a [ModelField], path: &str) -> &'a ModelField {
    fields
        .iter()
        .find(|f| f.path.as_ref() == path)
        .unwrap_or_else(|| panic!("missing field {path}; expected complete finishing observation"))
}

#[test]
fn packed_runtime_words_keep_all_upper_bits() {
    let packed = [0, u32::MAX, 0x80000000, 3, 2, 1];
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 / 6 <= 1) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let root = root_prefix(kind % 6);
        let label = packed_label(kind / 6);
        assert_packed(
            &fields,
            &format!("{root}.payload.{label}"),
            packed[usize::from(kind % 6)],
        );
        if kind / 6 == 0 {
            assert_packed(
                &fields,
                &format!("{root}.payload.packed_884_888"),
                packed[usize::from((kind + 1) % 6)],
            );
        }
    }
}

fn packed_label(kind: u8) -> &'static str {
    match kind {
        0 => "packed_890_88c",
        _ => "packed_840_844",
    }
}

fn assert_packed(fields: &[ModelField], path: &str, expected: u32) {
    let actual = &field(fields, path).value;
    assert_eq!(*actual, ModelFieldValue::U32(expected));
}

const GUSSET_POINTERS: &[&str] = &[
    "list_858",
    "list_860",
    "dimension_8d8",
    "dimension_898",
    "dimension_8a0",
    "dimension_8a8",
    "dimension_8b0",
    "dimension_8b8",
    "dimension_8c0",
    "dimension_8c8",
    "dimension_8d0",
    "reference_908",
];
const BEAD_POINTERS: &[&str] = &[
    "display_850",
    "cabinet_860",
    "side_828",
    "side_830",
    "list_838",
    "dimension_800",
    "dimension_808",
    "dimension_810",
    "dimension_818",
    "dimension_820",
];

fn pointer_labels(kind: u8) -> &'static [&'static str] {
    match kind {
        0 => GUSSET_POINTERS,
        1 => BEAD_POINTERS,
        _ => &["list_30", "list_38", "list_40"],
    }
}

#[test]
fn typed_finishing_pointers_reject_unrelated_cached_arrays() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 % 6 == 2 && r.1 == 19000)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let scope = pointer_scope(kind / 6);
        for label in pointer_labels(kind / 6) {
            let path = format!("{scope}.{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected unrelated cached-array rejection for {path}")
            };
            assert!(error.to_string().contains(label));
        }
    }
}

fn pointer_scope(kind: u8) -> &'static str {
    match kind {
        2 => "root.object.items[0]",
        _ => "root.object.items[0].payload",
    }
}

#[test]
fn ordinary_bead_loading_requires_the_serialized_cabinet_field() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.payload.cabinet_860",
            &[],
        );
        assert!(decode_graph(version, &changed, false).is_err());
    }
}

fn object<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let ModelFieldValue::ObjectTag(tag) = &field(fields, path).value else {
        panic!("field {path}; expected object tag")
    };
    &tag.record
}

#[test]
fn both_bead_sides_share_one_complete_side_data_object() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 7) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let ObjectRecord::New { index, .. } = object(&fields, "root.object.payload.side_828")
        else {
            panic!("expected fresh first side")
        };
        let ObjectRecord::Reference { index: cached, .. } =
            object(&fields, "root.object.payload.side_830")
        else {
            panic!("expected cached second side")
        };
        assert_eq!(index, cached);
    }
}
