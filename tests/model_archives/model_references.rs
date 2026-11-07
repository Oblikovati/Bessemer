use super::annotation_view_tests::decode;
use super::lightweight_section_tests::next_case;
use super::{ModelField, ModelFieldValue, encode_fields};
use crate::archive_codec::{ArchiveCursor, ObjectRecord};

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/model-reference-cases.bin");

#[test]
fn shared_model_wrappers_match_independent_fields_transforms_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (64, 3576));
}

#[test]
fn shared_model_wrapper_prefixes_and_trailing_bytes_reject_incomplete_bodies() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        for length in 0..bytes.len() {
            assert!(
                decode(version, &bytes[..length]).is_err(),
                "version {version} prefix {length}"
            );
        }
        let mut trailing = bytes.to_vec();
        trailing.push(0);
        assert!(decode(version, &trailing).is_err());
    }
}

fn replace_component_class(fields: &mut [ModelField]) {
    let field = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.component_reference")
        .unwrap();
    let ModelFieldValue::ObjectTag(tag) = &mut field.value else {
        panic!("expected component pointer")
    };
    let ObjectRecord::New { class, .. } = &mut tag.record else {
        panic!("expected new component")
    };
    class.name = "moNodeName_c".into();
}

#[test]
fn model_wrapper_component_families_reject_unrelated_runtime_classes() {
    let mut cursor = ArchiveCursor::new(CASES);
    next_case(&mut cursor);
    let (_, version, bytes, _) = next_case(&mut cursor);
    let mut fields = decode(version, bytes).unwrap();
    replace_component_class(&mut fields);
    let error = decode(version, &encode_fields(&fields, 30000).unwrap()).unwrap_err();
    assert_eq!(error.class, crate::FailureClass::Invalid);
    assert!(error.cause.contains("moNodeName_c"));
}

#[test]
fn model_wrappers_own_input_strings_and_both_optional_and_direct_transforms() {
    let mut cursor = ArchiveCursor::new(CASES);
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let mut source = bytes.to_vec();
        let fields = decode(version, &source).unwrap();
        source.fill(0xa5);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
}
