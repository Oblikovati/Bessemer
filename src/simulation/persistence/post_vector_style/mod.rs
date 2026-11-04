//! C-886 stored vector controls and explicit local write effects; native callbacks remain separate (#166).
mod views;
use super::SimulationRgbaColor;

/// Raw vector controls with no inferred units, selector domain or rendering validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationVectorStyleControls {
    /// Stored style28C integer.
    pub size_raw: i32,
    /// Stored style290 integer.
    pub density_first_raw: i32,
    /// Stored style294 integer.
    pub density_second_raw: i32,
    /// Stored style298 integer.
    pub color_selector_raw: i32,
    /// Every original style29C component, including alpha.
    pub color: SimulationRgbaColor,
}

impl SimulationVectorStyleControls {
    /// Model only the bound size write with explicit active-object presence; e.g. `controls.after_size_write(-1, true)`.
    pub fn after_size_write(self, size_raw: i32, active_object_present: bool) -> Self {
        if !active_object_present {
            return self;
        }
        Self { size_raw, ..self }
    }

    /// Model both field writes before downstream density callbacks; e.g. `controls.after_density_writes(50, 0, true)`.
    pub fn after_density_writes(
        self,
        density_first_raw: i32,
        density_second_raw: i32,
        active_object_present: bool,
    ) -> Self {
        if !active_object_present {
            return self;
        }
        Self {
            density_first_raw,
            density_second_raw,
            ..self
        }
    }

    /// Model packed construction followed by whole-color copy, including alpha one; e.g. `controls.after_color_copy_nearest_even(1, 0x00112233, true)`.
    pub fn after_color_copy_nearest_even(
        self,
        color_selector_raw: i32,
        packed: u32,
        active_object_present: bool,
    ) -> Self {
        if !active_object_present {
            return self;
        }
        Self {
            color_selector_raw,
            color: SimulationRgbaColor::from_colorref_nearest_even(packed),
            ..self
        }
    }
}
