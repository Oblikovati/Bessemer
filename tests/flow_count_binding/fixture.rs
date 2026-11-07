use std::io::{Cursor, Read};

pub struct CountCase {
    pub name: String,
    pub compressed: bool,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub prefix: ExpectedCounts,
    pub binding: Option<ExpectedBinding>,
    pub arrays: Vec<ExpectedArrays>,
}

pub struct ExpectedCounts {
    pub version: i32,
    pub counts: [i32; 2],
    pub native: i32,
    pub mathematical: u64,
    pub restored: bool,
    pub opaque: Vec<u8>,
}

pub struct ExpectedBinding {
    pub expected_count: u32,
    pub pointer_slot_bytes: u32,
    pub pointer_array_bytes: u64,
    pub starts: [u32; 2],
    pub offsets: [u64; 2],
    pub flags: [bool; 5],
}

pub struct ExpectedArrays {
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub selector: i32,
    pub count: i32,
    pub first: Option<Vec<i32>>,
    pub second: Vec<i32>,
    pub conditional_count_matched: bool,
}

pub fn cases() -> Vec<CountCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-count-binding/binding.bin.gz")[..],
    )
    .take(2_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FCB1");
    assert_eq!(word(&mut cursor), 193);
    let result = (0..193).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> CountCase {
    CountCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        compressed: boolean(cursor),
        stored: blob(cursor),
        decoded: blob(cursor),
        prefix: counts(cursor),
        binding: boolean(cursor).then(|| binding(cursor)),
        arrays: (0..word(cursor)).map(|_| arrays(cursor)).collect(),
    }
}

fn counts(cursor: &mut Cursor<&[u8]>) -> ExpectedCounts {
    ExpectedCounts {
        version: signed(cursor),
        counts: std::array::from_fn(|_| signed(cursor)),
        native: signed(cursor),
        mathematical: u64::from_le_bytes(number(cursor)),
        restored: boolean(cursor),
        opaque: blob(cursor),
    }
}

fn binding(cursor: &mut Cursor<&[u8]>) -> ExpectedBinding {
    ExpectedBinding {
        expected_count: word(cursor),
        pointer_slot_bytes: word(cursor),
        pointer_array_bytes: u64::from_le_bytes(number(cursor)),
        starts: std::array::from_fn(|_| word(cursor)),
        offsets: std::array::from_fn(|_| u64::from_le_bytes(number(cursor))),
        flags: std::array::from_fn(|_| boolean(cursor)),
    }
}

fn arrays(cursor: &mut Cursor<&[u8]>) -> ExpectedArrays {
    let stored = blob(cursor);
    let decoded = blob(cursor);
    let selector = signed(cursor);
    let count = signed(cursor);
    let first_present = boolean(cursor);
    let first = signed_words(cursor);
    assert!(first_present || first.is_empty());
    ExpectedArrays {
        stored,
        decoded,
        selector,
        count,
        first: first_present.then_some(first),
        second: signed_words(cursor),
        conditional_count_matched: boolean(cursor),
    }
}

fn signed_words(cursor: &mut Cursor<&[u8]>) -> Vec<i32> {
    (0..word(cursor)).map(|_| signed(cursor)).collect()
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut raw = [0; N];
    cursor.read_exact(&mut raw).unwrap();
    raw
}

fn boolean(cursor: &mut Cursor<&[u8]>) -> bool {
    let byte = number::<1>(cursor)[0];
    assert!(byte <= 1);
    byte != 0
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
