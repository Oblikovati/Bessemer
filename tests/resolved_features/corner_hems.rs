use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue, encode_fields};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/corner-hems/cases.bin"),
        b"CHM1",
        378,
    )
}

fn selected<'a>(fields: &'a [ModelField], path: &str) -> &'a ModelFieldValue {
    &fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap_or_else(|| panic!("missing {path}; expected complete hem field"))
        .value
}

fn scoped_size(fields: &[ModelField], prefix: &str) -> usize {
    let scoped: Vec<_> = fields
        .iter()
        .filter(|field| field.path.starts_with(prefix))
        .cloned()
        .collect();
    encode_fields(&scoped, 30000).unwrap().len()
}

#[test]
fn complete_corner_hem_graphs_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records(records());
}

#[test]
fn corner_hems_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn geometry_chain_and_relief_minima_have_exact_body_extents() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let expected = match kind / 6 {
            1 | 2 => 48,
            5 => 10,
            6 | 7 => 2,
            8 => 36,
            _ => continue,
        };
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(encode_fields(&fields[1..], 30000).unwrap().len(), expected);
    }
}

#[test]
fn direct_hem_data_and_corner_payloads_have_exact_minimal_widths() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 0 | 18)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            scoped_size(&fields, "root.object.payload."),
            if kind == 0 { 42 } else { 46 }
        );
        if kind == 18 {
            assert_eq!(scoped_size(&fields, "root.object.payload.data."), 40);
        }
    }
}

#[test]
fn stored_corner_zero_is_preserved_before_native_runtime_normalization() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            *selected(&fields, "root.object.payload.raw_9a0"),
            ModelFieldValue::I32(0)
        );
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn three_hem_arrays_have_independent_counts_and_no_tagged_owned_data() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 21) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for (label, count) in [("array_9a8", 3), ("data.array_20", 2), ("data.array_90", 1)] {
            assert_eq!(
                *selected(
                    &fields,
                    &format!("root.object.payload.{label}.count.count_word")
                ),
                ModelFieldValue::U16(count)
            );
        }
        assert!(
            !fields
                .iter()
                .any(|f| f.path.as_ref() == "root.object.payload.data")
        );
    }
}

#[test]
fn required_hem_owners_reject_null_in_direct_and_tagged_data() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 18 | 24)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let path = if kind == 18 {
            "root.object.payload.data.owner_08"
        } else {
            "root.object.owner_08"
        };
        let changed =
            super::manager_context_tests::replace_field(&fields, path, &0u16.to_le_bytes());
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected required owner failure at {path}")
        };
        assert!(error.to_string().contains("owner_08"));
    }
}

#[test]
fn chain_edge_and_profile_roles_reject_unrelated_cached_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 32 | 44)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let label = if kind == 32 { "profile_10" } else { "edge_08" };
        let path = format!("root.object.items[0].{label}");
        let changed =
            super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected chain-role rejection at {path}")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}

#[test]
fn impossible_chain_array_count_is_bounded_before_iteration() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 36) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.array_08.count.count_word",
            &[255; 6],
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected bounded chain array count")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}
