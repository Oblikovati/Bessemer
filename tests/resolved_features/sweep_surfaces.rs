use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use crate::model_archives::ModelFieldValue;

const POINTER_LABELS: [&[&str]; 2] = [
    &[
        "payload.angle_bd8",
        "payload.angle_be0",
        "payload.edge_data[0]",
        "payload.radius_be8",
    ],
    &["endpoint_18", "endpoint_20", "endpoint_28"],
];

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/sweep-reference-surfaces/cases.bin"),
        b"SRS1",
        162,
    )
}

#[test]
fn sweep_surfaces_match_all_independent_primitives_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn sweep_surfaces_reject_every_truncated_prefix_appended_bytes_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn edge_and_keeper_minima_have_no_invented_feature_base_or_owner() {
    for (kind, version, bytes, _, slots) in
        records().into_iter().filter(|r| r.0 % 6 == 0 && r.0 >= 6)
    {
        let name_length = usize::from(u16::from_le_bytes([bytes[4], bytes[5]]));
        assert_eq!(bytes.len() - 6 - name_length, if kind < 12 { 6 } else { 0 });
        assert_eq!(slots, 3);
        assert_eq!(decode_graph(version, bytes, false).unwrap().1, 3);
    }
}

#[test]
fn surface_signed_counts_preserve_negative_values_without_consuming_edges() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 1 || r.0 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let path = "root.object.payload.edge_data_count";
        let count = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
        assert_eq!(
            count.value,
            ModelFieldValue::I32(if kind == 1 { -1 } else { i32::MIN })
        );
        assert!(
            !fields
                .iter()
                .any(|f| f.path.starts_with("root.object.payload.edge_data["))
        );
    }
}

#[test]
fn surface_edge_count_requires_full_i32_and_bounds_positive_sequences() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].payload.edge_data_count",
            &i32::MAX.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected bounded positive edge-data count")
        };
        assert_ne!(error.class, crate::FailureClass::Unsupported);
    }
}

#[test]
fn sweep_surface_typed_pointers_reject_unrelated_cached_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 2 && r.0 < 12) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for label in POINTER_LABELS[usize::from(kind >= 6)] {
            let path = format!("root.object.items[0].{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("{path}: expected rejected cached suObArray")
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}
