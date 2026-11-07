use super::*;
pub fn record(case: &PrimaryCase) -> Vec<u8> {
    assert!((0..=3).contains(&case.version));
    let mut raw = case.version.to_le_bytes().to_vec();
    count(&mut raw, case.items.len());
    for (ordinal, item) in case.items.iter().enumerate() {
        append_item(&mut raw, item, ordinal, case.version);
    }
    assert_eq!(raw.len(), case.end as usize);
    assert!(case.complete);
    assert!(!case.engineering);
    raw
}
fn append_item(raw: &mut Vec<u8>, item: &PrimaryItem, ordinal: usize, version: i32) {
    assert_eq!(item.ordinal as usize, ordinal);
    assert_eq!(raw.len(), item.start as usize);
    raw.extend_from_slice(&[item.kind, item.element_kind]);
    assert_eq!(item.groups.len(), 2);
    for (ordinal, group) in item.groups.iter().enumerate() {
        append_group(raw, item, group, ordinal, version);
    }
    assert_eq!(raw.len(), item.end as usize);
}
fn append_group(
    raw: &mut Vec<u8>,
    item: &PrimaryItem,
    group: &PrimaryGroup,
    ordinal: usize,
    version: i32,
) {
    assert_eq!(group.ordinal as usize, ordinal);
    assert_eq!(raw.len(), group.start as usize);
    assert_eq!(group.mode.is_some(), version == 3);
    if let Some(mode) = group.mode {
        raw.push(mode);
    }
    for member in group.members {
        raw.extend_from_slice(&member.to_le_bytes());
    }
    raw.push(group.presence);
    assert_eq!(group.body.is_some(), group.presence != 0);
    if let Some(body) = &group.body {
        append_body(raw, body, version);
    }
    append_elements(raw, item, group);
    assert_eq!(raw.len(), group.end as usize);
}
fn append_body(raw: &mut Vec<u8>, body: &PrimaryBody, version: i32) {
    assert_eq!(raw.len(), body.start as usize);
    count(raw, body.first.len());
    words(raw, &body.first);
    count(raw, body.second.len());
    words(raw, &body.second);
    for member in body.triplet {
        raw.extend_from_slice(&member.to_le_bytes());
    }
    assert_eq!(body.wide6.len(), 6);
    assert_eq!(body.wide3.len(), 3);
    wides(raw, &body.wide6);
    wides(raw, &body.wide3);
    append_body_tail(raw, body, version);
}
fn append_body_tail(raw: &mut Vec<u8>, body: &PrimaryBody, version: i32) {
    assert_eq!(body.legacy.is_some(), version == 0);
    if let Some(legacy) = &body.legacy {
        assert_eq!(legacy.len(), 9);
        wides(raw, legacy);
    }
    assert_eq!(body.flag.is_some(), version >= 2);
    if let Some(flag) = body.flag {
        raw.push(flag);
    }
    assert_eq!(body.native_flag, body.flag == Some(1));
    assert_eq!(raw.len(), body.end as usize);
}
fn append_elements(raw: &mut Vec<u8>, item: &PrimaryItem, group: &PrimaryGroup) {
    assert_eq!(group.elements.is_some(), matches!(item.kind, 2 | 3));
    let Some(elements) = &group.elements else {
        return;
    };
    count(raw, elements.len());
    if !elements.is_empty() {
        assert!((1..=4).contains(&item.element_kind));
    }
    for element in elements {
        assert_eq!(element.kind, item.element_kind);
        append_element(raw, element);
    }
}
fn append_element(raw: &mut Vec<u8>, element: &PrimaryElement) {
    assert_eq!(raw.len(), element.start as usize);
    count(raw, element.rows.len());
    for row in &element.rows {
        append_row(raw, row, element.kind);
    }
    count(raw, element.blocks.len());
    for block in &element.blocks {
        assert_eq!(raw.len(), block.start as usize);
        assert_eq!(block.values.len(), 16);
        wides(raw, &block.values);
        assert_eq!(raw.len(), block.end as usize);
    }
    assert_eq!(raw.len(), element.end as usize);
}
fn append_row(raw: &mut Vec<u8>, row: &PrimaryRow, kind: u8) {
    assert_eq!(raw.len(), row.start as usize);
    match &row.values {
        RowValues::Words(values) => {
            assert_eq!(kind, 1);
            count(raw, values.len());
            words(raw, values);
        }
        RowValues::Pairs(values) => append_pairs(raw, values, kind),
        RowValues::Wide(values) => append_wide_pairs(raw, values, kind),
        RowValues::Mixed(values) => append_mixed(raw, values, kind),
    }
    assert_eq!(raw.len(), row.end as usize);
}
fn append_pairs(raw: &mut Vec<u8>, values: &[(u32, u32)], kind: u8) {
    assert_eq!(kind, 2);
    count(raw, values.len());
    for (first, second) in values {
        words(raw, &[*first, *second]);
    }
}
fn append_wide_pairs(raw: &mut Vec<u8>, values: &[(u32, u64)], kind: u8) {
    assert_eq!(kind, 3);
    count(raw, values.len());
    for (first, second) in values {
        words(raw, &[*first]);
        wides(raw, &[*second]);
    }
}
fn append_mixed(raw: &mut Vec<u8>, values: &[(u32, u64, u32)], kind: u8) {
    assert_eq!(kind, 4);
    count(raw, values.len());
    for (first, second, third) in values {
        words(raw, &[*first]);
        wides(raw, &[*second]);
        words(raw, &[*third]);
    }
}

fn count(raw: &mut Vec<u8>, count: usize) {
    raw.extend_from_slice(&(count as i32).to_le_bytes());
}
fn words(raw: &mut Vec<u8>, values: &[u32]) {
    for value in values {
        raw.extend_from_slice(&value.to_le_bytes());
    }
}
fn wides(raw: &mut Vec<u8>, values: &[u64]) {
    for value in values {
        raw.extend_from_slice(&value.to_le_bytes());
    }
}
