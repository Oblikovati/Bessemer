use super::types::*;
pub fn body(case: &ReferenceCase) -> Vec<u8> {
    assert!([1, 2].contains(&case.kind));
    assert_eq!(case.key.len(), 16);
    assert!(!case.runtime);
    let keys = existing_keys(case);
    let mut raw = vec![0, 0];
    raw.extend_from_slice(&case.key);
    append_state(&mut raw, case);
    raw.extend_from_slice(&(case.references.len() as i32).to_le_bytes());
    for reference in &case.references {
        assert_eq!(reference.len(), 16);
        assert!(keys.contains(reference.as_slice()));
        raw.extend_from_slice(reference);
    }
    assert_eq!(raw.len(), case.end as usize);
    raw.extend_from_slice(&case.tail);
    raw
}
fn append_state(raw: &mut Vec<u8>, case: &ReferenceCase) {
    if case.kind == 2 {
        assert_eq!(case.state, None);
        assert_eq!(case.loaded_state, None);
        return;
    }
    raw.push(case.state.unwrap());
    assert_eq!(case.loaded_state, Some(1));
}

fn existing_keys(case: &ReferenceCase) -> std::collections::BTreeSet<&[u8]> {
    let keys: std::collections::BTreeSet<_> = case
        .keys
        .iter()
        .map(|key| {
            assert_eq!(key.len(), 16);
            key.as_slice()
        })
        .collect();
    keys
}
