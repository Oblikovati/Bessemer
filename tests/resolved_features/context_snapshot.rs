use super::{ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartConfigurationArchive};
use crate::Document;
use crate::archive_codec::{ArchiveCursor, ObjectReader, ObjectRecord, ObjectWriter};
use crate::header::ArchiveVersion;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

pub(super) fn owned_configuration() -> PartConfigurationArchive {
    let document = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    configuration_from(&document)
}

pub(super) fn configuration_from(document: &Document) -> PartConfigurationArchive {
    PartConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn complete_owned_configuration_retains_the_reference_map_and_every_loaded_identity() {
    let source = owned_configuration();
    assert_eq!(source.parsed.object_map.slot_count(), 97);
    let mut inherited = ObjectReader::from_snapshot(&source.parsed.object_map);
    for field in source.fields() {
        let ModelFieldValue::ObjectTag(tag) = &field.value else {
            continue;
        };
        let ObjectRecord::New { index, class } = &tag.record else {
            continue;
        };
        let mut bytes = Vec::new();
        ObjectWriter::reference(*index, &mut bytes);
        let reference = inherited.object(&mut ArchiveCursor::new(&bytes)).unwrap();
        assert_eq!(
            reference,
            ObjectRecord::Reference {
                index: *index,
                class: class.clone()
            }
        );
    }
    assert_eq!(inherited.into_snapshot().slot_count(), 97);
}

#[test]
fn inherited_builder_growth_and_failures_leave_published_configuration_unchanged() {
    let source = owned_configuration();
    let bytes = source.to_bytes().unwrap();
    let mut inherited = ObjectReader::from_snapshot(&source.parsed.object_map);
    assert_eq!(inherited.append_loaded_object(None).unwrap().value(), 97);
    assert!(inherited.object(&mut ArchiveCursor::new(&[98, 0])).is_err());
    assert_eq!(inherited.into_snapshot().slot_count(), 98);
    assert_eq!(source.parsed.object_map.slot_count(), 97);
    assert_eq!(source.to_bytes().unwrap(), bytes);
    let mut fresh = ObjectReader::from_snapshot(&source.parsed.object_map);
    assert!(fresh.object(&mut ArchiveCursor::new(&[97, 0])).is_err());
}

#[test]
fn owned_configuration_manager_completes_and_forces_exact_typed_reconstruction() {
    use super::configuration_manager::{SavedHelperState, parse};
    let document = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    let bytes = document.read_stream("Contents/CMgr").unwrap();
    assert_eq!(bytes.len(), 1968);
    let parsed = parse(&bytes, ArchiveVersion::new(19000), LIMITS).unwrap();
    assert_eq!(parsed.helper_state, SavedHelperState::Active);
    let rebuilt = super::encode_fields(&parsed.fields, LIMITS.maximum_bytes).unwrap();
    assert_eq!(rebuilt, bytes);
    let forced = parse(&rebuilt, ArchiveVersion::new(19000), LIMITS).unwrap();
    assert_eq!(forced.fields, parsed.fields);
}
