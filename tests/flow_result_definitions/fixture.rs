use super::packet::*;

pub(super) struct DefinitionArray {
    pub start: u32,
    pub end: u32,
    pub marker: i32,
    pub count: u32,
    pub raw: Vec<u8>,
}
pub(super) struct ImsBody {
    pub start: u32,
    pub end: u32,
    pub version: i32,
    pub allocation: Option<u8>,
    pub arrays: [Option<DefinitionArray>; 4],
}
pub(super) struct DefinitionObject {
    pub identifier: i32,
    pub guid: Vec<u8>,
    pub body: ImsBody,
}
pub(super) struct DefinitionGroup {
    pub start: u32,
    pub end: u32,
    pub objects: Vec<DefinitionObject>,
}
pub(super) struct DefinitionCase {
    pub name: String,
    pub native: bool,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub version: u8,
    pub groups: Vec<DefinitionGroup>,
    pub end: u32,
    pub complete: bool,
    pub runtime: bool,
}
fn array(cursor: &mut PacketCursor) -> DefinitionArray {
    DefinitionArray {
        start: word(cursor),
        end: word(cursor),
        marker: signed(cursor),
        count: word(cursor),
        raw: blob(cursor),
    }
}
fn body(cursor: &mut PacketCursor) -> ImsBody {
    ImsBody {
        start: word(cursor),
        end: word(cursor),
        version: signed(cursor),
        allocation: optional(cursor, byte),
        arrays: std::array::from_fn(|_| optional(cursor, array)),
    }
}
fn object(cursor: &mut PacketCursor) -> DefinitionObject {
    DefinitionObject {
        identifier: signed(cursor),
        guid: blob(cursor),
        body: body(cursor),
    }
}
fn group(cursor: &mut PacketCursor) -> DefinitionGroup {
    DefinitionGroup {
        start: word(cursor),
        end: word(cursor),
        objects: counted(cursor, object),
    }
}
fn case(cursor: &mut PacketCursor) -> DefinitionCase {
    DefinitionCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native: boolean(cursor),
        stored: blob(cursor),
        decoded: blob(cursor),
        version: byte(cursor),
        groups: counted(cursor, group),
        end: word(cursor),
        complete: boolean(cursor),
        runtime: boolean(cursor),
    }
}
pub(super) fn cases() -> Vec<DefinitionCase> {
    let raw = include_bytes!("../fixtures/flow-result-definitions/record500.bin.gz");
    let mut cursor = cursor(raw, *b"FDF1");
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
