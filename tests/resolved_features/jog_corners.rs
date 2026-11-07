use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue, encode_fields};

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/jog-corners/cases.bin"),
        b"JGC1",
        108,
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
        .unwrap_or_else(|| panic!("missing {path}; expected complete jog/corner field"))
        .value
}

#[test]
fn complete_jog_corner_graphs_match_independent_fields_bytes_and_map_slots() {
    assert_complete_records(records());
}

#[test]
fn jog_corner_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn minimal_jog_and_break_suffixes_have_exact_independent_widths() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 0 | 6)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let suffix: Vec<_> = fields
            .into_iter()
            .filter(|field| field.path.starts_with("root.object.payload."))
            .collect();
        let expected = if kind == 0 {
            109 + usize::from(version >= 0x3afe) * 2
        } else {
            66 + usize::from(version >= 0x46cb) * 4
        };
        assert_eq!(encode_fields(&suffix, 30000).unwrap().len(), expected);
    }
}

#[test]
fn every_nonzero_presence_byte_reads_both_optional_reference_tags() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        check_reference_presence(&fields, root(kind), [0, 1, 255, 128, 254, 0][kind as usize]);
    }
}

fn check_reference_presence(fields: &[ModelField], root: &str, presence: u8) {
    assert_eq!(
        *value(fields, &format!("{root}.payload.reference_presence")),
        ModelFieldValue::U8(presence)
    );
    for label in ["point_970", "reference_960"] {
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == format!("{root}.payload.{label}")),
            presence != 0
        );
    }
}

#[test]
fn stored_zero_jog_option_is_not_replaced_by_the_native_fresh_loaded_value() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(
            *value(&fields, "root.object.payload.raw_8f4"),
            ModelFieldValue::I32(0)
        );
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn final_boundary_and_final_word_follow_their_separate_version_gates() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 0 | 6)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let (label, present) = if kind == 0 {
            ("boundary_928", version >= 0x3afe)
        } else {
            ("raw_918", version >= 0x46cb)
        };
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == format!("root.object.payload.{label}")),
            present
        );
    }
}

#[test]
fn empty_keeper_is_six_direct_bytes_without_a_keeper_object_tag() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let keeper: Vec<_> = fields
            .iter()
            .filter(|field| field.path.starts_with("root.object.payload.keeper_888."))
            .cloned()
            .collect();
        assert_eq!(encode_fields(&keeper, 30000).unwrap().len(), 6);
        assert!(
            !fields
                .iter()
                .any(|field| field.path.as_ref() == "root.object.payload.keeper_888")
        );
        assert!(matches!(
            value(&fields, "root.object.payload.array_900"),
            ModelFieldValue::ObjectTag(_)
        ));
    }
}

#[test]
fn optional_point_and_wrapper_roles_reject_unrelated_cached_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for label in ["point_970", "reference_960"] {
            let path = format!("root.object.items[0].payload.{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected typed reference rejection at {path}")
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}

#[test]
fn direct_keeper_counts_reject_negative_and_unbounded_extents() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for count in [-1, i32::MAX] {
            let changed = super::manager_context_tests::replace_field(
                &fields,
                "root.object.payload.keeper_888.surface_ids.count",
                &count.to_le_bytes(),
            );
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected bounded keeper count {count}")
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}
