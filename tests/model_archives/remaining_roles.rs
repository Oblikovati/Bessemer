use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelObjectRole, encode_fields};
use crate::archive_codec::{ArchiveClass, ArchiveCursor};
const CASES: &[u8] = include_bytes!("../fixtures/model-archives/remaining-role-cases.bin");

fn decode(mode: u8, version: u32, bytes: &[u8]) -> crate::Result<Vec<ModelField>> {
    decode_collection(version, bytes, |reader, map| {
        if mode == 0 {
            return reader.object("object", map, ModelObjectRole::Any);
        }
        for name in ["moSkDimHandleValG2_c", "moDisplayDistanceDim_c"] {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
        }
        super::reference_tail::increment(reader, map)
    })
}

#[test]
fn remaining_roles_match_independent_new_objects_and_loaded_reference_values() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut values) = (0, 0);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, oracle) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes).unwrap();
        values += super::transform_rows::assert_fields(&fields, oracle);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, values), (8, 211));
}

#[test]
fn remaining_role_boundaries_reject_prefixes_and_keep_owned_fields() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(mode, version, &source).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        for length in 0..bytes.len() {
            assert!(decode(mode, version, &bytes[..length]).is_err());
        }
        assert!(decode(mode, version, &[bytes, &[0]].concat()).is_err());
    }
}

#[test]
fn remaining_role_declarations_still_reject_unrelated_new_objects() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (mode, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(mode, version, bytes).unwrap();
        let index = fields
            .iter()
            .position(|field| {
                matches!(
                    field.path.rsplit('.').next(),
                    Some("first" | "handle" | "reference_geometry" | "component_reference")
                )
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
            decode(mode, version, &changed).unwrap_err().class,
            crate::FailureClass::Invalid
        );
    }
}
