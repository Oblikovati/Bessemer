use super::*;
use bessemer::entity_visual::EntityVisualStoredEdit as Edit;

const MODEL_VISUAL_STREAM: &str = "Contents/Config-0-ResolvedFeatures";

fn prepared_model_visual_document(raw: &[u8]) -> Document {
    let mut document =
        Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    document
        .replace_stream(MODEL_VISUAL_STREAM, raw.to_vec())
        .unwrap();
    Document::from_bytes(&document.to_bytes().unwrap()).unwrap()
}

fn reopened_model_visuals(document: &Document) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        VISUAL_LIMITS,
    )
    .unwrap()
}

fn verify_model_visual_other_streams(before: &Document, after: &Document) {
    crate::unselected_document_streams::assert_unselected_document_streams(
        before,
        after,
        MODEL_VISUAL_STREAM,
    );
}

fn model_document_rgb_edit(
    source: &PartResolvedFeaturesArchive,
    index: usize,
) -> PartResolvedFeaturesEdit {
    let state = source
        .entity_visual_state(&source.field_handle(index).unwrap())
        .unwrap();
    let color = state.words().color_word ^ 0x00a53cf0;
    source
        .edit_entity_visual_state(
            &state,
            Edit::ColorRgb([color as u8, (color >> 8) as u8, (color >> 16) as u8]),
        )
        .unwrap()
}

fn model_document_rgb_reversal(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesArchive,
    index: usize,
) -> Vec<u8> {
    let original = source
        .entity_visual_state(&source.field_handle(index).unwrap())
        .unwrap();
    let changed = result
        .entity_visual_state(&result.field_handle(index).unwrap())
        .unwrap();
    let reverted = result
        .edit_entity_visual_state(&changed, Edit::ColorWord(original.words().color_word))
        .unwrap();
    assert_eq!(reverted.archive().fields(), source.fields());
    assert_eq!(
        reverted.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    reverted.archive().to_bytes().unwrap()
}

fn verify_model_visual_document_edit(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    index: usize,
) {
    let original = document.to_bytes().unwrap();
    let result = model_document_rgb_edit(source, index);
    let mut output = document.clone();
    output
        .replace_stream(MODEL_VISUAL_STREAM, result.archive().to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let actual = reopened_model_visuals(&reopened);
    assert_eq!(actual.fields(), result.archive().fields());
    verify_model_visual_other_streams(document, &reopened);
    output
        .replace_stream(
            MODEL_VISUAL_STREAM,
            model_document_rgb_reversal(source, &actual, index),
        )
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), original);
    assert_eq!(document.to_bytes().unwrap(), original);
}

#[test]
fn complete_model_sketch_and_display_document_controls_reopen_and_reverse_typed_visuals() {
    for corpus in [
        folder_visual_corpus(),
        sketch_visual_corpus(),
        nested_display_visual_corpus(),
    ] {
        let mut cursor = corpus.packet;
        let columns: Vec<_> = corpus.words.lines().next().unwrap().split('\t').collect();
        let raw = visual_case_bytes(&mut cursor, &columns);
        let document = prepared_model_visual_document(raw);
        let source = reopened_model_visuals(&document);
        assert_eq!(source.to_bytes().unwrap(), raw);
        for index in visual_group_positions(source.fields(), corpus.suffix) {
            verify_model_visual_document_edit(&document, &source, index);
        }
    }
}
