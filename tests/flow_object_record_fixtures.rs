//! Owned complete record-304 projections before introducing a typed editor.
#[path = "flow_field_values/packet.rs"]
mod packet;
use packet::*;
use std::collections::BTreeSet;

struct ObjectItem {
    ordinal: u32,
    start: u32,
    end: u32,
    member: i32,
    legacy: Option<Vec<u8>>,
    block: Vec<u8>,
    extra: Option<Vec<u8>>,
    state: Option<Vec<u8>>,
    native_state: u8,
    extension: Option<Vec<u8>>,
}
struct ObjectTrailer {
    member: i32,
    block: Vec<u8>,
    appended: bool,
}
struct ObjectCase {
    name: String,
    native_count: Option<i32>,
    raw: Vec<u8>,
    version: i32,
    count: i32,
    end: u32,
    items: Vec<ObjectItem>,
    trailer: Option<ObjectTrailer>,
}
fn object_item(cursor: &mut PacketCursor) -> ObjectItem {
    ObjectItem {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        member: signed(cursor),
        legacy: optional(cursor, blob),
        block: blob(cursor),
        extra: optional(cursor, blob),
        state: optional(cursor, blob),
        native_state: byte(cursor),
        extension: optional(cursor, blob),
    }
}
fn object_trailer(cursor: &mut PacketCursor) -> ObjectTrailer {
    ObjectTrailer {
        member: signed(cursor),
        block: blob(cursor),
        appended: boolean(cursor),
    }
}
fn object_case(cursor: &mut PacketCursor) -> ObjectCase {
    let result = ObjectCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native_count: optional(cursor, signed),
        raw: blob(cursor),
        version: signed(cursor),
        count: signed(cursor),
        end: word(cursor),
        items: counted(cursor, object_item),
        trailer: optional(cursor, object_trailer),
    };
    assert!(boolean(cursor));
    assert!(!boolean(cursor) && !boolean(cursor));
    result
}
fn cases() -> Vec<ObjectCase> {
    let mut cursor = cursor(
        include_bytes!("fixtures/flow-object-records/record304.bin.gz"),
        *b"FOB1",
    );
    let result = counted(&mut cursor, object_case);
    finish(&cursor);
    result
}
fn item_bytes(item: &ObjectItem) -> Vec<u8> {
    let mut raw = item.member.to_le_bytes().to_vec();
    raw.extend(item.legacy.iter().flatten());
    raw.extend_from_slice(&item.block);
    raw.extend(item.extra.iter().flatten());
    raw.extend(item.state.iter().flatten());
    raw.extend(item.extension.iter().flatten());
    raw
}
fn check_item(item: &ObjectItem, ordinal: usize, version: i32, stride: usize) {
    assert_eq!(item.ordinal as usize, ordinal);
    assert_eq!(item.start as usize, 8 + ordinal * stride);
    assert_eq!(item.end as usize, 8 + (ordinal + 1) * stride);
    check_field_lengths(item, version);
    assert_eq!(
        item.native_state,
        item.state.as_ref().map_or(0, |raw| raw[0])
    );
    assert_eq!(item_bytes(item).len(), stride);
}
fn check_field_lengths(item: &ObjectItem, version: i32) {
    let actual = [
        item.legacy.as_ref().map(Vec::len),
        Some(item.block.len()),
        item.extra.as_ref().map(Vec::len),
        item.state.as_ref().map(Vec::len),
        item.extension.as_ref().map(Vec::len),
    ];
    let expected = [
        (version < 5).then_some(16),
        Some(16),
        (version >= 4).then_some(4),
        (version >= 6).then_some(1),
        (version == 7).then_some(56),
    ];
    assert_eq!(actual, expected);
}
fn rebuilt(row: &ObjectCase) -> Vec<u8> {
    let mut raw = row.version.to_le_bytes().to_vec();
    raw.extend_from_slice(&row.count.to_le_bytes());
    for item in &row.items {
        raw.extend_from_slice(&item_bytes(item));
    }
    if let Some(trailer) = &row.trailer {
        assert_eq!(trailer.block.len(), 16);
        assert_eq!(trailer.appended, row.count == 1);
        raw.extend_from_slice(&trailer.member.to_le_bytes());
        raw.extend_from_slice(&trailer.block);
    }
    raw
}
#[test]
fn complete_owned_projections_rebuild_every_field_interval_trailer_and_eof() {
    let rows = cases();
    assert_eq!(rows.len(), 2690);
    for row in rows {
        let stride = [36, 36, 36, 36, 40, 24, 25, 81][row.version as usize];
        assert_eq!(row.items.len(), row.count as usize, "{}", row.name);
        for (ordinal, item) in row.items.iter().enumerate() {
            check_item(item, ordinal, row.version, stride);
        }
        assert_eq!(row.trailer.is_some(), row.count > 0 && row.version < 5);
        assert_eq!(rebuilt(&row), row.raw, "{}", row.name);
        assert_eq!(row.end as usize, row.raw.len());
    }
}
#[test]
fn all_versions_signed_patterns_state_bytes_and_count_boundaries_are_owned() {
    let rows = cases();
    for version in 0..8 {
        let subset: Vec<_> = rows.iter().filter(|row| row.version == version).collect();
        let members: BTreeSet<_> = subset
            .iter()
            .flat_map(|row| &row.items)
            .map(|item| item.member as u32)
            .collect();
        for bit in 0..32 {
            assert!(members.contains(&(1u32 << bit)) && members.contains(&!(1u32 << bit)));
        }
        let counts: BTreeSet<_> = subset.iter().map(|row| row.count).collect();
        assert!(counts.is_superset(&BTreeSet::from([
            0, 1, 2, 3, 7, 16, 31, 32, 255, 256, 257, 1024
        ])));
        check_states(&subset, version);
    }
}
fn check_states(rows: &[&ObjectCase], version: i32) {
    let states: BTreeSet<_> = rows
        .iter()
        .flat_map(|row| &row.items)
        .map(|item| item.native_state)
        .collect();
    let expected: BTreeSet<_> = if version >= 6 {
        (0..=255).collect()
    } else {
        BTreeSet::from([0])
    };
    assert_eq!(states, expected);
}
#[test]
fn both_native_sources_retain_complete_legacy_rows_and_appended_trailer() {
    let rows = cases();
    let native: Vec<_> = rows
        .iter()
        .filter(|row| row.native_count.is_some())
        .collect();
    assert_eq!(native.len(), 2);
    assert_eq!(native[0].raw, native[1].raw);
    for row in native {
        assert_eq!(
            (row.version, row.count, row.end, row.native_count),
            (3, 1, 64, Some(1))
        );
        let expected: Vec<_> = [1936i32, 624, 1424, 0]
            .into_iter()
            .flat_map(i32::to_le_bytes)
            .collect();
        assert_eq!(row.items[0].legacy, Some(expected));
        assert_eq!(row.items[0].block, [0; 16]);
        assert_eq!(row.trailer.as_ref().unwrap().member, 1);
    }
}
