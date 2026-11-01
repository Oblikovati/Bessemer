use super::*;

fn changed_value(value: &SimulationPostInfoValue) -> SimulationPostInfoValue {
    match value {
        SimulationPostInfoValue::Signed(word) => SimulationPostInfoValue::Signed(word ^ 32),
        SimulationPostInfoValue::RawWordBits(bits) => {
            SimulationPostInfoValue::RawWordBits(bits ^ 32)
        }
        SimulationPostInfoValue::Float32Bits(bits) => {
            SimulationPostInfoValue::Float32Bits(bits ^ 32)
        }
        SimulationPostInfoValue::Text(text) => {
            let mut raw = text.payload().to_vec();
            raw.extend_from_slice(&[0, 0xd8]);
            SimulationPostInfoValue::Text(
                SimulationPostInfoText::from_payload(&raw, LIMITS.maximum_text_bytes).unwrap(),
            )
        }
    }
}

fn assert_changed(body: &SimulationPostInfo, ordinal: usize) {
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
        SimulationPostInfoCompletion::Changed
    );
    assert!(edit.archive().field(&id).is_err());
    assert_eq!(edit.history().len(), body.fields().len());
    assert!(edit.nested_history().is_empty());
    let (start, end) = body.field_offsets()[ordinal];
    let (_, edited_end) = edit.archive().field_offsets()[ordinal];
    assert_eq!(original[..start], edit.archive().to_bytes()[..start]);
    assert_eq!(original[end..], edit.archive().to_bytes()[edited_end..]);
}

#[test]
fn every_field_changes_and_noops_preserve_complete_inputs() {
    for body in owned_infos() {
        for ordinal in 0..body.fields().len() {
            let id = body.field_id(ordinal).unwrap();
            let value = body.field(&id).unwrap().value.clone();
            let noop = body.edit_field(&id, value).unwrap();
            assert_eq!(
                noop.coverage().completion,
                SimulationPostInfoCompletion::Unchanged
            );
            assert_eq!(noop.result(), id);
            assert_eq!(noop.archive().to_bytes(), body.to_bytes());
            assert_changed(&body, ordinal);
        }
    }
}

#[test]
fn wrong_kinds_paths_foreign_ids_and_caller_mutation_are_rejected_or_isolated() {
    let body = owned_infos().remove(0);
    let original = body.to_bytes();
    let mut supplied = original.clone();
    let copied = SimulationPostInfo::from_bytes(&supplied, body.version(), LIMITS).unwrap();
    supplied.fill(0xff);
    assert_eq!(copied.to_bytes(), original);
    assert!(copied.field(&body.field_id(0).unwrap()).is_err());
    assert!(body.field_id(body.fields().len()).is_err());
    assert!(
        body.edit_field(
            &body.field_id(0).unwrap(),
            SimulationPostInfoValue::Float32Bits(0)
        )
        .is_err()
    );
    let mut fields = body.fields().to_vec();
    fields[0].path = "unexpected".into();
    assert!(SimulationPostInfo::from_fields(fields, body.version(), LIMITS).is_err());
    assert_eq!(body.to_bytes(), original);
}

#[test]
fn isolated_operations_match_single_thread_results() {
    let body = owned_infos().remove(0);
    let expected = body
        .edit_field(
            &body.field_id(0).unwrap(),
            SimulationPostInfoValue::RawWordBits(123),
        )
        .unwrap()
        .archive()
        .to_bytes();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (body, expected) = (&body, &expected);
            scope.spawn(move || {
                for _ in 0..100 {
                    let edit = body
                        .edit_field(
                            &body.field_id(0).unwrap(),
                            SimulationPostInfoValue::RawWordBits(123),
                        )
                        .unwrap();
                    assert_eq!(&edit.archive().to_bytes(), expected);
                }
            });
        }
    });
}
