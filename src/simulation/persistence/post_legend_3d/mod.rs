//! C852 local allocation/request/background facts without renderer execution (#166).
mod background;
mod body;
mod plans;
mod types;
use crate::{BessemerError, FailureClass, Result};
pub use plans::{SimulationLegend3DBackgroundPlan, SimulationLegend3DBodyPlan};
pub use types::*;

fn required_3d_fact<T: Copy + std::fmt::Debug>(value: Option<T>, entity: &str) -> Result<T> {
    value.ok_or_else(|| {
        BessemerError::new(
            "legend 3D local projection",
            entity,
            FailureClass::Missing,
            format!(
                "{value:?}; expected Some(explicit {entity} value) at the reached native dependency"
            ),
        )
    })
}
