use super::*;
use FlowPrimaryFieldKind as Kind;
use FlowPrimaryScope as Scope;
use FlowPrimaryValue::{Byte, Signed32, WideBits, WordBits};
use std::collections::BTreeMap;
pub(super) type Leaves = BTreeMap<usize, (FlowPrimaryValue, Scope, Kind)>;

pub(super) fn leaves(case: &PrimaryCase) -> Leaves {
    let mut result = Leaves::new();
    insert(
        &mut result,
        0,
        Signed32(case.version),
        Scope::Record,
        Kind::Version,
    );
    counter(
        &mut result,
        4,
        case.items.len(),
        Scope::Record,
        Kind::ItemCount,
    );
    for (i, item) in case.items.iter().enumerate() {
        item_leaves(&mut result, item, i);
    }
    result
}

fn item_leaves(result: &mut Leaves, item: &PrimaryItem, i: usize) {
    let scope = Scope::Item(i);
    insert(
        result,
        item.start as usize,
        Byte(item.kind),
        scope,
        Kind::Kind,
    );
    insert(
        result,
        item.start as usize + 1,
        Byte(item.element_kind),
        scope,
        Kind::ElementKind,
    );
    for (g, group) in item.groups.iter().enumerate() {
        group_leaves(result, group, i, g);
    }
}

fn group_leaves(result: &mut Leaves, group: &PrimaryGroup, i: usize, g: usize) {
    let scope = Scope::Group(i, g);
    let mut offset = group_header(result, group, scope);
    if let Some(body) = &group.body {
        body_leaves(result, body, Scope::Body(i, g));
        offset = body.end as usize;
    }
    if let Some(elements) = &group.elements {
        counter(result, offset, elements.len(), scope, Kind::ElementCount);
        for (e, element) in elements.iter().enumerate() {
            element_leaves(result, element, i, g, e);
        }
    }
}

fn group_header(result: &mut Leaves, group: &PrimaryGroup, scope: Scope) -> usize {
    let mut offset = group.start as usize;
    if let Some(mode) = group.mode {
        insert(result, offset, Byte(mode), scope, Kind::Mode);
        offset += 1;
    }
    put(
        result,
        offset,
        group.members.into_iter().map(Signed32),
        scope,
        Kind::Members,
    );
    insert(
        result,
        offset + 8,
        Byte(group.presence),
        scope,
        Kind::Presence,
    );
    offset + 9
}

fn body_leaves(result: &mut Leaves, body: &PrimaryBody, scope: Scope) {
    let mut offset = body_vectors(result, body, scope);
    offset = put(
        result,
        offset,
        body.triplet.into_iter().map(Signed32),
        scope,
        Kind::Triplet,
    );
    offset = wides(result, offset, &body.wide6, scope, Kind::WideSix);
    offset = wides(result, offset, &body.wide3, scope, Kind::WideThree);
    if let Some(legacy) = &body.legacy {
        offset = wides(result, offset, legacy, scope, Kind::LegacyWideNine);
    }
    if let Some(flag) = body.flag {
        insert(result, offset, Byte(flag), scope, Kind::StoredFlag);
        offset += 1;
    }
    assert_eq!(offset, body.end as usize);
}

fn body_vectors(result: &mut Leaves, body: &PrimaryBody, scope: Scope) -> usize {
    let mut offset = vector(
        result,
        body.start as usize,
        &body.first,
        scope,
        (Kind::FirstCount, Kind::FirstWords),
    );
    offset = vector(
        result,
        offset,
        &body.second,
        scope,
        (Kind::SecondCount, Kind::SecondWords),
    );
    offset
}

fn vector(
    result: &mut Leaves,
    offset: usize,
    values: &[u32],
    scope: Scope,
    kinds: (Kind, Kind),
) -> usize {
    counter(result, offset, values.len(), scope, kinds.0);
    put(
        result,
        offset + 4,
        values.iter().copied().map(WordBits),
        scope,
        kinds.1,
    )
}

fn wides(result: &mut Leaves, offset: usize, values: &[u64], scope: Scope, kind: Kind) -> usize {
    put(
        result,
        offset,
        values.iter().copied().map(WideBits),
        scope,
        kind,
    )
}

fn counter(result: &mut Leaves, offset: usize, count: usize, scope: Scope, kind: Kind) {
    insert(result, offset, Signed32(count as i32), scope, kind);
}

fn element_leaves(result: &mut Leaves, element: &PrimaryElement, i: usize, g: usize, e: usize) {
    let scope = Scope::Element(i, g, e);
    let offset = element_rows(result, element, i, g, e);
    counter(
        result,
        offset,
        element.blocks.len(),
        scope,
        Kind::WideBlockCount,
    );
    for (b, block) in element.blocks.iter().enumerate() {
        let scope = Scope::WideBlock(i, g, e, b);
        let end = wides(
            result,
            block.start as usize,
            &block.values,
            scope,
            Kind::WideBlock,
        );
        assert_eq!(end, block.end as usize);
    }
}

fn element_rows(
    result: &mut Leaves,
    element: &PrimaryElement,
    i: usize,
    g: usize,
    e: usize,
) -> usize {
    let scope = Scope::Element(i, g, e);
    counter(
        result,
        element.start as usize,
        element.rows.len(),
        scope,
        Kind::RowCount,
    );
    let mut offset = element.start as usize + 4;
    for (r, row) in element.rows.iter().enumerate() {
        row_leaves(result, row, Scope::Row(i, g, e, r));
        offset = row.end as usize;
    }
    offset
}

