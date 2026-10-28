use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ObjectRecord;

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/end-caps/cases.bin"),
        b"ECF1",
        306,
    )
}

#[test]
fn complete_end_caps_match_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}

#[test]
fn end_caps_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn null_pointer_suffix_is_forty_bytes_after_the_complete_body_parent() {
    for (_, version, bytes, _, slots) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let suffix: Vec<_> = fields
            .into_iter()
            .filter(|f| f.path.starts_with("root.object.payload."))
            .collect();
        assert_eq!(suffix.len(), 12);
        assert_eq!(super::encode_fields(&suffix, 30000).unwrap().len(), 40);
        assert_eq!(slots, 5);
    }
}

#[test]
fn scalar_member_order_retains_signed_words_and_exact_double_bits() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        assert_scalar_suffix(&fields, usize::from(kind / 6));
    }
}

fn assert_scalar_suffix(fields: &[ModelField], kind: usize) {
    let words = [i32::MIN, i32::MAX, -1, 0, -32769, 65535];
    for (index, offset) in ["808", "80c", "828", "82c", "830"].into_iter().enumerate() {
        let path = format!("root.object.payload.raw_{offset}");
        assert_eq!(
            field(fields, &path).value,
            ModelFieldValue::I32(words[(kind + index) % 6])
        );
    }
    let ModelFieldValue::F64(value) = field(fields, "root.object.payload.raw_820").value else {
        panic!("expected preserved end-cap double")
    };
    assert_eq!(
        value.bits(),
        [0, 0x8000000000000000, 0x7ff0000000000042][kind]
    );
}

fn field<'a>(fields: &'a [ModelField], path: &str) -> &'a ModelField {
    fields
        .iter()
        .find(|f| f.path.as_ref() == path)
        .unwrap_or_else(|| {
            panic!("missing end-cap field {path}; expected complete typed observation")
        })
}

fn object<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let ModelFieldValue::ObjectTag(tag) = &field(fields, path).value else {
        panic!("field {path}; expected object tag")
    };
    &tag.record
}

#[test]
fn distance_840_precedes_three_cached_references_to_the_same_object() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 1) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let ObjectRecord::New { index, .. } = object(&fields, "root.object.payload.dimension_840")
        else {
            panic!("expected first complete distance dimension at +840")
        };
        for offset in ["838", "848", "850"] {
            let ObjectRecord::Reference { index: cached, .. } =
                object(&fields, &format!("root.object.payload.dimension_{offset}"))
            else {
                panic!("expected cached distance dimension at +{offset}")
            };
            assert_eq!(index, cached);
        }
    }
}

const END_CAP_POINTER_LABELS: [&str; 6] = [
    "list_800",
    "dimension_840",
    "dimension_838",
    "dimension_848",
    "dimension_850",
    "dimension_858",
];

#[test]
fn end_cap_pointer_roles_reject_unrelated_cached_arrays() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 2 && r.1 == 19000) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        for label in END_CAP_POINTER_LABELS {
            let path = format!("root.object.items[0].payload.{label}");
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &2u16.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected unrelated cached array rejection at {path}")
            };
            assert!(error.to_string().contains(label));
        }
    }
}

#[test]
fn unsupported_arc_length_body_is_never_aliased_to_distance_layout() {
    let name = b"moDisplayArcLengthDim_c";
    let mut header = vec![255, 255, 1, 0];
    header.extend_from_slice(&u16::try_from(name.len()).unwrap().to_le_bytes());
    header.extend_from_slice(name);
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let changed = super::manager_context_tests::replace_field(
            &fields,
            "root.object.payload.dimension_858",
            &header,
        );
        let Err(error) = decode_graph(version, &changed, false) else {
            panic!("expected unsupported complete arc-length body rejection")
        };
        assert!(error.to_string().contains("moDisplayArcLengthDim_c"));
        assert!(error.to_string().contains("Unsupported"));
    }
}
