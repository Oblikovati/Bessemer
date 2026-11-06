use std::io::{Cursor, Read};

pub struct LinkCase {
    pub name: String,
    pub compressed: bool,
    pub pointer_count: Option<i32>,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub version: i32,
    pub entries: Vec<LinkEntry>,
    pub variable_count: u32,
    pub tail: LinkTail,
    pub checked: bool,
    pub runtime: bool,
    pub roles: bool,
}

pub struct LinkEntry {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub references: [i32; 2],
    pub wide: [u64; 7],
    pub code: u8,
    pub native_code: u8,
    pub variable: Vec<i32>,
}

pub struct LinkTail {
    pub start: u32,
    pub members: [i32; 11],
    pub discarded: Vec<i32>,
    pub control: Option<i32>,
    pub conditional_start: u32,
    pub conditional_end: u32,
    pub conditional: Vec<i32>,
    pub numeric: Option<NumericTail>,
    pub final_tail: Option<FinalTail>,
}

pub struct NumericTail {
    pub start: u32,
    pub end: u32,
    pub wide: Vec<u64>,
    pub indices: Option<Vec<i32>>,
    pub runtime_default: Option<i32>,
}

pub struct FinalTail {
    pub start: u32,
    pub end: u32,
    pub width: u32,
    pub values: Vec<u64>,
}

pub fn cases() -> Vec<LinkCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/flow-link-records/links.bin.gz")[..])
        .take(10_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FLR1");
    let count = word(&mut cursor);
    let result = (0..count).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> LinkCase {
    LinkCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        compressed: boolean(cursor),
        pointer_count: optional(cursor),
        stored: blob(cursor),
        decoded: blob(cursor),
        version: signed(cursor),
        entries: (0..word(cursor)).map(|_| entry(cursor)).collect(),
        variable_count: word(cursor),
        tail: tail(cursor),
        checked: boolean(cursor),
        runtime: boolean(cursor),
        roles: boolean(cursor),
    }
}

fn entry(cursor: &mut Cursor<&[u8]>) -> LinkEntry {
    LinkEntry {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        references: std::array::from_fn(|_| signed(cursor)),
        wide: std::array::from_fn(|_| u64::from_le_bytes(number(cursor))),
        code: number::<1>(cursor)[0],
        native_code: number::<1>(cursor)[0],
        variable: integers(cursor),
    }
}

fn tail(cursor: &mut Cursor<&[u8]>) -> LinkTail {
    LinkTail {
        start: word(cursor),
        members: std::array::from_fn(|_| signed(cursor)),
        discarded: integers(cursor),
        control: optional(cursor),
        conditional_start: word(cursor),
        conditional_end: word(cursor),
        conditional: integers(cursor),
        numeric: numeric(cursor),
        final_tail: final_tail(cursor),
    }
}

fn numeric(cursor: &mut Cursor<&[u8]>) -> Option<NumericTail> {
    if !boolean(cursor) {
        return None;
    }
    let start = word(cursor);
    let end = word(cursor);
    let count = word(cursor);
    let wide = (0..count)
        .map(|_| u64::from_le_bytes(number(cursor)))
        .collect();
    Some(NumericTail {
        start,
        end,
        wide,
        indices: boolean(cursor).then(|| (0..count).map(|_| signed(cursor)).collect()),
        runtime_default: optional(cursor),
    })
}

fn final_tail(cursor: &mut Cursor<&[u8]>) -> Option<FinalTail> {
    if !boolean(cursor) {
        return None;
    }
    let start = word(cursor);
    let end = word(cursor);
    let width = word(cursor);
    assert!(width == 4 || width == 8);
    let values = (0..word(cursor))
        .map(|_| final_value(cursor, width))
        .collect();
    Some(FinalTail {
        start,
        end,
        width,
        values,
    })
}

fn final_value(cursor: &mut Cursor<&[u8]>, width: u32) -> u64 {
    if width == 8 {
        return u64::from_le_bytes(number(cursor));
    }
    u64::from(word(cursor))
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

fn optional(cursor: &mut Cursor<&[u8]>) -> Option<i32> {
    boolean(cursor).then(|| signed(cursor))
}

fn integers(cursor: &mut Cursor<&[u8]>) -> Vec<i32> {
    (0..word(cursor)).map(|_| signed(cursor)).collect()
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let count = word(cursor);
    let mut raw = vec![0; count as usize];
    cursor.read_exact(&mut raw).unwrap();
    raw
}
