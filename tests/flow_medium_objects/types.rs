pub struct BodyCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub version: u8,
    pub base: u8,
    pub base_dispatch: bool,
    pub key: Vec<u8>,
    pub text: BodyText,
    pub curves: Vec<BodyCurve>,
    pub first_count: u32,
    pub first: Vec<Vec<u64>>,
    pub second_count: u32,
    pub second: Vec<Vec<u64>>,
    pub footer: BodyFooter,
}
pub struct BodyText {
    pub start: u32,
    pub end: u32,
    pub units: u32,
    pub text_start: u32,
    pub raw: Vec<u8>,
    pub native: Vec<u8>,
}
pub struct BodyCurve {
    pub start: u32,
    pub end: u32,
    pub version: u8,
    pub scalar: u64,
    pub count: u32,
    pub first: Vec<u64>,
    pub second: Vec<u64>,
    pub flag: u8,
    pub extra_count: Option<u32>,
    pub extra_first: Vec<u64>,
    pub extra_second: Vec<u64>,
}

pub struct BodyFooter {
    pub scalars: Vec<u64>,
    pub state: u8,
    pub loaded_state: u8,
    pub end: u32,
    pub tail: Vec<u8>,
    pub complete: bool,
    pub runtime: bool,
    pub engineering: bool,
}
