use super::*;
use bessemer::history::{ArchiveString, TextEncoding};

const TEXT_ROLES: [ModelSplineEquationTextRole; 6] = [
    ModelSplineEquationTextRole::XExpression,
    ModelSplineEquationTextRole::YExpression,
    ModelSplineEquationTextRole::ZExpression,
    ModelSplineEquationTextRole::DeclaredCharacters,
    ModelSplineEquationTextRole::RangeStart,
    ModelSplineEquationTextRole::RangeEnd,
];

fn typed_equation_source(kind: usize, style: usize) -> PartResolvedFeaturesArchive {
    let document = equation_fixture_document(OWNED_EQUATIONS[kind][style]);
    read_owned_resolved_document(&document, EQUATION_LIMITS)
}

#[test]
fn typed_states_cover_both_prototypes_all_saved_roles_and_empty_equation_sections() {
    for (kind, style) in (0..2).flat_map(|kind| (0..8).map(move |style| (kind, style))) {
        let source = typed_equation_source(kind, style);
        let states = source.spline_equation_states().unwrap();
        assert_eq!(states.len(), usize::from(style < 6));
        if let Some(state) = states.first() {
            assert_typed_roles(&source, state, style);
        }
    }
}

fn assert_typed_roles(
    source: &PartResolvedFeaturesArchive,
    state: &ModelSplineEquationStoredState,
    style: usize,
) {
    assert_eq!(state.flags_word(), EQUATION_FLAGS[style]);
    assert_eq!(state.lock_start(), EQUATION_FLAGS[style] & 0x10 != 0);
    assert_eq!(state.lock_end(), EQUATION_FLAGS[style] & 0x20 != 0);
    assert_eq!(state.saved_option(), EQUATION_OPTIONS[style]);
    assert_eq!(state.compatibility_word(), i32::MIN);
    for (slot, role) in TEXT_ROLES.iter().enumerate() {
        assert_eq!(state.text(*role).text(), format!("Ω equation {slot}"));
    }
    assert_eq!(state.coverage(), source.coverage());
    assert_eq!(state.completion(), ModelEditCompletion::Unchanged);
    assert_eq!(state.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(state.achieved_stored_value_error(), 0);
    assert_eq!(state.diagnostics().len(), 3);
    assert_eq!(
        state.source().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    assert_equation_copies(source, state);
}

fn assert_equation_copies(
    source: &PartResolvedFeaturesArchive,
    state: &ModelSplineEquationStoredState,
) {
    assert_eq!(state.history().len(), 9);
    for (field, relation) in state.source_fields().iter().zip(state.history()) {
        assert_eq!(&relation.parents[0], field);
        assert_eq!(&relation.result, field);
        assert_eq!(
            state.source().resolve_field(&relation.parents[0]).unwrap(),
            source.resolve_field(field).unwrap()
        );
    }
    let selected = source
        .spline_equation_state(&state.source_fields()[0])
        .unwrap();
    assert_eq!(selected.source_fields(), state.source_fields());
}

#[test]
fn typed_lock_edit_matches_independent_goldens_and_complete_document_reversal() {
    for (kind, golden) in EQUATION_LOCK_GOLDENS.iter().enumerate() {
        let document = equation_fixture_document(OWNED_EQUATIONS[kind][0]);
        let source = read_owned_resolved_document(&document, EQUATION_LIMITS);
        let state = source.spline_equation_states().unwrap().remove(0);
        let edit = source
            .edit_spline_equation(&state, ModelSplineEquationStoredEdit::LockStart(true))
            .unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), *golden);
        assert_eq!(edit.completion(), ModelEditCompletion::Changed);
        assert_eq!(edit.changes().len(), 1);
        assert_reopened_equation_document(&document, &source, &edit);
    }
}

#[test]
fn full_signed_options_and_all_text_slots_use_complete_validated_archive_edits() {
    let source = typed_equation_source(0, 0);
    let state = source.spline_equation_states().unwrap().remove(0);
    for value in [i32::MIN, -1, 0, 1, 2, i32::MAX] {
        let edit = source
            .edit_spline_equation(&state, ModelSplineEquationStoredEdit::SavedOption(value))
            .unwrap();
        let current = edit.archive().spline_equation_states().unwrap().remove(0);
        assert_eq!(current.saved_option(), value);
        assert_eq!(current.flags_word(), state.flags_word());
    }
    for role in TEXT_ROLES {
        let value = ArchiveString::new("λ replacement", TextEncoding::Utf16).unwrap();
        let edit = source
            .edit_spline_equation(&state, ModelSplineEquationStoredEdit::Text { role, value })
            .unwrap();
        let current = edit.archive().spline_equation_states().unwrap().remove(0);
        assert_eq!(current.text(role).text(), "λ replacement");
        assert_eq!(edit.changes().len(), 1);
    }
}

#[test]
fn foreign_and_wrong_selections_fail_without_source_mutation() {
    let source = typed_equation_source(0, 0);
    let state = source.spline_equation_states().unwrap().remove(0);
    let foreign = typed_equation_source(0, 0);
    assert!(
        foreign
            .spline_equation_state(&state.source_fields()[0])
            .is_err()
    );
    assert!(
        foreign
            .edit_spline_equation(&state, ModelSplineEquationStoredEdit::LockEnd(true))
            .is_err()
    );
    assert!(
        source
            .spline_equation_state(&source.field_handle(0).unwrap())
            .is_err()
    );
    assert_eq!(source.to_bytes().unwrap(), OWNED_EQUATIONS[0][0]);
}

#[test]
fn unchanged_option_requests_keep_source_revision_and_field_identity() {
    let source = typed_equation_source(0, 0);
    let state = source.spline_equation_states().unwrap().remove(0);
    let noop = source
        .edit_spline_equation(
            &state,
            ModelSplineEquationStoredEdit::SavedOption(state.saved_option()),
        )
        .unwrap();
    assert_eq!(noop.completion(), ModelEditCompletion::Unchanged);
    assert!(
        noop.archive()
            .resolve_field(&state.source_fields()[0])
            .is_ok()
    );
}

#[test]
fn changed_results_reject_stale_state_parents_without_mutating_original() {
    let source = typed_equation_source(0, 0);
    let state = source.spline_equation_states().unwrap().remove(0);
    let changed = source
        .edit_spline_equation(&state, ModelSplineEquationStoredEdit::LockEnd(true))
        .unwrap();
    assert!(
        changed
            .archive()
            .edit_spline_equation(&state, ModelSplineEquationStoredEdit::SavedOption(7))
            .is_err()
    );
    assert_eq!(source.to_bytes().unwrap(), OWNED_EQUATIONS[0][0]);
}

#[test]
fn all_published_equation_types_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<ModelSplineEquationTextRole>();
    require::<ModelSplineEquationStoredEdit>();
    require::<ModelSplineEquationFieldCopy>();
    require::<ModelSplineEquationDiagnostic>();
    require::<ModelSplineEquationStoredState>();
}
