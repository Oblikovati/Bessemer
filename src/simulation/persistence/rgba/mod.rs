//! Raw-preserving COGLColor views shared by standalone styles and parent dialogs (#166).
mod collect;
mod value;

pub(super) use collect::collect_wire_colors;
pub use value::{SimulationColorChannel, SimulationPostColorView, SimulationRgbaColor};
