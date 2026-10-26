use super::direct_collection_tests::decode_collection;
use super::lightweight_section_tests::next_case;
use super::{ModelField, encode_fields};
use crate::archive_codec::ArchiveCursor;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/loop-dimension-role-cases.bin");

fn decode(version: u32, bytes: &[u8]) -> crate::Result<Vec<ModelField>> {
    decode_collection(version, bytes, super::sketch_managers::loops)
}

#[test]
fn sketch_loop_dimensions_accept_all_complete_derived_bodies() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut values) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let fields = decode(version, bytes).unwrap();
        values += super::transform_rows::assert_fields(&fields, oracle);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, values), (5, 1218));
}

#[test]
fn sketch_loop_dimension_roles_keep_exact_boundaries_and_owned_fields() {
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
fn loop_dimensions_accept_loaded_family_members_and_reject_unrelated_classes() {
    use super::{ModelFieldValue, ModelObjectRecord};
    use crate::archive_codec::ArchiveClass;
    let (_, version, bytes, _) = next_case(&mut ArchiveCursor::new(CASES));
    let fields = decode(version, bytes).unwrap();
    let dimension = fields
        .iter()
        .position(|field| field.path.as_ref() == "root.slots[0].dimension")
        .unwrap();
    let prefix = [
        encode_fields(&fields[..dimension], 30000).unwrap(),
        vec![1, 0],
    ]
    .concat();
    for (name, accepted) in [
        ("moDisplayDim_c", true),
        ("moDisplayDistanceDim_c", true),
        ("moDisplayRadialDim_c", true),
        ("moDisplayAngularDim_c", true),
        ("moDisplayRevolveDim_c", true),
        ("moEnvFolder_c", false),
        ("moDisplayDim_c_suffix", false),
    ] {
        let result = decode_collection(version, &prefix, |reader, map| {
            map.append_loaded_object(Some(ArchiveClass {
                name: name.into(),
                schema: 1,
            }))?;
            super::sketch_managers::loops(reader, map)
        });
        assert_eq!(result.is_ok(), accepted, "{name}: {result:?}");
        if let Ok(fields) = result {
            assert!(
                matches!(&fields.last().unwrap().value, ModelFieldValue::ObjectTag(tag)
                if matches!(&tag.record, ModelObjectRecord::Reference { class, .. } if class.name == name))
            );
        }
    }
}
