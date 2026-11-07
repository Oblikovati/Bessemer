use super::packet::*;
use super::types::*;

pub fn state_cases() -> Vec<StateCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-metadata/states.bin.gz"),
        *b"FIS1",
    );
    let result = counted(&mut cursor, state_case);
    finish(&cursor);
    result
}

fn state_case(cursor: &mut PacketCursor) -> StateCase {
    StateCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        raw: blob(cursor),
        existing: counted(cursor, map_entry),
        version_word: word(cursor),
        entries: counted(cursor, state_entry),
        end: word(cursor),
        native_dispatch: boolean(cursor),
        expected: counted(cursor, map_entry),
        assignments: counted(cursor, assignment),
        preserved: boolean(cursor),
        native_hash: boolean(cursor),
    }
}

fn map_entry(cursor: &mut PacketCursor) -> (i32, u8) {
    (signed(cursor), byte(cursor))
}

fn state_entry(cursor: &mut PacketCursor) -> StateEntry {
    StateEntry {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        identifier: signed(cursor),
        state: byte(cursor),
    }
}

fn assignment(cursor: &mut PacketCursor) -> StateAssignment {
    StateAssignment {
        ordinal: word(cursor),
        identifier: signed(cursor),
        previous: optional(cursor, byte),
        state: byte(cursor),
    }
}

pub fn prefix_cases() -> Vec<PrefixCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-metadata/prefix.bin.gz"),
        *b"FMP1",
    );
    let result = counted(&mut cursor, prefix_case);
    finish(&cursor);
    result
}

fn prefix_case(cursor: &mut PacketCursor) -> PrefixCase {
    PrefixCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        raw: blob(cursor),
        string_mode: signed(cursor),
        version: byte(cursor),
        signed_version: byte(cursor) as i8,
        active: boolean(cursor),
        vectors: counted(cursor, vector),
        objects: counted(cursor, object),
        end: word(cursor),
        known: word(cursor),
        unknown: word(cursor),
        tail: blob(cursor),
        requires_backend: boolean(cursor),
        full_schema: boolean(cursor),
        runtime: boolean(cursor),
    }
}

fn vector(cursor: &mut PacketCursor) -> PrefixVector {
    PrefixVector {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        values: counted(cursor, signed),
    }
}

fn object(cursor: &mut PacketCursor) -> PrefixObject {
    PrefixObject {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        first: text(cursor),
        second: text(cursor),
        state: byte(cursor),
        native_state: boolean(cursor),
        word_c4: signed(cursor),
        third: text(cursor),
        fourth: optional(cursor, text),
        native_fourth: blob(cursor),
        derived_fourth: boolean(cursor),
    }
}

fn text(cursor: &mut PacketCursor) -> PrefixString {
    PrefixString {
        start: word(cursor),
        end: word(cursor),
        units: word(cursor),
        text_start: word(cursor),
        raw: blob(cursor),
        native_text: blob(cursor),
    }
}
