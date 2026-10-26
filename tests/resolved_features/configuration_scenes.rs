use super::annotation_view_tests::LIMITS;
use super::direct_collection_tests::{decode_collection, decode_limited};
use super::{ModelArchiveLimits, ModelField, ModelFieldValue, ModelObjectRole, StoredDouble};
use crate::archive_codec::{ArchiveClass, ObjectWriter, scene_data};
use crate::{FailureClass, Result};

const CUBE: &[u8] = include_bytes!("../fixtures/resolved-features/configuration-scene-cube.bin");
const SPHERE: &[u8] =
    include_bytes!("../fixtures/resolved-features/configuration-scene-sphere.bin");
const EMPTY: &[u8] = include_bytes!("../fixtures/resolved-features/configuration-scene-empty.bin");
const MAP: &[u8] = include_bytes!("../fixtures/resolved-features/configuration-scene-map.bin");

fn decode(version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_collection(version, bytes, |reader, map| {
        reader.configuration_scene(|reader| reader.object("object", map, ModelObjectRole::Any))
    })
}

#[test]
fn ordinary_model_archives_cannot_infer_scene_permission_from_class_bytes() {
    let error = super::annotation_view_tests::decode(19000, CUBE).unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("Unselected"));
    let restored = decode_collection(19000, &[], |reader, _| {
        reader.configuration_scene(|_| Ok(()))?;
        reader.require_configuration_scene()
    });
    assert!(restored.unwrap_err().cause.contains("Unselected"));
}

fn decode_map(bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        super::configuration_scenes::owned_map(reader, map, "scene_map")
    })
}

#[test]
fn complete_ordinary_scenes_match_independent_primitives_for_both_kinds_and_empty_counts() {
    for (bytes, oracle, reads) in [
        (
            CUBE,
            include_str!("../fixtures/resolved-features/configuration-scene-cube-primitives.tsv"),
            291,
        ),
        (
            SPHERE,
            include_str!("../fixtures/resolved-features/configuration-scene-sphere-primitives.tsv"),
            291,
        ),
        (
            EMPTY,
            include_str!("../fixtures/resolved-features/configuration-scene-empty-primitives.tsv"),
            195,
        ),
    ] {
        let fields = decode(19000, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(&fields, bytes, oracle, reads);
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    }
}

#[test]
fn owned_scene_maps_preserve_duplicate_signed_keys_and_shared_scene_objects() {
    let fields = decode_map(MAP).unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &fields,
        MAP,
        include_str!("../fixtures/resolved-features/configuration-scene-map-primitives.tsv"),
        302,
    );
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), MAP);
    for path in [
        "root.scene_map.associations[0].raw_key",
        "root.scene_map.associations[1].raw_key",
    ] {
        assert_eq!(
            fields
                .iter()
                .find(|field| field.path.as_ref() == path)
                .unwrap()
                .value,
            ModelFieldValue::I32(7)
        );
    }
    let negative: Vec<u8> = [-2i32, -3].into_iter().flat_map(i32::to_le_bytes).collect();
    assert_eq!(
        super::encode_fields(&decode_map(&negative).unwrap(), 8).unwrap(),
        negative
    );
}

#[test]
fn scene_and_owned_map_prefixes_extra_bytes_and_unsupported_versions_reject() {
    for bytes in [CUBE, SPHERE, EMPTY] {
        for length in 0..bytes.len() {
            assert!(decode(19000, &bytes[..length]).is_err());
        }
        assert!(decode(19000, &[bytes, &[0]].concat()).is_err());
        for version in [18000, 18999, 19001] {
            assert!(decode(version, bytes).is_err());
        }
    }
    for length in 0..MAP.len() {
        assert!(decode_map(&MAP[..length]).is_err());
    }
    assert!(decode_map(&[MAP, &[0]].concat()).is_err());
    assert!(decode_map(&i32::MAX.to_le_bytes()).is_err());
}

#[test]
fn scene_exposure_edit_matches_independent_bytes_and_keeps_owned_inputs_isolated() {
    let mut input = CUBE.to_vec();
    let original = decode(19000, &input).unwrap();
    input.fill(0);
    let mut fields = original.clone();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.hdr_exposure")
        .unwrap()
        .value = ModelFieldValue::F64(StoredDouble::from_bits(0x4037000000000000));
    let expected = include_bytes!("../fixtures/resolved-features/configuration-scene-edited.bin");
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), expected);
    assert_eq!(decode(19000, expected).unwrap(), fields);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), CUBE);
}

#[test]
fn ordinary_scene_helper_slots_reject_fresh_objects_before_their_bodies_are_read() {
    let fields = decode(19000, CUBE).unwrap();
    let mut fresh = Vec::new();
    ObjectWriter::new()
        .object(
            &ArchiveClass {
                name: "moNodeName_c".into(),
                schema: 1,
            },
            &mut fresh,
        )
        .unwrap();
    for path in [
        "root.object.visual_properties",
        "root.object.merged_list.nodes[0]",
        "root.object.floor_selection",
    ] {
        let bytes = super::manager_context_tests::replace_field(&fields, path, &fresh);
        let error = decode(19000, &bytes).unwrap_err();
        assert_eq!(error.class, FailureClass::Unsupported);
        assert!(error.cause.contains("already loaded object"));
    }
}

#[test]
fn shared_scene_counts_and_all_model_operation_budgets_remain_explicit() {
    let result = decode_collection(19000, &[0; 4], |reader, _| {
        reader.scope("invalid", |reader| {
            scene_data::nonnegative_count(reader, "count", 0).map(|_| ())
        })
    });
    assert!(result.unwrap_err().cause.contains("minimum 0"));
    for limits in [
        ModelArchiveLimits {
            maximum_bytes: CUBE.len() - 1,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_fields: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_entries: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_objects: 0,
            ..LIMITS
        },
        ModelArchiveLimits {
            maximum_depth: 0,
            ..LIMITS
        },
    ] {
        let error = decode_limited(19000, CUBE, limits, |reader, map| {
            reader.configuration_scene(|reader| reader.object("object", map, ModelObjectRole::Any))
        })
        .unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains("expected <="), "{error}");
    }
}
