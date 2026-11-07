use std::io::{Cursor, Read};

pub struct MappedCase {
    pub name: String,
    pub payload: Vec<u8>,
    pub contexts: Option<Vec<Option<i32>>>,
    pub version: u32,
    pub objects: Vec<ExpectedObject>,
}

pub struct ExpectedObject {
    pub start: usize,
    pub end: usize,
    pub presence: u8,
    pub body: Option<ExpectedBody>,
}

pub struct ExpectedBody {
    pub version: u32,
    pub state: u8,
    pub numeric: Vec<u64>,
    pub pairs: Vec<(i32, i32)>,
    pub factor: u32,
    pub matched: Option<i32>,
    pub mapped_count: usize,
    pub mapped: Vec<u64>,
    pub legacy: Option<ExpectedLegacy>,
    pub context_validated: bool,
}

pub struct ExpectedLegacy {
    pub prefix: Vec<u64>,
    pub arrays: [Vec<u64>; 3],
    pub tripled: Vec<u64>,
}

pub fn cases() -> Vec<MappedCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-mapped-objects/mapped.bin.gz")[..],
    )
    .take(1_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FMO1");
    let count = word(&mut cursor);
    assert_eq!(count, 192);
    let cases = (0..count).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    cases
}

fn case(cursor: &mut Cursor<&[u8]>) -> MappedCase {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let payload = blob(cursor);
    let contexts = contexts(cursor);
    let version = word(cursor);
    let count = word(cursor);
    let objects = (0..count).map(|_| object(cursor)).collect();
    MappedCase {
        name,
        payload,
        contexts,
        version,
        objects,
    }
}

fn contexts(cursor: &mut Cursor<&[u8]>) -> Option<Vec<Option<i32>>> {
    if number::<1>(cursor)[0] == 0 {
        return None;
    }
    let count = word(cursor);
    Some((0..count).map(|_| context(cursor)).collect())
}

fn context(cursor: &mut Cursor<&[u8]>) -> Option<i32> {
    let present = number::<1>(cursor)[0] != 0;
    let value = i32::from_le_bytes(number(cursor));
    present.then_some(value)
}

fn object(cursor: &mut Cursor<&[u8]>) -> ExpectedObject {
    let start = word(cursor) as usize;
    let end = word(cursor) as usize;
    let presence = number::<1>(cursor)[0];
    let body = (presence != 0).then(|| body(cursor));
    ExpectedObject {
        start,
        end,
        presence,
        body,
    }
}

fn body(cursor: &mut Cursor<&[u8]>) -> ExpectedBody {
    ExpectedBody {
        version: word(cursor),
        state: number::<1>(cursor)[0],
        numeric: words(cursor),
        pairs: pairs(cursor),
        factor: word(cursor),
        matched: context(cursor),
        mapped_count: word(cursor) as usize,
        mapped: words(cursor),
        legacy: legacy(cursor),
        context_validated: number::<1>(cursor)[0] != 0,
    }
}

fn pairs(cursor: &mut Cursor<&[u8]>) -> Vec<(i32, i32)> {
    let count = word(cursor);
    (0..count)
        .map(|_| {
            (
                i32::from_le_bytes(number(cursor)),
                i32::from_le_bytes(number(cursor)),
            )
        })
        .collect()
}

fn legacy(cursor: &mut Cursor<&[u8]>) -> Option<ExpectedLegacy> {
    if number::<1>(cursor)[0] == 0 {
        return None;
    }
    Some(ExpectedLegacy {
        prefix: words(cursor),
        arrays: std::array::from_fn(|_| words(cursor)),
        tripled: words(cursor),
    })
}

fn words(cursor: &mut Cursor<&[u8]>) -> Vec<u64> {
    let count = word(cursor);
    (0..count)
        .map(|_| u64::from_le_bytes(number(cursor)))
        .collect()
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
