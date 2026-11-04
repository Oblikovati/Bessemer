use super::*;
use crate::model_operation_snapshot::{ModelVisualOperation, model_visual_operation};
use crate::visual_workers::assert_eight_visual_workers;
use bessemer::BessemerError;

#[derive(Debug, PartialEq, Eq)]
struct AppearanceConcurrentOutcome {
    changed: ModelVisualOperation,
    unchanged: ModelVisualOperation,
    foreign: BessemerError,
    stale: BessemerError,
}

fn appearance_original_bits(state: &Appearance, role: usize) -> i32 {
    let words = state.words();
    match role {
        0 => words.matching_color_percentage,
        1 => words.transparent_color as i32,
        2 => words.ink_markup_texture.unwrap(),
        _ => panic!("appearance role {role}; expected 0..3"),
    }
}

fn appearance_concurrent_failures(
    foreign: &PartResolvedFeaturesArchive,
    changed: &PartResolvedFeaturesEdit,
    state: &Appearance,
    request: AppearanceEdit,
) -> (BessemerError, BessemerError) {
    let leading = &state.source_fields()[0];
    let rejected = foreign.edit_bitmap_appearance(state, request).unwrap_err();
    let stale = changed
        .archive()
        .edit_bitmap_appearance(state, request)
        .unwrap_err();
    assert_eq!(rejected, foreign.resolve_field(leading).unwrap_err());
    assert_eq!(stale, changed.archive().resolve_field(leading).unwrap_err());
    (rejected, stale)
}

fn appearance_concurrent_edits(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &Appearance,
    role: usize,
    offset: usize,
) -> (PartResolvedFeaturesEdit, PartResolvedFeaturesEdit) {
    let original = appearance_original_bits(state, role);
    let request = appearance_request(role, !original).0;
    let changed = source.edit_bitmap_appearance(state, request).unwrap();
    let no_change = appearance_request(role, original).0;
    let unchanged = source.edit_bitmap_appearance(state, no_change).unwrap();
    verify_appearance_result(source, raw, state, role, offset, !original, &changed);
    verify_appearance_result(source, raw, state, role, offset, original, &unchanged);
    assert_eq!(changed.completion(), ModelEditCompletion::Changed);
    assert_eq!(unchanged.completion(), ModelEditCompletion::Unchanged);
    (changed, unchanged)
}

fn appearance_concurrent_outcome(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &Appearance,
    role: usize,
    offset: usize,
) -> AppearanceConcurrentOutcome {
    let (changed, unchanged) = appearance_concurrent_edits(source, raw, state, role, offset);
    let request = appearance_request(role, !appearance_original_bits(state, role)).0;
    let (foreign, stale) = appearance_concurrent_failures(foreign, &changed, state, request);
    AppearanceConcurrentOutcome {
        changed: model_visual_operation(source, &changed),
        unchanged: model_visual_operation(source, &unchanged),
        foreign,
        stale,
    }
}

fn appearance_concurrent_case(index: usize, case: &AppearanceCase<'_>) {
    let source = appearance_archive(case);
    let foreign = appearance_archive(case);
    for (owner, state) in source
        .bitmap_appearance_states()
        .unwrap()
        .iter()
        .enumerate()
    {
        appearance_concurrent_owner(index, owner, case, &source, &foreign, state);
    }
    assert_eq!(source.to_bytes().unwrap(), case.raw);
    assert_eq!(foreign.to_bytes().unwrap(), case.raw);
}

#[test]
fn current_appearance_settings_match_serial_across_eight_mixed_workers() {
    for (index, case) in appearance_cases()
        .into_iter()
        .enumerate()
        .filter(|(_, case)| case.version == 19000)
    {
        appearance_concurrent_case(index, &case);
    }
}

#[test]
#[ignore = "Requires explicit kernel schema and pinned11000/18000 contexts;43200 serial/worker visits"]
fn all_supported_appearance_settings_match_serial_across_eight_mixed_workers() {
    for (index, case) in appearance_cases().into_iter().enumerate() {
        appearance_concurrent_case(index, &case);
    }
}

fn appearance_concurrent_owner(
    index: usize,
    owner: usize,
    case: &AppearanceCase<'_>,
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    state: &Appearance,
) {
    let offsets = appearance_oracle(index, owner).0;
    for (role, field) in offsets
        .into_iter()
        .enumerate()
        .filter_map(|(role, offset)| offset.map(|offset| (role, offset)))
    {
        let project =
            || appearance_concurrent_outcome(source, foreign, case.raw, state, role, field);
        let serial = project();
        assert_eight_visual_workers(&serial, project);
    }
}
