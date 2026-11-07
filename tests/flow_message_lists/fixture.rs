use super::*;
pub struct MessageCase {
    pub name: String,
    pub version: i32,
    pub mode: Option<i32>,
    pub native: bool,
    pub raw: Vec<u8>,
    pub items: u32,
    pub fields: Vec<stored_fields::StoredField>,
    pub scopes: Vec<(String, u32, u32)>,
    pub counts: Vec<u32>,
    pub texts: Vec<MessageText>,
    pub complete: bool,
    pub interpretation: bool,
    pub engineering: bool,
}
pub struct MessageText {
    pub path: String,
    pub start: u32,
    pub end: u32,
    pub count: u32,
    pub legacy: bool,
    pub prefix: Option<Vec<u8>>,
}
pub fn cases() -> Vec<MessageCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-message-lists/messages.bin.gz"),
        *b"FM31",
    );
    let cases = counted(&mut cursor, case);
    finish(&cursor);
    cases
}
fn case(cursor: &mut PacketCursor) -> MessageCase {
    MessageCase {
        name: text(cursor),
        version: signed(cursor),
        mode: optional(cursor, signed),
        native: boolean(cursor),
        raw: blob(cursor),
        items: word(cursor),
        fields: counted(cursor, field),
        scopes: counted(cursor, scope),
        counts: counted(cursor, word),
        texts: counted(cursor, stored_text),
        complete: boolean(cursor),
        interpretation: boolean(cursor),
        engineering: boolean(cursor),
    }
}
fn text(cursor: &mut PacketCursor) -> String {
    String::from_utf8(blob(cursor)).unwrap()
}
fn field(cursor: &mut PacketCursor) -> stored_fields::StoredField {
    stored_fields::StoredField {
        path: text(cursor),
        offset: word(cursor),
        encoding: byte(cursor),
        count: word(cursor),
        raw: blob(cursor),
    }
}
fn scope(cursor: &mut PacketCursor) -> (String, u32, u32) {
    (text(cursor), word(cursor), word(cursor))
}
fn stored_text(cursor: &mut PacketCursor) -> MessageText {
    MessageText {
        path: text(cursor),
        start: word(cursor),
        end: word(cursor),
        count: word(cursor),
        legacy: boolean(cursor),
        prefix: optional(cursor, blob),
    }
}
