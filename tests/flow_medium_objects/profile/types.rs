use super::*;
pub struct ProfileCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub prefix: Vec<u8>,
    pub backend_start: u32,
    pub backend_end: u32,
    pub first: Vec<ProfileFirst>,
    pub second: Vec<ProfileSecond>,
    pub footer: ProfileFooter,
}
pub struct ProfileFirst {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub kind: i32,
    pub key: Vec<u8>,
    pub body: BodyCase,
}
pub struct ProfileSecond {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub kind: i32,
    pub key: Vec<u8>,
    pub state: u8,
    pub loaded: u8,
    pub references: Vec<Vec<u8>>,
}
pub struct ProfileFooter {
    pub keys: Vec<Vec<u8>>,
    pub attributes: Vec<i32>,
    pub final_counts: [u32; 2],
    pub end: u32,
    pub complete: bool,
    pub engineering: bool,
    pub fresh: bool,
    pub runtime: bool,
}
