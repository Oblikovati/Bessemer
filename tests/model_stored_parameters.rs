//! Typed parameter operations against complete independently assembled Source goldens (#202).
#[path = "model_stored_parameters/fixtures.rs"]
mod fixtures;
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
#[path = "support/unselected_document_streams.rs"]
mod unselected_document_streams;

use bessemer::model_archives::*;
use bessemer::{Document, FailureClass};
use owned_resolved_document::*;

const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const PRIMARY_BITS: [u64; 4] = [
    0x8000000000000000,
    0x7ff800000000002a,
    0xc023000000000000,
    1,
];
const ALTERNATE_BITS: [u64; 4] = [
    0x7ff0000000000000,
    0xfff0000000000000,
    0x7ff0000000000001,
    0xfff8000000000042,
];
const KINDS: [ModelParameterKind; 6] = [
    ModelParameterKind::Density,
    ModelParameterKind::Length,
    ModelParameterKind::Angle,
    ModelParameterKind::Integer,
    ModelParameterKind::AttributeScalar,
    ModelParameterKind::AttributeString,
];

fn fixture(raw: &[u8]) -> (Document, PartResolvedFeaturesArchive) {
    let document = owned_saved_manager_document::prepared_saved_manager_document(raw);
    let source = read_owned_resolved_document(&document, LIMITS);
    assert_eq!(source.to_bytes().unwrap(), raw);
    (document, source)
}

fn state(source: &PartResolvedFeaturesArchive) -> ModelParameterStoredState {
    let mut states = source.stored_parameters().unwrap();
    assert_eq!(states.len(), 1);
    states.remove(0)
}

fn edited_value(kind: usize) -> ModelParameterValue {
    match kind {
        3 => ModelParameterValue::Integer(12345),
        5 => ModelParameterValue::Text(ArchiveString::new("updated", TextEncoding::Utf16).unwrap()),
        _ => ModelParameterValue::Double(StoredDouble::from_bits(0x3ff4000000000000)),
    }
}

#[path = "model_stored_parameters/alternate_matrix.rs"]
mod alternate_matrix;
#[path = "model_stored_parameters/boundaries.rs"]
mod boundaries;
#[path = "model_stored_parameters/edits.rs"]
mod edits;
#[path = "model_stored_parameters/inherited.rs"]
mod inherited;
#[path = "model_stored_parameters/ownership.rs"]
mod ownership;
#[path = "model_stored_parameters/read.rs"]
mod read;
#[path = "model_stored_parameters/rejections.rs"]
mod rejections;

fn primary_request(kind: usize) -> ModelParameterStoredEdit {
    ModelParameterStoredEdit::Primary(edited_value(kind))
}

fn apply_parameter(
    source: &PartResolvedFeaturesArchive,
    parameter: &ModelParameterStoredState,
    request: ModelParameterStoredEdit,
) -> PartResolvedFeaturesEdit {
    source.edit_stored_parameter(parameter, request).unwrap()
}
