use super::packet::*;
pub(super) const SAVED: u32 = 0xffffff9c;
pub(super) const INDEX: u32 = 0xffffff9b;
pub(super) struct ControlFooter {
    pub start: u32,
    pub distance: u64,
    pub identifier: u32,
}
pub(super) struct ControlCase {
    pub name: String,
    pub reference: bool,
    pub identifier: u32,
    pub width: u8,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub marker: u64,
    pub count: u64,
    pub signed_count: Option<i32>,
    pub entries: Vec<u8>,
    pub compressed: u32,
    pub end: u32,
    pub footer: Option<ControlFooter>,
    pub complete: bool,
}
fn wide(cursor: &mut PacketCursor) -> u64 {
    u64::from_le_bytes(number(cursor))
}
fn footer(cursor: &mut PacketCursor) -> ControlFooter {
    ControlFooter {
        start: word(cursor),
        distance: wide(cursor),
        identifier: word(cursor),
    }
}
fn case(cursor: &mut PacketCursor) -> ControlCase {
    ControlCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        reference: boolean(cursor),
        identifier: word(cursor),
        width: byte(cursor),
        stored: blob(cursor),
        decoded: blob(cursor),
        marker: wide(cursor),
        count: wide(cursor),
        signed_count: optional(cursor, signed),
        entries: blob(cursor),
        compressed: word(cursor),
        end: word(cursor),
        footer: optional(cursor, footer),
        complete: boolean(cursor),
    }
}
fn packet(bytes: &[u8], magic: [u8; 4]) -> Vec<ControlCase> {
    let mut cursor = cursor(bytes, magic);
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
pub(super) fn cases() -> Vec<ControlCase> {
    let mut result = packet(
        include_bytes!("../fixtures/flow-control-records/saved-spans.bin.gz"),
        *b"FCS1",
    );
    result.extend(packet(
        include_bytes!("../fixtures/flow-control-records/terminal-index.bin.gz"),
        *b"FCI1",
    ));
    result
}
