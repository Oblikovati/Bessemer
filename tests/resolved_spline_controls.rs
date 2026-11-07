//! Complete spline-control document reconstruction, direct-handle edits and isolation (#54).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
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
const EMPTY: [&[u8]; 3] = [
    include_bytes!("fixtures/spline-controls/owned-0-0.bin"),
    include_bytes!("fixtures/spline-controls/owned-1-0.bin"),
    include_bytes!("fixtures/spline-controls/owned-2-0.bin"),
];
const ORIGINALS: [&[u8]; 3] = [
    include_bytes!("fixtures/spline-controls/owned-0-3.bin"),
    include_bytes!("fixtures/spline-controls/owned-1-3.bin"),
    include_bytes!("fixtures/spline-controls/owned-2-3.bin"),
];
const EDITS: [&[u8]; 3] = [
    include_bytes!("fixtures/spline-controls/owned-0-3-edited.bin"),
    include_bytes!("fixtures/spline-controls/owned-1-3-edited.bin"),
    include_bytes!("fixtures/spline-controls/owned-2-3-edited.bin"),
];

fn document_with(features: &[u8]) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("fixtures/sheet-metal/manager-0.bin").to_vec(),
        )
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
fn all_complete_control_documents_reconstruct_and_keep_direct_handles_out_of_the_map() {
    for (features, slots) in EMPTY
        .into_iter()
        .map(|bytes| (bytes, 99))
        .chain(ORIGINALS.into_iter().chain(EDITS).map(|bytes| (bytes, 101)))
    {
        let mut document = document_with(features);
        let before = document.to_bytes().unwrap();
        let archive = parsed(&document);
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), features);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), before);
        document.replace_stream(STREAM, vec![0xa5]).unwrap();
        assert_eq!(archive.to_bytes().unwrap(), features);
    }
}

#[test]
fn first_direct_handle_word_edits_match_independent_bytes_for_all_three_classes() {
    for (original, edited) in ORIGINALS.into_iter().zip(EDITS) {
        let document = document_with(original);
        let archive = parsed(&document);
        let mut fields = archive.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.ends_with(".reference_id"))
            .unwrap()
            .value = ModelFieldValue::I32(12345);
        let edit = archive.edit_fields(fields).unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), edited);
        assert_eq!(edit.changes().len(), 1);
        assert_eq!(edit.archive().object_slots(), (97, 101));
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}

#[test]
fn malformed_user_object_count_edits_do_not_publish_or_mutate_the_source() {
    for original in ORIGINALS {
        let document = document_with(original);
        let archive = parsed(&document);
        let mut fields = archive.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.ends_with("visual.user_objects.count"))
            .unwrap()
            .value = ModelFieldValue::I32(-1);
        assert!(archive.edit_fields(fields).is_err());
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}
