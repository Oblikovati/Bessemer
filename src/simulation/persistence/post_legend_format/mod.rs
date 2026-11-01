//! C846 exact local format construction and alternate selection, without display output (#166).
mod plan;
mod selection;
mod types;
mod views;
use crate::{BessemerError, FailureClass, Result};
pub use plan::SimulationLegendFormatPlan;
pub use types::*;

fn required_format_word(word: Option<i32>, member: &str) -> Result<i32> {
    word.ok_or_else(|| {
        BessemerError::new(
            "legend number format",
            member,
            FailureClass::Missing,
            "None; expected Some(explicit signed32 word) at the reached legend member",
        )
    })
}

fn constructed_format(mode: i32, precision: i32) -> String {
    let conversion = match mode {
        1 => 'f',
        2 => 'g',
        _ => 'e',
    };
    let width = precision.wrapping_add(7);
    format!("%-{width}.{precision}{conversion}")
}
