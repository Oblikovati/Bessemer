use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ObjectRecord;
const PRESENCE: [i32; 6] = [0, i32::MIN, 1, -1, i32::MAX, 255];
const FLOAT_BITS: [u64; 6] = [
    0,
    0x8000000000000000,
    0x7ff8000000000042,
    0x7ff0000000000000,
    0xfff0000000000000,
    0xfff8000000001234,
];

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/projected-line-subtypes/cases.bin"),
        b"PLS1",
        390,
    )
}

fn data_prefix(kind: u8) -> String {
    let root = if matches!(kind % 6, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    };
    if kind / 6 < 2 {
        format!("{root}.data_808")
    } else {
        root.into()
    }
}

#[test]
fn projected_line_bodies_match_every_independent_primitive_map_slot_and_byte() {
    assert_complete_records(records());
}

#[test]
fn projected_line_bodies_reject_every_truncated_prefix_appended_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn subtype_minima_have_one_owner_one_presence_and_the_exact_concrete_suffix() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0 || r.0 == 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let index = fields
            .iter()
            .position(|f| f.path.as_ref() == "root.object.data_808")
            .unwrap();
        assert_eq!(
            super::encode_fields(&fields[index + 1..], 30000)
                .unwrap()
                .len(),
            16
        );
    }
}

#[test]
fn nullable_arrays_preserve_raw_signed_presence_and_direct_empty_or_populated_bodies() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 24) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let path = format!("{}.array_10.presence", data_prefix(kind));
        let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
        assert_eq!(
            field.value,
            ModelFieldValue::I32(PRESENCE[usize::from(kind % 6)])
        );
    }
}

#[test]
fn draft_double_payloads_retain_negative_zero_nan_payloads_and_infinities() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 / 6 == 0 || r.0 / 6 == 2)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let path = format!("{}.raw_18", data_prefix(kind));
        let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
        let ModelFieldValue::F64(value) = field.value else {
            panic!("{path}: expected f64")
        };
        assert_eq!(value.bits(), FLOAT_BITS[usize::from(kind % 6)]);
    }
}

fn reject_target(version: u32, fields: &[ModelField], path: &str, tag: u16) {
    let changed = super::manager_context_tests::replace_field(fields, path, &tag.to_le_bytes());
    assert!(
        matches!(decode_graph(version,&changed,false),Err(e) if e.class==crate::FailureClass::Invalid)
    );
}

#[test]
fn required_parent_data_and_subtype_owners_reject_null_and_unrelated_cached_arrays() {
    for (kind, version, bytes, _, _) in records()
        .into_iter()
        .filter(|r| r.0 == 2 || r.0 == 8 || r.0 == 14 || r.0 == 20)
    {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for tag in [0, 2] {
            reject_target(
                version,
                &fields,
                &format!("{}.owner", data_prefix(kind)),
                tag,
            );
            if kind < 12 {
                reject_target(version, &fields, &data_prefix(kind), tag);
            }
        }
    }
}

#[test]
fn draft_direction_and_intersection_lists_keep_their_distinct_pointer_roles() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 2 && r.0 < 24) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let labels: &[&str] = if kind / 6 == 0 || kind / 6 == 2 {
            &["direction_20"]
        } else {
            &["list_20", "list_28", "list_30"]
        };
        for label in labels {
            reject_target(
                version,
                &fields,
                &format!("{}.{label}", data_prefix(kind)),
                2,
            );
        }
    }
}

fn object_record<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let value = &fields
        .iter()
        .find(|f| f.path.as_ref() == path)
        .unwrap()
        .value;
    let ModelFieldValue::ObjectTag(tag) = value else {
        panic!("{path}: expected tagged object")
    };
    &tag.record
}

#[test]
fn intersection_tagged_lists_share_actual_targets_without_becoming_direct_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 8 || r.0 == 20) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let prefix = data_prefix(kind);
        let ObjectRecord::New {
            index: first,
            class,
        } = object_record(&fields, &format!("{prefix}.list_20"))
        else {
            panic!("expected fresh first list")
        };
        assert_eq!(class.name.as_str(), "suObList");
        let ObjectRecord::Reference { index: last, .. } =
            object_record(&fields, &format!("{prefix}.list_30"))
        else {
            panic!("expected cached third list")
        };
        assert_eq!(first, last);
    }
}

#[test]
fn impossible_direct_array_counts_fail_before_reading_entries() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 5 && r.0 < 24) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let path = format!(
            "{}.array_10.array.items.count.count_dword",
            data_prefix(kind)
        );
        let changed =
            super::manager_context_tests::replace_field(&fields, &path, &u32::MAX.to_le_bytes());
        assert!(decode_graph(version, &changed, false).is_err());
    }
}

#[test]
fn fresh_base_surface_bodies_work_through_the_existing_typed_axis_surface_role() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 28) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let path = "root.object.axis_data.reference_80";
        let ObjectRecord::New { class, .. } = object_record(&fields, path) else {
            panic!("expected fresh base surface")
        };
        assert_eq!(class.name.as_str(), "moSurfaceRef_w");
        reject_target(version, &fields, path, 2);
    }
}
