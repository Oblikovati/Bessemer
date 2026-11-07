use super::{Cursor, checks};
use bessemer::display_lists::DisplayEditCompletion;
use bessemer::simulation::*;

pub fn field_case(image: &SimulationEgrArchive, cursor: &mut Cursor<'_>) {
    let input = range_input(cursor);
    let (ordinal, first, last, physical, start, end) = input.fields;
    let id = image.field_range_id(ordinal, first, last).unwrap();
    let view = image.field_range(&id).unwrap();
    assert_view(&view, input.fields, input.original);
    let edit = image.edit_field_range(&id, input.replacement).unwrap();
    assert_eq!(edit.archive.to_bytes(), input.golden);
    assert_edit(
        &edit,
        physical,
        start,
        end,
        input.replacement,
        input.original != input.replacement,
    );
    let noop = image.edit_field_range(&id, input.original).unwrap();
    assert_edit(&noop, physical, start, end, input.original, false);
}

fn assert_view(
    view: &SimulationEgrFieldRange,
    fields: (u32, u32, u32, u32, usize, usize),
    expected: &[u8],
) {
    assert_eq!(
        (
            view.logical_ordinal,
            view.first_field,
            view.last_field,
            view.record_number,
            view.row_offset,
            view.row_end
        ),
        fields
    );
    assert_eq!(view.bytes.as_ref(), expected);
    assert_eq!(
        view.check_mode,
        SimulationEgrCheckMode::ExactExistingLiveFieldRange
    );
    assert!(!view.native_cache_state_projected);
    assert_eq!(view.source.field_range(&view.id).unwrap().bytes, view.bytes);
}

fn assert_edit(
    edit: &SimulationEgrFieldEdit,
    physical: u32,
    start: usize,
    end: usize,
    replacement: &[u8],
    changed: bool,
) {
    assert_eq!(edit.requested_pattern_error_bits, 0);
    assert_eq!(edit.replacement.as_ref(), replacement);
    assert_eq!(edit.result.bytes.as_ref(), replacement);
    assert_edit_completion(edit, changed);
    assert_eq!(edit.source.revision() == edit.archive.revision(), !changed);
    assert_eq!(
        edit.coverage.check_mode,
        SimulationEgrCheckMode::ExactLiveFieldEditAndPreservedOtherBytes
    );
    assert_eq!(edit.history.len(), edit.source.coverage().allocated_records);
    assert_unchanged_metadata(edit);
    for (index, entry) in edit.history.iter().enumerate() {
        assert_relation(edit, entry, index as u32 + 1, physical, start, end, changed);
    }
}

fn assert_relation(
    edit: &SimulationEgrFieldEdit,
    entry: &SimulationEgrRecordHistory,
    number: u32,
    selected: u32,
    start: usize,
    end: usize,
    changed: bool,
) {
    assert_eq!(entry.parent, edit.source.physical_id(number).unwrap());
    assert_eq!(entry.result, edit.archive.physical_id(number).unwrap());
    let mut expected = *edit.source.physical_record(&entry.parent).unwrap();
    let relation = if changed && number == selected {
        expected[start..end].copy_from_slice(&edit.replacement);
        SimulationEgrRecordRelation::Derived
    } else {
        SimulationEgrRecordRelation::Copied
    };
    assert_eq!(entry.relation, relation);
    assert_eq!(
        edit.archive.physical_record(&entry.result).unwrap(),
        &expected
    );
}

pub fn assert_identity_guards(image: &SimulationEgrArchive) {
    assert!(image.live_row_id(0).is_err());
    assert!(image.live_row_id(u32::MAX).is_err());
    assert!(image.physical_id(0).is_err());
    assert!(image.physical_id(u32::MAX).is_err());
    for ordinal in 1..=image.directory().len() as u32 {
        let Some(id) = image.live_row_id(ordinal).unwrap() else {
            continue;
        };
        assert_live_identity(image, &id, ordinal);
    }
}

pub struct RangeInput<'a> {
    pub fields: (u32, u32, u32, u32, usize, usize),
    pub original: &'a [u8],
    pub replacement: &'a [u8],
    pub golden: &'a [u8],
}

pub fn range_input<'a>(cursor: &mut Cursor<'a>) -> RangeInput<'a> {
    let ordinal = cursor.word();
    let first = cursor.word();
    let last = cursor.word();
    let physical = cursor.word();
    let start = cursor.word() as usize;
    let end = cursor.word() as usize;
    RangeInput {
        fields: (ordinal, first, last, physical, start, end),
        original: cursor.span(),
        replacement: cursor.span(),
        golden: cursor.span(),
    }
}

fn assert_unchanged_metadata(edit: &SimulationEgrFieldEdit) {
    let original = edit.source.parts();
    let result = edit.archive.parts();
    assert_eq!(original.header, result.header);
    assert_eq!(original.field_directory, result.field_directory);
    assert_eq!(original.row_directory, result.row_directory);
}

fn assert_live_identity(image: &SimulationEgrArchive, id: &SimulationEgrRowId, ordinal: u32) {
    assert_eq!(id.logical_ordinal(), ordinal);
    assert_eq!(
        id.record_number(),
        image.directory()[ordinal as usize - 1] as u32
    );
    let raw = image.live_row(id).unwrap();
    assert_eq!(
        raw,
        image
            .physical_record(&image.physical_id(id.record_number()).unwrap())
            .unwrap()
    );
    let foreign = checks::table(&image.to_bytes());
    assert!(foreign.live_row(id).is_err());
}

fn assert_edit_completion(edit: &SimulationEgrFieldEdit, changed: bool) {
    assert_eq!(
        edit.completion,
        if changed {
            DisplayEditCompletion::Changed
        } else {
            DisplayEditCompletion::Unchanged
        }
    );
}
