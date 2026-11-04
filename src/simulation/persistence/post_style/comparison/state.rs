use super::super::super::rgba::{SimulationColorChannel, SimulationRgbaColor};
use super::super::SimulationLegendComparisonState;
use super::{
    SimulationStyleComparisonColor, SimulationStyleComparisonScalar, SimulationStyleComparisonWord,
};

pub(super) const STYLE_CHANNELS: [SimulationColorChannel; 4] = [
    SimulationColorChannel::Red,
    SimulationColorChannel::Green,
    SimulationColorChannel::Blue,
    SimulationColorChannel::Alpha,
];

/// Explicit owned facts; unknown channels remain unknown and scalar/color aliases share storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationStyleComparisonState {
    words: [Option<u32>; 11],
    channels: [Option<u32>; 28],
    pub(super) legend: SimulationLegendComparisonState,
}

impl SimulationStyleComparisonState {
    /// Start with explicit legend facts and unknown style inputs; e.g. `Self::from_legend_state(legend)`.
    pub fn from_legend_state(legend: SimulationLegendComparisonState) -> Self {
        Self {
            words: [None; 11],
            channels: [None; 28],
            legend,
        }
    }

    /// Borrow the original owned legend facts; e.g. `state.legend_state()`.
    pub fn legend_state(&self) -> &SimulationLegendComparisonState {
        &self.legend
    }

    /// Inspect one typed raw input without applying defaults; e.g. `state.word_bits(SimulationStyleComparisonWord::Word1F0)`.
    pub fn word_bits(&self, member: SimulationStyleComparisonWord) -> Option<u32> {
        self.words[member as usize]
    }

    /// Supply actual runtime facts or retain unknown state; e.g. `state.set_word_bits(SimulationStyleComparisonWord::Word1F0, Some(2))`.
    pub fn set_word_bits(&mut self, member: SimulationStyleComparisonWord, bits: Option<u32>) {
        self.words[member as usize] = bits;
    }

    /// Inspect a raw color channel, including partially known runtime colors; e.g. `state.component_bits(color, SimulationColorChannel::Alpha)`.
    pub fn component_bits(
        &self,
        color: SimulationStyleComparisonColor,
        channel: SimulationColorChannel,
    ) -> Option<u32> {
        self.channels[color as usize * 4 + channel as usize]
    }

    /// Supply one actual channel while preserving the other facts; e.g. `state.set_component_bits(color, SimulationColorChannel::Red, Some(0))`.
    pub fn set_component_bits(
        &mut self,
        color: SimulationStyleComparisonColor,
        channel: SimulationColorChannel,
        bits: Option<u32>,
    ) {
        self.channels[color as usize * 4 + channel as usize] = bits;
    }

    /// Supply an entire runtime color or mark all channels unknown; e.g. `state.set_color(color, Some(rgba))`.
    pub fn set_color(
        &mut self,
        color: SimulationStyleComparisonColor,
        value: Option<SimulationRgbaColor>,
    ) {
        for channel in STYLE_CHANNELS {
            self.set_component_bits(color, channel, value.map(|rgba| rgba.channel_bits(channel)));
        }
    }

    /// Read the shared alpha slot, never an independent scalar copy; e.g. `state.scalar_bits(SimulationStyleComparisonScalar::Alpha22C)`.
    pub fn scalar_bits(&self, scalar: SimulationStyleComparisonScalar) -> Option<u32> {
        self.component_bits(scalar.color(), SimulationColorChannel::Alpha)
    }

    /// Update the shared alpha slot without changing RGB; e.g. `state.set_scalar_bits(SimulationStyleComparisonScalar::Alpha22C, Some(0))`.
    pub fn set_scalar_bits(&mut self, scalar: SimulationStyleComparisonScalar, bits: Option<u32>) {
        self.set_component_bits(scalar.color(), SimulationColorChannel::Alpha, bits);
    }
}
