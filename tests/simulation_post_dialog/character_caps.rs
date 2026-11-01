use super::*;
use std::collections::BTreeSet;

fn replace_character_frame(body: &SimulationPostDialog, ordinal: usize, count: i32) -> Vec<u8> {
    let original = body.to_bytes();
    let (start, end) = body.field_offsets()[ordinal];
    let mut replacement = count.to_le_bytes().to_vec();
    for _ in 0..count.max(0) {
        replacement.extend_from_slice(&[0, 0xd8]);
    }
    [&original[..start], replacement.as_slice(), &original[end..]].concat()
}

fn assert_character_cap(body: &SimulationPostDialog, ordinal: usize) {
    let cap = if body.fields()[ordinal].path.as_ref() == "style.text" {
        512
    } else {
        1024
    };
    for count in [-1, cap, cap + 1] {
        let raw = replace_character_frame(body, ordinal, count);
        let result = SimulationPostDialog::from_bytes(&raw, body.version(), LIMITS);
        assert_eq!(
            result.is_ok(),
            count == cap,
            "{} {count}",
            body.fields()[ordinal].path
        );
        if let Ok(parsed) = result {
            assert_eq!(parsed.to_bytes(), raw);
            let span = parsed.field_offsets()[ordinal];
            assert_eq!(span.1 - span.0, 4 + cap as usize * 2);
        }
    }
}

#[test]
fn every_reached_character_position_keeps_its_independent_imported_or_style_cap() {
    let mut paths = BTreeSet::new();
    for body in owned_dialogs() {
        for (ordinal, field) in body.fields().iter().enumerate() {
            if !matches!(field.value, SimulationPostDialogValue::CharacterText(_)) {
                continue;
            }
            if !paths.insert(field.path.clone()) {
                continue;
            }
            assert_character_cap(&body, ordinal);
        }
    }
    assert_eq!(paths.len(), 27);
}
