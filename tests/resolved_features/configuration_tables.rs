use super::direct_collection_tests::decode_collection;
use super::{ModelField, ModelFieldValue};
use crate::Result;
use crate::archive_codec::{ArchiveClass, ObjectWriter};

fn payload() -> Vec<u8> {
    let mut bytes = b"\x03raw".to_vec();
    for word in [-7i32, 123, 3] {
        bytes.extend_from_slice(&word.to_le_bytes());
    }
    bytes.extend_from_slice(b"\x01A\x03one\xff\xfe\xff\x01A\0\x03two\0\0");
    bytes.extend_from_slice(&u32::MAX.to_le_bytes());
    bytes
}

fn decode_payload(version: u32, bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_collection(version, bytes, |reader, _| {
        super::configuration_tables::payload(reader)
    })
}

#[test]
fn native_table_payload_matches_independently_specified_duplicates_unicode_and_four_byte_time() {
    let bytes = payload();
    let fields = decode_payload(0x374e, &bytes).unwrap();
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
    assert_eq!(fields.last().unwrap().value, ModelFieldValue::U32(u32::MAX));
    assert!(fields.iter().any(
        |field| matches!(&field.value, ModelFieldValue::Text(value) if value.text() == "two")
    ));
    assert!(decode_payload(0x374d, &[]).unwrap().is_empty());
    assert!(decode_payload(0x374d, &bytes).is_err());
}

#[test]
fn table_pair_count_rejects_negative_impossible_truncated_and_missing_time_inputs() {
    let bytes = payload();
    for length in 0..bytes.len() {
        assert!(decode_payload(19000, &bytes[..length]).is_err());
    }
    assert!(decode_payload(19000, &[bytes.clone(), vec![0]].concat()).is_err());
    let mut negative = bytes.clone();
    negative[12..16].copy_from_slice(&(-1i32).to_le_bytes());
    assert!(decode_payload(19000, &negative).is_err());
    let mut impossible = bytes;
    impossible[12..16].copy_from_slice(&i32::MAX.to_le_bytes());
    assert!(decode_payload(19000, &impossible).is_err());
}

#[test]
fn complete_native_and_design_table_objects_reuse_the_ordinary_model_feature_body() {
    for (name, suffix) in [
        ("moNativeConfigurationTableFeature_c", payload()),
        ("moDesignTableFeature_c", vec![]),
    ] {
        let mut bytes = Vec::new();
        ObjectWriter::new()
            .object(
                &ArchiveClass {
                    name: name.into(),
                    schema: 1,
                },
                &mut bytes,
            )
            .unwrap();
        bytes.extend_from_slice(&[0; 143]);
        bytes.extend_from_slice(&suffix);
        let fields = super::annotation_view_tests::decode(19000, &bytes).unwrap();
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        for length in 0..bytes.len() {
            assert!(super::annotation_view_tests::decode(19000, &bytes[..length]).is_err());
        }
        assert!(super::annotation_view_tests::decode(19000, &[bytes, vec![0]].concat()).is_err());
    }
}
