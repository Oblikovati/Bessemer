use super::*;
use crate::owned_resolved_document::{
    OWNED_RESOLVED_STREAM, prepared_owned_resolved_document, read_owned_resolved_document,
    replace_owned_resolved_stream,
};

fn document_reference_edit(
    source: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
    offset: usize,
) -> PartResolvedFeaturesEdit {
    let words = state.words();
    let expected = [
        words.span_count_state ^ 0x1357_9bdf,
        words.backward_offset_state ^ 0x2468_ace0,
    ];
    let edit = ModelIntersectionReferenceStoredEdit::SpanAndBackwardStates {
        span_count_state: expected[0],
        backward_offset_state: expected[1],
    };
    let result = source
        .edit_intersection_reference_state(state, edit)
        .unwrap();
    let raw = source.to_bytes().unwrap();
    crate::contracts::verify_reference_edit(source, &raw, state, &result, offset, expected);
    result
}

fn document_reference_reversal(
    source: &PartResolvedFeaturesArchive,
    reopened: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
) -> Vec<u8> {
    let handle = reopened
        .field_handle(state.source_fields()[0].index())
        .unwrap();
    let changed = reopened.intersection_reference_state(&handle).unwrap();
    let original = state.words();
    let reverse = ModelIntersectionReferenceStoredEdit::SpanAndBackwardStates {
        span_count_state: original.span_count_state,
        backward_offset_state: original.backward_offset_state,
    };
    let result = reopened
        .edit_intersection_reference_state(&changed, reverse)
        .unwrap();
    assert_eq!(result.archive().fields(), source.fields());
    assert_eq!(
        result.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    result.archive().to_bytes().unwrap()
}

fn verify_reference_document(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
    offset: usize,
) {
    let original = document.to_bytes().unwrap();
    let result = document_reference_edit(source, state, offset);
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
    replace_owned_resolved_stream(
        &mut output,
        document_reference_reversal(source, &actual, state),
    );
    assert_eq!(output.to_bytes().unwrap(), original);
    assert_eq!(document.to_bytes().unwrap(), original);
}

fn verify_reference_document_case(raw: &[u8]) {
    let document = prepared_owned_resolved_document(raw);
    let source = read_owned_resolved_document(&document, LIMITS);
    assert_eq!(source.to_bytes().unwrap(), raw);
    let states = source.intersection_reference_states().unwrap();
    assert_eq!(states.len(), 8);
    for (state, offset) in states
        .iter()
        .zip([440, 460, 480, 500, 1187, 1207, 1227, 1247])
    {
        verify_reference_document(&document, &source, state, offset);
    }
}

#[test]
fn complete_reference_document_control_reopens_and_reverses_every_group() {
    verify_reference_document_case(CONTROL);
}

#[test]
fn every_reference_matrix_document_reopens_preserves_unrelated_streams_and_reverses_exactly() {
    let mut cursor = MATRIX;
    for index in 0..1344 {
        verify_reference_document_case(next_intersection_image(&mut cursor));
        println!("INTERSECTION_DOCUMENT_MATRIX\t{index}\t8");
    }
    assert!(cursor.is_empty());
}
