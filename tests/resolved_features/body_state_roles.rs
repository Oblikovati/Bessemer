use super::direct_collection_tests::decode_collection;
use super::{ModelField, ModelFieldValue, ModelObjectRecord};
use crate::Result;
use crate::archive_codec::ArchiveClass;

fn decode_body_state(bytes: &[u8], name: &str) -> Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, map| {
        map.append_loaded_object(Some(ArchiveClass {
            name: name.into(),
            schema: 1,
        }))?;
        super::configuration_suffix::body_states(reader, map)
    })
}

fn body_state_bytes() -> Vec<u8> {
    [
        1i32.to_le_bytes().as_slice(),
        &[1, 0],
        i32::MIN.to_le_bytes().as_slice(),
        &[0, 0],
    ]
    .concat()
}

#[test]
fn configuration_body_states_accept_only_the_three_implemented_body_classes() {
    let bytes = body_state_bytes();
    for name in [
        "moCompAtomBody_c",
        "moCompSolidBody_c",
        "moCompSurfaceBody_c",
    ] {
        let fields = decode_body_state(&bytes, name).unwrap();
        assert_eq!(super::encode_fields(&fields, 30000).unwrap(), bytes);
        assert!(fields.iter().any(|field| matches!(&field.value, ModelFieldValue::ObjectTag(tag)
            if matches!(&tag.record, ModelObjectRecord::Reference { class, .. } if class.name == name))));
    }
    for name in [
        "moCompFace_c",
        "moCompAtomBody_c_suffix",
        "moVisualProperties_c",
    ] {
        assert!(
            decode_body_state(&bytes, name).is_err(),
            "unrelated class {name}"
        );
    }
}

#[test]
fn configuration_body_state_truncation_missing_reference_and_extra_bytes_reject() {
    let bytes = body_state_bytes();
    for length in 0..bytes.len() {
        assert!(decode_body_state(&bytes[..length], "moCompSolidBody_c").is_err());
    }
    let mut missing = bytes.clone();
    missing[4] = 2;
    assert!(decode_body_state(&missing, "moCompSolidBody_c").is_err());
    assert!(decode_body_state(&[bytes, vec![0]].concat(), "moCompSolidBody_c").is_err());
}
