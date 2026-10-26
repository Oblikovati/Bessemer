use super::annotation_view_tests::decode;
use super::direct_collection_tests::decode_collection;
use super::{ModelField, ModelFieldValue};
use crate::Result;
use crate::archive_codec::{ArchiveClass, ArchiveCursor};

const CASES: &[u8] = include_bytes!("../fixtures/resolved-features/sensor-data-cases.bin");
const OBJECT: &[u8] = include_bytes!("../fixtures/resolved-features/sensor-object.bin");
const PAYLOAD: &[u8] = include_bytes!("../fixtures/resolved-features/sensor-payload.bin");

fn decode_data(version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_collection(version, bytes, |reader, _| {
        super::sensor_records::data(reader)
    })
}

fn next_case(cursor: &mut ArchiveCursor<'static>) -> (u32, &'static [u8], &'static str) {
    let version = cursor.u32().unwrap();
    let length = cursor.u32().unwrap() as usize;
    let rows = cursor.u32().unwrap() as usize;
    (
        version,
        cursor.take(length).unwrap(),
        std::str::from_utf8(cursor.take(rows).unwrap()).unwrap(),
    )
}

#[test]
fn sensor_data_matches_independent_primitives_at_every_precision_and_tail_gate() {
    let mut cursor = ArchiveCursor::new(CASES);
    assert_eq!(cursor.u32().unwrap(), 32);
    for _ in 0..32 {
        let (version, bytes, rows) = next_case(&mut cursor);
        let fields = decode_data(version, bytes).unwrap();
        super::manager_oracle_tests::assert_reference_fields(
            &fields,
            bytes,
            rows,
            rows.lines().count(),
        );
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        let first = fields
            .iter()
            .find(|field| field.path.as_ref() == "root.raw_b0")
            .unwrap();
        assert_eq!(
            matches!(first.value, ModelFieldValue::F32(_)),
            version < 0x3765
        );
    }
    assert_eq!(cursor.remaining(), 0);
}

#[test]
fn sensor_prefixes_and_appended_bytes_reject_partial_data_and_objects() {
    let mut cursor = ArchiveCursor::new(CASES);
    let count = cursor.u32().unwrap();
    for _ in 0..count {
        let (version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(decode_data(version, &bytes[..length]).is_err());
        }
        assert!(decode_data(version, &[bytes, &[0]].concat()).is_err());
    }
    for length in 0..OBJECT.len() {
        assert!(decode(19000, &OBJECT[..length]).is_err());
    }
    assert!(decode(19000, &[OBJECT, &[0]].concat()).is_err());
}

#[test]
fn complete_sensor_preserves_repeated_members_and_native_point_field_order() {
    let fields = decode(19000, OBJECT).unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &fields,
        OBJECT,
        include_str!("../fixtures/resolved-features/sensor-object-primitives.tsv"),
        149,
    );
    for (path, expected) in [
        ("data.raw_88", 101),
        ("data.raw_88_repeated", 202),
        ("data.points[0].raw_34", 52),
        ("data.points[0].raw_30", 48),
    ] {
        let path = format!("root.object.payload.{path}");
        assert_eq!(
            fields
                .iter()
                .find(|field| field.path.as_ref() == path)
                .unwrap()
                .value,
            ModelFieldValue::I32(expected)
        );
    }
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), OBJECT);
}

fn decode_payload(bytes: &[u8], wrapper: &str) -> Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        for name in [wrapper, "moDispObjRef_c"] {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
        }
        super::sensor_records::payload(reader, map)
    })
}

#[test]
fn sensor_vectors_retain_loaded_wrapper_aliases_and_reject_wrong_reference_classes() {
    let fields = decode_payload(PAYLOAD, "moRefWrapper_c").unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &fields,
        PAYLOAD,
        include_str!("../fixtures/resolved-features/sensor-payload-primitives.tsv"),
        103,
    );
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), PAYLOAD);
    assert!(decode_payload(PAYLOAD, "moDispObjRef_c").is_err());
    let mut excessive = PAYLOAD.to_vec();
    let count = excessive.len() - 20;
    excessive[count..count + 4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(decode_payload(&excessive, "moRefWrapper_c").is_err());
}

#[test]
fn sensor_wrapper_vectors_accept_only_the_verified_concrete_wrapper_family() {
    for name in super::model_references::WRAPPER_CLASSES {
        let fields = decode_payload(PAYLOAD, name).unwrap();
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), PAYLOAD);
    }
    for name in ["moPointRef_w_suffix", "moCompVertex_c", "moDispObjRef_c"] {
        assert!(decode_payload(PAYLOAD, name).is_err(), "{name}");
    }
}

#[test]
fn sensor_repeated_member_edit_matches_independent_bytes_without_mutating_owned_inputs() {
    let mut input = OBJECT.to_vec();
    let original = decode(19000, &input).unwrap();
    input.fill(0);
    let mut fields = original.clone();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.payload.data.raw_88_repeated")
        .unwrap()
        .value = ModelFieldValue::I32(-202);
    let expected = include_bytes!("../fixtures/resolved-features/sensor-object-edited.bin");
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), expected);
    assert_eq!(decode(19000, expected).unwrap(), fields);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), OBJECT);
}
