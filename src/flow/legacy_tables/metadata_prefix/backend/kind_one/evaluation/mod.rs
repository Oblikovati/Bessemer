//! Versioned evaluation of the three documented finite/ordered accessors.
mod compute;
mod types;
mod view;
use super::*;
pub use types::*;
impl FlowMediumKindOne {
    /// Evaluates a documented accessor under FiniteOrderedV1; e.g. `let result=body.evaluate_curve(FlowMediumCurveAccessor::CurveTwo,300.)?`.
    pub fn evaluate_curve(
        &self,
        accessor: FlowMediumCurveAccessor,
        query: f64,
    ) -> Result<FlowMediumCurveEvaluation> {
        compute::evaluate(self, accessor, query).map_err(|error| table_boundary(error, 424))
    }
}
