use super::{packet::*, types::*};
pub fn cases() -> Vec<BodyCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-medium-objects/kind1.bin.gz"),
        *b"FMO1",
    );
    let cases = counted(&mut cursor, body);
    finish(&cursor);
    cases
}
pub(super) fn body(cursor: &mut PacketCursor) -> BodyCase {
    BodyCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        raw: blob(cursor),
        prefix: prefix(cursor),
        footer: footer(cursor),
    }
}
fn footer(cursor: &mut PacketCursor) -> BodyFooter {
    BodyFooter {
        scalars: words(cursor),
        state: byte(cursor),
        loaded_state: byte(cursor),
        end: word(cursor),
        tail: blob(cursor),
        complete: boolean(cursor),
        runtime: boolean(cursor),
        engineering: boolean(cursor),
    }
}
fn text(cursor: &mut PacketCursor) -> BodyText {
    BodyText {
        start: word(cursor),
        end: word(cursor),
        units: count_word(cursor),
        text_start: word(cursor),
        raw: blob(cursor),
        native: blob(cursor),
    }
}
pub(super) fn curve(cursor: &mut PacketCursor) -> BodyCurve {
    BodyCurve {
        start: word(cursor),
        end: word(cursor),
        version: byte(cursor),
        scalar: wide(cursor),
        count: count_word(cursor),
        first: words(cursor),
        second: words(cursor),
        flag: byte(cursor),
        extra_count: optional(cursor, count_word),
        extra_first: words(cursor),
        extra_second: words(cursor),
    }
}
pub(super) fn wide(cursor: &mut PacketCursor) -> u64 {
    u64::from_le_bytes(number(cursor))
}
pub(super) fn words(cursor: &mut PacketCursor) -> Vec<u64> {
    counted(cursor, wide)
}

pub(super) fn count_word(cursor: &mut PacketCursor) -> u32 {
    u32::try_from(signed(cursor)).unwrap()
}

pub(super) fn prefix(cursor: &mut PacketCursor) -> BodyPrefix {
    BodyPrefix {
        version: byte(cursor),
        base: byte(cursor),
        base_dispatch: boolean(cursor),
        key: blob(cursor),
        text: text(cursor),
        curves: counted(cursor, curve),
        first_count: count_word(cursor),
        first: counted(cursor, words),
        second_count: count_word(cursor),
        second: counted(cursor, words),
    }
}
