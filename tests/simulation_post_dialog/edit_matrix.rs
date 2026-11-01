use super::*;
use bessemer::model_archives::{SimulationEmbeddedArchive, SimulationEmbeddedProfile};

struct DialogEditControl<'a> {
    ordinal: usize,
    tag: u32,
    grammar: bool,
    isolated: bool,
    span: (usize, usize),
    replacement: &'a [u8],
}

fn read_control<'a>(cursor: &mut &'a [u8]) -> DialogEditControl<'a> {
    let ordinal = read_word(cursor) as usize;
    let tag = read_word(cursor);
    let grammar = read_word(cursor) != 0;
    let isolated = read_word(cursor) != 0;
    let span = (read_word(cursor) as usize, read_word(cursor) as usize);
    let length = read_word(cursor) as usize;
    DialogEditControl {
        ordinal,
        tag,
        grammar,
        isolated,
        span,
        replacement: take_projection(cursor, length),
    }
}

fn replacement(control: &DialogEditControl<'_>) -> SimulationPostDialogValue {
    let raw = control.replacement;
    match control.tag {
        0 => SimulationPostDialogValue::Signed(i32::from_le_bytes(raw.try_into().unwrap())),
        7 => SimulationPostDialogValue::RawWordBits(u32::from_le_bytes(raw.try_into().unwrap())),
        8 => SimulationPostDialogValue::Float32Bits(u32::from_le_bytes(raw.try_into().unwrap())),
        9 => SimulationPostDialogValue::Raw64(raw.try_into().unwrap()),
        3 => SimulationPostDialogValue::PersistText(
            SimulationPersistText::read_from(raw, 0, LIMITS.maximum_text_bytes).unwrap(),
        ),
        _ => variable_replacement(control),
    }
}

fn variable_replacement(control: &DialogEditControl<'_>) -> SimulationPostDialogValue {
    let raw = control.replacement;
    match control.tag {
        6 => SimulationPostDialogValue::CharacterText(
            SimulationPostDialogText::from_payload(&raw[4..], LIMITS.maximum_text_bytes).unwrap(),
        ),
        10 => SimulationPostDialogValue::CountedBytes(
            SimulationPostDialogBytes::from_payload(&raw[4..], LIMITS.maximum_bytes).unwrap(),
        ),
        5 => SimulationPostDialogValue::Embedded(Box::new(embedded_replacement(raw))),
        tag => panic!("{tag}; expected character/byte/embedded replacement"),
    }
}

fn embedded_replacement(raw: &[u8]) -> SimulationEmbeddedArchive {
    let version = u32::from_le_bytes(raw[..4].try_into().unwrap());
    let profile = if version == 4281 {
        SimulationEmbeddedProfile::Legacy4281
    } else {
        SimulationEmbeddedProfile::Modern
    };
    SimulationEmbeddedArchive::from_bytes(raw, profile, LIMITS.embedded).unwrap()
}

fn expected_image(body: &SimulationPostDialog, control: &DialogEditControl<'_>) -> Vec<u8> {
    let original = body.to_bytes();
    let mut bytes = original[..control.span.0].to_vec();
    bytes.extend_from_slice(control.replacement);
    bytes.extend_from_slice(&original[control.span.1..]);
    bytes
}

fn assert_result_fields(
    body: &SimulationPostDialog,
    edit: &SimulationPostDialogEdit,
    control: &DialogEditControl<'_>,
) {
    let after = edit.archive();
    assert_eq!(after.fields().len(), body.fields().len());
    for (index, (old, new)) in body.fields().iter().zip(after.fields()).enumerate() {
        assert_eq!(new.path, old.path);
        let expected = if index == control.ordinal {
            control.replacement.to_vec()
        } else {
            encoded_value(&old.value).1
        };
        assert_eq!(
            encoded_value(&new.value),
            (encoded_value(&old.value).0, expected)
        );
        assert_field_span(body, after, index, control);
    }
}

