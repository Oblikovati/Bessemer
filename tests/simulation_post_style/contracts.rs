use super::*;

fn changed_value(value: &SimulationPostStyleValue) -> SimulationPostStyleValue {
    match value {
        SimulationPostStyleValue::Signed(word) => SimulationPostStyleValue::Signed(word ^ 32),
        SimulationPostStyleValue::RawWordBits(bits) => {
            SimulationPostStyleValue::RawWordBits(bits ^ 32)
        }
        SimulationPostStyleValue::Float32Bits(bits) => {
            SimulationPostStyleValue::Float32Bits(bits ^ 32)
        }
        SimulationPostStyleValue::Text(text) => {
            let mut raw = text.payload().to_vec();
            raw.extend_from_slice(&[0, 0xd8]);
            SimulationPostStyleValue::Text(
                SimulationPostStyleText::from_payload(&raw, LIMITS.maximum_text_bytes).unwrap(),
            )
        }
    }
}

fn assert_changed(body: &SimulationPostStyle, ordinal: usize) {
    let original = body.to_bytes();
    let id = body.field_id(ordinal).unwrap();
    let replacement = changed_value(&body.field(&id).unwrap().value);
    let edit = body.edit_field(&id, replacement.clone()).unwrap();
    assert_eq!(edit.source().to_bytes(), original);
    assert_eq!(body.to_bytes(), original);
    assert_eq!(
        edit.archive().field(&edit.result()).unwrap().value,
        replacement
    );
    assert_eq!(edit.parent(), id);
    assert_eq!(edit.replacement(), &replacement);
    assert_eq!(
        edit.coverage().completion,
        SimulationPostStyleCompletion::Changed
    );
    assert!(edit.archive().field(&id).is_err());
    assert_eq!(edit.history().len(), body.fields().len());
    assert!(edit.nested_history().is_empty());
    assert_preserved_style_extents(body, &edit, ordinal);
}

#[test]
fn every_field_changes_and_noops_preserve_complete_inputs() {
    for body in owned_styles() {
        for ordinal in 0..body.fields().len() {
            let id = body.field_id(ordinal).unwrap();
            let value = body.field(&id).unwrap().value.clone();
            let noop = body.edit_field(&id, value).unwrap();
            assert_eq!(
                noop.coverage().completion,
                SimulationPostStyleCompletion::Unchanged
            );
            assert_eq!(noop.result(), id);
            assert_eq!(noop.archive().to_bytes(), body.to_bytes());
            assert_changed(&body, ordinal);
        }
    }
}

#[test]
fn wrong_kinds_paths_foreign_ids_and_caller_mutation_are_rejected_or_isolated() {
    let body = owned_styles().remove(0);
    let original = body.to_bytes();
    let mut supplied = original.clone();
    let copied = SimulationPostStyle::from_bytes(&supplied, body.version(), LIMITS).unwrap();
    supplied.fill(0xff);
    assert_eq!(copied.to_bytes(), original);
    assert!(copied.field(&body.field_id(0).unwrap()).is_err());
    assert!(body.field_id(body.fields().len()).is_err());
    assert!(
        body.edit_field(
            &body.field_id(0).unwrap(),
            SimulationPostStyleValue::Float32Bits(0)
        )
        .is_err()
    );
    let mut fields = body.fields().to_vec();
    fields[0].path = "unexpected".into();
    assert!(SimulationPostStyle::from_fields(fields, body.version(), LIMITS).is_err());
    assert_eq!(body.to_bytes(), original);
}

#[test]
fn isolated_operations_match_single_thread_results() {
    let body = owned_styles().remove(0);
    let expected = body
        .edit_field(
            &body.field_id(0).unwrap(),
            SimulationPostStyleValue::RawWordBits(123),
        )
        .unwrap()
        .archive()
        .to_bytes();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (body, expected) = (&body, &expected);
            scope.spawn(move || assert_repeated_style_edit(body, expected));
        }
    });
}

fn assert_preserved_style_extents(
    body: &SimulationPostStyle,
    edit: &SimulationPostStyleEdit,
    ordinal: usize,
) {
    let original = body.to_bytes();
    let (start, end) = body.field_offsets()[ordinal];
    let (_, edited_end) = edit.archive().field_offsets()[ordinal];
    assert_eq!(original[..start], edit.archive().to_bytes()[..start]);
    assert_eq!(original[end..], edit.archive().to_bytes()[edited_end..]);
}

fn assert_repeated_style_edit(body: &SimulationPostStyle, expected: &[u8]) {
    for _ in 0..100 {
        let edit = body
            .edit_field(
                &body.field_id(0).unwrap(),
                SimulationPostStyleValue::RawWordBits(123),
            )
            .unwrap();
        assert_eq!(edit.archive().to_bytes(), expected);
    }
}
