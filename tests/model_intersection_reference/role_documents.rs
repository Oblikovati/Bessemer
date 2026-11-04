use super::*;
use crate::owned_resolved_document::{
    OWNED_RESOLVED_STREAM, prepared_owned_resolved_document, read_owned_resolved_document,
    replace_owned_resolved_stream,
};

fn verify_role_document_roundtrip(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    limits: ModelArchiveLimits,
    reverse: impl FnOnce(&PartResolvedFeaturesArchive) -> PartResolvedFeaturesEdit,
) {
    let original = document.to_bytes().unwrap();
    let mut output = document.clone();
    replace_owned_resolved_stream(&mut output, result.archive().to_bytes().unwrap());
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let actual = read_owned_resolved_document(&reopened, limits);
    assert_eq!(actual.fields(), result.archive().fields());
    crate::unselected_document_streams::assert_unselected_document_streams(
        document,
        &reopened,
        OWNED_RESOLVED_STREAM,
    );
    let reversed = reverse(&actual);
    assert_eq!(reversed.archive().fields(), source.fields());
    assert_eq!(
        reversed.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    replace_owned_resolved_stream(&mut output, reversed.archive().to_bytes().unwrap());
    assert_eq!(output.to_bytes().unwrap(), original);
    assert_eq!(document.to_bytes().unwrap(), original);
}

fn reverse_document_classifier(
    actual: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
) -> PartResolvedFeaturesEdit {
    let field = actual
        .field_handle(state.source_fields()[0].index())
        .unwrap();
    let current = actual.intersection_reference_state(&field).unwrap();
    actual
        .edit_intersection_reference_classification(&current, state.classification_selector())
        .unwrap()
}

fn reverse_document_index(
    actual: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionEdgeStoredIndex,
) -> PartResolvedFeaturesEdit {
    let field = actual.field_handle(state.source_field().index()).unwrap();
    let current = actual.intersection_edge_index(&field).unwrap();
    actual
        .edit_intersection_edge_index(&current, state.value())
        .unwrap()
}

fn verify_classifier_documents(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
) {
    let states = source.intersection_reference_states().unwrap();
    assert_eq!(states.len(), 8);
    for (state, offset) in states
        .iter()
        .zip([440, 460, 480, 500, 1187, 1207, 1227, 1247])
    {
        let result = super::classification::verify_classification_edit(
            source,
            raw,
            state,
            offset,
            state.classification_selector() ^ 0xa55a,
        );
        verify_role_document_roundtrip(document, source, &result, LIMITS, |actual| {
            reverse_document_classifier(actual, state)
        });
    }
}

fn verify_index_documents(document: &Document, source: &PartResolvedFeaturesArchive, raw: &[u8]) {
    let states = source.intersection_edge_indices().unwrap();
    assert_eq!(states.len(), 2);
    for (state, offset) in states.iter().zip([436, 1183]) {
        let result =
            super::edge_index::verify_index_edit(source, raw, state, offset, !state.value());
        verify_role_document_roundtrip(document, source, &result, LIMITS, |actual| {
            reverse_document_index(actual, state)
        });
    }
}

fn verify_complete_role_document(raw: &[u8]) {
    let document = prepared_owned_resolved_document(raw);
    let source = read_owned_resolved_document(&document, LIMITS);
    assert_eq!(source.to_bytes().unwrap(), raw);
    verify_classifier_documents(&document, &source, raw);
    verify_index_documents(&document, &source, raw);
    assert_eq!(source.to_bytes().unwrap(), raw);
}

#[test]
fn every_bounded_role_document_reopens_preserves_unselected_streams_and_reverses_exactly() {
    let mut cursor = ROLE_PACKET;
    for index in 0..264 {
        verify_complete_role_document(next_intersection_image(&mut cursor));
        println!("INTERSECTION_ROLE_DOCUMENT\t{index}\t8\t2");
    }
    assert_eq!(next_intersection_image(&mut cursor).len(), 1672584);
    assert!(cursor.is_empty());
}

fn verify_large_classifier_document(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
) {
    let index = source
        .fields()
        .iter()
        .position(|field| field.path.ends_with(".edges_48[16383].group_110.raw_00"))
        .unwrap();
    let state = source
        .intersection_reference_state(&source.field_handle(index).unwrap())
        .unwrap();
    assert_eq!(state.classification_selector(), 65535);
    let offset = role_domain_scalar_offset("group", 65535);
    let result =
        super::classification::verify_classification_edit(source, raw, &state, offset, 0x5aa5);
    verify_role_document_roundtrip(document, source, &result, ROLE_FULL_LIMITS, |actual| {
        reverse_document_classifier(actual, &state)
    });
}

fn verify_large_index_document(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
) {
    let index = source
        .fields()
        .iter()
        .position(|field| field.path.ends_with(".edges_48[16383].raw_168"))
        .unwrap();
    let state = source
        .intersection_edge_index(&source.field_handle(index).unwrap())
        .unwrap();
    assert_eq!(state.value(), i32::MAX);
    let offset = role_domain_scalar_offset("index", 16383);
    let result = super::edge_index::verify_index_edit(source, raw, &state, offset, i32::MIN);
    verify_role_document_roundtrip(document, source, &result, ROLE_FULL_LIMITS, |actual| {
        reverse_document_index(actual, &state)
    });
}

#[test]
fn complete_large_role_document_reopens_and_reverses_last_edge_index_and_selector() {
    let (_, raw) = super::classification::full_selector_domain_archive();
    let document = prepared_owned_resolved_document(raw);
    let source = read_owned_resolved_document(&document, ROLE_FULL_LIMITS);
    assert_eq!(source.to_bytes().unwrap(), raw);
    verify_large_classifier_document(&document, &source, raw);
    verify_large_index_document(&document, &source, raw);
    assert_eq!(source.to_bytes().unwrap(), raw);
}
