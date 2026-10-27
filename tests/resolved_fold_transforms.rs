//! Complete fold-folder operations with independent property and transform edits (#54).
use bessemer::Document;
use bessemer::header::{ArchiveVersion, StoredDouble};
use bessemer::model_archives::{
    ArchiveString, ConfigurationId, ModelArchiveLimits, ModelFieldValue,
    PartResolvedFeaturesArchive, TextEncoding,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const RICH: &[u8] = include_bytes!("fixtures/fold-transforms/owned-rich.bin");
const PROPERTY_EDITED: &[u8] = include_bytes!("fixtures/fold-transforms/owned-rich-edited.bin");
const TRANSFORM_EDITED: &[u8] =
    include_bytes!("fixtures/fold-transforms/owned-transform-edited.bin");

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
fn complete_fold_folders_reconstruct_documents_with_real_imported_map_slots() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/fold-transforms/owned-empty.bin").as_slice(),
            105,
        ),
        (RICH, 109),
        (PROPERTY_EDITED, 109),
        (TRANSFORM_EDITED, 109),
    ] {
        let mut document = document_with(features);
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
fn shared_property_text_edit_matches_independent_bytes_without_changing_source_inputs() {
    let document = document_with(RICH);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.ends_with(".string_20"))
        .unwrap()
        .value =
        ModelFieldValue::Text(ArchiveString::new("Bessemer Ω", TextEncoding::Utf16).unwrap());
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), PROPERTY_EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 109));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}

#[test]
fn typed_sketch_transform_edit_preserves_other_transform_and_property_records() {
    let document = document_with(RICH);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let field = fields
        .iter_mut()
        .find(|field| field.path.ends_with(".transforms_e8[0]"))
        .unwrap();
    let ModelFieldValue::Transform(transform) = &mut field.value else {
        panic!("expected typed transform")
    };
    transform.translation[0] = StoredDouble::from_bits(123.5f64.to_bits());
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), TRANSFORM_EDITED);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}

#[test]
fn rejected_transform_shape_edit_does_not_publish_or_mutate_a_folder() {
    let document = document_with(RICH);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let field = fields
        .iter_mut()
        .find(|field| field.path.ends_with(".transforms_e8[0]"))
        .unwrap();
    let ModelFieldValue::Transform(transform) = &mut field.value else {
        panic!("expected typed transform")
    };
    transform.flags = 0xff;
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}
