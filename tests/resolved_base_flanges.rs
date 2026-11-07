//! Complete base-flange documents and independent direction edits (#54).
use bessemer::Document;
use bessemer::header::{ArchiveVersion, StoredDouble};
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartResolvedFeaturesArchive,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const RICH: &[u8] = include_bytes!("fixtures/base-flanges/owned-rich-1.bin");
const EDITED: &[u8] = include_bytes!("fixtures/base-flanges/owned-rich-1-edited.bin");

fn document_with(features: &[u8], extended: bool) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
    let manager = if extended {
        include_bytes!("fixtures/sheet-metal/manager-1.bin").as_slice()
    } else {
        include_bytes!("fixtures/sheet-metal/manager-0.bin").as_slice()
    };
    document
        .replace_stream("Contents/CMgr", manager.to_vec())
        .unwrap();
    document
}

fn parsed(document: &Document) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn complete_flange_documents_preserve_both_saved_states_and_inherited_slots() {
    for (features, extended, slots) in [
        (
            include_bytes!("fixtures/base-flanges/owned-empty.bin").as_slice(),
            false,
            101,
        ),
        (
            include_bytes!("fixtures/base-flanges/owned-rich-0.bin").as_slice(),
            false,
            119,
        ),
        (RICH, true, 121),
        (EDITED, true, 121),
    ] {
        let mut document = document_with(features, extended);
        let original = document.to_bytes().unwrap();
        let archive = parsed(&document);
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), features);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

#[test]
fn selected_direction_edit_matches_independent_bytes_and_preserves_inputs() {
    let document = document_with(RICH, true);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].payload.directions[1].raw_extra")
        .unwrap()
        .value = ModelFieldValue::F64(StoredDouble::from_bits(123.5f64.to_bits()));
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 121));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}

#[test]
fn rejected_handle_count_edit_keeps_the_document_and_archive_unchanged() {
    let document = document_with(RICH, true);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].payload.handle_arrays.958.count")
        .unwrap()
        .value = ModelFieldValue::I32(-1);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}
