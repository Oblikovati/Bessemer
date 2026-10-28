use super::solid_reference_tests::decode_graph;
use super::trim_edge_identifier_tests::{
    TrimEdgeRecord, assert_complete_records, assert_rejected_boundaries, records_with_count,
};
use super::{ModelField, ModelFieldValue};
use crate::archive_codec::ObjectRecord;
const CLASSES: [&str; 3] = [
    "sgSplineTanMagnitudeDim",
    "sgArcLenDist",
    "sg3DPlane3DPlaneDist",
];

fn records() -> Vec<TrimEdgeRecord> {
    records_with_count(
        include_bytes!("../fixtures/additional-valued-dimensions/cases.bin"),
        b"AVD1",
        306,
    )
}

fn group(kind: u8) -> &'static str {
    ["valued_g1", "valued_g3", "valued_g2"][usize::from(kind / 6)]
}

fn object_tag<'a>(fields: &'a [ModelField], path: &str) -> &'a ObjectRecord {
    let field = fields.iter().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(record) = &field.value else {
        panic!("{path}: expected object tag")
    };
    &record.record
}

#[test]
fn native_arc_length_collection_regression_accepts_its_exact_derived_class() {
    assert_complete_records(
        records()
            .into_iter()
            .filter(|r| r.0 == 10 && r.1 == 19000)
            .collect(),
    );
}

#[test]
fn all_derived_dimension_graphs_match_independent_fields_bytes_and_slots() {
    assert_complete_records(records());
}

#[test]
fn all_derived_dimensions_reject_every_prefix_extra_byte_and_old_context() {
    assert_rejected_boundaries(records());
}

#[test]
fn native_class_identity_is_preserved_without_an_extra_derived_suffix() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 0) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let ObjectRecord::New { class, .. } = object_tag(&fields, "root.object") else {
            panic!("expected new derived class")
        };
        assert_eq!(class.name, CLASSES[usize::from(kind / 6)]);
        let common = 40 + usize::from(version > 0x42ee);
        let tail = [36, 8, 38][usize::from(kind / 6)]
            + usize::from(kind >= 12 && version > 0x2f79 && version < 0x2fa0) * 2;
        assert_eq!(bytes.len() - 6 - class.name.len(), common + tail);
    }
}

#[test]
fn plane_distance_keeps_the_exact_transitional_word_interval() {
    for (_, version, bytes, _, _) in records().into_iter().filter(|r| r.0 == 12) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let word = fields.iter().find(|f| f.path.ends_with(".legacy_word"));
        assert_eq!(word.is_some(), version > 0x2f79 && version < 0x2fa0);
        if let Some(field) = word {
            assert_eq!(field.value, ModelFieldValue::U16(65527));
        }
    }
}

#[test]
fn arc_length_quadrants_retain_zero_and_maximum_without_runtime_normalization() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| matches!(r.0, 6 | 7)) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let quadrant = fields
            .iter()
            .find(|f| f.path.ends_with(".quadrant"))
            .unwrap();
        assert_eq!(
            quadrant.value,
            ModelFieldValue::U16(if kind == 6 { 0 } else { 65535 })
        );
    }
}

fn assert_cached_collection(fields: &[ModelField], kind: u8) {
    let ObjectRecord::New {
        index: expected, ..
    } = object_tag(fields, "root.object.items[0]")
    else {
        panic!("expected seeded dimension")
    };
    for index in 0..3 {
        let path = format!("root.object.items[1].{}[{index}]", group(kind));
        let ObjectRecord::Reference { index: actual, .. } = object_tag(fields, &path) else {
            panic!("expected cached dimension")
        };
        assert_eq!(actual, expected);
    }
}

#[test]
fn each_collection_retains_actual_cached_dimension_identities() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 5) {
        assert_cached_collection(&decode_graph(version, bytes, false).unwrap().0, kind);
    }
}

#[test]
fn extended_collections_still_reject_null_and_cached_unrelated_arrays() {
    for (kind, version, bytes, _, _) in records().into_iter().filter(|r| r.0 % 6 == 5) {
        let fields = decode_graph(version, bytes, false).unwrap().0;
        let path = format!("root.object.items[1].{}[0]", group(kind));
        for index in [0u16, 2] {
            let changed =
                super::manager_context_tests::replace_field(&fields, &path, &index.to_le_bytes());
            let Err(error) = decode_graph(version, &changed, false) else {
                panic!("expected rejected collection entry")
            };
            assert_eq!(error.class, crate::FailureClass::Invalid);
        }
    }
}
