use super::super::SimulationRgbaColor;
use super::super::triangle_snapshot::validate_triangle_group_snapshot;
use super::{SimulationAuxiliaryTriangleControls, types::*};
use crate::Result;

/// Immutable owned requests, provenance, completion and local coverage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationAuxTrianglePlan {
    controls: SimulationAuxiliaryTriangleControls,
    context: SimulationAuxTriangleRuntime,
    stage: SimulationAuxTriangleStage,
    initial_color: Option<SimulationRgbaColor>,
    calls: Box<[SimulationAuxTriangleCall]>,
    coverage: SimulationAuxTriangleCoverage,
}

impl SimulationAuxiliaryTriangleControls {
    /// Plan documented requests with explicit runtime facts and budget; e.g. `controls.plan_masked_exceptions(context, &groups, 100)?`.
    pub fn plan_masked_exceptions(
        self,
        context: SimulationAuxTriangleRuntime,
        groups: &[SimulationAuxTriangleGroup],
        maximum_groups: usize,
    ) -> Result<SimulationAuxTrianglePlan> {
        let stage = admitted_stage(self, context);
        let count = inspected_count(stage, context);
        validate_triangle_group_snapshot(
            "auxiliary triangle call plan",
            count,
            groups.len(),
            maximum_groups,
        )?;
        let color = if stage == SimulationAuxTriangleStage::HelperEntered {
            self.renderer_color_masked_exceptions()
        } else {
            None
        };
        let calls = planned_calls(self, color, &groups[..count]);
        Ok(SimulationAuxTrianglePlan::new(
            self,
            context,
            stage,
            color,
            calls,
            count,
            maximum_groups,
        ))
    }
}

fn admitted_stage(
    controls: SimulationAuxiliaryTriangleControls,
    context: SimulationAuxTriangleRuntime,
) -> SimulationAuxTriangleStage {
    if !controls.helper_requested_masked_exceptions() {
        return SimulationAuxTriangleStage::CallerBypassed;
    }
    match helper_rejection(context) {
        Some(reason) => SimulationAuxTriangleStage::HelperRejected(reason),
        None => SimulationAuxTriangleStage::HelperEntered,
    }
}

fn helper_rejection(
    context: SimulationAuxTriangleRuntime,
) -> Option<SimulationAuxTriangleRejection> {
    if !context.core_object_present {
        return Some(SimulationAuxTriangleRejection::MissingCoreObject);
    }
    if context.plot_word_170 == 51 {
        return Some(SimulationAuxTriangleRejection::ExcludedPlot);
    }
    if context.group_count_raw == 0 {
        return Some(SimulationAuxTriangleRejection::EmptyGroupCount);
    }
    if !context.group_array_present {
        return Some(SimulationAuxTriangleRejection::MissingGroupArray);
    }
    if (3..=5).contains(&context.renderer_type_code) {
        return Some(SimulationAuxTriangleRejection::ExcludedRenderer);
    }
    None
}

fn inspected_count(
    stage: SimulationAuxTriangleStage,
    context: SimulationAuxTriangleRuntime,
) -> usize {
    if stage != SimulationAuxTriangleStage::HelperEntered {
        return 0;
    }
    let signed = context.group_count_raw as i32;
    if signed <= 0 {
        return 0;
    }
    signed as usize
}

fn planned_calls(
    controls: SimulationAuxiliaryTriangleControls,
    color: Option<SimulationRgbaColor>,
    groups: &[SimulationAuxTriangleGroup],
) -> Box<[SimulationAuxTriangleCall]> {
    let Some(color) = color else {
        return Box::new([]);
    };
    groups
        .iter()
        .enumerate()
        .filter(|(_, group)| group.triangle_count > 0)
        .map(|(index, group)| {
            planned_call(index, *group, color, controls.per_group_rgb_requested())
        })
        .collect()
}

fn planned_call(
    index: usize,
    group: SimulationAuxTriangleGroup,
    supplied: SimulationRgbaColor,
    per_group: bool,
) -> SimulationAuxTriangleCall {
    let mut bits = supplied.bits();
    let color_source = if per_group {
        bits[..3].copy_from_slice(&group.color.bits()[..3]);
        SimulationAuxTriangleColorSource::GroupRgbWithSuppliedAlpha
    } else {
        SimulationAuxTriangleColorSource::Supplied
    };
    SimulationAuxTriangleCall {
        group_index: index as u32,
        triangle_count: group.triangle_count,
        payload_tokens: group.payload_tokens,
        color: SimulationRgbaColor::from_bits(bits),
        color_source,
    }
}

impl SimulationAuxTrianglePlan {
    fn new(
        controls: SimulationAuxiliaryTriangleControls,
        context: SimulationAuxTriangleRuntime,
        stage: SimulationAuxTriangleStage,
        initial_color: Option<SimulationRgbaColor>,
        calls: Box<[SimulationAuxTriangleCall]>,
        count: usize,
        maximum_groups: usize,
    ) -> Self {
        Self {
            controls,
            context,
            stage,
            initial_color,
            calls,
            coverage: SimulationAuxTriangleCoverage {
                local_requests_only: true,
                declared_group_count: context.group_count_raw,
                inspected_groups: count,
                maximum_groups,
                achieved_pattern_error_bits: 0,
            },
        }
    }
    /// Inspect ordered owned requests; e.g. `plan.calls().len()`.
    pub fn calls(&self) -> &[SimulationAuxTriangleCall] {
        &self.calls
    }
    /// Inspect the furthest admitted local stage; e.g. `plan.stage()`.
    pub fn stage(&self) -> SimulationAuxTriangleStage {
        self.stage
    }
    /// Read copied caller controls for provenance; e.g. `plan.source_controls()`.
    pub fn source_controls(&self) -> SimulationAuxiliaryTriangleControls {
        self.controls
    }
    /// Read explicit runtime facts used by this plan; e.g. `plan.runtime_context()`.
    pub fn runtime_context(&self) -> SimulationAuxTriangleRuntime {
        self.context
    }
    /// Read admitted setup color even when no group has positive count; e.g. `plan.initial_renderer_color()`.
    pub fn initial_renderer_color(&self) -> Option<SimulationRgbaColor> {
        self.initial_color
    }
    /// Inspect coverage and the actual budget; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationAuxTriangleCoverage {
        self.coverage
    }
    /// Distinguish completed empty planning from completed requests; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationAuxTriangleCompletion {
        if self.calls.is_empty() {
            SimulationAuxTriangleCompletion::CompletedEmpty
        } else {
            SimulationAuxTriangleCompletion::CompletedCalls
        }
    }
    /// Keep native/geometry exclusions alongside completion; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationAuxTriangleDiagnostic; 3] {
        [
            SimulationAuxTriangleDiagnostic::NativeNotExecuted,
            SimulationAuxTriangleDiagnostic::PayloadNotValidated,
            SimulationAuxTriangleDiagnostic::SceneTraversalNotModeled,
        ]
    }
}
