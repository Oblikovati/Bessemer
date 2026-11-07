use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/move-copy-bodies/cases.bin"),
        b"MCB1",
        162,
    )
}

#[test]
fn complete_move_copy_graphs_match_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}

#[test]
fn move_copy_rejects_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn every_outer_and_payload_version_boundary_preserves_field_presence() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let has = |suffix: &str| fields.iter().any(|f| f.path.ends_with(suffix));
        if kind / 6 == 0 {
            assert_eq!(has("dimension_808"), version > 0x3b0b);
            assert_eq!(has("dimension_800"), version > 0x42cc);
        }
        if kind / 6 < 2 {
            assert_eq!(has("conformal_90.flags_1b8"), version > 0x2f69);
            assert_eq!(has("raw_484"), version > 0x3b0b);
            assert_eq!(has("raw_490"), version > 0x42cc);
        }
    }
}

#[test]
fn unsupported_multivectors_function_objects_and_fixed_kinds_fail_explicitly() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 6 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for (path, raw) in [
        (
            "root.object.conformal_90.multivector_00.scale",
            1_f64.to_bits().to_le_bytes().to_vec(),
        ),
        (
            "root.object.conformal_90.flags_1b8",
            0x100_u32.to_le_bytes().to_vec(),
        ),
        (
            "root.object.transform_08[28]",
            0x10_u32.to_le_bytes().to_vec(),
        ),
    ] {
        let changed = super::manager_context_tests::replace_field(&fields, path, &raw);
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unsupported documented branch")
        };
        assert_eq!(error.class, crate::FailureClass::Unsupported);
    }
}

#[test]
fn all_declared_move_copy_targets_reject_cached_unrelated_payloads() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 0 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for suffix in [
        "data_9c8",
        "matrix_helper_9d8",
        "dimension_808",
        "dimension_800",
    ] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{suffix}"),
            &2_u16.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, false).is_err());
    }
}

#[test]
fn both_optional_multivectors_reject_nonzero_and_nan_scales() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 7 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    for offset in ["30", "60"] {
        for bits in [1_f64.to_bits(), 0x7ff8000000000042] {
            let path = format!("root.object.conformal_90.multivector_{offset}.scale");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &bits.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected rejected nonzero or NaN optional multivector")
            };
            assert_eq!(error.class, crate::FailureClass::Unsupported);
        }
    }
}
