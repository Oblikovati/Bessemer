use super::direct_collection_tests::decode_collection;
use super::{ArchiveString, ModelField, ModelFieldValue, ModelObjectRole, TextEncoding};
use crate::Result;
use crate::archive_codec::{ArchiveClass, ObjectWriter};

const RAW: &[u8] = include_bytes!("../fixtures/resolved-features/custom-properties.bin");
const EDITED: &[u8] = include_bytes!("../fixtures/resolved-features/custom-properties-edited.bin");

fn decode(bytes: &[u8]) -> Result<Vec<ModelField>> {
    decode_collection(19000, bytes, |reader, _| {
        super::custom_properties::list(reader, "properties")
    })
}

#[test]
fn complete_custom_properties_match_reference_primitive_partition_and_independent_edit() {
    let original = decode(RAW).unwrap();
    super::manager_oracle_tests::assert_reference_fields(
        &original,
        RAW,
        include_str!("../fixtures/resolved-features/custom-properties-primitives.tsv"),
        41,
    );
    let mut edited = original.clone();
    let target = edited
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.properties[0].string_20")
        .unwrap();
    target.value =
        ModelFieldValue::Text(ArchiveString::new("独立 Ω", TextEncoding::Utf16).unwrap());
    assert_eq!(super::encode_fields(&edited, 30000).unwrap(), EDITED);
    assert_eq!(decode(EDITED).unwrap(), edited);
    assert_eq!(super::encode_fields(&original, 30000).unwrap(), RAW);
}

#[test]
fn custom_properties_reject_all_truncated_prefixes_negative_counts_and_trailing_bytes() {
    for length in 0..RAW.len() {
        assert!(decode(&RAW[..length]).is_err(), "prefix {length}");
    }
    assert!(decode(&[RAW, &[0]].concat()).is_err());
    assert!(decode(&(-1i32).to_le_bytes()).is_err());
    assert!(decode(&i32::MAX.to_le_bytes()).is_err());
    assert_eq!(
        super::encode_fields(&decode(&[0; 4]).unwrap(), 4).unwrap(),
        [0; 4]
    );
}

#[test]
fn tagged_property_lists_share_the_complete_direct_body_and_own_their_fields() {
    let mut bytes = Vec::new();
    ObjectWriter::new()
        .object(
            &ArchiveClass {
                name: "moCustomPropertyList".into(),
                schema: 1,
            },
            &mut bytes,
        )
        .unwrap();
    bytes.extend_from_slice(RAW);
    let expected = bytes.clone();
    let fields = decode_collection(19000, &bytes, |reader, map| {
        reader.object(
            "list",
            map,
            ModelObjectRole::Declared(&["moCustomPropertyList"]),
        )
    })
    .unwrap();
    bytes.fill(0);
    assert_eq!(super::encode_fields(&fields, 30000).unwrap(), expected);
}
