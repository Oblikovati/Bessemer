use super::*;
pub fn cases() -> Vec<PrimaryCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-primary-table/primary.bin.gz"),
        *b"F439",
    );
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
fn case(cursor: &mut PacketCursor) -> PrimaryCase {
    PrimaryCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        raw: blob(cursor),
        version: signed(cursor),
        items: counted(cursor, item),
        end: word(cursor),
        complete: boolean(cursor),
        engineering: boolean(cursor),
    }
}
fn item(cursor: &mut PacketCursor) -> PrimaryItem {
    PrimaryItem {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        kind: byte(cursor),
        element_kind: byte(cursor),
        groups: counted(cursor, group),
    }
}
fn group(cursor: &mut PacketCursor) -> PrimaryGroup {
    PrimaryGroup {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        mode: optional(cursor, byte),
        members: [signed(cursor), signed(cursor)],
        presence: byte(cursor),
        body: optional(cursor, body),
        elements: optional(cursor, elements),
    }
}
fn elements(cursor: &mut PacketCursor) -> Vec<PrimaryElement> {
    counted(cursor, element)
}
fn body(cursor: &mut PacketCursor) -> PrimaryBody {
    PrimaryBody {
        start: word(cursor),
        end: word(cursor),
        first: counted(cursor, word),
        second: counted(cursor, word),
        triplet: [signed(cursor), signed(cursor), signed(cursor)],
        wide6: counted(cursor, wide),
        wide3: counted(cursor, wide),
        legacy: optional(cursor, wides),
        flag: optional(cursor, byte),
        native_flag: boolean(cursor),
    }
}
fn wides(cursor: &mut PacketCursor) -> Vec<u64> {
    counted(cursor, wide)
}
fn element(cursor: &mut PacketCursor) -> PrimaryElement {
    let start = word(cursor);
    let end = word(cursor);
    let kind = byte(cursor);
    let rows = (0..word(cursor)).map(|_| row(cursor, kind)).collect();
    PrimaryElement {
        start,
        end,
        kind,
        rows,
        blocks: counted(cursor, block),
    }
}
fn row(cursor: &mut PacketCursor, kind: u8) -> PrimaryRow {
    let start = word(cursor);
    let end = word(cursor);
    let values = match kind {
        1 => RowValues::Words(counted(cursor, word)),
        2 => RowValues::Pairs(counted(cursor, pair)),
        3 => RowValues::Wide(counted(cursor, mixed_wide)),
        4 => RowValues::Mixed(counted(cursor, mixed_word)),
        other => panic!("Fixture element kind {other}; expected1..=4"),
    };
    PrimaryRow { start, end, values }
}
fn pair(cursor: &mut PacketCursor) -> (u32, u32) {
    (word(cursor), word(cursor))
}
fn mixed_wide(cursor: &mut PacketCursor) -> (u32, u64) {
    (word(cursor), wide(cursor))
}
fn mixed_word(cursor: &mut PacketCursor) -> (u32, u64, u32) {
    (word(cursor), wide(cursor), word(cursor))
}
fn block(cursor: &mut PacketCursor) -> PrimaryBlock {
    PrimaryBlock {
        start: word(cursor),
        end: word(cursor),
        values: counted(cursor, wide),
    }
}
