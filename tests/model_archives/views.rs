use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{
    ModelDecoder, ModelField, ModelFieldLimits, ModelFieldValue, ModelObjectRole, encode_fields,
    environment, views,
};
use crate::Result;
use crate::archive_codec::{ArchiveCursor, ObjectReader};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/view-cases.bin");
const NATIVE: &[u8] = include_bytes!("../fixtures/model-archives/native-views.bin");
const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 30000,
    maximum_fields: 6000,
    maximum_entries: 6000,
    maximum_depth: 32,
    maximum_objects: 128,
};

fn decoder(bytes: &[u8], version: u32) -> ModelDecoder<'_> {
    ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap()
}

fn decode(
    group: bool,
    version: u32,
    bytes: &[u8],
    mut map: ObjectReader,
) -> Result<Vec<ModelField>> {
    let mut reader = decoder(bytes, version);
    if group {
        views::read(&mut reader, &mut map)?;
    } else {
        reader.object("object", &mut map, ModelObjectRole::Declared(&["moView_c"]))?;
    }
    reader.finish()
}

pub(super) fn inherited_map() -> ObjectReader {
    let mut map = super::environment_tests::inherited_map();
    let raw = include_bytes!("../fixtures/model-archives/native-environment.bin");
    let mut reader = decoder(raw, 19000);
    environment::read(&mut reader, &mut map).unwrap();
    reader.finish().unwrap();
    map
}

#[test]
fn ordinary_views_and_all_transform_variants_match_independent_fields_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(false, version, bytes, ObjectReader::new()).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (40, 1780));
}

#[test]
fn native_view_group_preserves_full_transforms_and_inherited_map_identity() {
    let decoded = decode(true, 19000, NATIVE, inherited_map()).unwrap();
    let oracle = include_str!("../fixtures/model-archives/native-views.txt");
    assert_eq!(super::transform_rows::assert_fields(&decoded, oracle), 292);
    assert_eq!(encode_fields(&decoded, 30000).unwrap(), NATIVE);
    assert!(decode(true, 19000, NATIVE, ObjectReader::new()).is_err());
}

#[test]
fn every_view_prefix_and_trailing_byte_fails_complete_publication() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(false, version, &bytes[..length], ObjectReader::new()).is_err(),
                "version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(false, version, &trailing, ObjectReader::new()).is_err());
    }
}

#[test]
fn invalid_mutated_transform_shape_cannot_be_serialized_as_a_valid_view() {
    let mut fields = decode(true, 19000, NATIVE, inherited_map()).unwrap();
    let field = fields
        .iter_mut()
        .find(|field| matches!(field.value, ModelFieldValue::Transform(_)))
        .unwrap();
    let ModelFieldValue::Transform(value) = &mut field.value else {
        panic!("expected transform")
    };
    value.has_matrix = 0;
    value.matrix = Some([[crate::archive_codec::StoredDouble::from_bits(0); 3]; 3]);
    assert!(encode_fields(&fields, 30000).is_err());
}
