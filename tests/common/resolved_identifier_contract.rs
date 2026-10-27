use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    CompressedKernelContext, ConfigurationId, ModelArchiveLimits, ModelFieldValue,
    PartResolvedFeaturesArchive,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
pub fn document_with(features: &[u8]) -> Document {
    let mut document =
        Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("../fixtures/sheet-metal/manager-0.bin").to_vec(),
        )
        .unwrap();
    document
}

pub fn parsed_with_kernel(
    document: &Document,
    kernel: Option<CompressedKernelContext>,
) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        kernel,
        ModelArchiveLimits {
            maximum_bytes: 30000,
            maximum_fields: 10000,
            maximum_entries: 10000,
            maximum_depth: 64,
            maximum_objects: 256,
        },
    )
    .unwrap()
}

pub fn check_document(features: &[u8], slots: usize) {
    check_document_with_kernel(features, slots, None);
}

pub fn check_document_with_kernel(
    features: &[u8],
    slots: usize,
    kernel: Option<CompressedKernelContext>,
) {
    let mut document = document_with(features);
    let before = document.to_bytes().unwrap();
    let archive = parsed_with_kernel(&document, kernel);
    assert_eq!(archive.object_slots(), (97, slots));
    assert_eq!(archive.to_bytes().unwrap(), features);
    document
        .replace_stream(STREAM, archive.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), before);
    document.replace_stream(STREAM, vec![0xa5]).unwrap();
    assert_eq!(archive.to_bytes().unwrap(), features);
}

pub fn check_word_edit(original: &[u8], edited: &[u8], path: &str) {
    check_word_edit_with_kernel(original, edited, path, None);
}

pub fn check_word_edit_with_kernel(
    original: &[u8],
    edited: &[u8],
    path: &str,
    kernel: Option<CompressedKernelContext>,
) {
    let document = document_with(original);
    let archive = parsed_with_kernel(&document, kernel);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), edited);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), original);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}

pub fn check_rejected_edit(original: &[u8], path: &str) {
    check_rejected_edit_with_kernel(original, path, None);
}

pub fn check_rejected_edit_with_kernel(
    original: &[u8],
    path: &str,
    kernel: Option<CompressedKernelContext>,
) {
    let document = document_with(original);
    let archive = parsed_with_kernel(&document, kernel);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = ModelFieldValue::U16(7);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), original);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}
