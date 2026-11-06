pub struct ReferenceCase {
    pub frame: ReferenceFrame,
    pub version: i32,
    pub discarded_header: Option<i32>,
    pub stored_header: Option<i32>,
    pub native_header_default: i32,
    pub groups: [ReferenceGroup; 2],
    pub exclusions: [bool; 4],
}

pub struct ReferenceFrame {
    pub name: String,
    pub compressed: bool,
    pub mapping: bool,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
}

pub struct ReferenceGroup {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub entries: Vec<ReferenceEntry>,
    pub level: Option<i32>,
    pub mapping: Vec<i32>,
    pub wide: Vec<u64>,
}

pub struct ReferenceEntry {
    pub start: u32,
    pub end: u32,
    pub member20: Option<i32>,
    pub references: Vec<i32>,
    pub prefix: Option<u8>,
    pub state: u8,
    pub flag2: Option<u8>,
    pub flag4: Option<u8>,
    pub descriptors: [u16; 7],
    pub first: Vec<u32>,
    pub second_count: i32,
    pub raw_small_count: Option<u16>,
    pub second: Vec<u32>,
    pub suffix: Option<u16>,
}
