use super::super::SimulationRgbaColor;
use super::{group::*, mode3_required, types::*};
use crate::Result;

/// Immutable one-group color plan retaining every reached copied parent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationMode3RgbPlan {
    inputs: SimulationMode3RgbInputs,
    group: SimulationMode3RgbGroup,
    copied_stored: SimulationRgbaColor,
    candidate: [u32; 3],
    copied_group: Option<[SimulationRgbaColor; 2]>,
    selected: Option<[u32; 3]>,
    origin: Option<SimulationMode3RgbOrigin>,
    stage: SimulationMode3RgbStage,
}
impl SimulationMode3RgbInputs {
    /// Plan one reached primary/reflected group with exact copied provenance; e.g. `inputs.plan_group_rgb(group)?`.
    pub fn plan_group_rgb(self, group: SimulationMode3RgbGroup) -> Result<SimulationMode3RgbPlan> {
        let (mut plan, mode) = mode3_candidate_plan(self, group)?;
        if let Some(stage) = mode3_group_rejection(group)? {
            plan.stage = stage;
            return Ok(plan);
        }
        let (copied, selected, origin) = mode3_selected_color(mode, plan.candidate, group)?;
        plan.copied_group = copied;
        plan.selected = Some(selected);
        plan.origin = Some(origin);
        plan.stage = mode3_submission_stage(self, group)?;
        Ok(plan)
    }
}
impl SimulationMode3RgbPlan {
    /// Inspect actual optional file facts; e.g. `plan.source_inputs()`.
    pub fn source_inputs(&self) -> SimulationMode3RgbInputs {
        self.inputs
    }
    /// Inspect the supplied reached group context; e.g. `plan.source_group()`.
    pub fn source_group(&self) -> SimulationMode3RgbGroup {
        self.group
    }
    /// Retain the unconditional complete color copy; e.g. `plan.copied_stored_rgba()`.
    pub fn copied_stored_rgba(&self) -> SimulationRgbaColor {
        self.copied_stored
    }
    /// Inspect the initial stored/default candidate; e.g. `plan.candidate_rgb_bits()`.
    pub fn candidate_rgb_bits(&self) -> [u32; 3] {
        self.candidate
    }
    /// Inspect the initial candidate's exact source, including the half literal; e.g. `plan.candidate_origin()`.
    pub fn candidate_origin(&self) -> SimulationMode3RgbOrigin {
        if self.inputs.mode_1b8_raw == Some(3) {
            return SimulationMode3RgbOrigin::Stored1dc;
        }
        SimulationMode3RgbOrigin::DefaultHalf
    }
    /// Inspect both copied group parents whenever replacement was reached; e.g. `plan.copied_group_colors()`.
    pub fn copied_group_colors(&self) -> Option<[SimulationRgbaColor; 2]> {
        self.copied_group
    }
    /// Inspect selected RGB even when primary filtering suppressed submission; e.g. `plan.selected_rgb_bits()`.
    pub fn selected_rgb_bits(&self) -> Option<[u32; 3]> {
        self.selected
    }
    /// Inspect the exact selected RGB parent; e.g. `plan.selected_origin()`.
    pub fn selected_origin(&self) -> Option<SimulationMode3RgbOrigin> {
        self.origin
    }
    /// Inspect the furthest local stage; e.g. `plan.stage()`.
    pub fn stage(&self) -> SimulationMode3RgbStage {
        self.stage
    }
    /// Inspect a fixed count1/index5 RGB request, without executing it; e.g. `plan.array_request_rgb()`.
    pub fn array_request_rgb(&self) -> Option<[u32; 3]> {
        if self.stage == SimulationMode3RgbStage::ArrayRequested {
            return self.selected;
        }
        None
    }
    /// Distinguish completed empty from selected array requests; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationMode3RgbCompletion {
        if self.array_request_rgb().is_some() {
            return SimulationMode3RgbCompletion::CompletedSelection;
        }
        SimulationMode3RgbCompletion::CompletedEmpty
    }
    /// Inspect exact local copy coverage; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationMode3RgbCoverage {
        SimulationMode3RgbCoverage {
            stored_words_copied: 4,
            group_words_copied: 8 * usize::from(self.copied_group.is_some()),
            selected_words: 3 * usize::from(self.selected.is_some()),
            written_words: 0,
            destination_bounds_checked: false,
            exact_word_checks: true,
            geometry_validated: false,
            achieved_pattern_error_bits: 0,
        }
    }
    /// Retain whole-caller, renderer and geometry exclusions; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationMode3RgbDiagnostic; 4] {
        [
            SimulationMode3RgbDiagnostic::NativeNotExecuted,
            SimulationMode3RgbDiagnostic::WholeCallerUnmodeled,
            SimulationMode3RgbDiagnostic::RendererStateUnmodeled,
            SimulationMode3RgbDiagnostic::AllocatorAndGeometryUnmodeled,
        ]
    }
}

fn mode3_candidate_parents(
    inputs: SimulationMode3RgbInputs,
) -> Result<(i32, SimulationRgbaColor, [u32; 3])> {
    let mode = mode3_required(
        inputs.mode_1b8_raw,
        "stored1B8",
        "Some(raw signed32 stored mode)",
    )?;
    let copied = mode3_required(
        inputs.color_1dc,
        "stored1DC",
        "Some(complete original RGBA)",
    )?;
    let candidate = if mode == 3 {
        mode3_rgb_words(copied)
    } else {
        [0x3f000000; 3]
    };
    Ok((mode, copied, candidate))
}
fn mode3_candidate_plan(
    inputs: SimulationMode3RgbInputs,
    group: SimulationMode3RgbGroup,
) -> Result<(SimulationMode3RgbPlan, i32)> {
    let (mode, copied_stored, candidate) = mode3_candidate_parents(inputs)?;
    Ok((
        SimulationMode3RgbPlan {
            inputs,
            group,
            copied_stored,
            candidate,
            copied_group: None,
            selected: None,
            origin: None,
            stage: SimulationMode3RgbStage::GroupHidden,
        },
        mode,
    ))
}
