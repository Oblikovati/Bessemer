//! Lossless documented Cosworks persistence frames with explicit caller context.
mod beam;
mod beam_groups;
mod component_persist_id;
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
mod mesh_control_header;
mod mesh_control_records;
mod mesh_property;
mod parameter_sets;
mod post_dialog;
mod post_graph;
mod post_info;
mod post_render;
mod post_style;
mod post_style_preferences;
mod post_units;
mod post_vector;
mod results_property;
mod scalars;
mod sheet_property;
mod shell_composite;
mod shell_group;
mod shell_manager_header;
mod shell_profile;
mod shell_property;
mod solid_body;
mod solid_children;
mod solid_manager;
mod study;
mod study_edit;
mod surface_body;
mod text;
mod text_sequence;
mod thermal;
mod version;
use super::{Result, invalid, require};
pub use beam::*;
pub use beam_groups::*;
pub use component_persist_id::*;
pub use coverage::*;
pub use inertial::*;
pub use lbc_final::*;
pub use legacy::*;
pub use manager_property::*;
pub use mass_element::*;
pub use material::*;
pub use mesh_control_header::*;
pub use mesh_control_records::*;
pub use mesh_property::*;
pub use parameter_sets::*;
pub use post_dialog::*;
pub use post_graph::*;
pub use post_info::*;
pub use post_render::*;
pub use post_style::*;
pub use post_style_preferences::*;
pub use post_units::*;
pub use post_vector::*;
pub use results_property::*;
pub use scalars::*;
pub use sheet_property::*;
pub use shell_composite::*;
pub use shell_group::*;
pub use shell_manager_header::*;
pub use shell_profile::*;
pub use shell_property::*;
pub use solid_body::*;
pub use solid_children::*;
pub use solid_manager::*;
pub use study::*;
pub use study_edit::*;
pub use surface_body::*;
pub use text::*;
pub use thermal::*;
pub use version::*;

mod rgba;
pub use rgba::*;

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
