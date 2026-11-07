use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{ConfigurationId, ModelArchiveLimits, PartResolvedFeaturesArchive};

pub(crate) const OWNED_RESOLVED_STREAM: &str = "Contents/Config-0-ResolvedFeatures";

/// Retains a serialized compression checkpoint in the real owned document; e.g. `prepared_owned_resolved_document(raw)`.
pub(crate) fn prepared_owned_resolved_document(raw: &[u8]) -> Document {
    let mut document =
        Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    replace_owned_resolved_stream(&mut document, raw.to_vec());
    Document::from_bytes(&document.to_bytes().unwrap()).unwrap()
}

/// Replaces only the selected feature stream; e.g. `replace_owned_resolved_stream(&mut document, bytes)`.
pub(crate) fn replace_owned_resolved_stream(document: &mut Document, bytes: Vec<u8>) {
    document
        .replace_stream(OWNED_RESOLVED_STREAM, bytes)
        .unwrap();
}

/// Reopens the complete model context and selected feature stream; e.g. `read_owned_resolved_document(&document, limits)`.
pub(crate) fn read_owned_resolved_document(
    document: &Document,
    limits: ModelArchiveLimits,
) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        limits,
    )
    .unwrap()
}
