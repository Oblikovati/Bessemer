//! Complete imported/base-body operations with explicit typed kernel context (#54).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ArchiveString, CompressedKernelContext, ConfigurationId, ModelArchiveLimits, ModelFieldValue,
    PartResolvedFeaturesArchive, TextEncoding,
};
use bessemer::parasolid::{BaseSchema, TransmitLimits};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const COMPRESSED: &[u8] =
    include_bytes!("fixtures/imported-bodies/owned-base-populated-compressed.bin");
const EDITED: &[u8] =
    include_bytes!("fixtures/imported-bodies/owned-base-populated-compressed-edited.bin");

fn kernel() -> CompressedKernelContext {
    CompressedKernelContext {
        base: BaseSchema::from_bytes(include_bytes!("fixtures/parasolid/base-schema.s_t")).unwrap(),
        maximum_expanded: 1024 * 1024,
        transmit_limits: TransmitLimits {
            maximum_records: 1000,
            maximum_field_elements: 10000,
        },
    }
}

fn document_with(features: &[u8]) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
    document
}

fn parsed(document: &Document, with_kernel: bool) -> bessemer::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        with_kernel.then(kernel),
        LIMITS,
    )
}

#[test]
fn imported_classes_reconstruct_whole_documents_for_all_body_modes_and_empty_strings() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/imported-bodies/owned-imported-populated-external.bin")
                .as_slice(),
            103,
        ),
        (
            include_bytes!("fixtures/imported-bodies/owned-imported-populated-suppressed.bin")
                .as_slice(),
            103,
        ),
        (
            include_bytes!("fixtures/imported-bodies/owned-imported-populated-compressed.bin")
                .as_slice(),
            103,
        ),
        (
            include_bytes!("fixtures/imported-bodies/owned-base-populated-external.bin").as_slice(),
            103,
        ),
        (
            include_bytes!("fixtures/imported-bodies/owned-base-populated-suppressed.bin")
                .as_slice(),
            103,
        ),
        (
            include_bytes!("fixtures/imported-bodies/owned-imported-minimal-external.bin")
                .as_slice(),
            101,
        ),
        (
            include_bytes!("fixtures/imported-bodies/owned-base-minimal-external.bin").as_slice(),
            101,
        ),
        (COMPRESSED, 103),
    ] {
        let mut document = document_with(features);
        let original = document.to_bytes().unwrap();
        let archive = parsed(&document, true).unwrap();
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), features);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

#[test]
fn imported_compressed_bodies_require_context_without_mutating_the_document() {
    let document = document_with(COMPRESSED);
    let error = parsed(&document, false).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Missing);
    assert!(error.cause.contains("context None"));
    assert_eq!(document.read_stream(STREAM).unwrap(), COMPRESSED);
    assert_eq!(
        parsed(&document, true).unwrap().to_bytes().unwrap(),
        COMPRESSED
    );
}

#[test]
fn body_name_edit_preserves_the_complete_typed_partition_vector_and_source() {
    let document = document_with(COMPRESSED);
    let archive = parsed(&document, true).unwrap();
    let mut fields = archive.fields().to_vec();
    let name = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].imported.body_name_808")
        .unwrap();
    name.value =
        ModelFieldValue::Text(ArchiveString::new("Bessemer Body Ω", TextEncoding::Utf16).unwrap());
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 103));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), COMPRESSED);
    assert_eq!(document.read_stream(STREAM).unwrap(), COMPRESSED);
    assert_eq!(
        parsed(&document_with(EDITED), true)
            .unwrap()
            .to_bytes()
            .unwrap(),
        EDITED
    );
}
