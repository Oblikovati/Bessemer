//! C854 explicit nearest-even width projection, separate from live font/rendering state (#166).
mod arithmetic;
mod plan;
mod selection;
mod types;
mod views;
pub use plan::SimulationLegendWidthPlan;
pub use types::*;
