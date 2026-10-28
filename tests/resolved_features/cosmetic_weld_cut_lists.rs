use super::ModelFieldValue;
use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/cosmetic-weld-cut-lists/cases.bin"),
        b"CWL1",
        132,
    )
}

#[test]
fn complete_cut_lists_match_independent_fields_bytes_and_object_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn cut_lists_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn five_signed_words_follow_native_wire_order_and_add_no_tags() {
    for (kind, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let prefix = if matches!(kind % 6, 2 | 5) {
            "root.object.items[0]"
        } else {
            "root.object"
        };
        assert_suffix(&fields, prefix, usize::from(kind % 6));
    }
}

fn assert_suffix(fields: &[super::ModelField], prefix: &str, style: usize) {
    let offsets = ["888", "780", "778", "784", "788"];
    let words = [i32::MIN, i32::MAX, -1, 0, -32769, 65535];
    let suffix_prefix = format!("{prefix}.raw_");
    let selected: Vec<&super::ModelField> = fields
        .iter()
        .filter(|f| f.path.starts_with(&suffix_prefix))
        .collect();
    assert_eq!(selected.len(), 5);
    for (index, field) in selected.into_iter().enumerate() {
        assert_eq!(
            field.path.as_ref(),
            format!("{prefix}.raw_{}", offsets[index])
        );
        assert_eq!(
            field.value,
            ModelFieldValue::I32(words[(style + index) % 6])
        );
    }
}

#[test]
fn present_wire_bodies_require_the_explicit_kernel_schema() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 >= 6) {
        let Err(error) = decode_graph(version, bytes, false) else {
            panic!("expected missing kernel context rejection")
        };
        assert!(error.to_string().contains("kernel"));
    }
}

#[test]
fn inherited_cut_list_owners_still_reject_unrelated_cached_arrays() {
    for (_, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 % 6 == 5 && r.1 == 19000)
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let owner = fields
            .iter()
            .find(|f| f.path.ends_with("chooser_4a0.body.owner"))
            .unwrap();
        let changed =
            super::manager_context_tests::replace_field(&fields, &owner.path, &2u16.to_le_bytes());
        let Err(error) = decode_graph(version, &changed, true) else {
            panic!("expected unrelated cut-list owner rejection")
        };
        assert!(error.to_string().contains("owner"));
        assert!(error.to_string().contains("suObArray"));
    }
}
