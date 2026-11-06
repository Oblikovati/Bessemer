use std::io::{Cursor, Read};

pub struct SimpleCase {
    pub frame: SimpleFrame,
    pub version: i32,
    pub expected: ExpectedSimpleTable,
    pub runtime: bool,
    pub roles: bool,
    pub tree_order: Option<bool>,
}

pub struct SimpleFrame {
    pub name: String,
    pub identifier: u32,
    pub compressed: bool,
    pub leaf: Option<u32>,
    pub legacy: Option<u32>,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
}

pub enum ExpectedSimpleTable {
    Parallel([ParallelGroup; 2]),
    LeafPairs(Vec<LeafPair>),
    FiveWords(FiveWords),
    Bytes(ByteTable),
}

pub struct ParallelGroup {
    pub ordinal: u32,
    pub count: u32,
    pub start: u32,
    pub end: u32,
    pub presence: Option<i32>,
    pub arrays: Vec<WideArray>,
}

pub struct WideArray {
    pub start: u32,
    pub end: u32,
    pub bits: Vec<u64>,
}

pub struct LeafPair {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub bits: [u64; 2],
}

pub struct FiveWords {
    pub count: Option<u32>,
    pub stored: bool,
    pub start: u32,
    pub wire_present: bool,
    pub rows: Vec<[u32; 5]>,
}

pub struct ByteTable {
    pub count: u32,
    pub stored: bool,
    pub start: u32,
    pub bytes: Vec<u8>,
}

pub fn cases() -> Vec<SimpleCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-simple-tables/tables.bin.gz")[..],
    )
    .take(10_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FST1");
    let result = (0..word(&mut cursor)).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> SimpleCase {
    let frame = frame(cursor);
    let version = signed(cursor);
    let expected = expected(cursor, frame.identifier);
    SimpleCase {
        frame,
        version,
        expected,
        runtime: boolean(cursor),
        roles: boolean(cursor),
        tree_order: boolean(cursor).then(|| boolean(cursor)),
    }
}

fn frame(cursor: &mut Cursor<&[u8]>) -> SimpleFrame {
    SimpleFrame {
        name: String::from_utf8(blob(cursor)).unwrap(),
        identifier: word(cursor),
        compressed: boolean(cursor),
        leaf: optional(cursor),
        legacy: optional(cursor),
        stored: blob(cursor),
        decoded: blob(cursor),
    }
}

fn expected(cursor: &mut Cursor<&[u8]>, identifier: u32) -> ExpectedSimpleTable {
    match identifier {
        407 => ExpectedSimpleTable::Parallel(std::array::from_fn(|_| parallel_group(cursor))),
        437 => {
            ExpectedSimpleTable::LeafPairs((0..word(cursor)).map(|_| leaf_pair(cursor)).collect())
        }
        411 => ExpectedSimpleTable::FiveWords(five_words(cursor)),
        413 => ExpectedSimpleTable::Bytes(byte_table(cursor)),
        _ => panic!("simple-table identifier {identifier}; expected 407/437/411/413"),
    }
}

fn parallel_group(cursor: &mut Cursor<&[u8]>) -> ParallelGroup {
    ParallelGroup {
        ordinal: word(cursor),
        count: word(cursor),
        start: word(cursor),
        end: word(cursor),
        presence: boolean(cursor).then(|| signed(cursor)),
        arrays: (0..word(cursor)).map(|_| wide_array(cursor)).collect(),
    }
}

fn wide_array(cursor: &mut Cursor<&[u8]>) -> WideArray {
    WideArray {
        start: word(cursor),
        end: word(cursor),
        bits: (0..word(cursor))
            .map(|_| u64::from_le_bytes(number(cursor)))
            .collect(),
    }
}

fn leaf_pair(cursor: &mut Cursor<&[u8]>) -> LeafPair {
    LeafPair {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        bits: std::array::from_fn(|_| u64::from_le_bytes(number(cursor))),
    }
}

fn five_words(cursor: &mut Cursor<&[u8]>) -> FiveWords {
    FiveWords {
        count: optional(cursor),
        stored: boolean(cursor),
        start: word(cursor),
        wire_present: boolean(cursor),
        rows: (0..word(cursor))
            .map(|_| std::array::from_fn(|_| word(cursor)))
            .collect(),
    }
}

fn byte_table(cursor: &mut Cursor<&[u8]>) -> ByteTable {
    ByteTable {
        count: word(cursor),
        stored: boolean(cursor),
        start: word(cursor),
        bytes: blob(cursor),
    }
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut raw = [0; N];
    cursor.read_exact(&mut raw).unwrap();
    raw
}

fn boolean(cursor: &mut Cursor<&[u8]>) -> bool {
    let value = number::<1>(cursor)[0];
    assert!(value <= 1);
    value != 0
}

fn word(cursor: &mut Cursor<&[u8]>) -> u32 {
    u32::from_le_bytes(number(cursor))
}

fn signed(cursor: &mut Cursor<&[u8]>) -> i32 {
    i32::from_le_bytes(number(cursor))
}

fn optional(cursor: &mut Cursor<&[u8]>) -> Option<u32> {
    boolean(cursor).then(|| word(cursor))
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let mut raw = vec![0; word(cursor) as usize];
    cursor.read_exact(&mut raw).unwrap();
    raw
}
