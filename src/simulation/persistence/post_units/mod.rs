//! C-881 numeric policy with explicit C-882/C-883 runtime dependencies; engineering labels remain unknown (#166).
mod families;
mod modifier;
mod views;

/// Explicit unit-provider controls; optional runtime state is never supplied from image initialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationUnitTypeInputs {
    /// Raw analysis A from dialog0C.
    pub analysis: i32,
    /// Raw plot control P from dialog10.
    pub plot_type: i32,
    /// Raw component C from dialog18, interpreted unsigned for interval tests.
    pub component: i32,
    /// Explicit loaded BE8 context, not inferred from a similarly named archive reference.
    pub word_be8: Option<i32>,
    /// Stored BEC where present, or explicit caller-supplied loaded state.
    pub word_bec: Option<i32>,
    /// Complete raw ActualFreqDispFlag; only exactly1 opens its unit gate.
    pub actual_freq_disp_flag: Option<i32>,
}

impl SimulationUnitTypeInputs {
    /// Return the native numeric code or a typed missing-context error; e.g. `inputs.unit_type_code()?`.
    pub fn unit_type_code(self) -> crate::Result<i32> {
        let component = self.component as u32;
        if let Some(code) = early_override(self.plot_type, component) {
            return Ok(code);
        }
        let fallback = plot_fallback(self.plot_type, component);
        families::analysis_code(self, component, fallback)
    }
}

fn early_override(plot: i32, component: u32) -> Option<i32> {
    match plot {
        53 => Some(if component == 4 { 0 } else { 9999 }),
        54 => Some(if component <= 2 { 2 } else { 3 }),
        55 => Some(2),
        15 => Some(8),
        18 => Some(match component {
            0..=1 => 2,
            2 => 3,
            _ => 9999,
        }),
        65 => Some(1),
        69 | 70 => Some(9999),
        _ => None,
    }
}

fn plot_fallback(plot: i32, component: u32) -> i32 {
    if plot != 71 {
        return 9999;
    }
    match component {
        0 => 38,
        1 => 39,
        2 => 40,
        3..=5 => 0,
        _ => 9999,
    }
}
