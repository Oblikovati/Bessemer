//! Explicit C855 counted assignment and C890 local text selection, separate from layout (#166).
mod assignment;
mod selection;
mod types;
mod views;
use super::SimulationPostStyleText;
use crate::{BessemerError, FailureClass, Result};
pub use assignment::SimulationPostTextAssignment;
pub use selection::SimulationPostTextPlan;
pub use types::*;

fn required_post_runtime_text<'a>(
    text: Option<&'a SimulationPostStyleText>,
    operation: &'static str,
    entity: &str,
) -> Result<&'a SimulationPostStyleText> {
    text.ok_or_else(|| BessemerError::new(operation, entity, FailureClass::Missing,
        "runtime CString None; expected Some(complete raw UTF16 code units), including Some(empty) for known empty"))
}

fn post_text_budget(operation: &'static str, units: usize, maximum: usize) -> Result<()> {
    if units <= maximum {
        return Ok(());
    }
    Err(BessemerError::new(
        operation,
        "code-unit budget",
        FailureClass::Invalid,
        format!("{units} code units; expected at most maximum_code_units {maximum}"),
    ))
}

fn post_text_coverage(
    wire: usize,
    result: usize,
    required: usize,
    maximum: usize,
) -> SimulationPostTextCoverage {
    SimulationPostTextCoverage {
        retained_wire_code_units: wire,
        result_code_units: result,
        required_runtime_strings: required,
        maximum_code_units: maximum,
        local_units_only: true,
        achieved_pattern_error_units: 0,
    }
}

fn post_text_completion(units: usize) -> SimulationPostTextCompletion {
    if units == 0 {
        return SimulationPostTextCompletion::CompletedEmpty;
    }
    SimulationPostTextCompletion::CompletedText
}

fn post_text_diagnostics() -> [SimulationPostTextDiagnostic; 4] {
    [
        SimulationPostTextDiagnostic::NativeNotExecuted,
        SimulationPostTextDiagnostic::UnicodeNotValidated,
        SimulationPostTextDiagnostic::LayoutNotModeled,
        SimulationPostTextDiagnostic::RuntimeProductionNotInferred,
    ]
}
