use super::super::{
    SimulationThermalCheckMode, SimulationThermalFieldId, SimulationThermalHistory,
    SimulationThermalHistoryParent, SimulationThermalRelation,
};

/// Native factor identity, independent of any particular version's wire offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPositionAxis {
    /// Object AC, the horizontal position factor.
    Horizontal,
    /// Object B0, the vertical position factor.
    Vertical,
}

impl SimulationLegendPositionAxis {
    /// Resolve the canonical stored path once; e.g. `SimulationLegendPositionAxis::Horizontal.field_path()`.
    pub fn field_path(self) -> &'static str {
        match self {
            Self::Horizontal => "legend.word_ac",
            Self::Vertical => "legend.word_b0",
        }
    }
}

/// One immutable exact source word with its revision-bound identity and physical span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLegendPositionFactor {
    pub(super) axis: SimulationLegendPositionAxis,
    pub(super) source_id: SimulationThermalFieldId,
    pub(super) wire_range: (usize, usize),
    pub(super) raw_bits: u32,
}

impl SimulationLegendPositionFactor {
    /// Inspect the bound role without guessing a screen origin; e.g. `factor.axis()`.
    pub fn axis(&self) -> SimulationLegendPositionAxis {
        self.axis
    }

    /// Borrow the original physical field identity; e.g. `factor.source_id().ordinal()`.
    pub fn source_id(&self) -> &SimulationThermalFieldId {
        &self.source_id
    }

    /// Inspect the absolute source body span; e.g. `factor.wire_range()`.
    pub fn wire_range(&self) -> (usize, usize) {
        self.wire_range
    }

    /// Retain every binary32 encoding, including NaN payloads; e.g. `factor.raw_bits()`.
    pub fn raw_bits(&self) -> u32 {
        self.raw_bits
    }

    /// Read a finite numeric view without clamping or losing signed zero; e.g. `factor.finite_value()`.
    pub fn finite_value(&self) -> Option<f32> {
        let value = f32::from_bits(self.raw_bits);
        value.is_finite().then_some(value)
    }

    /// Retain the copied source field as the sole parent; e.g. `factor.history().parents.len()`.
    pub fn history(&self) -> SimulationThermalHistory {
        SimulationThermalHistory {
            parents: vec![SimulationThermalHistoryParent::Source(
                self.source_id.clone(),
            )]
            .into(),
            result: self.source_id.clone(),
            relation: SimulationThermalRelation::Copied,
        }
    }
}

/// Complete view absence differs from a failed or partial archive read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPositionCompletion {
    /// The complete selected archive profile stores neither named factor.
    CompletedAbsent,
    /// Every present named factor was read exactly.
    CompletedFactors,
}

/// Exact local field coverage after complete selected-profile parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPositionCoverage {
    /// Complete selected profiles and exact source-word/span validation.
    pub check_mode: SimulationThermalCheckMode,
    /// Actual present factors; absence never counts as a known zero.
    pub present_factors: usize,
    /// Four checked source bytes for each present factor.
    pub checked_bytes: usize,
    /// False; raw values were not clamped or normalized.
    pub normalized: bool,
    /// False; a position view never edits archive bytes.
    pub wire_edited: bool,
    /// Exact local pattern preservation has zero achieved error.
    pub achieved_pattern_error_bits: u32,
}

/// Original C847 qualifications remain attached to every completed view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPositionDiagnostic {
    /// SolidWorks target code was not executed.
    NativeNotExecuted,
    /// Native pixel conversions, exceptional results and addition were not emulated.
    PixelConsumersNotModeled,
    /// Actual signed viewport arguments remain unknown.
    RuntimeDimensionsNotInferred,
    /// Native method names do not establish a universal screen-coordinate origin.
    ScreenOriginNotInferred,
}

/// Immutable optional factors; present nonfinite words remain distinct from absent fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLegendPositionFactors {
    pub(super) horizontal: Option<SimulationLegendPositionFactor>,
    pub(super) vertical: Option<SimulationLegendPositionFactor>,
}

impl SimulationLegendPositionFactors {
    /// Borrow only the actually stored horizontal factor; e.g. `factors.horizontal()`.
    pub fn horizontal(&self) -> Option<&SimulationLegendPositionFactor> {
        self.horizontal.as_ref()
    }

    /// Borrow only the actually stored vertical factor; e.g. `factors.vertical()`.
    pub fn vertical(&self) -> Option<&SimulationLegendPositionFactor> {
        self.vertical.as_ref()
    }

    /// Distinguish completed absence from completed present factors; e.g. `factors.completion()`.
    pub fn completion(&self) -> SimulationLegendPositionCompletion {
        if self.horizontal.is_none() && self.vertical.is_none() {
            return SimulationLegendPositionCompletion::CompletedAbsent;
        }
        SimulationLegendPositionCompletion::CompletedFactors
    }

    /// Inspect exact present-word coverage with zero pattern error; e.g. `factors.coverage()`.
    pub fn coverage(&self) -> SimulationLegendPositionCoverage {
        let count = usize::from(self.horizontal.is_some()) + usize::from(self.vertical.is_some());
        SimulationLegendPositionCoverage {
            check_mode: SimulationThermalCheckMode::ExactSelectedProfiles,
            present_factors: count,
            checked_bytes: count * 4,
            normalized: false,
            wire_edited: false,
            achieved_pattern_error_bits: 0,
        }
    }

    /// Retain native pixel/runtime exclusions with the result; e.g. `factors.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendPositionDiagnostic; 4] {
        [
            SimulationLegendPositionDiagnostic::NativeNotExecuted,
            SimulationLegendPositionDiagnostic::PixelConsumersNotModeled,
            SimulationLegendPositionDiagnostic::RuntimeDimensionsNotInferred,
            SimulationLegendPositionDiagnostic::ScreenOriginNotInferred,
        ]
    }
}
