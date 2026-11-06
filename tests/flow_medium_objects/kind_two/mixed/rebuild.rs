use super::*;
use std::collections::BTreeSet;
pub fn profile(case: &MixedCase) -> Vec<u8> {
    let mut raw = case.prefix.clone();
    assert_eq!(raw.len(), case.backend_start as usize);
    raw.extend_from_slice(&[0, 0]);
    raw.extend_from_slice(&(case.first.len() as i32).to_le_bytes());
    for (ordinal, entry) in case.first.iter().enumerate() {
        append_first(&mut raw, entry, ordinal);
    }
    raw.push(0);
    raw.extend_from_slice(&(case.second.len() as i32).to_le_bytes());
    for (ordinal, entry) in case.second.iter().enumerate() {
        append_second(&mut raw, case, entry, ordinal);
    }
    assert_eq!(raw.len(), case.backend_end as usize);
    append_footer(&mut raw, case);
    raw
}
fn append_first(raw: &mut Vec<u8>, entry: &MixedFirst, ordinal: usize) {
    assert_eq!(entry.ordinal as usize, ordinal);
    assert_eq!(raw.len(), entry.start as usize);
    raw.extend_from_slice(&entry.kind.to_le_bytes());
    append_key(raw, &entry.key);
    let body = match &entry.body {
        MixedBody::One(value) => {
            assert_eq!(entry.kind, 1);
            rebuild::body(value)
        }
        MixedBody::Two(value) => {
            assert_eq!(entry.kind, 2);
            kind_rebuild::body(value)
        }
    };
    raw.extend_from_slice(&body);
    assert_eq!(raw.len(), entry.end as usize);
}
fn append_second(raw: &mut Vec<u8>, case: &MixedCase, entry: &MixedSecond, ordinal: usize) {
    assert_eq!(entry.ordinal as usize, ordinal);
    assert_eq!(raw.len(), entry.start as usize);
    assert!(matches!(entry.kind, 1 | 2));
    let keys: BTreeSet<_> = case
        .first
        .iter()
        .filter(|first| first.kind == entry.kind)
        .map(|first| first.key.as_slice())
        .collect();
    raw.extend_from_slice(&entry.kind.to_le_bytes());
    raw.extend_from_slice(&[0, 0]);
    append_key(raw, &entry.key);
    append_state(raw, entry);
    raw.extend_from_slice(&(entry.references.len() as i32).to_le_bytes());
    for reference in &entry.references {
        assert!(keys.contains(reference.as_slice()));
        append_key(raw, reference);
    }
    assert_eq!(raw.len(), entry.end as usize);
}
fn append_state(raw: &mut Vec<u8>, entry: &MixedSecond) {
    if entry.kind == 1 {
        raw.push(entry.state.unwrap());
        assert_eq!(entry.loaded, Some(1));
        return;
    }
    assert_eq!(entry.state, None);
    assert_eq!(entry.loaded, None);
}
fn append_footer(raw: &mut Vec<u8>, case: &MixedCase) {
    let footer = &case.footer;
    let keys: BTreeSet<_> = case
        .second
        .iter()
        .map(|entry| entry.key.as_slice())
        .collect();
    assert_eq!(footer.keys.len(), footer.attributes.len());
    raw.extend_from_slice(&(footer.keys.len() as i32).to_le_bytes());
    for key in &footer.keys {
        assert!(keys.contains(key.as_slice()));
        append_key(raw, key);
    }
    for attribute in &footer.attributes {
        raw.extend_from_slice(&attribute.to_le_bytes());
    }
    assert_eq!(footer.final_counts, [0, 0]);
    raw.extend_from_slice(&[0; 8]);
    assert_eq!(raw.len(), footer.end as usize);
    assert_eq!(footer.flags, [true, false, true, false, false]);
}
fn append_key(raw: &mut Vec<u8>, key: &[u8]) {
    assert_eq!(key.len(), 16);
    raw.extend_from_slice(key);
}
