use super::*;
pub fn cases() -> Vec<LoadCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-load-state/state.bin.gz"),
        *b"FL11",
    );
    let cases = counted(&mut cursor, case);
    finish(&cursor);
    cases
}
fn case(cursor: &mut PacketCursor) -> LoadCase {
    LoadCase {
        name: text(cursor),
        load_mode: boolean(cursor),
        origin: byte(cursor),
        raw: blob(cursor),
        version: signed(cursor),
        count: word(cursor),
        member: signed(cursor),
        width: word(cursor),
        end: word(cursor),
        fields: counted(cursor, field),
        scopes: counted(cursor, scope),
        counts: counted(cursor, word),
        texts: counted(cursor, stored_text),
        mode_tail: optional(cursor, word),
        complete: boolean(cursor),
        runtime: boolean(cursor),
        engineering: boolean(cursor),
    }
}
fn text(cursor: &mut PacketCursor) -> String {
    String::from_utf8(blob(cursor)).unwrap()
}
fn field(cursor: &mut PacketCursor) -> LoadField {
    LoadField {
        path: text(cursor),
        offset: word(cursor),
        encoding: byte(cursor),
        count: word(cursor),
        raw: blob(cursor),
    }
}
fn scope(cursor: &mut PacketCursor) -> LoadScope {
    LoadScope {
        path: text(cursor),
        start: word(cursor),
        end: word(cursor),
    }
}
fn stored_text(cursor: &mut PacketCursor) -> LoadText {
    LoadText {
        path: text(cursor),
        start: word(cursor),
        end: word(cursor),
        units: word(cursor),
        prefix_units: word(cursor),
        prefix_raw: blob(cursor),
    }
}
