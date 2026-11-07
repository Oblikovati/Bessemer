pub struct PointerCase {
    pub frame: PointerFrame,
    pub version: i32,
    pub pools: Vec<PointerPool>,
    pub groups: [PointerGroup; 3],
    pub exclusions: [bool; 4],
}

pub struct PointerFrame {
    pub name: String,
    pub compressed: bool,
    pub leaf: u32,
    pub counts: [u32; 3],
    pub codes: Option<Vec<u8>>,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
}

pub struct PointerPool {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub count: u32,
    pub bits: Vec<u64>,
}

pub struct PointerGroup {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub count: u32,
    pub entries: Vec<PointerEntry>,
}

pub struct PointerEntry {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub code: Option<u8>,
    pub body: PointerEntryBody,
}

pub enum PointerEntryBody {
    Single(Vec<u8>),
    Wide { count: u32, bits: Vec<u64> },
    Multiple(Vec<Vec<u8>>),
    Tagged(TaggedPointerBody),
}

pub struct TaggedPointerBody {
    pub tag: u8,
    pub count: Option<u32>,
    pub bits: Option<Vec<u64>>,
    pub raw: Option<Vec<u8>>,
    pub packed_decoded: Option<bool>,
}