fn assert_nested_replacement(
    edit: &SimulationPostDialogEdit,
    control: &DialogEditControl<'_>,
    value: &SimulationPostDialogValue,
) {
    let SimulationPostDialogValue::Embedded(before) =
        &edit.source().fields()[control.ordinal].value
    else {
        return;
    };
    let SimulationPostDialogValue::Embedded(after) =
        &edit.archive().fields()[control.ordinal].value
    else {
        panic!("expected retained embedded archive");
    };
    let SimulationPostDialogValue::Embedded(supplied) = value else {
        panic!("expected complete supplied archive");
    };
    let rows = typed_history::archived_rows(edit.nested_history(), edit.parent(), edit.result());
    typed_history::assert_nested_parents(before, after, Some(supplied), &rows);
}

fn assert_changed(
    body: &SimulationPostDialog,
    edit: &SimulationPostDialogEdit,
    control: &DialogEditControl<'_>,
    value: &SimulationPostDialogValue,
) {
    assert_eq!(edit.source().to_bytes(), body.to_bytes());
    assert_eq!(edit.archive().to_bytes(), expected_image(body, control));
    assert_eq!(edit.archive().field(&edit.result()).unwrap().value, *value);
    assert_eq!(edit.replacement(), value);
    assert_eq!(edit.parent(), body.field_id(control.ordinal).unwrap());
    assert_eq!(
        edit.coverage().completion,
        SimulationPostDialogCompletion::Changed
    );
    assert!(edit.archive().field(&edit.parent()).is_err());
    typed_history::assert_outer(
        body.fields().len(),
        |i| body.field_id(i).unwrap(),
        |i| edit.archive().field_id(i).unwrap(),
        edit.history(),
        control.ordinal,
    );
    assert_result_fields(body, edit, control);
    assert_nested_replacement(edit, control, value);
}

fn assert_control(body: &SimulationPostDialog, control: &DialogEditControl<'_>) {
    assert_eq!(body.field_offsets()[control.ordinal], control.span);
    assert_grammar(body, control);
    let value = replacement(control);
    assert_eq!(
        encoded_value(&value),
        (control.tag as usize, control.replacement.to_vec())
    );
    let edit = body.edit_field(&body.field_id(control.ordinal).unwrap(), value.clone());
    assert_eq!(
        edit.is_ok(),
        control.isolated,
        "{} {:?}",
        body.fields()[control.ordinal].path,
        body.version()
    );
    if let Ok(edit) = edit {
        assert_changed(body, &edit, control, &value);
    }
}

#[test]
fn all_owned_field_changes_match_independent_grammar_and_isolation_outcomes() {
    let mut cursor =
        include_bytes!("../fixtures/simulation-post-dialog/edit-matrix.bin").as_slice();
    assert_eq!(read_word(&mut cursor), 1);
    let bodies = owned_dialogs();
    assert_eq!(read_word(&mut cursor) as usize, bodies.len());
    let mut count = 0;
    for body in bodies {
        assert_eq!(read_word(&mut cursor) as i32, body.version().value());
        assert_eq!(read_word(&mut cursor) as usize, body.fields().len());
        for ordinal in 0..body.fields().len() {
            let control = read_control(&mut cursor);
            assert_eq!(control.ordinal, ordinal);
            assert_control(&body, &control);
            count += 1;
        }
    }
    assert!(cursor.is_empty());
    assert_eq!(count, 117435);
}

fn assert_field_span(
    body: &SimulationPostDialog,
    after: &SimulationPostDialog,
    index: usize,
    control: &DialogEditControl<'_>,
) {
    let shifted = typed_values::shifted_span(
        body.field_offsets()[index],
        control.span,
        index,
        control.ordinal,
        control.replacement.len(),
    );
    assert_eq!(after.field_offsets()[index], shifted);
}

fn assert_grammar(body: &SimulationPostDialog, control: &DialogEditControl<'_>) {
    let expected = expected_image(body, control);
    assert_eq!(
        SimulationPostDialog::from_bytes(&expected, body.version(), LIMITS).is_ok(),
        control.grammar
    );
}
