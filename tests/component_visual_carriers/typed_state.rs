use super::*;
use bessemer::entity_visual::{EntityVisualStoredEdit as Edit, EntityVisualStoredWords};

fn typed_display_values(words: EntityVisualStoredWords) -> Vec<DisplayFieldValue> {
    let mut values = vec![
        DisplayFieldValue::U16(words.layer_number),
        DisplayFieldValue::U16(words.by_layer_flags),
        DisplayFieldValue::U8(words.layer_attribute_cache),
        DisplayFieldValue::U32(words.color_word),
        DisplayFieldValue::U16(words.line_style),
        DisplayFieldValue::I16(words.standard_width),
    ];
    if let Some(custom) = words.custom_width {
        values.push(DisplayFieldValue::F32(custom));
    }
    values
}

pub(super) fn verify_display_visuals(source: &DisplayArchive, raw: &[u8], count: usize) {
    let states = source.entity_visual_states().unwrap();
    assert_eq!(states.len(), count);
    for state in states {
        assert_eq!(
            source.entity_visual_state(state.source_field()).unwrap(),
            state
        );
        let (offset, index) = display_visual_projection(source, raw, &state);
        verify_display_rgb(source, raw, offset, index, &state);
        verify_display_noop(source, &state);
        verify_display_width(source, raw, offset, &state);
    }
}

fn display_visual_projection(
    source: &DisplayArchive,
    raw: &[u8],
    state: &DisplayEntityVisualStoredState,
) -> (usize, usize) {
    let offset = component_visual_offset(raw);
    let (_, size) = component_visual_shape(source);
    assert_eq!(
        typed_display_values(state.words()),
        bit_edits::component_literal_words(&raw[offset..offset + size])
    );
    let index = source
        .contents()
        .fields
        .iter()
        .position(|field| field == source.resolve_field(state.source_field()).unwrap())
        .unwrap();
    (offset, index)
}

pub(super) fn verify_display_rgb(
    source: &DisplayArchive,
    raw: &[u8],
    offset: usize,
    index: usize,
    state: &DisplayEntityVisualStoredState,
) -> Vec<u8> {
    let color = state.words().color_word ^ 0x00a53cf0;
    let rgb = [color as u8, (color >> 8) as u8, (color >> 16) as u8];
    let result = source
        .edit_entity_visual_state(state, Edit::ColorRgb(rgb), archive_support::LIMITS)
        .unwrap();
    let mut expected = raw.to_vec();
    expected[offset + 5..offset + 9].copy_from_slice(&color.to_le_bytes());
    let actual = result.archive().to_bytes().unwrap();
    assert_eq!(actual, expected);
    verify_display_rgb_identity(source, &result, state, index, color);
    assert_eq!(source.to_bytes().unwrap(), raw);
    actual
}

fn verify_display_rgb_identity(
    source: &DisplayArchive,
    result: &DisplayArchiveEdit,
    state: &DisplayEntityVisualStoredState,
    index: usize,
    color: u32,
) {
    let archive = result.archive();
    assert_eq!(result.completion(), DisplayEditCompletion::Changed);
    assert_eq!(result.changes().len(), 1);
    verify_component_field_history(source, result, index + 3);
    assert!(
        archive
            .edit_entity_visual_state(state, Edit::ColorWord(color), archive_support::LIMITS)
            .is_err()
    );
    assert!(archive.entity_visual_state(state.source_field()).is_err());
    assert!(
        archive
            .entity_visual_state(result.changes()[0].result().unwrap())
            .is_err()
    );
}

fn verify_display_noop(source: &DisplayArchive, state: &DisplayEntityVisualStoredState) {
    let result = source
        .edit_entity_visual_state(
            state,
            Edit::ColorRgb(state.words().color_rgb()),
            archive_support::LIMITS,
        )
        .unwrap();
    assert_eq!(result.completion(), DisplayEditCompletion::Unchanged);
    assert!(result.changes().is_empty());
    assert!(result.archive().resolve_field(state.source_field()).is_ok());
    verify_display_foreign_state(source, state);
}

