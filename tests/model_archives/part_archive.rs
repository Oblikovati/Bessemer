use super::{ModelField, ModelFieldLimits, ModelFieldValue, part_archive};
use crate::archive_codec::{ArchiveString, TextEncoding};
use crate::header::ArchiveVersion;
use crate::{Document, Result};
use std::sync::Arc;

const LIMITS: ModelFieldLimits = ModelFieldLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn native_stream() -> Vec<u8> {
    Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT"))
        .unwrap()
        .read_stream("Contents/Config-0")
        .unwrap()
}

fn decode(bytes: &[u8]) -> Result<part_archive::ParsedPartArchive> {
    part_archive::read(
        bytes,
        Arc::new(super::header_load_plan::native_header()),
        ArchiveVersion::new(19000),
        LIMITS,
    )
}

#[test]
fn owned_part_configuration_reaches_exact_eof_and_reconstructs_all_24028_bytes() {
    let bytes = native_stream();
    let parsed = decode(&bytes).unwrap();
    assert_eq!(bytes.len(), 24028);
    assert_eq!(parsed.to_bytes().unwrap(), bytes);
    assert!(parsed.fields().len() > 3000);
    assert_eq!(parsed.lightweight().node.unknown_2c, Some(0x8100));
    assert_eq!(parsed.lightweight().raw_1614, 0);
    assert_eq!(parsed.lightweight().imports.slots().len(), 1);
}

fn renamed_fields(parsed: &part_archive::ParsedPartArchive) -> Vec<ModelField> {
    let mut fields = parsed.fields().to_vec();
    let name = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.node.name.name")
        .unwrap();
    name.value =
        ModelFieldValue::Text(ArchiveString::new("Bessemer Ω", TextEncoding::Utf16).unwrap());
    fields
}

#[test]
fn whole_root_construction_derives_lengths_matches_independent_edit_and_reverts() {
    let original = decode(&native_stream()).unwrap();
    assert_eq!(
        original
            .with_fields(original.fields().to_vec())
            .unwrap()
            .to_bytes()
            .unwrap(),
        native_stream()
    );
    let changed = original.with_fields(renamed_fields(&original)).unwrap();
    let expected = include_bytes!("../fixtures/model-archives/native-part-name-edited.bin");
    assert_eq!(changed.to_bytes().unwrap(), expected);
    assert_eq!(decode(expected).unwrap().to_bytes().unwrap(), expected);
    assert_eq!(
        changed
            .with_fields(original.fields().to_vec())
            .unwrap()
            .to_bytes()
            .unwrap(),
        native_stream()
    );
    assert_eq!(original.to_bytes().unwrap(), native_stream());
}

#[test]
fn model_edit_reopens_in_whole_document_and_preserves_header_and_other_streams() {
    let original = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let source = original.read_stream("Contents/Config-0").unwrap();
    let parsed = decode(&source).unwrap();
    let changed = parsed.with_fields(renamed_fields(&parsed)).unwrap();
    let mut document = original.clone();
    document
        .replace_stream("Contents/Config-0", changed.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    assert_eq!(
        decode(&reopened.read_stream("Contents/Config-0").unwrap())
            .unwrap()
            .to_bytes()
            .unwrap(),
        changed.to_bytes().unwrap()
    );
    assert_other_streams_equal(&original, &reopened);
    document
        .replace_stream("Contents/Config-0", source)
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), original.to_bytes().unwrap());
}

fn assert_other_streams_equal(original: &Document, changed: &Document) {
    for stream in original.streams() {
        if stream.name == "Contents/Config-0" {
            continue;
        }
        assert_eq!(
            changed.read_stream(stream.name).unwrap(),
            original.read_stream(stream.name).unwrap(),
            "{}",
            stream.name
        );
    }
}

#[test]
fn constructed_field_metadata_and_missing_sections_cannot_bypass_reparse_validation() {
    let original = decode(&native_stream()).unwrap();
    let mut fields = original.fields().to_vec();
    fields[0].path = "forged.root.path".into();
    assert!(original.with_fields(fields).is_err());
    let mut fields = original.fields().to_vec();
    fields.retain(|field| field.path.as_ref() != "root.resolved_size");
    assert!(original.with_fields(fields).is_err());
    let mut fields = original.fields().to_vec();
    let duplicate = fields
        .iter()
        .find(|field| field.path.as_ref() == "root.lightweight_size")
        .unwrap()
        .clone();
    fields.push(duplicate);
    assert!(original.with_fields(fields).is_err());
}

#[test]
fn full_part_sections_truncation_trailing_bytes_and_resource_limits_are_enforced() {
    let bytes = native_stream();
    for length in [0, 14, 18, 2920, 2921, 2925, 20501, 21367, 23379, 24027] {
        assert!(decode(&bytes[..length]).is_err(), "prefix {length}");
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(decode(&trailing).is_err());
    for offset in [14, 2921] {
        let mut malformed = bytes.clone();
        malformed[offset..offset + 4].copy_from_slice(&i32::MAX.to_le_bytes());
        assert!(decode(&malformed).is_err(), "section at {offset}");
    }
    let mut limits = LIMITS;
    limits.maximum_fields = 1;
    assert!(
        part_archive::read(
            &bytes,
            Arc::new(super::header_load_plan::native_header()),
            ArchiveVersion::new(19000),
            limits
        )
        .is_err()
    );
}

#[test]
fn published_part_owns_input_and_parallel_writes_preserve_original_after_failed_edit() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<part_archive::ParsedPartArchive>();
    let mut bytes = native_stream();
    let parsed = decode(&bytes).unwrap();
    bytes.fill(0xa5);
    assert!(parsed.with_fields(Vec::new()).is_err());
    std::thread::scope(|scope| {
        let writers: Vec<_> = (0..4)
            .map(|_| scope.spawn(|| parsed.to_bytes().unwrap()))
            .collect();
        for writer in writers {
            assert_eq!(writer.join().unwrap(), native_stream());
        }
    });
}
