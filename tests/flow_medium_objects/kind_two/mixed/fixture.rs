use super::*;
use std::io::Read;
pub fn cases() -> Vec<MixedCase> {
    let bytes = include_bytes!("../../../fixtures/flow-medium-mixed/profiles.bin.gz");
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(bytes.as_slice())
        .take(21_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    assert!(raw.len() <= 21_000_000);
    let mut cursor = std::io::Cursor::new(raw);
    assert_eq!(number::<4>(&mut cursor), *b"FML1");
    let result = counted(&mut cursor, profile);
    finish(&cursor);
    result
}
fn profile(cursor: &mut PacketCursor) -> MixedCase {
    MixedCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        raw: blob(cursor),
        prefix: blob(cursor),
        backend_start: word(cursor),
        backend_end: word(cursor),
        first: counted(cursor, first),
        second: counted(cursor, second),
        footer: footer(cursor),
    }
}
fn first(cursor: &mut PacketCursor) -> MixedFirst {
    let ordinal = word(cursor);
    let start = word(cursor);
    let end = word(cursor);
    let kind = signed(cursor);
    let key = blob(cursor);
    let body = match kind {
        1 => MixedBody::One(fixture::body(cursor)),
        2 => MixedBody::Two(kind_fixture::body(cursor)),
        other => panic!("Fixture kind {other}; expected 1 or 2"),
    };
    MixedFirst {
        ordinal,
        start,
        end,
        kind,
        key,
        body,
    }
}
fn second(cursor: &mut PacketCursor) -> MixedSecond {
    MixedSecond {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        kind: signed(cursor),
        key: blob(cursor),
        state: optional(cursor, byte),
        loaded: optional(cursor, byte),
        references: counted(cursor, blob),
    }
}
fn footer(cursor: &mut PacketCursor) -> MixedFooter {
    MixedFooter {
        keys: counted(cursor, blob),
        attributes: counted(cursor, signed),
        final_counts: [word(cursor), word(cursor)],
        end: word(cursor),
        flags: std::array::from_fn(|_| boolean(cursor)),
    }
}
