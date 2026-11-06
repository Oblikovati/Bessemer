pub struct FieldCase {
    pub frame: FieldFrame,
    pub version: i32,
    pub projection: FieldProjection,
    pub body_end: u32,
    pub byte_counts: [u32; 4],
    pub flags: [bool; 4],
}

pub struct FieldFrame {
    pub name: String,
    pub compressed: bool,
    pub restriction: bool,
    pub context: FieldContext,
    pub stored: Vec<u8>,
    pub decoded: Vec<u8>,
}

pub enum FieldContext {
    Legacy {
        table_count: u32,
        object_widths: Vec<u32>,
        expected: Option<Vec<Option<u32>>>,
    },
    Regions {
        rows: Vec<u32>,
        widths: Vec<u32>,
        active: Vec<bool>,
    },
}

pub enum FieldProjection {
    Legacy {
        table: Vec<i32>,
        checked: bool,
        objects: Vec<FieldObject>,
    },
    Regions {
        count_checked: bool,
        widths_checked: bool,
        regions: Vec<FieldRegion>,
    },
}

pub struct FieldObject {
    pub ordinal: u32,
    pub width: u32,
    pub start: u32,
    pub end: u32,
    pub bits: Vec<u64>,
}

pub struct FieldRegion {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub rows: u32,
    pub width: u32,
    pub active: bool,
    pub bits: Vec<u64>,
}

pub struct WidthCase {
    pub code: i32,
    pub cached: Option<i64>,
    pub index: Option<i32>,
    pub numeric: Option<Vec<WidthNumber>>,
    pub expected: Option<u32>,
}

pub enum WidthNumber {
    Signed(i64),
    FloatBits(u64),
}
