//! C857 explicit reader zero events, separate from wire absence and other object state (#166).
mod plan;
mod selection;
mod types;
mod views;
pub use plan::SimulationLegendResetPlan;
pub use types::*;
