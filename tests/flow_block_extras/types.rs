pub struct ExtraCase {
    pub name: String,
    pub compressed: bool,
    pub leaves: u32,
    pub enabled: bool,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
    pub version: i32,
    pub blocks: Vec<ExtraBlock>,
    pub tree_order: bool,
    pub references: bool,
    pub runtime: bool,
}

pub struct ExtraBlock {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub token: i32,
    pub flag: Option<u8>,
    pub body: Option<ExtraBody>,
}

pub struct ExtraBody {
    pub start: u32,
    pub end: u32,
    pub first: Vec<ExtraFirst>,
    pub second: Vec<ExtraSecond>,
    pub third: Vec<ExtraThird>,
    pub fourth_wide: Vec<u64>,
    pub fourth_bytes: Vec<u8>,
    pub outer: u8,
    pub nested_flag: Option<u8>,
    pub nested: Option<Vec<Vec<ExtraNested>>>,
    pub final_flag: Option<u8>,
    pub final_values: Option<ExtraFinal>,
    pub extra_flag: Option<u8>,
    pub extra: Option<ExtraMixedTail>,
    pub last: Option<u8>,
}

pub struct ExtraFirst {
    pub start: u32,
    pub end: u32,
    pub pair: [u8; 2],
    pub words: [i32; 5],
    pub mixed: ExtraMixed,
    pub references: [u64; 2],
    pub bytes: Vec<u8>,
    pub additional_flag: Option<u8>,
    pub additional: Vec<i32>,
}

pub struct ExtraMixed {
    pub start: u32,
    pub end: u32,
    pub interleaved: bool,
    pub words: Vec<i32>,
    pub wide: Vec<u64>,
}

pub struct ExtraSecond {
    pub start: u32,
    pub end: u32,
    pub pair: [u8; 2],
    pub tag: u8,
    pub bytes: Vec<u8>,
}

pub struct ExtraThird {
    pub start: u32,
    pub end: u32,
    pub word: i32,
    pub wide: [u64; 4],
}

pub struct ExtraNested {
    pub start: u32,
    pub end: u32,
    pub stored_index: Option<i32>,
    pub native_index: i32,
    pub words: Vec<i32>,
    pub wide: Vec<u64>,
}

pub struct ExtraFinal {
    pub word: i32,
    pub values: Vec<i32>,
}

pub struct ExtraMixedTail {
    pub word: i32,
    pub mixed: ExtraMixed,
}
