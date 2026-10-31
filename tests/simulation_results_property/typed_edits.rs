use super::*;

pub(super) fn choices(body: &SimulationResultsProperty) -> Vec<(usize, SimulationResultsValue)> {
    body.fields()
        .iter()
        .enumerate()
        .flat_map(|(index, field)| {
            replacements(field)
                .into_iter()
                .filter(|value| value != &field.value)
                .map(move |value| (index, value))
        })
        .collect()
}

fn replacements(field: &SimulationResultsField) -> Vec<SimulationResultsValue> {
    match &field.value {
        SimulationResultsValue::Signed(word) if field.path.as_ref() != "text_list.count" => {
            vec![SimulationResultsValue::Signed(word ^ 32)]
        }
        SimulationResultsValue::Unsigned(word) => {
            vec![SimulationResultsValue::Unsigned(word ^ 0x80000000)]
        }
        SimulationResultsValue::Text(_) => text_replacements(),
        _ => Vec::new(),
    }
}

fn text_replacements() -> Vec<SimulationResultsValue> {
    [
        None,
        Some(&b"\0\0"[..]),
        Some(&b"X\0\0"[..]),
        Some(&b"\0\xd8\0\0"[..]),
        Some(&b"Long\0\0"[..]),
    ]
    .into_iter()
    .map(|raw| {
        SimulationResultsValue::Text(
            SimulationPersistText::from_payload(raw, LIMITS.maximum_text_bytes).unwrap(),
        )
    })
    .collect()
}

pub(super) fn replace(
    body: &SimulationResultsProperty,
    index: usize,
    replacement: SimulationResultsValue,
) -> SimulationResultsEdit {
    let original = body.to_bytes();
    let (start, end) = body.field_offsets()[index];
    let wire = typed_load_values::wire(&replacement);
    let expected = [&original[..start], &wire, &original[end..]].concat();
    let parent = body.field_id(index).unwrap();
    let edit = body.edit_field(&parent, replacement.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), expected);
    assert_eq!(body.to_bytes(), original);
    assert_eq!(edit.source().to_bytes(), original);
    assert_eq!(edit.parent(), parent);
    assert_eq!(edit.replacement(), &replacement);
    assert_ne!(edit.result(), parent);
    assert_preservation(body, &edit, index, wire.len());
    edit
}

fn assert_preservation(
    body: &SimulationResultsProperty,
    edit: &SimulationResultsEdit,
    changed: usize,
    width: usize,
) {
    assert_field_spans(body, edit, changed, width);
    assert_constructed(edit.archive());
    typed_load_history::assert_outer(
        body.fields().len(),
        |index| body.field_id(index).unwrap(),
        |index| edit.archive().field_id(index).unwrap(),
        edit.history(),
        changed,
    );
    assert!(edit.nested_history().is_empty());
    assert_edit_coverage(body, edit);
}

fn assert_edit_coverage(body: &SimulationResultsProperty, edit: &SimulationResultsEdit) {
    assert_eq!(
        edit.coverage().completion,
        SimulationResultsCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationResultsCheckMode::ExactSelectedProfilesAndPreservation
    );
    assert_eq!(edit.coverage().context, body.context());
    assert_eq!(
        edit.coverage().declared_entries,
        body.coverage().declared_entries
    );
}

fn assert_field_spans(
    body: &SimulationResultsProperty,
    edit: &SimulationResultsEdit,
    changed: usize,
    width: usize,
) {
    assert_eq!(body.fields().len(), edit.archive().fields().len());
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

#[test]
fn every_stored_signed_unsigned_and_raw_text_value_supports_isolated_changed_edits() {
    let mut counts = [0; 6];
    contracts::each_owned(|_, _, body| {
        for (index, value) in choices(&body) {
            counts[typed_load_values::value_kind(&value)] += 1;
            replace(&body, index, value);
        }
    });
    assert_eq!(counts, [1030, 370, 0, 1635, 0, 0]);
    println!("complete changed-value counts: {counts:?}");
}
