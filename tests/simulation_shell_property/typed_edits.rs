use super::*;

pub(super) fn replace(
    body: &SimulationShellProperty,
    index: usize,
    replacement: SimulationShellPropertyValue,
) -> SimulationShellPropertyEdit {
    let raw = body.to_bytes();
    let (start, end) = body.field_offsets()[index];
    let wire = typed_values::wire(&replacement);
    let expected = [&raw[..start], &wire, &raw[end..]].concat();
    let parent = body.field_id(index).unwrap();
    let edit = body.edit_field(&parent, replacement.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), expected);
    assert_eq!(edit.source().to_bytes(), raw);
    assert_eq!(body.to_bytes(), raw);
    assert_eq!(edit.parent(), parent);
    assert_eq!(edit.replacement(), &replacement);
    assert_ne!(edit.result(), parent);
    assert_preservation(body, &edit, index, wire.len());
    typed_history::assert_outer(body, &edit, index);
    typed_history::assert_nested(body, &edit, index);
    edit
}

fn assert_preservation(
    body: &SimulationShellProperty,
    edit: &SimulationShellPropertyEdit,
    changed: usize,
    width: usize,
) {
    let (start, end) = body.field_offsets()[changed];
    let delta = width as isize - (end - start) as isize;
    assert_eq!(edit.archive().context(), body.context());
    assert_eq!(edit.archive().fields().len(), body.fields().len());
    assert_field_spans(body, edit, changed, width);
    assert_edit_coverage(body, edit, end, delta);
}

fn assert_field_spans(
    body: &SimulationShellProperty,
    edit: &SimulationShellPropertyEdit,
    changed: usize,
    width: usize,
) {
    for (index, field) in body.fields().iter().enumerate() {
        if index != changed {
            assert_eq!(field, &edit.archive().fields()[index]);
        }
        let expected = typed_load_values::shifted_span(
            body.field_offsets()[index],
            body.field_offsets()[changed],
            index,
            changed,
            width,
        );
        assert_eq!(edit.archive().field_offsets()[index], expected);
    }
}

fn assert_edit_coverage(
    body: &SimulationShellProperty,
    edit: &SimulationShellPropertyEdit,
    end: usize,
    delta: isize,
) {
    let base = body.coverage().base_end;
    let expected_base = if end <= base {
        base.checked_add_signed(delta).unwrap()
    } else {
        base
    };
    assert_eq!(edit.coverage().base_end, expected_base);
    assert_eq!(
        edit.coverage().completion,
        SimulationShellPropertyCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationShellPropertyCheckMode::ExactSelectedProfilesAndPreservation
    );
}

#[test]
fn all_owned_contexts_support_six_value_kinds_variable_text_and_complete_supplied_archive_provenance()
 {
    let mut counts = [0_usize; 6];
    for body in contracts::owned_properties() {
        for (index, replacement) in typed_values::choices(&body) {
            let kind = value_kind(&replacement);
            replace(&body, index, replacement);
            counts[kind] += 1;
        }
    }
    assert_eq!(counts, [2284, 2284, 3168, 48894, 2034, 1656]);
}

pub(super) use super::typed_load_values::value_kind;
