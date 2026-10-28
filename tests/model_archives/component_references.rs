use super::annotation_view_tests::{LIMITS, decode};
use super::lightweight_section_tests::next_case;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldValue, ModelObjectRole, encode_fields};
use crate::FailureClass;
use crate::archive_codec::{ArchiveCursor, ObjectReader, ObjectRecord};
use crate::header::ArchiveVersion;

const CASES: &[u8] = include_bytes!("../fixtures/model-archives/component-reference-cases.bin");

#[test]
fn component_reference_cases_match_independent_fields_and_bytes() {
    let mut cursor = ArchiveCursor::new(CASES);
    let (mut cases, mut fields) = (0, 0);
    while cursor.remaining() != 0 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let decoded = decode(version, bytes).unwrap();
        fields += super::transform_rows::assert_fields(&decoded, oracle);
        assert_eq!(encode_fields(&decoded, 30000).unwrap(), bytes);
        cases += 1;
    }
    assert_eq!((cases, fields), (188, 5402));
}

#[test]
fn component_reference_prefixes_and_trailing_bytes_never_publish_partial_fields() {
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

#[test]
fn unsupported_component_branches_and_invalid_edge_counts_fail_explicitly() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut mutations = 0;
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        let fields = decode(version, bytes).unwrap();
        for (suffix, value, class) in [
            ("unknown_84", 7, FailureClass::Unsupported),
            ("has_reference_info", -1, FailureClass::Unsupported),
            ("reference_count", -1, FailureClass::Invalid),
        ] {
            mutations += rejected_mutation(version, &fields, suffix, value, class);
        }
    }
    assert_eq!(mutations, 188 + 152 + 48);
}

fn rejected_mutation(
    version: u32,
    fields: &[super::ModelField],
    suffix: &str,
    value: i32,
    class: FailureClass,
) -> usize {
    let mut fields = fields.to_vec();
    let Some(field) = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == format!("root.object.{suffix}"))
    else {
        return 0;
    };
    field.value = ModelFieldValue::I32(value);
    let error = decode(version, &encode_fields(&fields, 30000).unwrap()).unwrap_err();
    assert_eq!(error.class, class);
    1
}

#[test]
fn component_results_own_input_and_parallel_encoders_match_original_bytes() {
    let (version, bytes) = richest_case();
    let mut source = bytes.to_vec();
    let fields = decode(version, &source).unwrap();
    source.fill(0xa5);
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| encode_fields(&fields, 30000).unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), bytes);
        }
    });
}

fn richest_case() -> (u32, &'static [u8]) {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut selected = None;
    while cursor.remaining() != 0 {
        let (_, version, bytes, _) = next_case(&mut cursor);
        selected = Some((version, bytes));
    }
    selected.unwrap()
}

fn decode_prefix_record(
    version: u32,
    bytes: &[u8],
) -> crate::Result<(ObjectRecord, Vec<ModelField>)> {
    let mut reader = ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(version),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )?;
    let mut map = ObjectReader::new();
    reader.object_tag(
        "object",
        &mut map,
        ModelObjectRole::Declared(&["moCompRef_c"]),
    )?;
    let component = reader.scope("object", |reader| {
        super::component_references::base_record(reader, &mut map)
    })?;
    Ok((component, reader.finish()?))
}

#[test]
fn complete_component_prefix_returns_the_same_stored_null_and_cached_identities() {
    let mut cursor = ArchiveCursor::new(CASES);
    let mut nulls = 0;
    for _ in 0..20 {
        let (_, version, bytes, oracle) = next_case(&mut cursor);
        let (component, fields) = decode_prefix_record(version, bytes).unwrap();
        assert_eq!(&component, stored_component_record(&fields));
        nulls += usize::from(matches!(component, ObjectRecord::Null));
        super::transform_rows::assert_fields(&fields, oracle);
        assert_eq!(encode_fields(&fields, 30000).unwrap(), bytes);
    }
    assert_eq!(nulls, 10);
}

fn stored_component_record(fields: &[ModelField]) -> &ObjectRecord {
    let stored = fields
        .iter()
        .find(|f| f.path.as_ref() == "root.object.component")
        .unwrap();
    let ModelFieldValue::ObjectTag(tag) = &stored.value else {
        panic!("expected component pointer")
    };
    &tag.record
}
