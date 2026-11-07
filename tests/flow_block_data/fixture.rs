use std::io::{Cursor, Read};

pub struct BlockCase {
    pub name: String,
    pub compressed: bool,
    pub leaf_count: u32,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub prefix: ExpectedPrefix,
    pub bitmap: Vec<u8>,
    pub totals: [u32; 3],
    pub entries_equal_total: bool,
    pub blocks: Vec<ExpectedBlock>,
}

pub struct ExpectedPrefix {
    pub version: i32,
    pub fluid: i32,
    pub solid: i32,
    pub mathematical: u64,
    pub native: i32,
}

pub struct ExpectedBlock {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub byte: u8,
    pub word_10: u16,
    pub raw_count: u16,
    pub signed_count: i16,
    pub native_assign: bool,
    pub entries: Vec<ExpectedEntry>,
}

pub struct ExpectedEntry {
    pub start: u32,
    pub end: u32,
    pub identifier: i32,
    pub legacy_index: Option<i32>,
    pub member_20: Option<i32>,
    pub member_28: i32,
    pub discarded_prefix: Option<u8>,
    pub state: u8,
    pub flag_2: Option<u8>,
    pub flag_4: Option<u8>,
    pub tail: ExpectedTail,
}

pub struct ExpectedTail {
    pub descriptors: [u16; 7],
    pub first: Vec<u32>,
    pub second_count: i32,
    pub raw_second: Option<u16>,
    pub second: Vec<u32>,
    pub discarded_suffix: Option<u16>,
}

pub fn cases() -> Vec<BlockCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/flow-block-data/blocks.bin.gz")[..])
        .take(3_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FBD1");
    assert_eq!(word(&mut cursor), 273);
    let result = (0..273).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> BlockCase {
    BlockCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        compressed: boolean(cursor),
        leaf_count: word(cursor),
        stored: blob(cursor),
        decoded: blob(cursor),
        prefix: prefix(cursor),
        bitmap: blob(cursor),
        totals: std::array::from_fn(|_| word(cursor)),
        entries_equal_total: boolean(cursor),
        blocks: (0..word(cursor)).map(|_| block(cursor)).collect(),
    }
}

fn prefix(cursor: &mut Cursor<&[u8]>) -> ExpectedPrefix {
    ExpectedPrefix {
        version: signed(cursor),
        fluid: signed(cursor),
        solid: signed(cursor),
        mathematical: u64::from_le_bytes(number(cursor)),
        native: signed(cursor),
    }
}

fn block(cursor: &mut Cursor<&[u8]>) -> ExpectedBlock {
    ExpectedBlock {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        byte: byte(cursor),
        word_10: short(cursor),
        raw_count: short(cursor),
        signed_count: i16::from_le_bytes(number(cursor)),
        native_assign: boolean(cursor),
        entries: (0..word(cursor)).map(|_| entry(cursor)).collect(),
    }
}

fn entry(cursor: &mut Cursor<&[u8]>) -> ExpectedEntry {
    ExpectedEntry {
        start: word(cursor),
        end: word(cursor),
        identifier: signed(cursor),
        legacy_index: optional(cursor, signed),
        member_20: optional(cursor, signed),
        member_28: signed(cursor),
        discarded_prefix: optional(cursor, byte),
        state: byte(cursor),
        flag_2: optional(cursor, byte),
        flag_4: optional(cursor, byte),
        tail: tail(cursor),
    }
}

fn tail(cursor: &mut Cursor<&[u8]>) -> ExpectedTail {
    ExpectedTail {
        descriptors: std::array::from_fn(|_| short(cursor)),
        first: words(cursor),
        second_count: signed(cursor),
        raw_second: optional(cursor, short),
        second: words(cursor),
        discarded_suffix: optional(cursor, short),
    }
}

fn optional<T>(
    cursor: &mut Cursor<&[u8]>,
    read: impl FnOnce(&mut Cursor<&[u8]>) -> T,
) -> Option<T> {
    boolean(cursor).then(|| read(cursor))
}

fn words(cursor: &mut Cursor<&[u8]>) -> Vec<u32> {
    (0..word(cursor)).map(|_| word(cursor)).collect()
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
    let raw = byte(cursor);
    assert!(raw <= 1);
    raw != 0
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

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let mut raw = vec![0; word(cursor) as usize];
    cursor.read_exact(&mut raw).unwrap();
    raw
}
