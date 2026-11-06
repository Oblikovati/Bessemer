use super::types::*;
use std::collections::BTreeMap;

pub fn state_wire(case: &StateCase) -> Vec<u8> {
    let mut raw = case.version_word.to_le_bytes().to_vec();
    raw.extend_from_slice(&(case.entries.len() as i32).to_le_bytes());
    for (ordinal, entry) in case.entries.iter().enumerate() {
        assert_eq!(entry.ordinal as usize, ordinal);
        assert_eq!(raw.len(), entry.start as usize);
        raw.extend_from_slice(&entry.identifier.to_le_bytes());
        raw.push(entry.state);
        assert_eq!(raw.len(), entry.end as usize);
    }
    assert_eq!(raw.len(), case.end as usize);
    assert!(!case.native_dispatch);
    raw
}

pub fn apply_states(case: &StateCase) {
    assert!(case.existing.windows(2).all(|pair| pair[0].0 < pair[1].0));
    let mut states: BTreeMap<i32, u8> = case.existing.iter().copied().collect();
    assert_eq!(case.assignments.len(), case.entries.len());
    for (entry, assignment) in case.entries.iter().zip(&case.assignments) {
        assert_eq!(assignment.ordinal, entry.ordinal);
        assert_eq!(assignment.identifier, entry.identifier);
        assert_eq!(assignment.state, entry.state);
        assert_eq!(
            states.insert(entry.identifier, entry.state),
            assignment.previous
        );
    }
    assert_eq!(states.into_iter().collect::<Vec<_>>(), case.expected);
    assert!(case.preserved);
    assert!(!case.native_hash);
}

pub fn prefix_wire(case: &PrefixCase) -> Vec<u8> {
    assert_eq!(case.string_mode, 0);
    assert_eq!(case.signed_version, case.version as i8);
    assert_eq!(case.active, case.signed_version >= 2);
    let mut raw = vec![case.version];
    if case.active {
        active_prefix(&mut raw, case);
    } else {
        assert!(case.vectors.is_empty() && case.objects.is_empty());
    }
    assert_eq!(raw.len(), case.end as usize);
    assert_eq!(case.known, case.end);
    assert_eq!(case.unknown as usize, case.tail.len());
    assert_eq!(case.requires_backend, case.active);
    assert!(!case.full_schema && !case.runtime);
    raw.extend_from_slice(&case.tail);
    raw
}

fn active_prefix(raw: &mut Vec<u8>, case: &PrefixCase) {
    raw.extend_from_slice(&(case.vectors.len() as i32).to_le_bytes());
    for (ordinal, vector) in case.vectors.iter().enumerate() {
        assert_eq!(vector.ordinal as usize, ordinal);
        assert_eq!(raw.len(), vector.start as usize);
        raw.extend_from_slice(&(vector.values.len() as i32).to_le_bytes());
        for value in &vector.values {
            raw.extend_from_slice(&value.to_le_bytes());
        }
        assert_eq!(raw.len(), vector.end as usize);
    }
    raw.extend_from_slice(&(case.objects.len() as i32).to_le_bytes());
    for (ordinal, object) in case.objects.iter().enumerate() {
        assert_eq!(object.ordinal as usize, ordinal);
        append_object(raw, object, case.signed_version);
    }
}

fn append_object(raw: &mut Vec<u8>, object: &PrefixObject, version: i8) {
    assert_eq!(raw.len(), object.start as usize);
    append_text(raw, &object.first);
    append_text(raw, &object.second);
    raw.push(object.state);
    assert_eq!(object.native_state, object.state != 0);
    raw.extend_from_slice(&object.word_c4.to_le_bytes());
    append_text(raw, &object.third);
    assert_eq!(object.fourth.is_some(), version >= 3);
    assert_eq!(object.derived_fourth, version == 2);
    let native = if let Some(fourth) = &object.fourth {
        append_text(raw, fourth);
        &fourth.native_text
    } else {
        &object.first.native_text
    };
    assert_eq!(object.native_fourth, *native);
    assert_eq!(raw.len(), object.end as usize);
}

fn append_text(raw: &mut Vec<u8>, text: &PrefixString) {
    assert_eq!(raw.len(), text.start as usize);
    raw.extend_from_slice(&(text.units as i32).to_le_bytes());
    assert_eq!(raw.len(), text.text_start as usize);
    assert_eq!(text.raw.len(), text.units as usize * 2);
    assert_eq!(native_text(&text.raw), text.native_text);
    raw.extend_from_slice(&text.raw);
    assert_eq!(raw.len(), text.end as usize);
}

fn native_text(raw: &[u8]) -> Vec<u8> {
    let (pairs, remainder) = raw.as_chunks::<2>();
    assert!(remainder.is_empty());
    if !pairs.is_empty() {
        assert_eq!(pairs.last(), Some(&[0, 0]));
    }
    pairs
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .take_while(|unit| *unit != 0)
        .flat_map(u16::to_le_bytes)
        .collect()
}
