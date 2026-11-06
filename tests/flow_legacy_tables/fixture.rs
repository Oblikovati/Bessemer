use std::io::{Cursor, Read};

pub struct TableCase {
    pub frame: TableFrame,
    pub version: i32,
    pub expected: ExpectedTable,
}

pub struct TableFrame {
    pub name: String,
    pub identifier: u32,
    pub compressed: bool,
    pub pointer_count: Option<u32>,
    pub legacy: Option<[u32; 2]>,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
}

pub enum ExpectedTable {
    Pointers(ExpectedPointers),
    Words(ExpectedWords),
}

pub struct ExpectedPointers {
    pub indices_checked: bool,
    pub groups: Vec<ExpectedGroup>,
    pub pointer_population: bool,
}

pub struct ExpectedGroup {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub count: u32,
    pub discarded_count: Option<i32>,
    pub body: Option<ExpectedGroupBody>,
}

pub struct ExpectedGroupBody {
    pub members: [i32; 3],
    pub lengths: [i32; 3],
    pub discarded: Vec<[i32; 3]>,
    pub arrays: Vec<ExpectedWideArray>,
    pub pre_pairs: Option<i32>,
    pub pairs: Vec<[i32; 2]>,
}

pub struct ExpectedWideArray {
    pub start: u32,
    pub end: u32,
    pub bits: Vec<u64>,
}

pub struct ExpectedWords {
    pub count: u32,
    pub count_stored: bool,
    pub words_start: u32,
    pub words: Vec<u32>,
    pub roles_resolved: bool,
}

pub fn cases() -> Vec<TableCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-legacy-tables/tables.bin.gz")[..],
    )
    .take(5_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FLT1");
    assert_eq!(word(&mut cursor), 173);
    let result = (0..173).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> TableCase {
    let frame = frame(cursor);
    let version = signed(cursor);
    let expected = if frame.identifier == 401 {
        ExpectedTable::Pointers(pointers(cursor))
    } else {
        assert_eq!(frame.identifier, 405);
        ExpectedTable::Words(words(cursor))
    };
    TableCase {
        frame,
        version,
        expected,
    }
}

fn frame(cursor: &mut Cursor<&[u8]>) -> TableFrame {
    TableFrame {
        name: String::from_utf8(blob(cursor)).unwrap(),
        identifier: word(cursor),
        compressed: boolean(cursor),
        pointer_count: boolean(cursor).then(|| word(cursor)),
        legacy: boolean(cursor).then(|| std::array::from_fn(|_| word(cursor))),
        stored: blob(cursor),
        decoded: blob(cursor),
    }
}

fn pointers(cursor: &mut Cursor<&[u8]>) -> ExpectedPointers {
    ExpectedPointers {
        indices_checked: boolean(cursor),
        groups: (0..2).map(|_| group(cursor)).collect(),
        pointer_population: boolean(cursor),
    }
}

fn group(cursor: &mut Cursor<&[u8]>) -> ExpectedGroup {
    ExpectedGroup {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        count: word(cursor),
        discarded_count: boolean(cursor).then(|| signed(cursor)),
        body: boolean(cursor).then(|| group_body(cursor)),
    }
}

fn group_body(cursor: &mut Cursor<&[u8]>) -> ExpectedGroupBody {
    ExpectedGroupBody {
        members: std::array::from_fn(|_| signed(cursor)),
        lengths: std::array::from_fn(|_| signed(cursor)),
        discarded: (0..word(cursor))
            .map(|_| std::array::from_fn(|_| signed(cursor)))
            .collect(),
        arrays: (0..3).map(|_| wide_array(cursor)).collect(),
        pre_pairs: boolean(cursor).then(|| signed(cursor)),
        pairs: (0..word(cursor))
            .map(|_| std::array::from_fn(|_| signed(cursor)))
            .collect(),
    }
}

fn wide_array(cursor: &mut Cursor<&[u8]>) -> ExpectedWideArray {
    ExpectedWideArray {
        start: word(cursor),
        end: word(cursor),
        bits: (0..word(cursor))
            .map(|_| u64::from_le_bytes(number(cursor)))
            .collect(),
    }
}

fn words(cursor: &mut Cursor<&[u8]>) -> ExpectedWords {
    ExpectedWords {
        count: word(cursor),
        count_stored: boolean(cursor),
        words_start: word(cursor),
        words: (0..word(cursor)).map(|_| word(cursor)).collect(),
        roles_resolved: boolean(cursor),
    }
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
