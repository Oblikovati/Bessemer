use crate::{BessemerError, FailureClass, Result};

/// Explicit guard facts; an unknown pointer is distinct from a known null pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationStylePreferenceColorRuntime {
    /// Presence of the pointer at style2D0.
    pub object_pointer_present: Option<bool>,
    /// Signed raw type at pointed+10, required only for a nonnull pointer.
    pub pointed_type_raw: Option<i32>,
}
impl SimulationStylePreferenceColorRuntime {
    /// Decide the local block after the unmodeled preceding legend call; e.g. `runtime.local_block_applies()?`.
    pub fn local_block_applies(self) -> Result<bool> {
        let pointer = self.object_pointer_present.ok_or_else(|| {
            missing_preference_context("style2D0 pointer", "Some(explicit pointer presence)")
        })?;
        if !pointer {
            return Ok(true);
        }
        let pointed = self.pointed_type_raw.ok_or_else(|| {
            missing_preference_context("pointed type10", "Some(raw signed32 type)")
        })?;
        Ok(pointed != 73)
    }
}
fn missing_preference_context(entity: &str, expected: &str) -> BessemerError {
    BessemerError::new(
        "style preference color",
        entity,
        FailureClass::Missing,
        format!("fact None; expected {expected} at the reached local guard"),
    )
}
/// Exact parent of the final alpha word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStylePreferenceAlphaOrigin {
    /// All four original alpha bytes were retained.
    Original,
    /// Raw preference bytes at the supplied route offset replaced alpha.
    PreferenceWord {
        /// Original byte offset within the supplied preference snapshot.
        byte_offset: usize,
    },
}
/// All consumed preference parents; skipped sources remain explicitly unread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationStylePreferenceColorHistory {
    /// Complete packed word including the ignored high byte, or None when skipped.
    pub packed_preference_word: Option<u32>,
    /// Raw replacement alpha word, or None for preserved/skipped alpha.
    pub alpha_preference_word: Option<u32>,
    /// Exact original/replacement alpha source.
    pub alpha_origin: SimulationStylePreferenceAlphaOrigin,
}
/// Completion concerns only the local color view, never a complete setter invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStylePreferenceColorCompletion {
    /// The local style block was skipped and the complete original color retained.
    CompletedSkipped,
    /// The reached packed/alpha sources were projected under explicit nearest-even arithmetic.
    CompletedProjection,
}
/// Exact local word coverage, without wire edits or native floating-state inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationStylePreferenceColorCoverage {
    /// Consumed packed plus optional alpha words, zero through two.
    pub preference_words_read: usize,
    /// Projected RGB words, zero or three even when their values were unchanged.
    pub rgb_words_projected: usize,
    /// Raw alpha replacements, zero or one.
    pub alpha_words_replaced: usize,
    /// Static ordering fact; it does not claim execution of that call.
    pub legend_call_precedes_guard: bool,
    /// False; the original archive bytes were not rewritten.
    pub wire_edited: bool,
    /// True; the model uses explicit exact nearest-even binary32 multiplication.
    pub exact_nearest_even_model: bool,
    /// Zero achieved raw-pattern error under the declared arithmetic model.
    pub achieved_pattern_error_bits: u32,
}
/// Exclusions remain attached to both skipped and projected local views.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStylePreferenceColorDiagnostic {
    /// Installed code was not executed.
    NativeNotExecuted,
    /// The preceding legend preference call is outside this local projection.
    LegendCallNotModeled,
    /// Other preference assignments and full setter state remain outside the view.
    WholeSetterNotModeled,
    /// Native MXCSR and actual per-file applicability are not inferred from stored colors.
    NativeFloatingEnvironmentUnknown,
}
