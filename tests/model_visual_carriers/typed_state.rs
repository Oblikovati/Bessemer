use super::*;
use bessemer::entity_visual::{EntityVisualStoredEdit as Edit, EntityVisualStoredWords};

fn projected_model_values(words: EntityVisualStoredWords) -> [ModelFieldValue; 7] {
    [
        ModelFieldValue::U16(words.layer_number),
        ModelFieldValue::U16(words.by_layer_flags),
        ModelFieldValue::U8(words.layer_attribute_cache),
        ModelFieldValue::U32(words.color_word),
        ModelFieldValue::U16(words.line_style),
        ModelFieldValue::I16(words.standard_width),
        ModelFieldValue::F32(words.custom_width.unwrap()),
    ]
}

pub(super) fn verify_typed_visual_states(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    columns: &[&str],
    suffix: &str,
) {
    assert!(
        source
            .entity_visual_state(&source.field_handle(0).unwrap())
            .is_err()
    );
    for (ordinal, index) in visual_group_positions(source.fields(), suffix)
        .into_iter()
        .enumerate()
    {
        let offset = columns[8 + ordinal].parse::<usize>().unwrap();
        let state = typed_visual_projection(source, raw, offset, index);
        verify_typed_rgb(source, raw, offset, index, &state);
        verify_typed_noop(source, &state);
        if columns[0] == "width" {
            verify_typed_width(source, raw, offset, index, &state);
        }
    }
}

fn typed_visual_projection(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    offset: usize,
    index: usize,
) -> ModelEntityVisualStoredState {
    let handle = source.field_handle(index).unwrap();
    let state = source.entity_visual_state(&handle).unwrap();
    assert_eq!(state.source_field(), &handle);
    assert_eq!(
        projected_model_values(state.words()),
        visual_literal_values(&raw[offset..offset + 17])
    );
    verify_typed_color_words(state.words(), &raw[offset..offset + 17]);
    state
}

fn verify_typed_color_words(words: EntityVisualStoredWords, raw: &[u8]) {
    assert_eq!(words.color_rgb(), [raw[5], raw[6], raw[7]]);
    assert_eq!(words.color_unresolved_high_byte(), raw[8]);
    assert_eq!(words.color_is_missing(), raw[5..9] == [255; 4]);
}

fn verify_typed_rgb(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    offset: usize,
    index: usize,
    state: &ModelEntityVisualStoredState,
) {
    let original = state.words().color_word;
    let color = original ^ 0x00a53cf0;
    let rgb = [color as u8, (color >> 8) as u8, (color >> 16) as u8];
    let result = source
        .edit_entity_visual_state(state, Edit::ColorRgb(rgb))
        .unwrap();
    let mut expected = raw.to_vec();
    expected[offset + 5..offset + 9].copy_from_slice(&color.to_le_bytes());
    assert_eq!(result.archive().to_bytes().unwrap(), expected);
    verify_typed_rgb_identity(source, &result, state, index, original);
    assert_eq!(state.words().color_word, original);
    assert_eq!(source.to_bytes().unwrap(), raw);
}

fn verify_typed_rgb_identity(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    state: &ModelEntityVisualStoredState,
    index: usize,
    original: u32,
) {
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(result.changes().len(), 1);
    assert_eq!(
        result.changes()[0].parents(),
        [source.field_handle(index + 3).unwrap()]
    );
    assert!(
        result
            .archive()
            .edit_entity_visual_state(state, Edit::ColorWord(original))
            .is_err()
    );
    assert!(
        result
            .archive()
            .entity_visual_state(state.source_field())
            .is_err()
    );
}

fn verify_typed_noop(source: &PartResolvedFeaturesArchive, state: &ModelEntityVisualStoredState) {
    let result = source
        .edit_entity_visual_state(state, Edit::ColorWord(state.words().color_word))
        .unwrap();
    assert_eq!(result.completion(), ModelEditCompletion::Unchanged);
    assert!(result.changes().is_empty());
    assert!(result.archive().resolve_field(state.source_field()).is_ok());
    verify_width_presence_failure(source, state);
}

