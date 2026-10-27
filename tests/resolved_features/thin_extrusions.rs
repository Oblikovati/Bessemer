use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/thin-extrusions/cases.bin"),
        b"THN1",
        360,
    )
}

fn root(kind: u8) -> &'static str {
    if matches!(kind % 6, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    }
}

fn spec_path(kind: u8) -> String {
    match kind / 6 {
        0 => root(kind).into(),
        4 => format!("{}.thin_specification_8e8", root(kind)),
        _ => format!("{}.thin_specification_860", root(kind)),
    }
}

fn tag<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(pointer) = &field.value else {
        panic!("{path}: expected tagged object")
    };
    &pointer.record
}

#[test]
fn all_thin_features_match_independent_primitives_slots_and_exact_bytes() {
    assert_complete_records(records());
}

#[test]
fn thin_suffix_graphs_reject_every_prefix_extra_byte_and_old_context() {
    // New suffixes have no version gates; inherited readers keep their exhaustive boundary corpora.
    let cases = records()
        .into_iter()
        .filter(|record| record.1 == 19000)
        .collect();
    assert_rejected_boundaries(cases);
}

#[test]
fn null_dimension_specifications_have_exactly_forty_two_body_bytes() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(super::encode_fields(&fields[1..], 30000).unwrap().len(), 42);
    }
}

fn rejected_target(fields: &[ModelField], version: u32, path: &str, pointer: u16) {
    let changed = super::manager_context_tests::replace_field(fields, path, &pointer.to_le_bytes());
    let Err(error) = decode_graph(version, &changed, false) else {
        panic!("{path}: expected rejected pointer {pointer}")
    };
    assert_eq!(error.class, crate::FailureClass::Invalid);
}

#[test]
fn thin_features_require_the_exact_nonnull_specification_class() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 / 6 > 0 && r.0 % 6 == 2)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for pointer in [0, 2] {
            rejected_target(&fields, version, &spec_path(kind), pointer);
        }
    }
}

#[test]
fn specification_dimension_roles_reject_cached_arrays_and_keep_radial_distinct() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for label in ["dimension_08", "dimension_10", "radial_dimension_48"] {
            rejected_target(&fields, version, &format!("{}.{label}", spec_path(kind)), 2);
        }
        let ObjectRecord::New { index, .. } = tag(&fields, "root.object.items[0].dimension_08")
        else {
            panic!("expected fresh distance dimension")
        };
        rejected_target(
            &fields,
            version,
            "root.object.items[0].radial_dimension_48",
            index.value() as u16,
        );
    }
}

#[test]
fn separate_feature_roots_retain_actual_shared_specification_identity() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 / 6 > 0 && r.0 % 6 == 2)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let first = spec_path(kind);
        let second = first.replace("items[0]", "items[1]");
        assert_shared_tag(&fields, &first, &second);
    }
}

fn assert_shared_tag(fields: &[ModelField], first: &str, second: &str) {
    let ObjectRecord::New {
        index: expected, ..
    } = tag(fields, first)
    else {
        panic!("expected fresh specification")
    };
    let ObjectRecord::Reference { index: actual, .. } = tag(fields, second) else {
        panic!("expected cached specification")
    };
    assert_eq!(expected, actual);
}

#[test]
fn bend_counts_are_plain_u16_and_triples_preserve_unsigned_bits_and_duplicates() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| (1..4).contains(&(r.0 / 6)))
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let expected = [0, 1, 3, 2, 0, 3][usize::from(kind % 6)];
        let prefix = format!("{}.bend_records_858", root(kind));
        assert_bend_count(&fields, &prefix, expected);
        if expected > 0 {
            assert_first_triple(&fields, &prefix);
        }
    }
}

fn assert_bend_count(fields: &[ModelField], prefix: &str, expected: u16) {
    let field = fields
        .iter()
        .find(|f| f.path.as_ref() == format!("{prefix}.count"))
        .unwrap();
    assert_eq!(field.value, ModelFieldValue::U16(expected));
}

fn assert_first_triple(fields: &[ModelField], prefix: &str) {
    for (column, word) in [u32::MAX, 0, 0x80000000].into_iter().enumerate() {
        let path = format!("{prefix}[0][{column}]");
        assert_eq!(
            fields
                .iter()
                .find(|f| f.path.as_ref() == path)
                .unwrap()
                .value,
            ModelFieldValue::U32(word)
        );
    }
}

#[test]
fn thin_revolutions_end_at_the_specification_without_a_bend_count() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 / 6 == 4) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert!(!fields.iter().any(|f| f.path.contains(".bend_records_858")));
    }
}

#[test]
fn impossible_bend_counts_reject_before_reading_any_triple() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| (1..4).contains(&(r.0 / 6)) && r.0 % 6 == 2)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let path = format!("{}.bend_records_858.count", root(kind));
        let changed =
            super::manager_context_tests::replace_field(&fields, &path, &u16::MAX.to_le_bytes());
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected rejected count 65535")
        };
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert!(error.cause.contains("thin bend records count 65535"));
    }
}

fn maximum_bend_limits() -> super::ModelFieldLimits {
    super::ModelFieldLimits {
        maximum_bytes: 786422,
        maximum_fields: 196606,
        maximum_entries: 65535,
        ..super::annotation_view_tests::LIMITS
    }
}

#[test]
fn all_65535_bend_triples_are_positive_not_an_escaped_count() {
    let bytes = &include_bytes!("../fixtures/thin-extrusions/max-bends.bin")[2..];
    let fields = super::direct_collection_tests::decode_limited(
        19000,
        bytes,
        maximum_bend_limits(),
        |reader, _| super::thin_extrusions::bend_records(reader),
    )
    .unwrap();
    assert_eq!(fields.len(), 196606);
    assert_eq!(fields[0].value, ModelFieldValue::U16(65535));
    assert_eq!(fields[1].value, ModelFieldValue::U32(u32::MAX));
    assert_eq!(fields[196605].value, ModelFieldValue::U32(7));
    assert_eq!(super::encode_fields(&fields, bytes.len()).unwrap(), bytes);
}

#[test]
fn large_positive_bends_enforce_byte_field_and_aggregate_budgets() {
    let bytes = &include_bytes!("../fixtures/thin-extrusions/max-bends.bin")[2..];
    for limit in [0, 1, 2] {
        let mut limits = maximum_bend_limits();
        match limit {
            0 => limits.maximum_bytes -= 1,
            1 => limits.maximum_fields -= 1,
            _ => limits.maximum_entries -= 1,
        }
        let result =
            super::direct_collection_tests::decode_limited(19000, bytes, limits, |reader, _| {
                super::thin_extrusions::bend_records(reader)
            });
        assert!(result.is_err());
    }
}
