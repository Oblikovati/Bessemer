use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};
use super::{ModelField, ModelFieldValue};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/cosmetic-weld-tree/cases.bin"),
        b"CWT1",
        324,
    )
}

fn prefix(style: u8) -> &'static str {
    match style {
        2 => "root.object.items[0]",
        3 | 5 => "root.object.items[1]",
        _ => "root.object",
    }
}

#[test]
fn complete_weld_tree_graphs_match_independent_fields_bytes_and_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn weld_tree_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn minimal_data_and_parameters_have_exact_widths_without_phantom_slots() {
    for (kind, version, bytes, _, slots) in
        records().into_iter().filter(|r| r.0 >= 12 && r.0 % 6 == 0)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        assert_eq!(
            super::encode_fields(&fields[1..], 30000).unwrap().len(),
            [113, 20, 24, 12][usize::from(kind / 6 - 2)]
        );
        assert_eq!(slots, 3);
    }
}

#[test]
fn negative_wire_map_counts_remain_signed_empty_lists() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 >= 30) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        assert_wire_counts(&fields, kind % 6);
    }
}

fn assert_wire_counts(fields: &[ModelField], style: u8) {
    let counts = [i32::MIN, -1, 0, 1, 3, 2];
    for (column, offset) in ["18", "28"].into_iter().enumerate() {
        let label = format!("{}.values_{offset}", prefix(style));
        let stored = counts[(usize::from(style) + column) % 6];
        assert_eq!(
            field(fields, &format!("{label}.count")).value,
            ModelFieldValue::I32(stored)
        );
        let entries = fields
            .iter()
            .filter(|f| f.path.starts_with(&format!("{label}[")))
            .count();
        assert_eq!(entries, usize::try_from(stored.max(0)).unwrap());
    }
}

fn field<'a>(fields: &'a [ModelField], path: &str) -> &'a ModelField {
    fields
        .iter()
        .find(|f| f.path.as_ref() == path)
        .unwrap_or_else(|| panic!("missing field {path}; expected complete weld-tree observation"))
}

#[test]
fn oversized_wire_map_counts_reject_before_reading_entries() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 30) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.values_18.count",
            &i32::MAX.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, true) else {
            panic!("expected impossible signed wire-map count rejection")
        };
        assert!(error.to_string().contains("count"));
    }
}

#[test]
fn nonnull_kernel_bodies_require_the_explicit_schema() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| matches!(r.0 / 6, 1 | 2) && matches!(r.0 % 6, 1 | 3 | 5))
    {
        let Err(error) = decode_graph(version, bytes, false) else {
            panic!("expected missing kernel schema rejection")
        };
        assert!(error.to_string().contains("kernel"));
    }
}

#[test]
fn typed_tree_pointers_reject_unrelated_cached_arrays() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 / 6 <= 2 && r.0 % 6 == 2 && r.1 == 19000)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        for label in pointer_labels(kind / 6) {
            let path = format!("root.object.items[0].{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, true) else {
                panic!("expected unrelated pointer rejection for {path}")
            };
            assert!(error.to_string().contains(label));
        }
    }
}

fn pointer_labels(kind: u8) -> &'static [&'static str] {
    match kind {
        0 => &["properties_350"],
        1 => &["data_6b8", "display_6c0"],
        _ => &["from_to_1c0", "intermittent_1c8", "symbol_1d0"],
    }
}

#[test]
fn endpoint_and_parameter_nan_payloads_remain_exact() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| matches!(r.0 / 6, 2..=4) && r.0 % 6 == 2)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let suffix = if kind / 6 == 2 {
            "endpoint_e8.raw_08"
        } else {
            "raw_10"
        };
        let ModelFieldValue::F64(value) =
            field(&fields, &format!("{}.{}", prefix(2), suffix)).value
        else {
            panic!("expected raw double")
        };
        assert_eq!(value.bits(), 0x7ff0000000000042);
    }
}
