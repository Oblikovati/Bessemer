//! C863 native dependency order with explicit masked-exception/DAZ-off comparison policy (#166).
mod checker;
mod entities;
mod plan;
mod state;
mod views;
pub use entities::*;
pub use plan::*;
pub use state::SimulationStyleComparisonState;
