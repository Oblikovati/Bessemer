#[path = "types.rs"]
mod types;
use std::io::{Cursor, Read};
pub use types::*;

pub fn cases() -> Vec<ExtraCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(
        &include_bytes!("../fixtures/flow-block-extras/extras.bin.gz")[..],
    )
    .take(30_000_001)
    .read_to_end(&mut raw)
    .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FBE1");
    let result = (0..word(&mut cursor)).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> ExtraCase {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let compressed = boolean(cursor);
    let leaves = word(cursor);
    let enabled = boolean(cursor);
    let stored = blob(cursor);
    let decoded = blob(cursor);
    let version = signed(cursor);
    ExtraCase {
        name,
        compressed,
        leaves,
        enabled,
        stored,
        decoded,
        version,
        blocks: (0..word(cursor)).map(|_| block(cursor, version)).collect(),
        tree_order: boolean(cursor),
        references: boolean(cursor),
        runtime: boolean(cursor),
    }
}

fn block(cursor: &mut Cursor<&[u8]>, version: i32) -> ExtraBlock {
    ExtraBlock {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        token: signed(cursor),
        flag: optional_byte(cursor),
        body: boolean(cursor).then(|| body(cursor, version)),
    }
}

fn body(cursor: &mut Cursor<&[u8]>, version: i32) -> ExtraBody {
    ExtraBody {
        start: word(cursor),
        end: word(cursor),
        first: (0..word(cursor)).map(|_| first(cursor, version)).collect(),
        second: (0..word(cursor)).map(|_| second(cursor)).collect(),
        third: (0..word(cursor)).map(|_| third(cursor)).collect(),
        fourth_wide: wide(cursor),
        fourth_bytes: blob(cursor),
        outer: byte(cursor),
        nested_flag: optional_byte(cursor),
        nested: nested_rows(cursor),
        final_flag: optional_byte(cursor),
        final_values: final_values(cursor),
        extra_flag: optional_byte(cursor),
        extra: extra(cursor),
        last: optional_byte(cursor),
    }
}

fn first(cursor: &mut Cursor<&[u8]>, version: i32) -> ExtraFirst {
    ExtraFirst {
        start: word(cursor),
        end: word(cursor),
        pair: number(cursor),
        words: std::array::from_fn(|_| signed(cursor)),
        mixed: mixed(cursor),
        references: std::array::from_fn(|_| reference(cursor, version)),
        bytes: blob(cursor),
        additional_flag: optional_byte(cursor),
        additional: integers(cursor),
    }
}

fn reference(cursor: &mut Cursor<&[u8]>, version: i32) -> u64 {
    if version < 5 {
        return u64::from(word(cursor));
    }
    u64::from_le_bytes(number(cursor))
}

fn mixed(cursor: &mut Cursor<&[u8]>) -> ExtraMixed {
    let start = word(cursor);
    let end = word(cursor);
    let interleaved = boolean(cursor);
    let words = integers(cursor);
    let wide = (0..words.len())
        .map(|_| u64::from_le_bytes(number(cursor)))
        .collect();
    ExtraMixed {
        start,
        end,
        interleaved,
        words,
        wide,
    }
}

fn second(cursor: &mut Cursor<&[u8]>) -> ExtraSecond {
    ExtraSecond {
        start: word(cursor),
        end: word(cursor),
        pair: number(cursor),
        tag: byte(cursor),
        bytes: blob(cursor),
    }
}

fn third(cursor: &mut Cursor<&[u8]>) -> ExtraThird {
    ExtraThird {
        start: word(cursor),
        end: word(cursor),
        word: signed(cursor),
        wide: std::array::from_fn(|_| u64::from_le_bytes(number(cursor))),
    }
}

fn nested_rows(cursor: &mut Cursor<&[u8]>) -> Option<Vec<Vec<ExtraNested>>> {
    if !boolean(cursor) {
        return None;
    }
    Some(
        (0..word(cursor))
            .map(|_| (0..word(cursor)).map(|_| nested(cursor)).collect())
            .collect(),
    )
}

fn nested(cursor: &mut Cursor<&[u8]>) -> ExtraNested {
    let start = word(cursor);
    let end = word(cursor);
    let stored_index = boolean(cursor).then(|| signed(cursor));
    let native_index = signed(cursor);
    let words = integers(cursor);
    let wide = (0..3 * words.len())
        .map(|_| u64::from_le_bytes(number(cursor)))
        .collect();
    ExtraNested {
        start,
        end,
        stored_index,
        native_index,
        words,
        wide,
    }
}

fn final_values(cursor: &mut Cursor<&[u8]>) -> Option<ExtraFinal> {
    boolean(cursor).then(|| ExtraFinal {
        word: signed(cursor),
        values: integers(cursor),
    })
}

fn extra(cursor: &mut Cursor<&[u8]>) -> Option<ExtraMixedTail> {
    boolean(cursor).then(|| ExtraMixedTail {
        word: signed(cursor),
        mixed: mixed(cursor),
    })
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

fn word(cursor: &mut Cursor<&[u8]>) -> u32 {
    u32::from_le_bytes(number(cursor))
}

fn signed(cursor: &mut Cursor<&[u8]>) -> i32 {
    i32::from_le_bytes(number(cursor))
}

fn optional_byte(cursor: &mut Cursor<&[u8]>) -> Option<u8> {
    boolean(cursor).then(|| byte(cursor))
}

fn integers(cursor: &mut Cursor<&[u8]>) -> Vec<i32> {
    (0..word(cursor)).map(|_| signed(cursor)).collect()
}

fn wide(cursor: &mut Cursor<&[u8]>) -> Vec<u64> {
    (0..word(cursor))
        .map(|_| u64::from_le_bytes(number(cursor)))
        .collect()
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let mut raw = vec![0; word(cursor) as usize];
    cursor.read_exact(&mut raw).unwrap();
    raw
}
