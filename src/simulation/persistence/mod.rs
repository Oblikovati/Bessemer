//! Lossless documented Cosworks persistence frames with explicit caller context.
mod coverage;
mod lbc_final;
mod legacy;
mod load_base;
mod load_edit;
mod load_identity;
mod load_reader;
mod load_snapshot;
mod manager_property;
mod parameter_sets;
mod scalars;
mod study;
mod study_edit;
mod text;
mod text_sequence;
mod thermal;
mod version;
use super::{Result, invalid, require};
pub use coverage::*;
pub use lbc_final::*;
pub use legacy::*;
pub use manager_property::*;
pub use parameter_sets::*;
pub use scalars::*;
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
