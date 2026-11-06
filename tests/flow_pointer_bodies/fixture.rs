#[path = "types.rs"]
mod types;
use std::io::{Cursor, Read};
pub use types::*;

pub fn cases() -> Vec<PointerCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-pointer-bodies/bodies.bin.gz")[..],
    )
    .take(20_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FPB1");
    let result = (0..word(&mut cursor)).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> PointerCase {
    let frame = frame(cursor);
    let version = i32::from_le_bytes(number(cursor));
    let pools = (0..frame.leaf).map(|_| pool(cursor)).collect();
    PointerCase {
        frame,
        version,
        pools,
        groups: std::array::from_fn(|_| group(cursor)),
        exclusions: std::array::from_fn(|_| boolean(cursor)),
    }
}

fn frame(cursor: &mut Cursor<&[u8]>) -> PointerFrame {
    PointerFrame {
        name: String::from_utf8(blob(cursor)).unwrap(),
        compressed: boolean(cursor),
        leaf: word(cursor),
        counts: std::array::from_fn(|_| word(cursor)),
        codes: boolean(cursor).then(|| blob(cursor)),
        stored: blob(cursor),
        decoded: blob(cursor),
    }
}

fn pool(cursor: &mut Cursor<&[u8]>) -> PointerPool {
    let ordinal = word(cursor);
    let start = word(cursor);
    let end = word(cursor);
    let count = word(cursor);
    let bits = wide(cursor, count as usize * 3);
    PointerPool {
        ordinal,
        start,
        end,
        count,
        bits,
    }
}

fn group(cursor: &mut Cursor<&[u8]>) -> PointerGroup {
    let ordinal = word(cursor);
    let start = word(cursor);
    let end = word(cursor);
    let count = word(cursor);
    let entries = (0..count).map(|_| entry(cursor)).collect();
    PointerGroup {
        ordinal,
        start,
        end,
        count,
        entries,
    }
}

fn entry(cursor: &mut Cursor<&[u8]>) -> PointerEntry {
    let ordinal = word(cursor);
    let start = word(cursor);
    let end = word(cursor);
    let mode = number::<1>(cursor)[0];
    let code = boolean(cursor).then(|| number::<1>(cursor)[0]);
    PointerEntry {
        ordinal,
        start,
        end,
        code,
        body: body(cursor, mode),
    }
}

fn body(cursor: &mut Cursor<&[u8]>, mode: u8) -> PointerEntryBody {
    match mode {
        0 => PointerEntryBody::Single(blob(cursor)),
        1 => wide_body(cursor),
        2 => PointerEntryBody::Multiple((0..word(cursor)).map(|_| blob(cursor)).collect()),
        3 => PointerEntryBody::Tagged(tagged(cursor)),
        _ => panic!("entry mode {mode}; expected 0..3"),
    }
}

fn wide_body(cursor: &mut Cursor<&[u8]>) -> PointerEntryBody {
    let count = word(cursor);
    PointerEntryBody::Wide {
        count,
        bits: wide(cursor, count as usize * 3),
    }
}

fn tagged(cursor: &mut Cursor<&[u8]>) -> TaggedPointerBody {
    TaggedPointerBody {
        tag: number::<1>(cursor)[0],
        count: boolean(cursor).then(|| word(cursor)),
        bits: boolean(cursor).then(|| {
            let count = word(cursor);
            wide(cursor, count as usize)
        }),
        raw: boolean(cursor).then(|| blob(cursor)),
        packed_decoded: boolean(cursor).then(|| boolean(cursor)),
    }
}

fn wide(cursor: &mut Cursor<&[u8]>, count: usize) -> Vec<u64> {
    (0..count)
        .map(|_| u64::from_le_bytes(number(cursor)))
        .collect()
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

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let mut raw = vec![0; word(cursor) as usize];
    cursor.read_exact(&mut raw).unwrap();
    raw
}
