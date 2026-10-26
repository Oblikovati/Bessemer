use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelObjectRole, encode_fields};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/sketch-role-regressions.bin");

fn decode(version: u32, bytes: &[u8]) -> crate::Result<Vec<ModelField>> {
    decode_collection(version, bytes, |reader, map| {
        reader.object("object", map, ModelObjectRole::Any)
    })
}

#[test]
fn documented_sketch_role_variants_match_independent_values_and_exact_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut values) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let fields = decode(version, bytes).unwrap();
        values += super::transform_rows::assert_fields(&fields, oracle);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, values), (4, 346));
}

#[test]
fn sketch_role_variants_reject_truncation_and_preserve_owned_source() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(version, &source).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        for length in 0..bytes.len() {
            assert!(decode(version, &bytes[..length]).is_err());
        }
        assert!(decode(version, &[bytes, &[0]].concat()).is_err());
    }
}

#[test]
fn sketch_role_variants_continue_to_reject_unrelated_new_classes() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(version, bytes).unwrap();
        let index = fields
            .iter()
            .position(|field| {
                field.path.ends_with("user_objects[0]") || field.path.ends_with("help_data")
            })
            .unwrap();
        let name = b"moNodeName_c";
        let tag = [
            65535u16.to_le_bytes().as_slice(),
            &1u16.to_le_bytes(),
            &(name.len() as u16).to_le_bytes(),
            name,
        ]
        .concat();
        let changed = [encode_fields(&fields[..index], 30000).unwrap(), tag].concat();
        assert_eq!(
            decode(version, &changed).unwrap_err().class,
            crate::FailureClass::Invalid
        );
    }
}
