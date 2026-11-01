//! Raw-preserving COGLColor views shared by standalone styles and parent dialogs (#166).
mod alpha_cap;
mod collect;
mod color_comparison;
mod colorref_import;
mod colorref_projection;
mod value;

pub(super) use alpha_cap::cap_alpha_one_masked_daz_ftz_off;
pub(super) use collect::collect_wire_colors;
pub(super) use color_comparison::unordered_nan as is_binary32_nan;
pub use colorref_projection::{SimulationColorrefDomainFailure, SimulationColorrefProjection};
pub use value::{SimulationColorChannel, SimulationPostColorView, SimulationRgbaColor};
