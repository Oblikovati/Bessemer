//! C-887 annotation requests and the separate glyph material adjustment gate (#166).
mod views;

/// Raw independent Min/Max controls; no extrema values, positions or rendering success are inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationExtremaAnnotationControls {
    /// Stored style234; any nonzero signed value requests the minimum annotation.
    pub minimum_raw: i32,
    /// Stored style238; any nonzero signed value requests the maximum annotation.
    pub maximum_raw: i32,
}

impl SimulationExtremaAnnotationControls {
    /// Model the minimum's own annotation branch; e.g. `controls.minimum_requested()`.
    pub fn minimum_requested(self) -> bool {
        self.minimum_raw != 0
    }

    /// Model the maximum's independent annotation branch; e.g. `controls.maximum_requested()`.
    pub fn maximum_requested(self) -> bool {
        self.maximum_raw != 0
    }

    /// Request the value/position helper without claiming its results; e.g. `controls.needs_value_position_sets()`.
    pub fn needs_value_position_sets(self) -> bool {
        self.minimum_requested() || self.maximum_requested()
    }

    /// Model only material adjustment; subsequent glyph helper calls remain separate; e.g. `controls.glyph_material_adjustment_requested(1, true)`.
    pub fn glyph_material_adjustment_requested(
        self,
        render_argument: i32,
        plot_object_present: bool,
    ) -> bool {
        render_argument != 0 && plot_object_present && self.needs_value_position_sets()
    }
}
