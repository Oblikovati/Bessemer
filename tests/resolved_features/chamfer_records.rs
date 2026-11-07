use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::{ModelField, ModelFieldValue};
use crate::Result;
use crate::archive_codec::ArchiveClass;

const PAYLOAD: &[u8] = include_bytes!("../fixtures/resolved-features/chamfer-payload.bin");
const OBJECT: &[u8] = include_bytes!("../fixtures/resolved-features/chamfer-object.bin");
const CLASSES: &[&str] = &[
    "moEntityIdRep_c",
    "moCompEdge_c",
    "suObList",
    "moDisplayDistanceDim_c",
    "moDisplayAngularDim_c",
    "moCompFace_c",
    "moCompVertex_c",
    "moRefWrapper_c",
];

fn decode_payload(bytes: &[u8], angular: &str) -> Result<Vec<ModelField>> {
    decode_payload_as(bytes, angular, "moRefWrapper_c")
}

fn decode_payload_as(bytes: &[u8], angular: &str, wrapper: &str) -> Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        for (index, name) in CLASSES.iter().enumerate() {
            let name = match index {
                4 => angular,
                7 => wrapper,
                _ => name,
            };
            map.append_loaded_object(Some(ArchiveClass {
                name: name.to_string(),
                schema: 1,
            }))?;
        }
        super::chamfer_records::payload(reader, map)
    })
}

#[test]
fn chamfer_optional_base_wrapper_pointer_keeps_the_same_finite_derived_family() {
    for name in super::model_references::WRAPPER_CLASSES {
        assert!(
            decode_payload_as(PAYLOAD, "moDisplayAngularDim_c", name).is_ok(),
            "{name}"
        );
    }
    for name in [
        "moCompFace_c",
        "moDisplayDistanceDim_c",
        "moPointRef_w_suffix",
    ] {
        assert!(
            decode_payload_as(PAYLOAD, "moDisplayAngularDim_c", name).is_err(),
            "{name}"
        );
    }
}

#[test]
fn complete_chamfer_and_direct_keeper_payload_match_independent_primitives() {
    let payload = decode_payload(PAYLOAD, "moDisplayAngularDim_c").unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &payload,
        PAYLOAD,
        include_str!("../fixtures/resolved-features/chamfer-payload-primitives.tsv"),
        35,
    );
    assert_eq!(super::encode_fields(&payload, 30000).unwrap(), PAYLOAD);
    let object = decode(19000, OBJECT).unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &object,
        OBJECT,
        include_str!("../fixtures/resolved-features/chamfer-object-primitives.tsv"),
        124,
    );
    assert_eq!(super::encode_fields(&object, 30000).unwrap(), OBJECT);
}

#[test]
fn chamfer_prefixes_appended_bytes_invalid_keeper_and_dimension_roles_reject() {
    for length in 0..PAYLOAD.len() {
        assert!(decode_payload(&PAYLOAD[..length], "moDisplayAngularDim_c").is_err());
    }
    for length in 0..OBJECT.len() {
        assert!(decode(19000, &OBJECT[..length]).is_err());
    }
    assert!(decode_payload(&[PAYLOAD, &[0]].concat(), "moDisplayAngularDim_c").is_err());
    assert!(decode(19000, &[OBJECT, &[0]].concat()).is_err());
    let mut negative = PAYLOAD.to_vec();
    negative[..4].copy_from_slice(&(-1i32).to_le_bytes());
    assert!(decode_payload(&negative, "moDisplayAngularDim_c").is_err());
    assert!(decode_payload(PAYLOAD, "moDisplayDistanceDim_c").is_err());
    assert!(decode_payload(PAYLOAD, "moDisplayAngularDim_c_suffix").is_err());
}

#[test]
fn chamfer_mixed_width_flags_and_unsigned_array_keep_original_values() {
    let fields = decode_payload(PAYLOAD, "moDisplayAngularDim_c").unwrap();
    for (name, expected) in [
        ("raw_858", 255),
        ("raw_898", 254),
        ("raw_870", 253),
        ("raw_874", 252),
        ("raw_8e8", 251),
    ] {
        assert_eq!(
            fields
                .iter()
                .find(|field| field.path.as_ref() == format!("root.{name}"))
                .unwrap()
                .value,
            ModelFieldValue::U8(expected)
        );
    }
    assert!(
        fields
            .iter()
            .any(|field| field.value == ModelFieldValue::U32(u32::MAX))
    );
    assert!(
        fields
            .iter()
            .any(|field| field.value == ModelFieldValue::I32(-9))
    );
    let empty = decode_payload(&[0; 53], "moDisplayAngularDim_c").unwrap();
    assert_eq!(super::encode_fields(&empty, 53).unwrap(), [0; 53]);
}

#[test]
fn chamfer_edit_matches_independent_expected_bytes_and_preserves_owned_inputs() {
    let mut input = PAYLOAD.to_vec();
    let original = decode_payload(&input, "moDisplayAngularDim_c").unwrap();
    input.fill(0);
    let mut fields = original.clone();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.raw_8b8")
        .unwrap()
        .value = ModelFieldValue::I32(123);
    let expected = include_bytes!("../fixtures/resolved-features/chamfer-payload-edited.bin");
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), expected);
    assert_eq!(
        decode_payload(expected, "moDisplayAngularDim_c").unwrap(),
        fields
    );
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), PAYLOAD);
}
