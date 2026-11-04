use super::*;
use bessemer::history::{ArchiveString, TextEncoding};
use std::ops::Range;

const LOCK_IMAGES: [[u32; 4]; 6] = [
    [0x04, 0x14, 0x04, 0x24],
    [0x05, 0x15, 0x05, 0x25],
    [0x80000004, 0x80000014, 0x80000004, 0x80000024],
    [0x80000005, 0x80000015, 0x80000005, 0x80000025],
    [0x24, 0x34, 0x14, 0x34],
    [0x24, 0x34, 0x04, 0x24],
];
const SIGNED_OPTIONS: [i32; 10] = [
    i32::MIN,
    i32::MIN + 1,
    -65536,
    -1,
    0,
    1,
    2,
    65535,
    i32::MAX - 1,
    i32::MAX,
];
const ROLES: [ModelSplineEquationTextRole; 6] = [
    ModelSplineEquationTextRole::XExpression,
    ModelSplineEquationTextRole::YExpression,
    ModelSplineEquationTextRole::ZExpression,
    ModelSplineEquationTextRole::DeclaredCharacters,
    ModelSplineEquationTextRole::RangeStart,
    ModelSplineEquationTextRole::RangeEnd,
];

struct LiteralEquationPositions {
    flags: usize,
    option: usize,
    texts: [(usize, usize); 6],
}

fn literal_positions(kind: usize, style: usize) -> LiteralEquationPositions {
    let prefix = format!("{kind}\t{style}\t");
    let line = include_str!("../fixtures/spline-equation-state/literal-positions.tsv")
        .lines()
        .find(|line| line.starts_with(&prefix))
        .unwrap();
    let values: Vec<usize> = line
        .split('\t')
        .map(|value| value.parse().unwrap())
        .collect();
    assert_eq!(values.len(), 16);
    LiteralEquationPositions {
        flags: values[2],
        option: values[3],
        texts: std::array::from_fn(|index| (values[4 + index * 2], values[5 + index * 2])),
    }
}

fn matrix_source(kind: usize, style: usize) -> PartResolvedFeaturesArchive {
    read_owned_resolved_document(
        &equation_fixture_document(OWNED_EQUATIONS[kind][style]),
        EQUATION_LIMITS,
    )
}

fn patched_literal(original: &[u8], span: Range<usize>, replacement: &[u8]) -> Vec<u8> {
    assert!(span.start <= span.end && span.end <= original.len());
    let mut expected = original[..span.start].to_vec();
    expected.extend_from_slice(replacement);
    expected.extend_from_slice(&original[span.end..]);
    expected
}

#[test]
fn every_owned_lock_request_matches_literal_words_and_preserves_all_other_fields() {
    for (kind, style) in (0..2).flat_map(|kind| (0..6).map(move |style| (kind, style))) {
        let source = matrix_source(kind, style);
        let state = source.spline_equation_states().unwrap().remove(0);
        let offset = literal_positions(kind, style).flags;
        for (request, word) in LOCK_IMAGES[style].iter().enumerate() {
            let edit = lock_request(request);
            let result = source.edit_spline_equation(&state, edit).unwrap();
            let expected = patched_literal(
                OWNED_EQUATIONS[kind][style],
                offset..offset + 4,
                &word.to_le_bytes(),
            );
            assert_matrix_edit(
                &source,
                &result,
                state.source_fields()[0].index(),
                &expected,
            );
        }
    }
}

fn lock_request(request: usize) -> ModelSplineEquationStoredEdit {
    match request {
        0 => ModelSplineEquationStoredEdit::LockStart(false),
        1 => ModelSplineEquationStoredEdit::LockStart(true),
        2 => ModelSplineEquationStoredEdit::LockEnd(false),
        3 => ModelSplineEquationStoredEdit::LockEnd(true),
        _ => panic!("lock request {request}; expected0..4"),
    }
}

#[test]
fn every_signed_option_boundary_matches_independent_little_endian_word_images() {
    for (kind, style) in (0..2).flat_map(|kind| (0..6).map(move |style| (kind, style))) {
        let source = matrix_source(kind, style);
        let state = source.spline_equation_states().unwrap().remove(0);
        let offset = literal_positions(kind, style).option;
        check_option_requests(&source, &state, OWNED_EQUATIONS[kind][style], offset);
    }
}

fn check_option_requests(
    source: &PartResolvedFeaturesArchive,
    state: &ModelSplineEquationStoredState,
    original: &[u8],
    offset: usize,
) {
    for value in SIGNED_OPTIONS {
        let edit = ModelSplineEquationStoredEdit::SavedOption(value);
        let result = source.edit_spline_equation(state, edit).unwrap();
        let expected = patched_literal(original, offset..offset + 4, &value.to_le_bytes());
        assert_matrix_edit(source, &result, state.source_fields()[8].index(), &expected);
    }
}

fn assert_matrix_edit(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    selected: usize,
    expected: &[u8],
) {
    assert_eq!(result.archive().to_bytes().unwrap(), expected);
    let changed = source.to_bytes().unwrap() != expected;
    assert_eq!(
        result.completion(),
        if changed {
            ModelEditCompletion::Changed
        } else {
            ModelEditCompletion::Unchanged
        }
    );
    assert_matrix_coverage(source, result, expected, changed);
    assert_unselected_equation_fields(source, result, selected);
    assert_matrix_history(source, result, selected, changed);
    let document = equation_fixture_document(&source.to_bytes().unwrap());
    assert_reopened_equation_document(&document, source, result);
    exports::export_actual_equation_result(result);
}

