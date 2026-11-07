use super::*;
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/read.rs"]
mod read;
fn open(case: &EvaluationCase) -> FlowMediumKindOne {
    crate::api::open(&case.body.raw, crate::api::options()).unwrap()
}
fn class_code(class: FlowMediumArithmeticClass) -> u8 {
    match class {
        FlowMediumArithmeticClass::Finite => 0,
        FlowMediumArithmeticClass::PositiveInfinity => 1,
        FlowMediumArithmeticClass::NegativeInfinity => 2,
        FlowMediumArithmeticClass::NotANumber => 3,
    }
}
fn outcome(case: &EvaluationCase) -> bessemer::Result<FlowMediumCurveEvaluation> {
    let accessor = FlowMediumCurveAccessor::try_from(case.ordinal)?;
    open(case).evaluate_curve(accessor, f64::from_bits(case.query))
}

#[path = "api/mixed.rs"]
mod mixed;

fn zero_case(cases: &[EvaluationCase]) -> &EvaluationCase {
    cases
        .iter()
        .find(|case| case.body.name.starts_with("constant-c2-p0000000000000000"))
        .unwrap()
}
