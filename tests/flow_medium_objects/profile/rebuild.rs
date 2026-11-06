use super::*;
use std::collections::BTreeSet;
pub fn profile(case: &ProfileCase) -> Vec<u8> {
    let mut raw = case.prefix.clone();
    assert_eq!(raw.len(), case.backend_start as usize);
    raw.extend_from_slice(&[0, 0]);
    raw.extend_from_slice(&(case.first.len() as i32).to_le_bytes());
    let first_keys: BTreeSet<_> = case
        .first
        .iter()
        .map(|entry| entry.key.as_slice())
        .collect();
    for (ordinal, entry) in case.first.iter().enumerate() {
        append_first(&mut raw, entry, ordinal);
    }
    raw.push(0);
    raw.extend_from_slice(&(case.second.len() as i32).to_le_bytes());
    for (ordinal, entry) in case.second.iter().enumerate() {
        append_second(&mut raw, entry, ordinal, &first_keys);
    }
    assert_eq!(raw.len(), case.backend_end as usize);
    append_footer(&mut raw, case);
    raw
}
fn append_first(raw: &mut Vec<u8>, entry: &ProfileFirst, ordinal: usize) {
    assert_eq!(entry.ordinal as usize, ordinal);
    assert_eq!(raw.len(), entry.start as usize);
    assert_eq!(entry.kind, 1);
    raw.extend_from_slice(&entry.kind.to_le_bytes());
    append_key(raw, &entry.key);
    assert!(entry.body.footer.tail.is_empty());
    raw.extend_from_slice(&rebuild::body(&entry.body));
    assert_eq!(raw.len(), entry.end as usize);
}
fn append_second(raw: &mut Vec<u8>, entry: &ProfileSecond, ordinal: usize, keys: &BTreeSet<&[u8]>) {
    assert_eq!(entry.ordinal as usize, ordinal);
    assert_eq!(raw.len(), entry.start as usize);
    assert_eq!(entry.kind, 1);
    raw.extend_from_slice(&entry.kind.to_le_bytes());
    raw.extend_from_slice(&[0, 0]);
    append_key(raw, &entry.key);
    raw.push(entry.state);
    assert_eq!(entry.loaded, 1);
    raw.extend_from_slice(&(entry.references.len() as i32).to_le_bytes());
    for reference in &entry.references {
        assert!(keys.contains(reference.as_slice()));
        append_key(raw, reference);
    }
    assert_eq!(raw.len(), entry.end as usize);
}
fn append_footer(raw: &mut Vec<u8>, case: &ProfileCase) {
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
    append_completion(raw, footer);
}
fn append_key(raw: &mut Vec<u8>, key: &[u8]) {
    assert_eq!(key.len(), 16);
    raw.extend_from_slice(key);
}

fn append_completion(raw: &mut Vec<u8>, footer: &ProfileFooter) {
    assert_eq!(footer.final_counts, [0, 0]);
    raw.extend_from_slice(&[0; 8]);
    assert_eq!(raw.len(), footer.end as usize);
    assert!(footer.complete);
    assert!(footer.fresh);
    assert!(!footer.engineering);
    assert!(!footer.runtime);
}