fn row_leaves(result: &mut Leaves, row: &PrimaryRow, scope: Scope) {
    let (count, values) = row_values(&row.values);
    insert(
        result,
        row.start as usize,
        Signed32(count as i32),
        scope,
        Kind::EntryCount,
    );
    let mut offset = row.start as usize + 4;
    for (value, kind) in values {
        insert(result, offset, value, scope, kind);
        offset += width(value);
    }
    assert_eq!(offset, row.end as usize);
}

fn row_values(row: &RowValues) -> (usize, Vec<(FlowPrimaryValue, Kind)>) {
    match row {
        RowValues::Words(v) => (
            v.len(),
            v.iter().map(|&a| (WordBits(a), Kind::RowFirst)).collect(),
        ),
        RowValues::Pairs(v) => (v.len(), pair_rows(v)),
        RowValues::Wide(v) => (v.len(), wide_rows(v)),
        RowValues::Mixed(v) => (
            v.len(),
            v.iter().flat_map(|&(a, b, c)| mixed(a, b, c)).collect(),
        ),
    }
}

fn pair_rows(values: &[(u32, u32)]) -> Vec<(FlowPrimaryValue, Kind)> {
    values
        .iter()
        .flat_map(|&(a, b)| pair(a, WordBits(b), Kind::RowSecond))
        .collect()
}

fn wide_rows(values: &[(u32, u64)]) -> Vec<(FlowPrimaryValue, Kind)> {
    values
        .iter()
        .flat_map(|&(a, b)| pair(a, WideBits(b), Kind::RowWide))
        .collect()
}

fn pair(first: u32, second: FlowPrimaryValue, kind: Kind) -> [(FlowPrimaryValue, Kind); 2] {
    [(WordBits(first), Kind::RowFirst), (second, kind)]
}

fn mixed(first: u32, wide: u64, last: u32) -> [(FlowPrimaryValue, Kind); 3] {
    [
        (WordBits(first), Kind::RowFirst),
        (WideBits(wide), Kind::RowWide),
        (WordBits(last), Kind::RowLast),
    ]
}

fn put(
    result: &mut Leaves,
    start: usize,
    values: impl Iterator<Item = FlowPrimaryValue>,
    scope: Scope,
    kind: Kind,
) -> usize {
    let mut offset = start;
    for value in values {
        insert(result, offset, value, scope, kind);
        offset += width(value);
    }
    offset
}

fn insert(result: &mut Leaves, offset: usize, value: FlowPrimaryValue, scope: Scope, kind: Kind) {
    assert!(result.insert(offset, (value, scope, kind)).is_none());
}

fn width(value: FlowPrimaryValue) -> usize {
    match value {
        Byte(_) => 1,
        Signed32(_) | WordBits(_) => 4,
        WideBits(_) => 8,
    }
}

pub(super) fn field_count(case: &PrimaryCase) -> usize {
    2 + case
        .items
        .iter()
        .map(|item| 2 + item.groups.iter().map(group_field_count).sum::<usize>())
        .sum::<usize>()
}

fn group_field_count(group: &PrimaryGroup) -> usize {
    let body = group.body.as_ref().map_or(0, |body| {
        7 + usize::from(body.legacy.is_some()) + usize::from(body.flag.is_some())
    });
    let elements = group.elements.as_ref().map_or(0, |elements| {
        1 + elements
            .iter()
            .map(|element| {
                let columns = match element.kind {
                    1 => 1,
                    2 | 3 => 2,
                    4 => 3,
                    _ => panic!("Expected Source factory1..4"),
                };
                2 + element.rows.len() * (1 + columns) + element.blocks.len()
            })
            .sum::<usize>()
    });
    2 + usize::from(group.mode.is_some()) + body + elements
}

pub(super) fn scopes(case: &PrimaryCase) -> Vec<FlowPrimaryScopeView> {
    let mut result = vec![scope(FlowPrimaryScope::Record, 0, case.end)];
    for (i, item) in case.items.iter().enumerate() {
        result.push(scope(FlowPrimaryScope::Item(i), item.start, item.end));
        for (g, group) in item.groups.iter().enumerate() {
            group_scopes(&mut result, i, g, group);
        }
    }
    result
}

fn group_scopes(result: &mut Vec<FlowPrimaryScopeView>, i: usize, g: usize, group: &PrimaryGroup) {
    result.push(scope(FlowPrimaryScope::Group(i, g), group.start, group.end));
    if let Some(body) = &group.body {
        result.push(scope(FlowPrimaryScope::Body(i, g), body.start, body.end));
    }
    if let Some(elements) = &group.elements {
        for (e, element) in elements.iter().enumerate() {
            element_scopes(result, i, g, e, element);
        }
    }
}

fn element_scopes(
    result: &mut Vec<FlowPrimaryScopeView>,
    i: usize,
    g: usize,
    e: usize,
    element: &PrimaryElement,
) {
    result.push(scope(
        FlowPrimaryScope::Element(i, g, e),
        element.start,
        element.end,
    ));
    for (r, row) in element.rows.iter().enumerate() {
        result.push(scope(FlowPrimaryScope::Row(i, g, e, r), row.start, row.end));
    }
    for (b, block) in element.blocks.iter().enumerate() {
        result.push(scope(
            FlowPrimaryScope::WideBlock(i, g, e, b),
            block.start,
            block.end,
        ));
    }
}

fn scope(scope: FlowPrimaryScope, start: u32, end: u32) -> FlowPrimaryScopeView {
    FlowPrimaryScopeView {
        scope,
        payload_interval: (start as usize, end as usize),
    }
}
