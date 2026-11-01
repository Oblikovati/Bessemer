use super::*;

fn malformed_case(cursor: &mut &[u8], body: &SimulationPostDialog) {
    let ordinal = read_word(cursor) as usize;
    assert_eq!(read_word(cursor), 3);
    assert_eq!(read_word(cursor), 0);
    assert_eq!(read_word(cursor), 0);
    let span = (read_word(cursor) as usize, read_word(cursor) as usize);
    let length = read_word(cursor) as usize;
    let replacement = take_projection(cursor, length);
    assert_eq!(span, body.field_offsets()[ordinal]);
    let original = body.to_bytes();
    let bytes = [&original[..span.0], replacement, &original[span.1..]].concat();
    assert!(SimulationPersistText::read_from(replacement, 0, LIMITS.maximum_text_bytes).is_err());
    assert!(SimulationPostDialog::from_bytes(&bytes, body.version(), LIMITS).is_err());
    assert_eq!(body.to_bytes(), original);
}

#[test]
fn all_source_missing_terminator_controls_remain_rejected() {
    let mut cursor =
        include_bytes!("../fixtures/simulation-post-dialog/bad-persist-terminator.bin").as_slice();
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 225);
    let mut count = 0;
    for body in owned_dialogs() {
        assert_eq!(read_word(&mut cursor) as i32, body.version().value());
        for _ in 0..read_word(&mut cursor) {
            malformed_case(&mut cursor, &body);
            count += 1;
        }
    }
    assert_eq!(count, 662);
    assert!(cursor.is_empty());
}

fn reference_name_case() -> (SimulationPostDialog, usize) {
    for body in owned_dialogs() {
        let ordinal = body
            .fields()
            .iter()
            .position(|field| matches!(field.value, SimulationPostDialogValue::PersistText(_)));
        if let Some(ordinal) = ordinal {
            return (body, ordinal);
        }
    }
    panic!("expected a complete owned reference-name control");
}

fn replace_name_bytes(body: &SimulationPostDialog, ordinal: usize, count: usize) -> Vec<u8> {
    let original = body.to_bytes();
    let (start, end) = body.field_offsets()[ordinal];
    let mut bytes = original[..start].to_vec();
    bytes.extend_from_slice(&(count as u32).to_le_bytes());
    bytes.resize(bytes.len() + count, 0);
    bytes.extend_from_slice(&original[end..]);
    bytes
}

#[test]
fn reference_names_keep_the_imported_persist_text_safety_cap() {
    let (body, ordinal) = reference_name_case();
    let exact = replace_name_bytes(&body, ordinal, 0x100000);
    let accepted = SimulationPostDialog::from_bytes(&exact, body.version(), LIMITS).unwrap();
    assert_eq!(accepted.to_bytes(), exact);
    let excessive = replace_name_bytes(&body, ordinal, 0x100002);
    assert!(SimulationPostDialog::from_bytes(&excessive, body.version(), LIMITS).is_err());
}
