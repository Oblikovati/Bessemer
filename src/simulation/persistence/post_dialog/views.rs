//! Bounded prefix accessors from specification 14.492–14.493; UI mode names remain unproven (#166).
use super::{SimulationPostDialog, SimulationPostDialogValue};
use crate::{Result, simulation::invalid};

/// Stored range patterns and the bounded selection documented for the dialog prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPostDialogRangeView {
    /// Original signed state, including unrecognized combinations.
    pub stored_combination: i32,
    /// Decoded state re-encoded as zero or one of the three recognized combinations.
    pub normalized_combination: i32,
    /// Whether the stored user minimum is selected.
    pub user_minimum_selected: bool,
    /// Whether the stored user maximum is selected.
    pub user_maximum_selected: bool,
    /// Baseline minimum float32 pattern.
    pub baseline_minimum_bits: u32,
    /// Baseline maximum float32 pattern.
    pub baseline_maximum_bits: u32,
    /// User minimum float32 pattern.
    pub user_minimum_bits: u32,
    /// User maximum float32 pattern.
    pub user_maximum_bits: u32,
    /// Selected minimum pattern, without numeric normalization.
    pub selected_minimum_bits: u32,
    /// Selected maximum pattern, without numeric normalization.
    pub selected_maximum_bits: u32,
}

/// Origin of the selected scale; UI mode names and geometry meaning remain unproven.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPostDialogScaleSource {
    /// Stored float32 at object offset 0x528.
    Stored528,
    /// Stored float32 at object offset 0x52c.
    Stored52c,
    /// Literal float32 one for any other selector.
    ConstantOne,
}

/// Raw-preserving deformation selector and its bounded accessor result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPostDialogDeformationView {
    /// Original signed selector, including unknown values.
    pub selector: i32,
    /// First stored float32 pattern.
    pub scale_528_bits: u32,
    /// Second stored float32 pattern.
    pub scale_52c_bits: u32,
    /// Selected stored pattern or literal float32 one.
    pub selected_scale_bits: u32,
    /// Exact stored or fallback origin.
    pub source: SimulationPostDialogScaleSource,
}

impl SimulationPostDialog {
    /// Select stored range endpoints without changing any float bits; e.g. `dialog.range_view()?.selected_minimum_bits`.
    pub fn range_view(&self) -> Result<SimulationPostDialogRangeView> {
        let state = signed_prefix(self, 14)?;
        let patterns = [
            scalar_prefix(self, 17)?,
            scalar_prefix(self, 18)?,
            scalar_prefix(self, 19)?,
            scalar_prefix(self, 20)?,
        ];
        Ok(SimulationPostDialogRangeView::from_stored(state, patterns))
    }

    /// Select a stored scale or literal one, without inferring a UI mode; e.g. `dialog.deformation_scale_view()?.source`.
    pub fn deformation_scale_view(&self) -> Result<SimulationPostDialogDeformationView> {
        let selector = signed_prefix(self, 13)?;
        let scale_528_bits = scalar_prefix(self, 15)?;
        let scale_52c_bits = scalar_prefix(self, 16)?;
        let (selected_scale_bits, source) = match selector {
            0 => (scale_528_bits, SimulationPostDialogScaleSource::Stored528),
            1 => (scale_52c_bits, SimulationPostDialogScaleSource::Stored52c),
            _ => (0x3f800000, SimulationPostDialogScaleSource::ConstantOne),
        };
        Ok(SimulationPostDialogDeformationView {
            selector,
            scale_528_bits,
            scale_52c_bits,
            selected_scale_bits,
            source,
        })
    }
}

fn signed_prefix(dialog: &SimulationPostDialog, ordinal: usize) -> Result<i32> {
    match dialog.fields().get(ordinal).map(|field| &field.value) {
        Some(SimulationPostDialogValue::Signed(value)) => Ok(*value),
        value => Err(invalid(
            "dialog prefix view",
            format!("field {ordinal}: {value:?}; expected signed32 word"),
        )),
    }
}

fn scalar_prefix(dialog: &SimulationPostDialog, ordinal: usize) -> Result<u32> {
    match dialog.fields().get(ordinal).map(|field| &field.value) {
        Some(SimulationPostDialogValue::Float32Bits(bits)) => Ok(*bits),
        value => Err(invalid(
            "dialog prefix view",
            format!("field {ordinal}: {value:?}; expected float32 pattern"),
        )),
    }
}

impl SimulationPostDialogRangeView {
    fn from_stored(state: i32, patterns: [u32; 4]) -> Self {
        let lower = matches!(state, 1 | 2);
        let upper = matches!(state, 1 | 3);
        Self {
            stored_combination: state,
            normalized_combination: if (1..=3).contains(&state) { state } else { 0 },
            user_minimum_selected: lower,
            user_maximum_selected: upper,
            baseline_minimum_bits: patterns[0],
            baseline_maximum_bits: patterns[1],
            user_minimum_bits: patterns[2],
            user_maximum_bits: patterns[3],
            selected_minimum_bits: pick_pattern(lower, patterns[2], patterns[0]),
            selected_maximum_bits: pick_pattern(upper, patterns[3], patterns[1]),
        }
    }
}

fn pick_pattern(selected: bool, user: u32, baseline: u32) -> u32 {
    if selected { user } else { baseline }
}
