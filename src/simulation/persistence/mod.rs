//! Lossless documented Cosworks persistence frames with explicit caller context.
mod beam;
mod beam_groups;
mod coverage;
mod inertial;
mod lbc_final;
mod legacy;
mod load_base;
mod load_edit;
mod load_identity;
mod load_persist_id;
mod load_prefix;
mod load_reader;
mod load_snapshot;
mod manager_property;
mod mass_element;
mod material;
mod parameter_sets;
mod scalars;
mod sheet_property;
mod solid_body;
mod solid_children;
mod solid_manager;
mod study;
mod study_edit;
mod text;
mod text_sequence;
mod thermal;
mod version;
use super::{Result, invalid, require};
pub use beam::*;
pub use beam_groups::*;
pub use coverage::*;
pub use inertial::*;
pub use lbc_final::*;
pub use legacy::*;
pub use manager_property::*;
pub use mass_element::*;
pub use material::*;
pub use parameter_sets::*;
pub use scalars::*;
pub use sheet_property::*;
pub use solid_body::*;
pub use solid_children::*;
pub use solid_manager::*;
pub use study::*;
pub use study_edit::*;
pub use text::*;
pub use thermal::*;
pub use version::*;

fn reserve_bytes(size: usize) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size).map_err(|error| {
        invalid(
            "persistence image allocation",
            format!("{size} bytes, {error}; expected available complete image storage"),
        )
    })?;
    Ok(bytes)
}
