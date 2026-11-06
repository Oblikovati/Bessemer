#[path = "types.rs"]
mod types;
use std::io::{Cursor, Read};
pub use types::*;

pub fn cases() -> Vec<ReferenceCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-reference-groups/groups.bin.gz")[..],
    )
    .take(80_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FRG1");
    let result = (0..word(&mut cursor)).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> ReferenceCase {
    ReferenceCase {
        frame: frame(cursor),
        version: signed(cursor),
        discarded_header: optional(cursor, signed),
        stored_header: optional(cursor, signed),
        native_header_default: signed(cursor),
        groups: std::array::from_fn(|_| group(cursor)),
        exclusions: std::array::from_fn(|_| boolean(cursor)),
    }
}

fn frame(cursor: &mut Cursor<&[u8]>) -> ReferenceFrame {
    ReferenceFrame {
        name: String::from_utf8(blob(cursor)).unwrap(),
        compressed: boolean(cursor),
        mapping: boolean(cursor),
        stored: blob(cursor),
        decoded: blob(cursor),
    }
}

fn group(cursor: &mut Cursor<&[u8]>) -> ReferenceGroup {
    let ordinal = word(cursor);
    let start = word(cursor);
    let end = word(cursor);
    let entries = (0..word(cursor)).map(|_| entry(cursor)).collect();
    ReferenceGroup {
        ordinal,
        start,
        end,
        entries,
        level: optional(cursor, signed),
        mapping: counted(cursor, signed),
        wide: counted(cursor, wide),
    }
}

fn entry(cursor: &mut Cursor<&[u8]>) -> ReferenceEntry {
    let start = word(cursor);
    let end = word(cursor);
    let (member20, references) = reference_preamble(cursor);
    ReferenceEntry {
        start,
        end,
        member20,
        references,
        prefix: optional(cursor, byte),
        state: byte(cursor),
        flag2: optional(cursor, byte),
        flag4: optional(cursor, byte),
        descriptors: std::array::from_fn(|_| short(cursor)),
        first: counted(cursor, word),
        second_count: signed(cursor),
        raw_small_count: optional(cursor, short),
        second: counted(cursor, word),
        suffix: optional(cursor, short),
    }
}

fn reference_preamble(cursor: &mut Cursor<&[u8]>) -> (Option<i32>, Vec<i32>) {
    let count = word(cursor);
    let member20 = optional(cursor, signed);
    let references = (0..count).map(|_| signed(cursor)).collect();
    (member20, references)
}

fn counted<T>(cursor: &mut Cursor<&[u8]>, read: fn(&mut Cursor<&[u8]>) -> T) -> Vec<T> {
    (0..word(cursor)).map(|_| read(cursor)).collect()
}

fn optional<T>(cursor: &mut Cursor<&[u8]>, read: fn(&mut Cursor<&[u8]>) -> T) -> Option<T> {
    boolean(cursor).then(|| read(cursor))
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut raw = [0; N];
    cursor.read_exact(&mut raw).unwrap();
    raw
}

fn byte(cursor: &mut Cursor<&[u8]>) -> u8 {
    number::<1>(cursor)[0]
}

fn boolean(cursor: &mut Cursor<&[u8]>) -> bool {
    let value = byte(cursor);
    assert!(value <= 1);
    value != 0
}

fn short(cursor: &mut Cursor<&[u8]>) -> u16 {
    u16::from_le_bytes(number(cursor))
}

fn word(cursor: &mut Cursor<&[u8]>) -> u32 {
    u32::from_le_bytes(number(cursor))
}

fn signed(cursor: &mut Cursor<&[u8]>) -> i32 {
    i32::from_le_bytes(number(cursor))
}

fn wide(cursor: &mut Cursor<&[u8]>) -> u64 {
    u64::from_le_bytes(number(cursor))
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let mut raw = vec![0; word(cursor) as usize];
    cursor.read_exact(&mut raw).unwrap();
    raw
}
