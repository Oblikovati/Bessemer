pub struct PrimaryCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub version: i32,
    pub items: Vec<PrimaryItem>,
    pub end: u32,
    pub complete: bool,
    pub engineering: bool,
}
pub struct PrimaryItem {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub kind: u8,
    pub element_kind: u8,
    pub groups: Vec<PrimaryGroup>,
}
pub struct PrimaryGroup {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub mode: Option<u8>,
    pub members: [i32; 2],
    pub presence: u8,
    pub body: Option<PrimaryBody>,
    pub elements: Option<Vec<PrimaryElement>>,
}
pub struct PrimaryBody {
    pub start: u32,
    pub end: u32,
    pub first: Vec<u32>,
    pub second: Vec<u32>,
    pub triplet: [i32; 3],
    pub wide6: Vec<u64>,
    pub wide3: Vec<u64>,
    pub legacy: Option<Vec<u64>>,
    pub flag: Option<u8>,
    pub native_flag: bool,
}
pub struct PrimaryElement {
    pub start: u32,
    pub end: u32,
    pub kind: u8,
    pub rows: Vec<PrimaryRow>,
    pub blocks: Vec<PrimaryBlock>,
}
pub struct PrimaryRow {
    pub start: u32,
    pub end: u32,
    pub values: RowValues,
}
pub enum RowValues {
    Words(Vec<u32>),
    Pairs(Vec<(u32, u32)>),
    Wide(Vec<(u32, u64)>),
    Mixed(Vec<(u32, u64, u32)>),
}
pub struct PrimaryBlock {
    pub start: u32,
    pub end: u32,
    pub values: Vec<u64>,
}