fn verify_display_width(
    source: &DisplayArchive,
    raw: &[u8],
    offset: usize,
    state: &DisplayEntityVisualStoredState,
) {
    let standard = ((state.words().standard_width as u16) ^ 0x8000) as i16;
    let custom = state
        .words()
        .custom_width
        .map(|value| StoredFloat::from_bits(value.bits() ^ 0x80000000));
    let result = source
        .edit_entity_visual_state(
            state,
            Edit::LineWidth { standard, custom },
            archive_support::LIMITS,
        )
        .unwrap();
    let mut expected = raw.to_vec();
    expected[offset + 11..offset + 13].copy_from_slice(&standard.to_le_bytes());
    if let Some(custom) = custom {
        expected[offset + 13..offset + 17].copy_from_slice(&custom.bits().to_le_bytes());
    }
    assert_eq!(result.archive().to_bytes().unwrap(), expected);
    assert_eq!(result.changes().len(), if custom.is_some() { 2 } else { 1 });
    verify_display_width_presence(source, state);
}

fn verify_display_width_presence(source: &DisplayArchive, state: &DisplayEntityVisualStoredState) {
    let request = if state.words().custom_width.is_some() {
        Edit::LineWidth {
            standard: 0,
            custom: None,
        }
    } else {
        Edit::CustomWidth(StoredFloat::from_bits(0))
    };
    assert!(
        source
            .edit_entity_visual_state(state, request, archive_support::LIMITS)
            .is_err()
    );
}

fn verify_display_foreign_state(source: &DisplayArchive, state: &DisplayEntityVisualStoredState) {
    let other = DisplayArchive::from_bytes(
        &source.to_bytes().unwrap(),
        source.context(),
        archive_support::LIMITS,
    )
    .unwrap();
    assert!(other.entity_visual_state(state.source_field()).is_err());
    assert!(
        other
            .edit_entity_visual_state(state, Edit::LayerNumber(0), archive_support::LIMITS)
            .is_err()
    );
}

pub(super) fn verify_typed_matrix_rgb(
    source: &DisplayArchive,
    raw: &[u8],
    index: usize,
    offset: usize,
) -> Vec<u8> {
    let states = source.entity_visual_states().unwrap();
    assert_eq!(states.len(), 1);
    let state = &states[0];
    assert_eq!(
        typed_display_values(state.words()),
        bit_edits::component_literal_words(&raw[offset..offset + 17])
    );
    let actual = verify_display_rgb(source, raw, offset, index, state);
    verify_display_noop(source, state);
    actual
}

#[test]
fn display_visual_states_are_send_sync_and_complete_empty_results_stay_empty() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<DisplayEntityVisualStoredState>();
    let source = archive_support::decode_document(
        include_bytes!("../fixtures/blank-part.SLDPRT"),
        DisplayDocumentKind::Part,
    );
    assert!(source.entity_visual_states().unwrap().is_empty());
}

fn shared_display_projection(
    source: &DisplayArchive,
    state: &DisplayEntityVisualStoredState,
) -> Vec<u8> {
    let mut output = Vec::new();
    for _ in 0..16 {
        let edit = Edit::LayerNumber(!state.words().layer_number);
        let result = source
            .edit_entity_visual_state(state, edit, archive_support::LIMITS)
            .unwrap();
        assert_eq!(result.completion(), DisplayEditCompletion::Changed);
        output.extend_from_slice(&result.archive().to_bytes().unwrap());
        verify_display_width_presence(source, state);
    }
    output
}

#[test]
fn shared_display_visual_source_survives_mixed_edits_in_eight_workers() {
    let case = retained_component_cases()
        .into_iter()
        .find(|case| case.version == 19000 && case.scenario == 21)
        .unwrap();
    let context = DisplayArchiveContext {
        kind: case.kind,
        ..archive_support::context()
    };
    let source = DisplayArchive::from_bytes(case.bytes, context, archive_support::LIMITS).unwrap();
    let state = &source.entity_visual_states().unwrap()[0];
    let serial = shared_display_projection(&source, state);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| shared_display_projection(&source, state)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}
