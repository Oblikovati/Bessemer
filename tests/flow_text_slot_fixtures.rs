//! Complete record311 slots preserve raw UTF-16, native prefixes and existing text state.
#[path = "flow_field_values/packet.rs"]
mod packet;
use packet::*;
use std::collections::BTreeSet;
struct TextSlot {
    ordinal: u32,
    start: u32,
    end: u32,
    text_start: u32,
    units: u32,
    raw: Vec<u8>,
    prefix: Vec<u8>,
    previous: Vec<u8>,
    applied: Vec<u8>,
}
struct TextSlotCase {
    name: String,
    native_header: Option<i32>,
    raw: Vec<u8>,
    header: i32,
    end: u32,
    dispatch: bool,
    slots: Vec<TextSlot>,
}
fn slot(cursor: &mut PacketCursor) -> TextSlot {
    TextSlot {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        text_start: word(cursor),
        units: word(cursor),
        raw: blob(cursor),
        prefix: blob(cursor),
        previous: blob(cursor),
        applied: blob(cursor),
    }
}
fn case(cursor: &mut PacketCursor) -> TextSlotCase {
    TextSlotCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native_header: optional(cursor, signed),
        raw: blob(cursor),
        header: signed(cursor),
        end: word(cursor),
        dispatch: boolean(cursor),
        slots: counted(cursor, slot),
    }
}
fn cases() -> Vec<TextSlotCase> {
    let mut cursor = cursor(
        include_bytes!("fixtures/flow-text-slots/record311.bin.gz"),
        *b"FTS1",
    );
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
fn native_prefix(raw: &[u8]) -> &[u8] {
    let units = raw.as_chunks::<2>().0;
    let count = units
        .iter()
        .position(|unit| *unit == [0, 0])
        .unwrap_or(units.len());
    &raw[..count * 2]
}
fn check_slot(slot: &TextSlot, ordinal: usize, start: usize) {
    assert_eq!(slot.ordinal as usize, ordinal);
    assert_eq!(slot.start as usize, start);
    assert_eq!(slot.text_start as usize, start + 4);
    assert!(slot.units <= i32::MAX as u32);
    assert_eq!(slot.raw.len(), slot.units as usize * 2);
    assert_eq!(slot.end as usize, start + 4 + slot.raw.len());
    if !slot.raw.is_empty() {
        assert_eq!(&slot.raw[slot.raw.len() - 2..], &[0, 0]);
    }
    assert_eq!(slot.prefix, native_prefix(&slot.raw));
    assert!(slot.previous.len().is_multiple_of(2) && slot.applied.len().is_multiple_of(2));
    let applied = if slot.prefix.is_empty() {
        &slot.previous
    } else {
        &slot.prefix
    };
    assert_eq!(&slot.applied, applied);
}
fn rebuilt(row: &TextSlotCase) -> Vec<u8> {
    let mut raw = row.header.to_le_bytes().to_vec();
    let mut start = 4;
    for (ordinal, slot) in row.slots.iter().enumerate() {
        check_slot(slot, ordinal, start);
        raw.extend_from_slice(&slot.units.to_le_bytes());
        raw.extend_from_slice(&slot.raw);
        start = slot.end as usize;
    }
    assert_eq!(start, row.raw.len());
    raw
}
#[test]
fn every_owned_slot_rebuilds_complete_raw_units_prefix_and_existing_state_projection() {
    let rows = cases();
    assert_eq!(rows.len(), 755);
    for row in rows {
        assert_eq!(row.slots.len(), 50);
        assert!(!row.dispatch);
        assert_eq!(row.end as usize, row.raw.len());
        assert_eq!(rebuilt(&row), row.raw, "{}", row.name);
    }
}
#[test]
fn all_header_bits_every_utf16_code_unit_and_boundary_lengths_are_owned() {
    let rows = cases();
    let headers: BTreeSet<_> = rows.iter().map(|row| row.header as u32).collect();
    for bit in 0..32 {
        assert!(headers.contains(&(1u32 << bit)) && headers.contains(&!(1u32 << bit)));
    }
    let slots: Vec<_> = rows.iter().flat_map(|row| &row.slots).collect();
    let units: BTreeSet<_> = slots
        .iter()
        .flat_map(|slot| slot.raw.as_chunks::<2>().0.iter())
        .map(|unit| u16::from_le_bytes(*unit))
        .collect();
    assert_eq!(units, (0..=u16::MAX).collect());
    let lengths: BTreeSet<_> = slots.iter().map(|slot| slot.units).collect();
    assert!(lengths.is_superset(&BTreeSet::from([
        0, 1, 2, 255, 256, 257, 4096, 65536, 65537
    ])));
}
#[test]
fn all_fifty_positions_retain_zero_count_nul_only_and_unpaired_surrogate_shapes() {
    let rows = cases();
    for ordinal in 0..50 {
        for raw in [
            &b""[..],
            &b"\0\0"[..],
            &b"\0\xd8\0\0"[..],
            &b"\0\xdc\0\0"[..],
        ] {
            assert!(rows.iter().any(|row| row.slots[ordinal].raw == raw));
        }
    }
}
#[test]
fn empty_native_prefix_retains_previous_embedded_nuls_hidden_text_and_surrogates() {
    let rows = cases();
    let slots: Vec<_> = rows
        .iter()
        .flat_map(|row| &row.slots)
        .filter(|slot| slot.prefix.is_empty())
        .collect();
    check_previous_shapes(&slots);
    assert!(
        slots
            .iter()
            .any(|slot| slot.raw.len() > 2 && slot.raw.starts_with(&[0, 0]))
    );
    for slot in slots {
        assert_eq!(slot.applied, slot.previous);
    }
}
fn check_previous_shapes(slots: &[&TextSlot]) {
    assert!(slots.iter().any(|slot| slot.previous.is_empty()));
    assert!(
        slots
            .iter()
            .any(|slot| slot.previous.starts_with(&[0, 0]) && slot.previous.len() > 2)
    );
    assert!(slots.iter().any(|slot| slot.previous == [0, 0xd8]));
}
#[test]
fn native_source_retains_the_complete_fifty_slots_and_220_stored_units() {
    let rows = cases();
    let native: Vec<_> = rows
        .iter()
        .filter(|row| row.native_header.is_some())
        .collect();
    assert_eq!(native.len(), 1);
    let row = native[0];
    assert_eq!(
        (row.header, row.native_header, row.slots.len(), row.end),
        (0, Some(0), 50, 644)
    );
    assert_eq!(row.slots.iter().map(|slot| slot.units).sum::<u32>(), 220);
    assert_eq!(
        (row.slots[0].start, row.slots[0].end, row.slots[0].units),
        (4, 170, 81)
    );
}
