pub(super) const ORIGINAL_PLANE_DATA_CLASSES: [&str; 9] = [
    "moRefPlnData_c",
    "moFixedRefPlnData_c",
    "moDefaultRefPlnData_c",
    "moFaceRefPlnData_c",
    "moFacePtRefPlnData_c",
    "moLinePtRefPlnData_c",
    "moPerLinePtRefPlnData_c",
    "moRefAtAngleData_c",
    "moRefThreePtsData_c",
];

use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelFieldValue, ModelObjectRole, encode_fields};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/body-feature-cases.bin");
const ORIGINAL: &[u8] = include_bytes!("../fixtures/model-archives/body-feature-original.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/model-archives/body-feature-edited.bin");

fn decode(version: u32, bytes: &[u8]) -> crate::Result<Vec<ModelField>> {
    decode_collection(version, bytes, |reader, map| {
        reader.object("object", map, ModelObjectRole::Any)
    })
}

#[test]
fn complete_body_features_planes_and_end_specs_match_independent_values_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (576, 37239));
}

#[test]
fn inherited_body_and_plane_prefixes_and_trailing_bytes_reject_partial_publication() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..length]).is_err(),
                "prefix {length}"
            );
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
    }
}

#[test]
fn body_feature_edit_matches_independent_bytes_and_isolates_the_original() {
    let original = decode(19000, ORIGINAL).unwrap();
    let mut changed = original.clone();
    changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.body_suffix.word_798")
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    assert_eq!(encode_fields(&changed, 30000).unwrap(), EDITED);
    assert_eq!(decode(19000, EDITED).unwrap(), changed);
    assert_eq!(encode_fields(&original, 30000).unwrap(), ORIGINAL);
}

#[test]
fn body_and_plane_fields_own_their_source_and_retain_chooser_cycles() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(version, &source).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn body_feature_parallel_writes_preserve_duplicate_maps_and_surface_children() {
    let fields = decode(19000, ORIGINAL).unwrap();
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), ORIGINAL);
        }
    });
}

#[test]
fn inherited_variable_array_counts_reject_negative_and_unbounded_values() {
    let fields = decode(19000, ORIGINAL).unwrap();
    let mut checked = 0;
    for (index, field) in fields.iter().enumerate() {
        if !field.path.ends_with(".count") || !matches!(field.value, ModelFieldValue::I32(_)) {
            continue;
        }
        for value in [i32::MIN, i32::MAX] {
            let mut changed = fields.clone();
            changed[index].value = ModelFieldValue::I32(value);
            assert!(
                decode(19000, &encode_fields(&changed, 30000).unwrap()).is_err(),
                "{}",
                field.path
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 9);
}

#[test]
fn body_and_plane_kinds_are_finite_and_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<super::body_features::BodyFeatureKind>();
    assert_send_sync::<super::reference_planes::PlaneDataKind>();
    for name in ["moBoss_c", "moExtrusion_c", "moRevolution_c", "moRib_c"] {
        assert!(super::body_features::BodyFeatureKind::parse(name).is_some());
    }
    for name in ORIGINAL_PLANE_DATA_CLASSES {
        assert!(super::reference_planes::PlaneDataKind::parse(name).is_some());
    }
    assert!(super::body_features::BodyFeatureKind::parse("moBoss_c ").is_none());
    assert!(super::reference_planes::PlaneDataKind::parse("moUnknownRefPlnData_c").is_none());
}

#[test]
fn inherited_body_arrays_charge_an_independently_counted_aggregate_budget() {
    use super::annotation_view_tests::LIMITS;
    use super::direct_collection_tests::decode_limited;
    for maximum_entries in [52, 53] {
        let limits = super::ModelFieldLimits {
            maximum_entries,
            ..LIMITS
        };
        let result = decode_limited(19000, ORIGINAL, limits, |reader, map| {
            reader.object("object", map, ModelObjectRole::Any)
        });
        assert_eq!(result.is_ok(), maximum_entries == 53);
    }
}

#[test]
fn present_body_id_keeper_must_be_a_nonnull_object() {
    let mut fields = decode(19000, ORIGINAL).unwrap();
    let index = fields
        .iter()
        .position(|field| field.path.ends_with("has_id_keeper"))
        .unwrap();
    fields[index].value = ModelFieldValue::U16(65535);
    let bytes = [encode_fields(&fields[..=index], 30000).unwrap(), vec![0, 0]].concat();
    let error = decode(19000, &bytes).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("id_keeper"));
}

#[test]
fn line_point_plane_forms_reject_null_required_curve_references() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut checked = 0;
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(version, bytes).unwrap();
        let Some(index) = fields
            .iter()
            .position(|field| field.path.ends_with("curve_reference_90"))
        else {
            continue;
        };
        let changed = [encode_fields(&fields[..index], 30000).unwrap(), vec![0, 0]].concat();
        assert_eq!(
            decode(version, &changed).unwrap_err().class,
            crate::FailureClass::Invalid
        );
        checked += 1;
    }
    assert_eq!(checked, 48);
}
