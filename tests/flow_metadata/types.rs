pub struct StateCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub existing: Vec<(i32, u8)>,
    pub version_word: u32,
    pub entries: Vec<StateEntry>,
    pub end: u32,
    pub native_dispatch: bool,
    pub expected: Vec<(i32, u8)>,
    pub assignments: Vec<StateAssignment>,
    pub preserved: bool,
    pub native_hash: bool,
}

pub struct StateEntry {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub identifier: i32,
    pub state: u8,
}

pub struct StateAssignment {
    pub ordinal: u32,
    pub identifier: i32,
    pub previous: Option<u8>,
    pub state: u8,
}

pub struct PrefixCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub string_mode: i32,
    pub version: u8,
    pub signed_version: i8,
    pub active: bool,
    pub vectors: Vec<PrefixVector>,
    pub objects: Vec<PrefixObject>,
    pub end: u32,
    pub known: u32,
    pub unknown: u32,
    pub tail: Vec<u8>,
    pub requires_backend: bool,
    pub full_schema: bool,
    pub runtime: bool,
}

pub struct PrefixVector {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub values: Vec<i32>,
}

pub struct PrefixObject {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub first: PrefixString,
    pub second: PrefixString,
    pub state: u8,
    pub native_state: bool,
    pub word_c4: i32,
    pub third: PrefixString,
    pub fourth: Option<PrefixString>,
    pub native_fourth: Vec<u8>,
    pub derived_fourth: bool,
}

pub struct PrefixString {
    pub start: u32,
    pub end: u32,
    pub units: u32,
    pub text_start: u32,
    pub raw: Vec<u8>,
    pub native_text: Vec<u8>,
}
