use super::super::SimulationRgbaColor;
use super::{SimulationStylePreferenceColorRoute, types::*};
use crate::{BessemerError, FailureClass, Result};

/// Immutable local preference-color projection with original and consumed parent words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationStylePreferenceColorPlan {
    original: SimulationRgbaColor,
    projected: SimulationRgbaColor,
    route: SimulationStylePreferenceColorRoute,
    runtime: SimulationStylePreferenceColorRuntime,
    source_length: Option<usize>,
    history: SimulationStylePreferenceColorHistory,
}
impl SimulationRgbaColor {
    /// Project one reached C907 route without editing wire bytes; e.g. `color.project_style_preference_nearest_even(route, Some(&bytes), runtime)?`.
    pub fn project_style_preference_nearest_even(
        self,
        route: SimulationStylePreferenceColorRoute,
        preferences: Option<&[u8]>,
        runtime: SimulationStylePreferenceColorRuntime,
    ) -> Result<SimulationStylePreferenceColorPlan> {
        let mut plan =
            preference_color_initial_plan(self, route, preferences.map(<[u8]>::len), runtime);
        if !runtime.local_block_applies()? {
            return Ok(plan);
        }
        let bytes = reached_preference_bytes(route, preferences)?;
        let packed = preference_color_word(bytes, route.packed_preference_offset())?;
        let mut projected = self.assigned_colorref_nearest_even(packed).bits();
        plan.history.packed_preference_word = Some(packed);
        if let Some(offset) = route.alpha_preference_offset() {
            let alpha = preference_color_word(bytes, offset)?;
            projected[3] = alpha;
            plan.history.alpha_preference_word = Some(alpha);
            plan.history.alpha_origin = SimulationStylePreferenceAlphaOrigin::PreferenceWord {
                byte_offset: offset,
            };
        }
        plan.projected = Self::from_bits(projected);
        Ok(plan)
    }
}
fn preference_color_initial_plan(
    original: SimulationRgbaColor,
    route: SimulationStylePreferenceColorRoute,
    source_length: Option<usize>,
    runtime: SimulationStylePreferenceColorRuntime,
) -> SimulationStylePreferenceColorPlan {
    SimulationStylePreferenceColorPlan {
        original,
        projected: original,
        route,
        runtime,
        source_length,
        history: SimulationStylePreferenceColorHistory {
            packed_preference_word: None,
            alpha_preference_word: None,
            alpha_origin: SimulationStylePreferenceAlphaOrigin::Original,
        },
    }
}
fn reached_preference_bytes(
    route: SimulationStylePreferenceColorRoute,
    preferences: Option<&[u8]>,
) -> Result<&[u8]> {
    let bytes = preferences.ok_or_else(|| {
        BessemerError::new(
            "style preference color",
            "preference bytes",
            FailureClass::Missing,
            format!(
                "source None; expected Some(preference bytes) through {} for member {:#x}",
                route.required_preference_bytes(),
                route.style_member()
            ),
        )
    })?;
    if bytes.len() < route.required_preference_bytes() {
        return Err(preference_color_bounds_failure(bytes.len(), route));
    }
    Ok(bytes)
}
fn preference_color_word(bytes: &[u8], offset: usize) -> Result<u32> {
    let raw: [u8; 4] = bytes
        .get(offset..offset + 4)
        .and_then(|word| word.try_into().ok())
        .ok_or_else(|| {
            BessemerError::new(
                "style preference color",
                "preference word",
                FailureClass::Invalid,
                format!(
                    "{} bytes at offset {offset:#x}; expected a complete little-endian32 word",
                    bytes.len()
                ),
            )
        })?;
    Ok(u32::from_le_bytes(raw))
}
impl SimulationStylePreferenceColorPlan {
    /// Inspect the complete unchanged original RGBA; e.g. `plan.original()`.
    pub fn original(&self) -> SimulationRgbaColor {
        self.original
    }
    /// Inspect the separately derived RGBA, including raw alpha policy; e.g. `plan.projected()`.
    pub fn projected(&self) -> SimulationRgbaColor {
        self.projected
    }
    /// Inspect the normalized member and preference offsets; e.g. `plan.route()`.
    pub fn route(&self) -> SimulationStylePreferenceColorRoute {
        self.route
    }
    /// Inspect supplied known/unknown guard facts, including unused pointed types; e.g. `plan.runtime_context()`.
    pub fn runtime_context(&self) -> SimulationStylePreferenceColorRuntime {
        self.runtime
    }
    /// Inspect original source availability/length without retaining borrowed bytes; e.g. `plan.preference_source_length()`.
    pub fn preference_source_length(&self) -> Option<usize> {
        self.source_length
    }
    /// Read every consumed packed/alpha parent and its derivation; e.g. `plan.history()`.
    pub fn history(&self) -> SimulationStylePreferenceColorHistory {
        self.history
    }
    /// Distinguish local skipping from complete local projection; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationStylePreferenceColorCompletion {
        if self.history.packed_preference_word.is_some() {
            return SimulationStylePreferenceColorCompletion::CompletedProjection;
        }
        SimulationStylePreferenceColorCompletion::CompletedSkipped
    }
    /// Inspect exact arithmetic/source coverage and the preceding legend boundary; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationStylePreferenceColorCoverage {
        let projected = usize::from(self.history.packed_preference_word.is_some());
        let alpha = usize::from(self.history.alpha_preference_word.is_some());
        SimulationStylePreferenceColorCoverage {
            preference_words_read: projected + alpha,
            rgb_words_projected: 3 * projected,
            alpha_words_replaced: alpha,
            legend_call_precedes_guard: true,
            wire_edited: false,
            exact_nearest_even_model: true,
            achieved_pattern_error_bits: 0,
        }
    }
    /// Retain native and whole-setter exclusions on every completed view; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationStylePreferenceColorDiagnostic; 4] {
        [
            SimulationStylePreferenceColorDiagnostic::NativeNotExecuted,
            SimulationStylePreferenceColorDiagnostic::LegendCallNotModeled,
            SimulationStylePreferenceColorDiagnostic::WholeSetterNotModeled,
            SimulationStylePreferenceColorDiagnostic::NativeFloatingEnvironmentUnknown,
        ]
    }
}

fn preference_color_bounds_failure(
    length: usize,
    route: SimulationStylePreferenceColorRoute,
) -> BessemerError {
    BessemerError::new(
        "style preference color",
        "preference bytes",
        FailureClass::Invalid,
        format!(
            "{length} source bytes; expected at least {} through member {:#x}'s reached packed/alpha sources",
            route.required_preference_bytes(),
            route.style_member()
        ),
    )
}
