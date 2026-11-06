use super::*;
pub struct EvaluationCase {
    pub body: BodyCase,
    pub ordinal: i32,
    pub query: u64,
    pub expected: Option<ExpectedEvaluation>,
}
pub struct ExpectedEvaluation {
    pub bits: u64,
    pub classification: u8,
    pub branch: u8,
    pub bracket: Option<(u32, u32)>,
    pub reference: u64,
    pub measured_error_bound: Option<u64>,
}
