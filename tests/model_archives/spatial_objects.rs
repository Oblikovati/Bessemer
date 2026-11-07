use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, ModelObjectRole, encode_fields,
};
use crate::Result;
use crate::archive_codec::ObjectReader;
use crate::header::ArchiveVersion;

const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 1024,
    maximum_fields: 32,
    maximum_entries: 32,
    maximum_depth: 8,
    maximum_objects: 4,
};

fn object_bytes(name: &str, body: &[u8]) -> Vec<u8> {
    let mut bytes = vec![255, 255, 1, 0];
    bytes.extend_from_slice(&(name.len() as u16).to_le_bytes());
    bytes.extend_from_slice(name.as_bytes());
    bytes.extend_from_slice(body);
    bytes
}

fn decode(bytes: &[u8]) -> Result<Vec<ModelField>> {
    let mut reader = ModelDecoder::new(bytes, ArchiveVersion::new(19000), LIMITS)?;
    reader.object("spatial", &mut ObjectReader::new(), ModelObjectRole::Any)?;
    reader.finish()
}

fn curvature_bytes() -> Vec<u8> {
    let words: [u64; 5] = [0, 1 << 63, 0x7ff000000000002a, 0x3ff0000000000000, 1];
    let mut body: Vec<u8> = words.into_iter().flat_map(u64::to_le_bytes).collect();
    body.extend_from_slice(&0x89abcdef_u32.to_le_bytes());
    body.extend_from_slice(&0xfedcba98_u32.to_le_bytes());
    object_bytes("gcCurvatureObject_c", &body)
}

fn plane_bytes() -> Vec<u8> {
    let words: [u32; 6] = [u32::MAX, 7, 0x80000000, 0xff123456, 0xfffffffe, 0xabcdef01];
    let body: Vec<u8> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    object_bytes("moTransRefPlaneData_c", &body)
}

#[test]
fn spatial_records_preserve_independent_scalar_bits_and_labels() {
    for (bytes, count, label) in [
        (curvature_bytes(), 8, "time_high"),
        (plane_bytes(), 7, "back_face_color"),
    ] {
        let fields = decode(&bytes).unwrap();
        assert_eq!(fields.len(), count);
        assert!(fields.iter().any(|field| field.path.ends_with(label)));
        assert_eq!(encode_fields(&fields, 1024).unwrap(), bytes);
    }
}

#[test]
fn curvature_keeps_signed_zero_nan_payload_and_timestamp_words() {
    let fields = decode(&curvature_bytes()).unwrap();
    let expected = [0, 1 << 63, 0x7ff000000000002a, 0x3ff0000000000000, 1];
    for (field, bits) in fields[1..6].iter().zip(expected) {
        let ModelFieldValue::F64(value) = field.value else {
            panic!("expected curvature double")
        };
        assert_eq!(value.bits(), bits);
    }
    assert_eq!(fields[6].value, ModelFieldValue::U32(0x89abcdef));
    assert_eq!(fields[7].value, ModelFieldValue::U32(0xfedcba98));
}

#[test]
fn reference_plane_preserves_signed_fields_and_unsigned_color() {
    let fields = decode(&plane_bytes()).unwrap();
    let expected = [
        ModelFieldValue::I32(-1),
        ModelFieldValue::I32(7),
        ModelFieldValue::U32(0x80000000),
        ModelFieldValue::U32(0xff123456),
        ModelFieldValue::I32(-2),
        ModelFieldValue::U32(0xabcdef01),
    ];
    for (field, value) in fields[1..].iter().zip(expected) {
        assert_eq!(field.value, value);
    }
}

#[test]
fn every_spatial_record_truncation_and_trailing_byte_fails() {
    for mut bytes in [curvature_bytes(), plane_bytes()] {
        for length in 0..bytes.len() {
            assert!(decode(&bytes[..length]).is_err(), "prefix {length}");
        }
        bytes.push(0);
        assert!(decode(&bytes).is_err());
    }
}

#[test]
fn spatial_objects_respect_field_budget_and_poison_failed_reads() {
    let bytes = curvature_bytes();
    let limits = ModelFieldLimits {
        maximum_fields: 3,
        ..LIMITS
    };
    let mut reader = ModelDecoder::new(&bytes, ArchiveVersion::new(19000), limits).unwrap();
    let error = reader
        .object("spatial", &mut ObjectReader::new(), ModelObjectRole::Any)
        .unwrap_err();
    assert_eq!(reader.finish().unwrap_err(), error);
}
