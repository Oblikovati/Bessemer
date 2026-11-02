use super::*;

pub(super) fn choices(
    body: &SimulationPostGraphParameters,
) -> Vec<(usize, SimulationPostGraphValue)> {
    body.fields()
        .iter()
        .enumerate()
        .flat_map(|(index, field)| {
            replacements(&field.value)
                .into_iter()
                .filter(|value| value != &field.value)
                .map(move |value| (index, value))
        })
        .collect()
}

fn replacements(value: &SimulationPostGraphValue) -> Vec<SimulationPostGraphValue> {
    match value {
        SimulationPostGraphValue::Signed(word) => vec![SimulationPostGraphValue::Signed(word ^ 32)],
        SimulationPostGraphValue::Raw64(raw) => {
            let mut changed = *raw;
            changed[0] ^= 255;
            vec![SimulationPostGraphValue::Raw64(changed)]
        }
        SimulationPostGraphValue::Text(_) => text_replacements(),
    }
}

fn text_replacements() -> Vec<SimulationPostGraphValue> {
    [
        &b""[..],
        &b"A\0"[..],
        &b"\0\xd8"[..],
        &b"\0\0\xff\xff"[..],
        &b"Longer\0\0"[..],
    ]
    .into_iter()
    .map(|raw| {
        SimulationPostGraphValue::Text(
            SimulationPostGraphText::from_payload(raw, LIMITS.maximum_text_bytes).unwrap(),
        )
    })
    .collect()
}

pub(super) fn replace(
    body: &SimulationPostGraphParameters,
    index: usize,
    replacement: SimulationPostGraphValue,
) -> SimulationPostGraphEdit {
    let original = body.to_bytes();
    let (start, end) = body.field_offsets()[index];
    let wire = graph_wire(&replacement).1;
    let expected = [&original[..start], &wire, &original[end..]].concat();
    let parent = body.field_id(index).unwrap();
    let edit = body.edit_field(&parent, replacement.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), expected);
    assert_eq!(edit.source().to_bytes(), original);
    assert_eq!(body.to_bytes(), original);
    assert_eq!(edit.parent(), parent);
    assert_eq!(edit.replacement(), &replacement);
    assert_ne!(edit.result(), parent);
    assert_preserved_fields(body, &edit, index, wire.len());
    assert_edit_result(body, &edit, index);
    edit
}

fn assert_preserved_fields(
    body: &SimulationPostGraphParameters,
    edit: &SimulationPostGraphEdit,
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

fn assert_edit_result(
    body: &SimulationPostGraphParameters,
    edit: &SimulationPostGraphEdit,
    changed: usize,
) {
    typed_load_history::assert_outer(
        body.fields().len(),
        |index| body.field_id(index).unwrap(),
        |index| edit.archive().field_id(index).unwrap(),
        edit.history(),
        changed,
    );
    assert!(edit.nested_history().is_empty());
    assert_constructed(edit.archive());
    assert_edit_coverage(edit);
}

fn assert_edit_coverage(edit: &SimulationPostGraphEdit) {
    assert_eq!(
        edit.coverage().completion,
        SimulationPostGraphCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationPostGraphCheckMode::ExactSelectedProfilesAndPreservation
    );
    let characters: usize = edit
        .archive()
        .fields()
        .iter()
        .map(|field| match &field.value {
            SimulationPostGraphValue::Text(text) => text.character_count(),
            _ => 0,
        })
        .sum();
    assert_eq!(edit.coverage().declared_entries, characters);
}

#[test]
fn every_owned_word_raw64_and_atomic_text_supports_isolated_changed_edits() {
    let mut counts = [0; 3];
    contracts::each_owned(|body| {
        for (index, replacement) in choices(&body) {
            let tag = match replacement {
                SimulationPostGraphValue::Signed(_) => 0,
                SimulationPostGraphValue::Raw64(_) => 1,
                SimulationPostGraphValue::Text(_) => 2,
            };
            counts[tag] += 1;
            replace(&body, index, replacement);
        }
    });
    assert_eq!(counts, [12720, 4260, 4410]);
    println!("complete graph changed-value counts: {counts:?}");
}
