use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/sculpt-features/cases.bin"),
        b"SCP1",
        450,
    )
}

#[test]
fn sculpt_records_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn sculpt_records_reject_every_prefix_extra_byte_and_unselected_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn sculpt_nonpositive_group_counts_preserve_native_empty_loops() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 1 | 4)) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let count = fields
            .iter()
            .find(|f| f.path.ends_with("payload.dword_array_count"))
            .unwrap();
        assert_eq!(
            count.value,
            ModelFieldValue::I32(if kind == 1 { -1 } else { i32::MIN })
        );
        assert!(!fields.iter().any(|f| f.path.contains("dword_arrays_960[")));
    }
}

#[test]
fn sculpt_group_counts_reject_impossible_work_before_iteration() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 2 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let changed = super::manager_context_tests::replace_field(
        &fields,
        "root.object.payload.dword_array_count",
        &i32::MAX.to_le_bytes(),
    );
    assert!(decode_graph(version, &changed, true).is_err());
}

fn right_identifier_index(fields: &[super::ModelField]) -> u16 {
    fields
        .iter()
        .find_map(|f| match &f.value {
            ModelFieldValue::ObjectTag(tag) => match &tag.record {
                ObjectRecord::New { index, class } if class.name == "moRightSculptEdgeIdRep_c" => {
                    Some(u16::try_from(index.value()).unwrap())
                }
                _ => None,
            },
            _ => None,
        })
        .unwrap()
}

#[test]
fn sculpt_chooser_owner_rejects_a_cached_identifier() {
    let (_, version, bytes, _, _) = records()
        .into_iter()
        .find(|r| r.0 == 2 && r.1 == 19000)
        .unwrap();
    let fields = decode_graph(version, bytes, true).unwrap().0;
    let index = right_identifier_index(&fields);
    for label in ["chooser_820", "chooser_8a8"] {
        let changed = super::manager_context_tests::replace_field(
            &fields,
            &format!("root.object.payload.{label}.owner"),
            &index.to_le_bytes(),
        );
        assert!(decode_graph(version, &changed, true).is_err());
    }
}