fn verify_typed_width(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    offset: usize,
    index: usize,
    state: &ModelEntityVisualStoredState,
) {
    let words = state.words();
    let standard = ((words.standard_width as u16) ^ 0x8000) as i16;
    let custom = StoredFloat::from_bits(words.custom_width.unwrap().bits() ^ 0x80000000);
    let result = source
        .edit_entity_visual_state(
            state,
            Edit::LineWidth {
                standard,
                custom: Some(custom),
            },
        )
        .unwrap();
    let mut expected = raw.to_vec();
    expected[offset + 11..offset + 13].copy_from_slice(&standard.to_le_bytes());
    expected[offset + 13..offset + 17].copy_from_slice(&custom.bits().to_le_bytes());
    assert_eq!(result.archive().to_bytes().unwrap(), expected);
    verify_typed_width_history(source, &result, index);
}

fn verify_typed_width_history(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    index: usize,
) {
    assert_eq!(result.changes().len(), 2);
    for (change, relative) in result.changes().iter().zip([6, 5]) {
        assert_eq!(
            change.parents(),
            [source.field_handle(index + relative).unwrap()]
        );
    }
}

fn first_owned_visual_bytes() -> &'static [u8] {
    let mut cursor = include_bytes!("../fixtures/entity-visual-state/model.bin").as_slice();
    let first = include_str!("../fixtures/entity-visual-state/model-words.tsv")
        .lines()
        .next()
        .unwrap();
    let columns: Vec<_> = first.split('\t').collect();
    visual_case_bytes(&mut cursor, &columns)
}

fn resolved_visual_copy(raw: &[u8]) -> PartResolvedFeaturesArchive {
    let (configuration, manager) = visual_model_context();
    PartResolvedFeaturesArchive::from_bytes(raw, configuration, &manager, VISUAL_LIMITS).unwrap()
}

#[test]
fn independently_decoded_visuals_reject_foreign_handles_and_isolate_caller_bytes() {
    let raw = first_owned_visual_bytes();
    let mut caller = raw.to_vec();
    let first = resolved_visual_copy(&caller);
    let second = resolved_visual_copy(&caller);
    let state = first_visual_state(&first);
    assert!(second.entity_visual_state(state.source_field()).is_err());
    assert!(
        second
            .edit_entity_visual_state(&state, Edit::LayerNumber(0))
            .is_err()
    );
    caller.fill(0);
    assert_eq!(first.to_bytes().unwrap(), raw);
    assert_eq!(second.to_bytes().unwrap(), raw);
    let noop = first
        .clone()
        .edit_entity_visual_state(&state, Edit::ColorWord(state.words().color_word))
        .unwrap();
    assert_eq!(noop.completion(), ModelEditCompletion::Unchanged);
}

fn shared_visual_projection(
    source: &PartResolvedFeaturesArchive,
    state: &ModelEntityVisualStoredState,
) -> Vec<u8> {
    let mut output = Vec::new();
    for _ in 0..16 {
        let request = Edit::LayerNumber(!state.words().layer_number);
        let result = source.edit_entity_visual_state(state, request).unwrap();
        assert_eq!(result.completion(), ModelEditCompletion::Changed);
        assert_eq!(
            result.changes()[0].parents(),
            [state.source_field().clone()]
        );
        output.extend_from_slice(&result.archive().to_bytes().unwrap());
        verify_width_presence_failure(source, state);
    }
    output
}

#[test]
fn shared_visual_snapshot_handles_mixed_success_and_failure_in_eight_workers() {
    let raw = first_owned_visual_bytes();
    let source = resolved_visual_copy(raw);
    let state = first_visual_state(&source);
    let serial = shared_visual_projection(&source, &state);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| shared_visual_projection(&source, &state)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    assert_eq!(source.to_bytes().unwrap(), raw);
}

fn verify_width_presence_failure(
    source: &PartResolvedFeaturesArchive,
    state: &ModelEntityVisualStoredState,
) {
    let invalid = Edit::LineWidth {
        standard: 0,
        custom: None,
    };
    assert!(source.edit_entity_visual_state(state, invalid).is_err());
}

fn first_visual_state(source: &PartResolvedFeaturesArchive) -> ModelEntityVisualStoredState {
    let index = visual_group_positions(source.fields(), ".entity_visual_properties.unknown_0")[0];
    source
        .entity_visual_state(&source.field_handle(index).unwrap())
        .unwrap()
}
