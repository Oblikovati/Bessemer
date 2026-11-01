//! C-884's four-input conditional predicate does not establish complete rendering safety or file validity (#166).
use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    rgba::is_binary32_nan,
};
use super::views::render_word;
use crate::Result;

/// Raw inputs to the named local IsSafeForRendering predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLocalRenderInputs {
    /// Signed stored base-palette count; no allocation bound is inferred.
    pub color_count: i32,
    /// Signed stored cell count.
    pub cell_count: i32,
    /// Exact horizontal position float32 bits.
    pub horizontal_bits: u32,
    /// Exact vertical position float32 bits.
    pub vertical_bits: u32,
}

impl SimulationLocalRenderInputs {
    /// Model only this predicate with masked exceptions and DAZ off; e.g. `inputs.passes_masked_exceptions_daz_off()`.
    pub fn passes_masked_exceptions_daz_off(self) -> bool {
        self.color_count > 0
            && self.cell_count > 0
            && nonnegative_or_unordered(self.horizontal_bits)
            && nonnegative_or_unordered(self.vertical_bits)
    }
}

impl SimulationPostDialog {
    /// Read only explicitly stored current legend inputs; e.g. `dialog.local_render_inputs()?.passes_masked_exceptions_daz_off()`.
    pub fn local_render_inputs(&self) -> Result<SimulationLocalRenderInputs> {
        local_render_from_snapshot(self)
    }
}

impl SimulationPostStyle {
    /// Preserve missing old migration state rather than inserting current position defaults; e.g. `style.local_render_inputs()?`.
    pub fn local_render_inputs(&self) -> Result<SimulationLocalRenderInputs> {
        local_render_from_snapshot(self)
    }
}

fn local_render_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
) -> Result<SimulationLocalRenderInputs> {
    Ok(SimulationLocalRenderInputs {
        color_count: render_word(snapshot, "legend.word_10")?,
        cell_count: render_word(snapshot, "legend.word_14")?,
        horizontal_bits: render_word(snapshot, "legend.word_ac")? as u32,
        vertical_bits: render_word(snapshot, "legend.word_b0")? as u32,
    })
}

fn nonnegative_or_unordered(bits: u32) -> bool {
    bits & 0x80000000 == 0 || bits & 0x7fffffff == 0 || is_binary32_nan(bits)
}
