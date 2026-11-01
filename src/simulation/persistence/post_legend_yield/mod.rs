//! C850 exact local call requests with explicit post-suffix state (#166).
mod plan;
mod selection;
mod types;
mod views;
use crate::{BessemerError, FailureClass, Result};
pub use plan::SimulationLegendYieldPlan;
pub use types::*;

fn required_yield_fact<T: Copy + std::fmt::Debug>(value: Option<T>, entity: &str) -> Result<T> {
    value.ok_or_else(|| {
        BessemerError::new(
            "legend yield call plan",
            entity,
            FailureClass::Missing,
            format!("{value:?}; expected Some(explicit fact) at the reached native dependency"),
        )
    })
}
