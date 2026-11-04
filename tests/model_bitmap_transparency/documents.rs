use super::*;
use crate::owned_resolved_document::{
    OWNED_RESOLVED_STREAM, prepared_owned_resolved_document, read_owned_resolved_document,
    replace_owned_resolved_stream,
};

fn reopened_bitmap_document(document: &Document, result: &PartResolvedFeaturesEdit) -> Document {
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
    reopened
}

fn reverse_bitmap_document(
    document: &Document,
    reopened: &Document,
    source: &PartResolvedFeaturesArchive,
    leading: usize,
    role: usize,
    original: i32,
) {
    let actual = read_owned_resolved_document(reopened, LIMITS);
    let field = actual.field_handle(leading).unwrap();
    let current = actual.bitmap_transparency_state(&field).unwrap();
    let reversal = actual
        .edit_bitmap_transparency(&current, setting_request(role, original))
        .unwrap();
    assert_eq!(reversal.archive().fields(), source.fields());
    assert_eq!(
        reversal.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    let mut restored = reopened.clone();
    replace_owned_resolved_stream(&mut restored, reversal.archive().to_bytes().unwrap());
    assert_eq!(restored.to_bytes().unwrap(), document.to_bytes().unwrap());
}

fn verify_bitmap_document_request(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
    role: usize,
    original: i32,
) {
    let before = document.to_bytes().unwrap();
    let result = source
        .edit_bitmap_transparency(state, setting_request(role, original ^ 0x5aa5_5aa5))
        .unwrap();
    verify_bitmap_result_contract(source, &result, true);
    let reopened = reopened_bitmap_document(document, &result);
    reverse_bitmap_document(
        document,
        &reopened,
        source,
        state.source_fields()[0].index(),
        role,
        original,
    );
    assert_eq!(document.to_bytes().unwrap(), before);
}

fn verify_bitmap_document_owner(
    case: usize,
    owner: usize,
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    state: &ModelBitmapTransparencyState,
) {
    for role in 0..3 {
        let original = bitmap_scalar(case, owner, role).1;
        verify_bitmap_document_request(document, source, state, role, original);
    }
}

#[test]
fn all_bitmap_document_edits_reopen_preserve_unselected_streams_and_reverse_exactly() {
    for (case, raw) in bitmap_images().into_iter().enumerate() {
        let document = prepared_owned_resolved_document(raw);
        let source = read_owned_resolved_document(&document, LIMITS);
        let states = source.bitmap_transparency_states().unwrap();
        assert_eq!(states.len(), 2);
        for (owner, state) in states.iter().enumerate() {
            verify_bitmap_document_owner(case, owner, &document, &source, state);
        }
    }
}
