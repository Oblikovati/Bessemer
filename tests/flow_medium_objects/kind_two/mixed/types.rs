use super::*;
pub struct MixedCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub prefix: Vec<u8>,
    pub backend_start: u32,
    pub backend_end: u32,
    pub first: Vec<MixedFirst>,
    pub second: Vec<MixedSecond>,
    pub footer: MixedFooter,
}
pub struct MixedFirst {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub kind: i32,
    pub key: Vec<u8>,
    pub body: MixedBody,
}
pub enum MixedBody {
    One(BodyCase),
    Two(KindTwoCase),
}
pub struct MixedSecond {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub kind: i32,
    pub key: Vec<u8>,
    pub state: Option<u8>,
    pub loaded: Option<u8>,
    pub references: Vec<Vec<u8>>,
}
pub struct MixedFooter {
    pub keys: Vec<Vec<u8>>,
    pub attributes: Vec<i32>,
    pub final_counts: [u32; 2],
    pub end: u32,
    pub flags: [bool; 5],
}
