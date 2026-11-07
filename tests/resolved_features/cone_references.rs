const CONE_BITS: [u64; 8] = [
    0,
    0x8000000000000000,
    0x7ff8000000001234,
    0x7ff0000000000042,
    0x7ff0000000000000,
    0xfff0000000000000,
    0x3ff0000000000000,
    0xbff0000000000000,
];

use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/cone-references/cases.bin"),
        b"CON1",
        162,
    )
}

#[test]
fn cone_surface_axis_regression_accepts_the_exact_derived_body() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 == 1 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn complete_cone_graphs_match_independent_fields_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn cones_reject_every_truncated_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn cone_suffix_is_eight_raw_doubles_without_cylinder_frame_mode() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let suffix = &fields[fields.len() - 8..];
        assert_eq!(super::encode_fields(suffix, 30000).unwrap().len(), 64);
        for (index, field) in suffix.iter().enumerate() {
            let super::ModelFieldValue::F64(value) = field.value else {
                panic!("expected cone double")
            };
            assert_eq!(value.bits(), CONE_BITS[(index + usize::from(kind / 6)) % 8]);
        }
        assert!(!fields.iter().any(|f| f.path.ends_with("frame_mode")));
    }
}

#[test]
fn surface_axis_references_reject_cached_unrelated_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 3) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[1].reference_80",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unrelated array rejection")
        };
        assert!(error.to_string().contains("reference_80"));
    }
}

fn wrapper_records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/cone-wrappers/cases.bin"),
        b"CWR1",
        90,
    )
}

pub(super) fn fresh_wrapper_class(kind: u8, path: &str) -> String {
    let (_, version, bytes, _, _) = wrapper_records()
        .into_iter()
        .find(|r| r.0 == kind * 6 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, false).unwrap().0;
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let super::ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected cone pointer")
    };
    let crate::archive_codec::ObjectRecord::New { class, .. } = &tag.record else {
        panic!("expected fresh cone")
    };
    assert_eq!(class.name, "moConeRef_w");
    class.name.clone()
}

#[test]
fn every_new_cone_wrapper_caller_matches_independent_fields_slots_and_bytes() {
    assert_complete_records(wrapper_records());
}

#[test]
fn cone_wrapper_callers_reject_truncations_trailing_bytes_and_old_contexts() {
    assert_rejected_boundaries(
        wrapper_records()
            .into_iter()
            .filter(|r| r.1 == 19000)
            .collect(),
    );
}
