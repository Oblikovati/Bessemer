use super::*;
pub struct KindTwoCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub prefix: BodyPrefix,
    pub scalar: u64,
    pub model: ModelCase,
    pub footer: KindTwoFooter,
}
pub struct ModelCase {
    pub kind: i32,
    pub start: u32,
    pub end: u32,
    pub version: u8,
    pub payload: ModelPayload,
}
pub enum ModelPayload {
    Curves(CurveModel),
    Scalars(Vec<u64>),
    Switched(SwitchedModel),
    Table(TableModel),
}
pub struct CurveModel {
    pub curves: Vec<BodyCurve>,
    pub scalars: Vec<u64>,
    pub states: Vec<u8>,
}
pub struct SwitchedModel {
    pub first: Vec<u64>,
    pub state: u8,
    pub last: u64,
}
pub struct TableModel {
    pub first_count: u32,
    pub second_count: u32,
    pub first: Vec<Vec<u64>>,
    pub second: Vec<u64>,
    pub word_patterns: Vec<u32>,
    pub third_count: u32,
    pub third: Vec<u64>,
    pub member: i32,
    pub scalars: Vec<u64>,
    pub states: Vec<u8>,
}
pub struct KindTwoFooter {
    pub state: u8,
    pub loaded: u8,
    pub end: u32,
    pub tail: Vec<u8>,
    pub complete: bool,
    pub installed: bool,
    pub runtime: bool,
}
