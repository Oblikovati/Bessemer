use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};
use super::{ModelFieldValue, encode_fields};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/sheet-folds/cases.bin"),
        b"SFD1",
        168,
    )
}

#[test]
fn complete_fold_families_match_independent_primitives_maps_and_kernel_bytes() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn fold_families_reject_every_truncated_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn minimal_suffixes_keep_exact_direct_collection_and_fixed_transform_sizes() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let payload: Vec<_> = fields
            .into_iter()
            .filter(|f| f.path.starts_with("root.object.payload."))
            .collect();
        let expected = match kind / 6 {
            0 => 58,
            1 | 2 => 12,
            _ => continue,
        };
        assert_eq!(encode_fields(&payload, 30000).unwrap().len(), expected);
        if kind / 6 == 2 {
            check_minimal_unfold_suffix(version, bytes);
        }
    }
}

fn check_minimal_unfold_suffix(version: u32, bytes: &[u8]) {
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let suffix: Vec<_> = fields
        .into_iter()
        .filter(|f| f.path.starts_with("root.object.unfold_payload."))
        .collect();
    assert_eq!(encode_fields(&suffix, 30000).unwrap().len(), 136);
}

#[test]
fn unfolds_preserve_negative_empty_body_counts_without_kernel_context() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 12 | 13)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let value = &fields
            .iter()
            .find(|f| f.path.ends_with("declared_body_count"))
            .unwrap()
            .value;
        assert_eq!(
            *value,
            ModelFieldValue::I32(if kind == 12 { i32::MIN } else { -1 })
        );
    }
}

#[test]
fn positive_unfold_body_counts_require_explicit_kernel_schema() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 15) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        assert_eq!(
            fields
                .iter()
                .filter(|f| matches!(f.value, ModelFieldValue::Kernel(_)))
                .count(),
            1
        );
        let Err(error) = decode_graph(version, bytes, false) else {
            panic!("expected missing unfold kernel schema")
        };
        assert_eq!(error.class, crate::FailureClass::Missing);
    }
}

#[test]
fn object_integer_maps_keep_repeated_keys_and_signed_wire_values() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 == 19) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let values: Vec<_> = fields
            .iter()
            .filter(|f| f.path.ends_with(".value"))
            .map(|f| f.value.clone())
            .collect();
        assert_eq!(
            values,
            [
                ModelFieldValue::I32(i32::MIN),
                ModelFieldValue::I32(i32::MAX),
                ModelFieldValue::I32(-1)
            ]
        );
        assert_eq!(slots, 5);
    }
}

#[test]
fn special_fixed_transform_kinds_stop_instead_of_using_compact_framing() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 12) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.unfold_payload.transform_8c0.words[28]",
            &0x40u32.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unsupported unfold transform kind")
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
        assert!(error.to_string().contains("0x00000040"));
    }
}

#[test]
fn fold_boundary_roles_reject_unrelated_cached_root_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 8) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].payload.boundary_820",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated fold boundary rejection")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}

#[test]
fn unfold_and_map_counts_reject_impossible_extents_before_iteration() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 12) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        reject_unbounded_count(
            version,
            &fields,
            "root.object.unfold_payload.declared_body_count",
            &i32::MAX.to_le_bytes(),
        );
        reject_unbounded_count(
            version,
            &fields,
            "root.object.unfold_payload.object_int_map_890.entries.count.count_word",
            &[255; 6],
        );
    }
}

fn reject_unbounded_count(
    version: u32,
    fields: &[super::ModelField],
    path: &str,
    replacement: &[u8],
) {
    let changed = super::manager_context_tests::replace_field(fields, path, replacement);
    let Err(error) = decode_graph(version, &changed, true) else {
        panic!("expected bounded count rejection at {path}")
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.to_string().contains("count"));
}
