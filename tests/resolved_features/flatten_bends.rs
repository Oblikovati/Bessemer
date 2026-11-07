use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records_with_kernel, assert_rejected_boundaries_with_kernel,
    records_with_count,
};
use super::{ModelField, ModelFieldValue, encode_fields};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/flatten-bends/cases.bin"),
        b"FBN1",
        84,
    )
}

fn root(style: u8) -> &'static str {
    if matches!(style, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    }
}

fn value<'a>(fields: &'a [ModelField], path: &str) -> &'a ModelFieldValue {
    &fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap_or_else(|| panic!("missing {path}; expected complete flatten field"))
        .value
}

#[test]
fn complete_flatten_process_graphs_match_independent_fields_frames_and_slots() {
    assert_complete_records_with_kernel(records(), true);
}

#[test]
fn flatten_process_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries_with_kernel(records(), true);
}

#[test]
fn containing_node_bit_selects_presence_without_using_unrelated_nonzero_flags() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        check_presence_gate(
            &fields,
            root(kind),
            [0, 1, 0x800, 0x40000800, 0x80000001, u32::MAX][kind as usize],
        );
    }
}

fn check_presence_gate(fields: &[ModelField], root: &str, flags: u32) {
    assert_eq!(
        *value(
            fields,
            &format!("{root}.body.model_feature.feature.node.flags_28")
        ),
        ModelFieldValue::U32(flags)
    );
    let path = format!("{root}.payload.body_presence_8c0");
    assert_eq!(
        fields.iter().any(|field| field.path.as_ref() == path),
        flags & 0x800 != 0
    );
}

#[test]
fn flag_clear_and_absent_body_suffixes_have_four_and_eight_bytes() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| matches!(r.0, 0 | 1 | 2 | 4))
    {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let prefix = format!("{}.payload.", root(kind));
        let suffix: Vec<_> = fields
            .into_iter()
            .filter(|field| field.path.starts_with(&prefix))
            .collect();
        assert_eq!(
            encode_fields(&suffix, 30000).unwrap().len(),
            if kind == 2 { 8 } else { 4 }
        );
    }
}

#[test]
fn process_minima_have_only_one_pointer_and_one_word() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let suffix: Vec<_> = fields
            .into_iter()
            .filter(|field| field.path.starts_with("root.object.payload."))
            .collect();
        assert_eq!(suffix.len(), 2);
        assert_eq!(encode_fields(&suffix, 30000).unwrap().len(), 6);
    }
}

#[test]
fn any_nonzero_signed_presence_reads_a_raw_body_without_suppression() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 3 | 5)) {
        let fields = decode_graph(version, bytes, true).unwrap().0;
        let prefix = root(kind);
        assert_eq!(
            *value(&fields, &format!("{prefix}.payload.body_presence_8c0")),
            ModelFieldValue::I32(if kind == 3 { -1 } else { i32::MIN })
        );
        assert!(matches!(
            value(&fields, &format!("{prefix}.payload.body_8c0.frame")),
            ModelFieldValue::Kernel(_)
        ));
        let Err(error) = decode_graph(version, bytes, false) else {
            panic!("expected missing raw-body kernel context")
        };
        assert_eq!(error.class, crate::FailureClass::Missing);
    }
}

#[test]
fn process_fixed_face_role_rejects_an_unrelated_cached_array() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 8) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.items[0].payload.fixed_face_888",
            &2u16.to_le_bytes(),
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected typed fixed-face rejection")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
    }
}