fn assert_matrix_coverage(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    expected: &[u8],
    changed: bool,
) {
    assert_eq!(
        result.coverage(),
        ModelArchiveCoverage {
            wire_bytes: expected.len(),
            ..source.coverage()
        }
    );
    assert_eq!(result.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(result.diagnostics(), source.diagnostics());
    assert_eq!(
        result.unchanged_fields(),
        source.fields().len() - usize::from(changed)
    );
    assert_eq!(result.changes().len(), usize::from(changed));
}

fn assert_unselected_equation_fields(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    selected: usize,
) {
    assert_eq!(result.archive().fields().len(), source.fields().len());
    for (index, (before, after)) in source
        .fields()
        .iter()
        .zip(result.archive().fields())
        .enumerate()
    {
        if index != selected {
            assert_eq!(before, after);
        }
    }
}

fn assert_matrix_history(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    selected: usize,
    changed: bool,
) {
    if let Some(relation) = result.changes().first() {
        assert_eq!(relation.relation(), ModelFieldRelation::Derived);
        assert_eq!(relation.path(), source.fields()[selected].path.as_ref());
        assert_eq!(
            relation.parents(),
            std::slice::from_ref(&source.field_handle(selected).unwrap())
        );
        assert_eq!(
            relation.result(),
            Some(&result.archive().field_handle(selected).unwrap())
        );
    }
    assert_eq!(
        result
            .archive()
            .resolve_field(&source.field_handle(selected).unwrap())
            .is_ok(),
        !changed
    );
}

fn text_matrix_requests() -> Vec<(ArchiveString, Vec<u8>)> {
    vec![
        text_request("", TextEncoding::Latin1, &[0]),
        text_request("A\0é", TextEncoding::Latin1, &[3]),
        text_request("λ 😀", TextEncoding::Utf16, &[0xff, 0xfe, 0xff, 4]),
        text_request("x\r\n\0y", TextEncoding::Utf16, &[0xff, 0xfe, 0xff, 5]),
        text_request(&"a".repeat(254), TextEncoding::Latin1, &[254]),
        text_request(&"a".repeat(255), TextEncoding::Latin1, &[0xff, 0xff, 0]),
        text_request(&"a".repeat(256), TextEncoding::Latin1, &[0xff, 0, 1]),
        text_request(
            &"😀".repeat(127),
            TextEncoding::Utf16,
            &[0xff, 0xfe, 0xff, 254],
        ),
        text_request(
            &"😀".repeat(128),
            TextEncoding::Utf16,
            &[0xff, 0xfe, 0xff, 0xff, 0, 1],
        ),
        text_request("e\u{301}", TextEncoding::Utf16, &[0xff, 0xfe, 0xff, 2]),
    ]
}

fn text_request(text: &str, encoding: TextEncoding, prefix: &[u8]) -> (ArchiveString, Vec<u8>) {
    let mut expected = prefix.to_vec();
    match encoding {
        TextEncoding::Latin1 => expected.extend(
            text.chars()
                .map(|character| u8::try_from(u32::from(character)).unwrap()),
        ),
        TextEncoding::Utf16 => expected.extend(text.encode_utf16().flat_map(u16::to_le_bytes)),
    }
    (ArchiveString::new(text, encoding).unwrap(), expected)
}

#[test]
fn all_text_slots_preserve_literal_encoding_boundaries_nul_and_unicode() {
    for (kind, style) in (0..2).flat_map(|kind| (0..6).map(move |style| (kind, style))) {
        check_text_matrix_case(kind, style);
    }
}

fn check_text_matrix_case(kind: usize, style: usize) {
    let source = matrix_source(kind, style);
    let state = source.spline_equation_states().unwrap().remove(0);
    let positions = literal_positions(kind, style);
    for (slot, role) in ROLES.iter().enumerate() {
        for (value, frame) in text_matrix_requests() {
            let edit = ModelSplineEquationStoredEdit::Text { role: *role, value };
            let result = source.edit_spline_equation(&state, edit).unwrap();
            let (offset, length) = positions.texts[slot];
            let expected = patched_literal(
                OWNED_EQUATIONS[kind][style],
                offset..offset + length,
                &frame,
            );
            let selected = state.source_fields()[slot + 1].index();
            assert_matrix_edit(&source, &result, selected, &expected);
        }
    }
}

#[test]
fn typed_text_edits_keep_every_admitted_count_prefix_transition() {
    let widths = (0..3).flat_map(|before| (0..3).map(move |after| (before, after)));
    for (before, after) in widths {
        for wide in [false, true] {
            check_typed_prefix_transition(wide, before, after);
        }
    }
}

fn check_typed_prefix_transition(wide: bool, before: usize, after: usize) {
    let original = cstring_framing_fixture::cstring_equation_wire(wide, before);
    let expected = cstring_framing_fixture::cstring_equation_wire(wide, after);
    let source =
        read_owned_resolved_document(&equation_fixture_document(&original), EQUATION_LIMITS);
    let target =
        read_owned_resolved_document(&equation_fixture_document(&expected), EQUATION_LIMITS);
    let state = source.spline_equation_states().unwrap().remove(0);
    let target = target.spline_equation_states().unwrap().remove(0);
    let role = ModelSplineEquationTextRole::XExpression;
    let edit = ModelSplineEquationStoredEdit::Text {
        role,
        value: target.text(role).clone(),
    };
    let result = source.edit_spline_equation(&state, edit).unwrap();
    assert_matrix_edit(
        &source,
        &result,
        state.source_fields()[1].index(),
        &expected,
    );
}
