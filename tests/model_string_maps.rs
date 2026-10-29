//! Real configuration/bend-note map composition through the public archive APIs (#99).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartConfigurationArchive,
};

const ORIGINAL: &[u8] = include_bytes!("fixtures/assembly-configurations/owned-map-latin1.bin");
const EDITED: &[u8] =
    include_bytes!("fixtures/assembly-configurations/owned-map-latin1-edited.bin");
const STREAM: &str = "Contents/Config-0";
const PATH: &str =
    "root.shared_model_tail.bend_format.substitutions.entries[0].key.map_text.latin1[0]";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn document() -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, ORIGINAL.to_vec()).unwrap();
    document
}

fn parsed(document: &Document) -> PartConfigurationArchive {
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
fn complete_configuration_map_rebuilds_reopens_and_preserves_unrelated_compressed_streams() {
    let original = document();
    let source = parsed(&original);
    let rebuilt = PartConfigurationArchive::from_fields(
        source.fields().to_vec(),
        source.context().clone(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), ORIGINAL);
    let mut changed = original.clone();
    changed
        .replace_stream(STREAM, rebuilt.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&changed.to_bytes().unwrap()).unwrap();
    assert_eq!(parsed(&reopened).to_bytes().unwrap(), ORIGINAL);
    assert_eq!(changed.to_bytes().unwrap(), original.to_bytes().unwrap());
}

#[test]
fn complete_configuration_key_edit_matches_source_golden_history_and_revert() {
    let document = document();
    let source = parsed(&document);
    let mut fields = source.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == PATH)
        .unwrap()
        .value = ModelFieldValue::U8(254);
    let edit = source.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(edit.changes()[0].path(), PATH);
    let mut saved = document.clone();
    saved
        .replace_stream(STREAM, edit.archive().to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&saved.to_bytes().unwrap()).unwrap();
    assert_eq!(parsed(&reopened).to_bytes().unwrap(), EDITED);
    for stream in document.streams() {
        if stream.name != STREAM {
            assert_eq!(
                reopened.read_stream(stream.name).unwrap(),
                document.read_stream(stream.name).unwrap()
            );
        }
    }
    let reverted = edit
        .archive()
        .edit_fields(source.fields().to_vec())
        .unwrap();
    assert_eq!(reverted.archive().to_bytes().unwrap(), ORIGINAL);
    assert_eq!(source.to_bytes().unwrap(), ORIGINAL);
    assert_eq!(document.read_stream(STREAM).unwrap(), ORIGINAL);
}

#[test]
fn invalid_unit_width_cannot_publish_and_parallel_operations_keep_independent_maps() {
    let original = document();
    let source = parsed(&original);
    let mut fields = source.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == PATH)
        .unwrap()
        .value = ModelFieldValue::U16(254);
    assert!(source.edit_fields(fields).is_err());
    std::thread::scope(|scope| {
        let threads: Vec<_> = (0..6)
            .map(|_| scope.spawn(|| parsed(&original).to_bytes().unwrap()))
            .collect();
        for thread in threads {
            assert_eq!(thread.join().unwrap(), ORIGINAL);
        }
    });
    assert_eq!(source.to_bytes().unwrap(), ORIGINAL);
}
