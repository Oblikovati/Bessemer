use super::packet::*;
pub(super) struct ValueArray {
    pub start: u32,
    pub end: u32,
    pub marker: i32,
    pub count: u32,
    pub raw: Vec<u8>,
}
pub(super) struct BlRows {
    pub start: u32,
    pub end: u32,
    pub version: i32,
    pub count: u32,
    pub native_state: u8,
    pub raw: Vec<u8>,
}
pub(super) struct ImsValueObject {
    pub identifier: i32,
    pub start: u32,
    pub end: u32,
    pub version: i32,
    pub first_flag: u8,
    pub first: Option<ValueArray>,
    pub main: ValueArray,
    pub bl_flag: u8,
    pub bl: Option<BlRows>,
    pub last_flag: u8,
    pub last: Option<ValueArray>,
}
pub(super) struct ValueContextFixture {
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub identifiers: Vec<i32>,
}
pub(super) struct ValueFrame {
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub version: u8,
    pub objects: Vec<ImsValueObject>,
    pub end: u32,
    pub qualifications: [bool; 4],
}
pub(super) struct ValueCase {
    pub name: String,
    pub native: bool,
    pub context: ValueContextFixture,
    pub frame: ValueFrame,
}
fn array(cursor: &mut PacketCursor) -> ValueArray {
    ValueArray {
        start: word(cursor),
        end: word(cursor),
        marker: signed(cursor),
        count: word(cursor),
        raw: blob(cursor),
    }
}
fn bl(cursor: &mut PacketCursor) -> BlRows {
    BlRows {
        start: word(cursor),
        end: word(cursor),
        version: signed(cursor),
        count: word(cursor),
        native_state: byte(cursor),
        raw: blob(cursor),
    }
}
fn object(cursor: &mut PacketCursor) -> ImsValueObject {
    ImsValueObject {
        identifier: signed(cursor),
        start: word(cursor),
        end: word(cursor),
        version: signed(cursor),
        first_flag: byte(cursor),
        first: optional(cursor, array),
        main: array(cursor),
        bl_flag: byte(cursor),
        bl: optional(cursor, bl),
        last_flag: byte(cursor),
        last: optional(cursor, array),
    }
}
fn context(cursor: &mut PacketCursor) -> ValueContextFixture {
    let stored = blob(cursor);
    let decoded = blob(cursor);
    let raw = blob(cursor);
    let (words, remainder) = raw.as_chunks::<4>();
    assert!(remainder.is_empty());
    let identifiers = words.iter().map(|word| i32::from_le_bytes(*word)).collect();
    ValueContextFixture {
        stored,
        decoded,
        identifiers,
    }
}
fn frame(cursor: &mut PacketCursor) -> ValueFrame {
    ValueFrame {
        stored: blob(cursor),
        decoded: blob(cursor),
        version: byte(cursor),
        objects: counted(cursor, object),
        end: word(cursor),
        qualifications: std::array::from_fn(|_| boolean(cursor)),
    }
}
fn case(cursor: &mut PacketCursor) -> ValueCase {
    ValueCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native: boolean(cursor),
        context: context(cursor),
        frame: frame(cursor),
    }
}
pub(super) fn cases() -> Vec<ValueCase> {
    let raw = include_bytes!("../fixtures/flow-result-values/record501.bin.gz");
    let mut cursor = cursor(raw, *b"FIV1");
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
