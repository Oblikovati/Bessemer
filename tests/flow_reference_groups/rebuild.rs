use super::fixture::*;

pub fn decoded(case: &ReferenceCase) -> Vec<u8> {
    let mut raw = case.version.to_le_bytes().to_vec();
    assert_eq!(case.discarded_header.is_some(), case.version < 6);
    assert_eq!(case.stored_header.is_some(), case.version >= 3);
    assert_eq!(case.native_header_default, 2);
    if let Some(word) = case.discarded_header {
        raw.extend_from_slice(&word.to_le_bytes());
    }
    if let Some(word) = case.stored_header {
        raw.extend_from_slice(&word.to_le_bytes());
    }
    for (ordinal, group) in case.groups.iter().enumerate() {
        assert_eq!(group.ordinal as usize, ordinal);
        append_group(&mut raw, group, case);
    }
    raw
}

fn append_group(raw: &mut Vec<u8>, group: &ReferenceGroup, case: &ReferenceCase) {
    assert_eq!(raw.len(), group.start as usize);
    raw.extend_from_slice(&(group.entries.len() as i32).to_le_bytes());
    for entry in &group.entries {
        append_entry(raw, entry, case.version);
    }
    append_mapping(raw, group, case.frame.mapping);
    let wide = group.ordinal == 1 || case.version >= 15;
    assert_eq!(
        group.wide.len(),
        if wide { 3 * group.entries.len() } else { 0 }
    );
    for bits in &group.wide {
        raw.extend_from_slice(&bits.to_le_bytes());
    }
    assert_eq!(raw.len(), group.end as usize);
}

fn append_mapping(raw: &mut Vec<u8>, group: &ReferenceGroup, mapping: bool) {
    let mapped = group.ordinal == 0 && mapping && !group.entries.is_empty();
    assert_eq!(group.level.is_some(), mapped);
    assert_eq!(
        group.mapping.len(),
        if mapped { 2 * group.entries.len() } else { 0 }
    );
    if let Some(level) = group.level {
        raw.extend_from_slice(&level.to_le_bytes());
    }
    for value in &group.mapping {
        raw.extend_from_slice(&value.to_le_bytes());
    }
}

fn append_entry(raw: &mut Vec<u8>, entry: &ReferenceEntry, version: i32) {
    assert_eq!(raw.len(), entry.start as usize);
    raw.extend_from_slice(&(entry.references.len() as i32).to_le_bytes());
    assert_eq!(entry.member20.is_some(), version < 12);
    if let Some(word) = entry.member20 {
        raw.extend_from_slice(&word.to_le_bytes());
    }
    for word in &entry.references {
        raw.extend_from_slice(&word.to_le_bytes());
    }
    append_selectors(raw, entry, version);
    append_arrays(raw, entry, version);
    assert_eq!(entry.suffix.is_some(), version >= 16);
    if let Some(word) = entry.suffix {
        raw.extend_from_slice(&word.to_le_bytes());
    }
    assert_eq!(raw.len(), entry.end as usize);
}

fn append_selectors(raw: &mut Vec<u8>, entry: &ReferenceEntry, version: i32) {
    assert_eq!(entry.prefix.is_some(), (11..=13).contains(&version));
    if let Some(value) = entry.prefix {
        raw.push(value);
    }
    raw.push(entry.state);
    assert_eq!(entry.flag2.is_some(), version >= 5);
    assert_eq!(entry.flag4.is_some(), version == 17);
    if let Some(value) = entry.flag2 {
        raw.push(value);
    }
    if let Some(value) = entry.flag4 {
        raw.push(value);
    }
}

fn append_arrays(raw: &mut Vec<u8>, entry: &ReferenceEntry, version: i32) {
    for value in entry.descriptors {
        if version == 1 {
            raw.push(u8::try_from(value).unwrap());
        } else {
            raw.extend_from_slice(&value.to_le_bytes());
        }
    }
    assert_eq!(entry.first.len(), entry.descriptors[6] as usize);
    for bits in &entry.first {
        raw.extend_from_slice(&bits.to_le_bytes());
    }
    append_secondary_count(raw, entry, version);
    assert_eq!(entry.second.len(), entry.second_count.max(0) as usize);
    for bits in &entry.second {
        raw.extend_from_slice(&bits.to_le_bytes());
    }
}

fn append_secondary_count(raw: &mut Vec<u8>, entry: &ReferenceEntry, version: i32) {
    assert_eq!(entry.raw_small_count.is_some(), version < 10);
    match version {
        1 => {
            let byte = u8::try_from(entry.raw_small_count.unwrap()).unwrap();
            assert_eq!(entry.second_count, i32::from(byte as i8));
            raw.push(byte);
        }
        0 | 2..=9 => {
            let value = entry.raw_small_count.unwrap();
            assert_eq!(entry.second_count, i32::from(value));
            raw.extend_from_slice(&value.to_le_bytes());
        }
        _ => raw.extend_from_slice(&entry.second_count.to_le_bytes()),
    }
}
