use super::*;
use crate::owned_resolved_document::{
    OWNED_RESOLVED_STREAM, prepared_owned_resolved_document, read_owned_resolved_document,
    replace_owned_resolved_stream,
};

fn lookup_document_edit(original: [i32; 2], kind: usize) -> (LookupEdit, [i32; 2]) {
    match kind {
        0 => (
            LookupEdit::ConfigurationObjectKey(!original[0]),
            [!original[0], original[1]],
        ),
        1 => (
            LookupEdit::ConfigurationRecordKey(!original[1]),
            [original[0], !original[1]],
        ),
        2 => (
            LookupEdit::BothKeys {
                configuration_object_key: !original[0],
                configuration_record_key: !original[1],
            },
            [!original[0], !original[1]],
        ),
        _ => panic!("lookup document kind {kind}; expected0..3"),
    }
}

fn reopened_lookup_document(
    document: &Document,
    result: &PartResolvedFeaturesEdit,
) -> (Document, Document) {
    let mut output = document.clone();
    replace_owned_resolved_stream(&mut output, result.archive().to_bytes().unwrap());
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let actual = read_owned_resolved_document(&reopened, LIMITS);
    assert_eq!(actual.fields(), result.archive().fields());
    crate::unselected_document_streams::assert_unselected_document_streams(
        document,
        &reopened,
        OWNED_RESOLVED_STREAM,
    );
    (output, reopened)
}

fn reverse_lookup_document(
    output: &mut Document,
    reopened: &Document,
    source: &PartResolvedFeaturesArchive,
    state: &LookupState,
    original: [i32; 2],
) {
    let actual = read_owned_resolved_document(reopened, LIMITS);
    let leading = actual
        .field_handle(state.source_fields()[0].index())
        .unwrap();
    let current = actual.sketch_reference_lookup_state(&leading).unwrap();
    let reverse = LookupEdit::BothKeys {
        configuration_object_key: original[0],
        configuration_record_key: original[1],
    };
    let result = actual
        .edit_sketch_reference_lookup_keys(&current, reverse)
        .unwrap();
    assert_eq!(result.archive().fields(), source.fields());
    assert_eq!(
        result.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    replace_owned_resolved_stream(output, result.archive().to_bytes().unwrap());
}

fn verify_lookup_document_request(
    case: usize,
    owner: usize,
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    state: &LookupState,
    kind: usize,
) {
    let before = document.to_bytes().unwrap();
    let (offsets, original) = lookup_literal_pair(case, owner);
    let (request, expected) = lookup_document_edit(original, kind);
    let result = source
        .edit_sketch_reference_lookup_keys(state, request)
        .unwrap();
    verify_lookup_operation(
        source,
        &source.to_bytes().unwrap(),
        state,
        offsets,
        expected,
        &result,
    );
    let (mut output, reopened) = reopened_lookup_document(document, &result);
    reverse_lookup_document(&mut output, &reopened, source, state, original);
    assert_eq!(output.to_bytes().unwrap(), before);
    assert_eq!(document.to_bytes().unwrap(), before);
}

fn verify_lookup_document_owner(
    case: usize,
    owner: usize,
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    state: &LookupState,
) {
    for kind in 0..3 {
        verify_lookup_document_request(case, owner, document, source, state, kind);
    }
}

#[test]
fn every_owned_lookup_document_reopens_preserves_other_streams_and_reverses_exactly() {
    for (case, raw) in lookup_images().into_iter().enumerate() {
        let document = prepared_owned_resolved_document(raw);
        let source = read_owned_resolved_document(&document, LIMITS);
        let states = source.sketch_reference_lookup_states().unwrap();
        assert_eq!(states.len(), 2);
        for (owner, state) in states.iter().enumerate() {
            verify_lookup_document_owner(case, owner, &document, &source, state);
        }
    }
}
