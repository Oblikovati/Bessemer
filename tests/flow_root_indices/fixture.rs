use std::io::{Cursor, Read};

pub struct RootCase {
    pub name: String,
    pub compressed: bool,
    pub maximum_depth: u16,
    pub legacy: Option<[[i32; 2]; 3]>,
    pub grid: Vec<u8>,
    pub trees: Vec<u8>,
    pub leaf_count: u32,
    pub roots: Vec<ExpectedRoot>,
    pub flags: [bool; 4],
}

pub struct ExpectedRoot {
    pub indices: [i32; 3],
    pub storage_index: i32,
    pub byte_offset: u64,
    pub leaves: Vec<ExpectedLeaf>,
}

pub struct ExpectedLeaf {
    pub ordinal: u32,
    pub path: Vec<u8>,
    pub start: u32,
    pub end: u32,
}

pub fn cases() -> Vec<RootCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-root-indices/indices.bin.gz")[..],
    )
    .take(10_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FRI1");
    assert_eq!(word(&mut cursor), 233);
    let result = (0..233).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> RootCase {
    RootCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        compressed: boolean(cursor),
        maximum_depth: u16::from_le_bytes(number(cursor)),
        legacy: boolean(cursor)
            .then(|| std::array::from_fn(|_| std::array::from_fn(|_| signed(cursor)))),
        grid: blob(cursor),
        trees: blob(cursor),
        leaf_count: word(cursor),
        roots: (0..word(cursor)).map(|_| root(cursor)).collect(),
        flags: std::array::from_fn(|_| boolean(cursor)),
    }
}

fn root(cursor: &mut Cursor<&[u8]>) -> ExpectedRoot {
    ExpectedRoot {
        indices: std::array::from_fn(|_| signed(cursor)),
        storage_index: signed(cursor),
        byte_offset: u64::from_le_bytes(number(cursor)),
        leaves: (0..word(cursor)).map(|_| leaf(cursor)).collect(),
    }
}

fn leaf(cursor: &mut Cursor<&[u8]>) -> ExpectedLeaf {
    ExpectedLeaf {
        ordinal: word(cursor),
        path: blob(cursor),
        start: word(cursor),
        end: word(cursor),
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
