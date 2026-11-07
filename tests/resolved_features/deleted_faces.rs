use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ObjectRecord;
const PRESENCE: [i32; 6] = [0, i32::MIN, 1, -1, i32::MAX, 255];

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/deleted-faces/cases.bin"),
        b"DEL1",
        108,
    )
}

fn root(kind: u8) -> &'static str {
    if matches!(kind % 6, 2 | 5) {
        "root.object.items[0]"
    } else {
        "root.object"
    }
}

fn payload(kind: u8) -> String {
    if kind < 6 {
        format!("{}.payload", root(kind))
    } else {
        root(kind).into()
    }
}

#[test]
fn deleted_face_graphs_match_all_independent_fields_slots_and_bytes() {
    assert_complete_records(records());
}

#[test]
fn deleted_face_graphs_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn minimal_data_is_sixteen_bytes_without_an_owner_or_object_base_prefix() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_eq!(super::encode_fields(&fields[1..], 30000).unwrap().len(), 16);
        assert_eq!(fields.len(), 5);
        assert!(!fields.iter().any(|f| f.path.ends_with(".owner")));
    }
}

#[test]
fn minimal_feature_payload_is_thirty_bytes_and_omits_constructor_only_word_860() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let first = fields
            .iter()
            .position(|f| f.path.as_ref() == "root.object.payload.array_848.presence")
            .unwrap();
        assert_eq!(
            super::encode_fields(&fields[first..], 30000).unwrap().len(),
            30
        );
        assert!(!fields.iter().any(|f| f.path.ends_with(".raw_860")));
    }
}

fn assert_presence(fields: &[ModelField], path: &str, expected: i32) {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    assert_eq!(field.value, ModelFieldValue::I32(expected));
}

#[test]
fn every_nullable_array_retains_independent_signed_presence_bits() {
    for (kind, version, bytes, _, _) in records() {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        if kind < 6 {
            assert_presence(
                &fields,
                &format!("{}.array_848.presence", payload(kind)),
                PRESENCE[usize::from(kind)],
            );
        } else {
            assert_data_presence(&fields, kind);
        }
    }
}

fn assert_data_presence(fields: &[ModelField], kind: u8) {
    let style = usize::from(kind % 6);
    for (column, offset) in ["08", "10", "18"].into_iter().enumerate() {
        let selected = if style == 0 {
            0
        } else {
            (style + 2 * column) % 6
        };
        assert_presence(
            fields,
            &format!("{}.array_{offset}.presence", payload(kind)),
            PRESENCE[selected],
        );
    }
}

#[test]
fn dword_arrays_keep_unsigned_high_bits_and_counts_independent_from_object_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 < 6) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let values = dword_values(&fields, &payload(kind));
        let count = [0, 3, 1, 4, 0, 3][usize::from(kind)];
        assert_eq!(values.len(), count);
        for (actual, expected) in values
            .into_iter()
            .zip([u32::MAX, 0, 0x89abcdef, 0x80000000])
        {
            assert_eq!(*actual, ModelFieldValue::U32(expected));
        }
    }
}

fn dword_values<'a>(fields: &'a [ModelField], prefix: &str) -> Vec<&'a ModelFieldValue> {
    fields
        .iter()
        .filter(|f| f.path.starts_with(&format!("{prefix}.dwords_820[")))
        .map(|f| &f.value)
        .collect()
}

fn object_tag<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("{path}: expected object tag")
    };
    &tag.record
}

#[test]
fn complete_nested_data_is_shared_between_distinct_deleted_face_roots() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let first = object_tag(&fields, "root.object.items[0].payload.array_800[0]");
        let next = object_tag(&fields, "root.object.items[1].payload.array_800[0]");
        let (
            ObjectRecord::New {
                index: expected,
                class,
            },
            ObjectRecord::Reference { index: actual, .. },
        ) = (first, next)
        else {
            panic!("expected a fresh data object and its cached target")
        };
        assert_eq!(class.name.as_str(), "moDelFaceData_c");
        assert_eq!(expected, actual);
    }
}

#[test]
fn impossible_direct_nullable_and_dword_counts_fail_before_iterating_entries() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 5 || r.0 == 11) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for suffix in escaped_count_paths(kind) {
            let path = format!("{}.{suffix}", payload(kind));
            let changed = super::manager_context_tests::replace_field(
                &fields,
                &path,
                &u32::MAX.to_le_bytes(),
            );
            assert!(decode_graph(version, &changed, false).is_err());
        }
    }
}

fn escaped_count_paths(kind: u8) -> &'static [&'static str] {
    if kind >= 6 {
        return &["array_08.array.items.count.count_dword"];
    }
    &[
        "array_848.array.items.count.count_dword",
        "array_800.count.count_dword",
        "dwords_820.count.count_dword",
        "array_868.count.count_dword",
    ]
}
