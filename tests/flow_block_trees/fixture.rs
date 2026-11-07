use std::io::{Cursor, Read};

pub struct TreeCase {
    pub name: String,
    pub compressed: bool,
    pub maximum_depth: u16,
    pub legacy_bounds: Option<[[i32; 2]; 3]>,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub version: i32,
    pub bounds: [[i32; 2]; 3],
    pub bounds_stored: bool,
    pub grid_count: u128,
    pub body_present: bool,
    pub body_presence: Option<u8>,
    pub roots: Option<Vec<ExpectedRoot>>,
    pub nodes: Vec<ExpectedNode>,
    pub native_order_proven: bool,
}

pub struct ExpectedRoot {
    pub start: usize,
    pub end: usize,
    pub indices: [i32; 3],
    pub presence: Option<u8>,
    pub node: Option<usize>,
}

pub struct ExpectedNode {
    pub start: usize,
    pub end: usize,
    pub marker: u8,
    pub depth: u16,
    pub level: u8,
    pub word: Option<u32>,
    pub children: Vec<(Option<u8>, usize)>,
}

pub fn cases() -> Vec<TreeCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/flow-block-trees/trees.bin.gz")[..])
        .take(20_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FBT1");
    assert_eq!(word(&mut cursor), 81);
    let result = (0..81).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> TreeCase {
    TreeCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        compressed: boolean(cursor),
        maximum_depth: u16::from_le_bytes(number(cursor)),
        legacy_bounds: boolean(cursor).then(|| bounds(cursor)),
        stored: blob(cursor),
        decoded: blob(cursor),
        version: i32::from_le_bytes(number(cursor)),
        bounds: bounds(cursor),
        bounds_stored: boolean(cursor),
        grid_count: u128::from_le_bytes(number(cursor)),
        body_present: boolean(cursor),
        body_presence: optional_byte(cursor),
        roots: roots(cursor),
        nodes: nodes(cursor),
        native_order_proven: boolean(cursor),
    }
}

fn roots(cursor: &mut Cursor<&[u8]>) -> Option<Vec<ExpectedRoot>> {
    let present = boolean(cursor);
    let count = word(cursor);
    let roots: Vec<ExpectedRoot> = (0..count).map(|_| root(cursor)).collect();
    if present {
        return Some(roots);
    }
    assert!(roots.is_empty());
    None
}

fn root(cursor: &mut Cursor<&[u8]>) -> ExpectedRoot {
    let start = word(cursor) as usize;
    let end = word(cursor) as usize;
    let indices = std::array::from_fn(|_| i32::from_le_bytes(number(cursor)));
    let presence = optional_byte(cursor);
    let node = i32::from_le_bytes(number(cursor));
    ExpectedRoot {
        start,
        end,
        indices,
        presence,
        node: (node != -1).then(|| usize::try_from(node).unwrap()),
    }
}

fn node(cursor: &mut Cursor<&[u8]>) -> ExpectedNode {
    ExpectedNode {
        start: word(cursor) as usize,
        end: word(cursor) as usize,
        marker: number::<1>(cursor)[0],
        depth: u16::from_le_bytes(number(cursor)),
        level: number::<1>(cursor)[0],
        word: boolean(cursor).then(|| word(cursor)),
        children: children(cursor),
    }
}

fn bounds(cursor: &mut Cursor<&[u8]>) -> [[i32; 2]; 3] {
    std::array::from_fn(|_| std::array::from_fn(|_| i32::from_le_bytes(number(cursor))))
}

fn optional_byte(cursor: &mut Cursor<&[u8]>) -> Option<u8> {
    boolean(cursor).then(|| number::<1>(cursor)[0])
}

fn boolean(cursor: &mut Cursor<&[u8]>) -> bool {
    match number::<1>(cursor)[0] {
        0 => false,
        1 => true,
        other => panic!("fixture Boolean {other}; expected zero or one"),
    }
}

fn word(cursor: &mut Cursor<&[u8]>) -> u32 {
    u32::from_le_bytes(number(cursor))
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let length = word(cursor) as usize;
    let mut bytes = vec![0; length];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut bytes = [0; N];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}

fn nodes(cursor: &mut Cursor<&[u8]>) -> Vec<ExpectedNode> {
    let count = word(cursor);
    (0..count).map(|_| node(cursor)).collect()
}

fn children(cursor: &mut Cursor<&[u8]>) -> Vec<(Option<u8>, usize)> {
    let count = number::<1>(cursor)[0];
    (0..count)
        .map(|_| (optional_byte(cursor), word(cursor) as usize))
        .collect()
}
