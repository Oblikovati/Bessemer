use super::*;

pub fn decoded(case: &PointerCase) -> Vec<u8> {
    let mut raw = case.version.to_le_bytes().to_vec();
    assert_eq!(case.pools.len(), case.frame.leaf as usize);
    for (ordinal, pool) in case.pools.iter().enumerate() {
        assert_eq!(pool.ordinal, ordinal as u32);
        assert_eq!(raw.len(), pool.start as usize);
        raw.push(u8::try_from(pool.count).unwrap());
        assert_eq!(pool.bits.len(), 3 * pool.count as usize);
        append_wide(&mut raw, &pool.bits);
        assert_eq!(raw.len(), pool.end as usize);
    }
    for (ordinal, group) in case.groups.iter().enumerate() {
        append_group(&mut raw, group, ordinal, case);
    }
    raw
}

fn append_group(raw: &mut Vec<u8>, group: &PointerGroup, ordinal: usize, case: &PointerCase) {
    assert_eq!(group.ordinal, ordinal as u32);
    assert_eq!(group.count, case.frame.counts[ordinal]);
    assert_eq!(group.entries.len(), group.count as usize);
    assert_eq!(raw.len(), group.start as usize);
    for (index, entry) in group.entries.iter().enumerate() {
        assert_eq!(entry.ordinal, index as u32);
        check_context(entry, ordinal, index, case);
        assert_eq!(raw.len(), entry.start as usize);
        append_body(raw, &entry.body, case.version);
        assert_eq!(raw.len(), entry.end as usize);
    }
    assert_eq!(raw.len(), group.end as usize);
}

fn check_context(entry: &PointerEntry, group: usize, ordinal: usize, case: &PointerCase) {
    if case.version >= 3 {
        assert!(entry.code.is_none());
        assert!(matches!(entry.body, PointerEntryBody::Tagged(_)));
        return;
    }
    if group < 2 {
        assert!(entry.code.is_none());
        assert!(matches!(entry.body, PointerEntryBody::Single(_)));
        return;
    }
    let code = case.frame.codes.as_ref().unwrap()[ordinal];
    assert_eq!(entry.code, Some(code));
    assert_eq!(
        matches!(entry.body, PointerEntryBody::Wide { .. }),
        is_wide(code)
    );
    assert!(matches!(
        entry.body,
        PointerEntryBody::Wide { .. } | PointerEntryBody::Multiple(_)
    ));
}

fn is_wide(code: u8) -> bool {
    let shifted = code.wrapping_sub(30);
    (shifted < 55 && (0x700001c0701c07_u64 >> (shifted & 63)) & 1 != 0)
        || code.wrapping_add(171) < 3
        || (code < 63 && (0x7000000000000088_u64 >> code) & 1 != 0)
        || code == 73
        || code == 77
}

fn append_body(raw: &mut Vec<u8>, body: &PointerEntryBody, version: i32) {
    match body {
        PointerEntryBody::Single(bytes) => {
            assert!(version < 3);
            append_vector(raw, bytes);
        }
        PointerEntryBody::Wide { count, bits } => {
            assert!(version < 3);
            raw.push(u8::try_from(*count).unwrap());
            assert_eq!(bits.len(), *count as usize * 3);
            append_wide(raw, bits);
        }
        PointerEntryBody::Multiple(vectors) => {
            assert!(version < 3);
            raw.push(u8::try_from(vectors.len()).unwrap());
            for vector in vectors {
                append_vector(raw, vector);
            }
        }
        PointerEntryBody::Tagged(tagged) => append_tagged(raw, tagged, version),
    }
}

fn append_tagged(raw: &mut Vec<u8>, tagged: &TaggedPointerBody, version: i32) {
    assert!((3..=5).contains(&version));
    raw.push(tagged.tag);
    let stored = tagged.tag == 1 || tagged.tag == 2;
    assert_eq!(tagged.count.is_some(), stored);
    assert_eq!(tagged.packed_decoded, stored.then_some(false));
    if let Some(count) = tagged.count {
        raw.extend_from_slice(&count.to_le_bytes());
    }
    let wide = version == 3 && tagged.tag == 2;
    assert_eq!(tagged.bits.is_some(), wide);
    assert_eq!(tagged.raw.is_some(), stored && !wide);
    if let Some(bits) = &tagged.bits {
        assert_eq!(bits.len(), tagged.count.unwrap() as usize * 3 + 1);
        append_wide(raw, bits);
    }
    if let Some(bytes) = &tagged.raw {
        assert_eq!(bytes.len(), tagged.count.unwrap() as usize);
        raw.extend_from_slice(bytes);
    }
}

fn append_vector(raw: &mut Vec<u8>, bytes: &[u8]) {
    raw.push(u8::try_from(bytes.len()).unwrap());
    raw.extend_from_slice(bytes);
}

fn append_wide(raw: &mut Vec<u8>, bits: &[u64]) {
    raw.extend(bits.iter().flat_map(|value| value.to_le_bytes()));
}
