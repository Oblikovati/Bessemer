//! C848 raw palette controls and bounded dispatch, independent of unfinished mapping algorithms (#166).
mod plan;
mod selection;
mod types;
mod views;
use crate::{BessemerError, FailureClass, Result};
pub use plan::SimulationLegendPalettePlan;
pub use types::*;

fn required_palette_word(word: Option<i32>, entity: &str) -> Result<i32> {
    word.ok_or_else(|| {
        BessemerError::new(
            "legend palette dispatch",
            entity,
            FailureClass::Missing,
            "None; expected Some(explicit signed32 value) at the reached dependency",
        )
    })
}
