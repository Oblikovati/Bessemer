use super::super::{SimulationRgbaColor, triangle_snapshot::validate_triangle_group_snapshot};
use super::{admission::triangle_color_stage, triangle_alpha_masked_daz_ftz_off, types::*};
use crate::{BessemerError, FailureClass, Result};

/// Immutable complete local plan; geometry and native side effects remain explicitly excluded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationTriangleColorPlan {
    controls: SimulationTriangleColorControls,
    runtime: SimulationTriangleColorRuntime,
    stage: SimulationTriangleColorStage,
    initial_color: Option<SimulationRgbaColor>,
    source_groups: Box<[SimulationTriangleColorGroup]>,
    calls: Box<[SimulationTriangleColorCall]>,
    coverage: SimulationTriangleColorCoverage,
}

impl SimulationTriangleColorControls {
    /// Plan both documented branches with masked exceptions and DAZ/FTZ off; e.g. `controls.plan_masked_daz_ftz_off(runtime, &groups, 100)?`.
    pub fn plan_masked_daz_ftz_off(
        self,
        runtime: SimulationTriangleColorRuntime,
        groups: &[SimulationTriangleColorGroup],
        maximum_groups: usize,
    ) -> Result<SimulationTriangleColorPlan> {
        let stage = triangle_color_stage(self, runtime)?;
        let count = checked_triangle_group_count(
            stage,
            runtime.group_count_raw,
            groups.len(),
            maximum_groups,
        )?;
        publish_triangle_color_plan(self, runtime, stage, &groups[..count], maximum_groups)
    }
}

fn checked_triangle_group_count(
    stage: SimulationTriangleColorStage,
    raw: u32,
    available: usize,
    maximum: usize,
) -> Result<usize> {
    let count = triangle_inspected_count(stage, raw);
    validate_triangle_group_snapshot("triangle color call plan", count, available, maximum)?;
    Ok(count)
}

fn publish_triangle_color_plan(
    controls: SimulationTriangleColorControls,
    runtime: SimulationTriangleColorRuntime,
    stage: SimulationTriangleColorStage,
    groups: &[SimulationTriangleColorGroup],
    maximum: usize,
) -> Result<SimulationTriangleColorPlan> {
    let initial_color = (stage == SimulationTriangleColorStage::HelperEntered)
        .then(|| triangle_initial_color(controls));
    let (calls, lookups) = triangle_color_calls(controls, runtime, initial_color, groups)?;
    Ok(SimulationTriangleColorPlan {
        controls,
        runtime,
        stage,
        initial_color,
        source_groups: owned_triangle_groups(groups)?,
        calls,
        coverage: triangle_color_coverage(runtime.group_count_raw, groups.len(), lookups, maximum),
    })
}

fn triangle_color_coverage(
    raw: u32,
    inspected: usize,
    lookups: usize,
    maximum: usize,
) -> SimulationTriangleColorCoverage {
    SimulationTriangleColorCoverage {
        declared_group_count: raw,
        inspected_groups: inspected,
        required_lookups: lookups,
        maximum_groups: maximum,
        local_requests_only: true,
        achieved_pattern_error_bits: 0,
        exceptions_masked: true,
        daz_enabled: false,
        ftz_enabled: false,
    }
}

fn triangle_inspected_count(stage: SimulationTriangleColorStage, raw: u32) -> usize {
    if stage != SimulationTriangleColorStage::HelperEntered || raw as i32 <= 0 {
        return 0;
    }
    raw as usize
}

fn triangle_initial_color(controls: SimulationTriangleColorControls) -> SimulationRgbaColor {
    let mut bits = controls.components().1.bits();
    bits[3] = triangle_alpha_masked_daz_ftz_off(bits[3]);
    SimulationRgbaColor::from_bits(bits)
}

fn owned_triangle_groups(
    groups: &[SimulationTriangleColorGroup],
) -> Result<Box<[SimulationTriangleColorGroup]>> {
    let mut owned = Vec::new();
    owned
        .try_reserve_exact(groups.len())
        .map_err(|error| triangle_allocation_error(groups.len(), error))?;
    owned.extend_from_slice(groups);
    Ok(owned.into_boxed_slice())
}

fn triangle_allocation_error(
    count: usize,
    error: std::collections::TryReserveError,
) -> BessemerError {
    BessemerError::new(
        "triangle color call plan",
        "group storage",
        FailureClass::Invalid,
        format!(
            "{count} groups, {error}; expected available storage within the explicit group budget"
        ),
    )
}

fn triangle_color_calls(
    controls: SimulationTriangleColorControls,
    runtime: SimulationTriangleColorRuntime,
    initial: Option<SimulationRgbaColor>,
    groups: &[SimulationTriangleColorGroup],
) -> Result<(Box<[SimulationTriangleColorCall]>, usize)> {
    let Some(supplied) = initial else {
        return Ok((Box::new([]), 0));
    };
    let mut calls = Vec::new();
    calls
        .try_reserve_exact(groups.len())
        .map_err(|error| triangle_allocation_error(groups.len(), error))?;
    let mut lookups = 0;
    for (index, group) in groups.iter().enumerate() {
        lookups += usize::from(append_triangle_color_call(
            &mut calls,
            index,
            *group,
            controls,
            runtime.selection_flag_raw,
            supplied,
        )?);
    }
    Ok((calls.into_boxed_slice(), lookups))
}

fn append_triangle_color_call(
    calls: &mut Vec<SimulationTriangleColorCall>,
    index: usize,
    group: SimulationTriangleColorGroup,
    controls: SimulationTriangleColorControls,
    flag: i32,
    supplied: SimulationRgbaColor,
) -> Result<bool> {
    let (eligible, lookup) = triangle_group_eligible(index, group, controls.hidden_class(), flag)?;
    if eligible {
        calls.push(triangle_color_call(
            index,
            group,
            controls.components().0,
            flag,
            supplied,
        ));
    }
    Ok(lookup)
}

fn triangle_group_eligible(
    index: usize,
    group: SimulationTriangleColorGroup,
    hidden: bool,
    flag: i32,
) -> Result<(bool, bool)> {
    if (group.class_raw != 0) != hidden {
        return Ok((false, false));
    }
    if flag == 0 {
        return Ok((group.triangle_count > 0, false));
    }
    let allowed = triangle_lookup_allowed(index, group)?;
    Ok((allowed && group.triangle_count > 0, true))
}

fn triangle_lookup_allowed(index: usize, group: SimulationTriangleColorGroup) -> Result<bool> {
    group.lookup_matches_end.ok_or_else(|| BessemerError::new("triangle color call plan",
        &format!("group[{index}].lookup({})", group.lookup_key_raw), FailureClass::Missing,
        "lookup outcome None; expected an explicit Some(bool) end-identity comparison before testing triangle count"))
}

fn triangle_color_call(
    index: usize,
    group: SimulationTriangleColorGroup,
    selector: i32,
    flag: i32,
    supplied: SimulationRgbaColor,
) -> SimulationTriangleColorCall {
    let (origin, rgb) = triangle_rgb_origin(selector, flag, group, supplied);
    let mut bits = supplied.bits();
    bits[..3].copy_from_slice(&rgb.bits()[..3]);
    SimulationTriangleColorCall {
        group_index: index as u32,
        triangle_count: group.triangle_count,
        payload_tokens: group.payload_tokens,
        color: SimulationRgbaColor::from_bits(bits),
        origin,
    }
}

fn triangle_rgb_origin(
    selector: i32,
    flag: i32,
    group: SimulationTriangleColorGroup,
    supplied: SimulationRgbaColor,
) -> (SimulationTriangleColorOrigin, SimulationRgbaColor) {
    if selector != 1 {
        return (SimulationTriangleColorOrigin::Supplied, supplied);
    }
    if flag == 0 {
        return (SimulationTriangleColorOrigin::GroupC, group.color_c);
    }
    (SimulationTriangleColorOrigin::Group1C, group.color_1c)
}

impl SimulationTriangleColorPlan {
    /// Inspect copied controls; e.g. `plan.source_controls()`.
    pub fn source_controls(&self) -> SimulationTriangleColorControls {
        self.controls
    }
    /// Inspect copied explicit runtime facts; e.g. `plan.runtime_context()`.
    pub fn runtime_context(&self) -> SimulationTriangleColorRuntime {
        self.runtime
    }
    /// Inspect the furthest reached local stage; e.g. `plan.stage()`.
    pub fn stage(&self) -> SimulationTriangleColorStage {
        self.stage
    }
    /// Inspect setup color, including admitted empty iteration; e.g. `plan.initial_renderer_color()`.
    pub fn initial_renderer_color(&self) -> Option<SimulationRgbaColor> {
        self.initial_color
    }
    /// Read owned parents of every inspected group; e.g. `plan.source_groups()`.
    pub fn source_groups(&self) -> &[SimulationTriangleColorGroup] {
        &self.source_groups
    }
    /// Read ordered local triangle requests; e.g. `plan.calls()`.
    pub fn calls(&self) -> &[SimulationTriangleColorCall] {
        &self.calls
    }
    /// Inspect exact coverage, budget and floating-point assumptions; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationTriangleColorCoverage {
        self.coverage
    }
    /// Distinguish completed empty planning from completed requests; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationTriangleColorCompletion {
        if self.calls.is_empty() {
            return SimulationTriangleColorCompletion::CompletedEmpty;
        }
        SimulationTriangleColorCompletion::CompletedCalls
    }
    /// Carry exclusions with every successful local plan; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationTriangleColorDiagnostic; 4] {
        [
            SimulationTriangleColorDiagnostic::NativeNotExecuted,
            SimulationTriangleColorDiagnostic::PayloadNotValidated,
            SimulationTriangleColorDiagnostic::OtherCallerPathsNotModeled,
            SimulationTriangleColorDiagnostic::SceneTraversalNotModeled,
        ]
    }
}

#[cfg(test)]
mod allocation_tests {
    #[test]
    fn capacity_failures_retain_count_class_and_cause() {
        let failure = Vec::<u8>::new().try_reserve_exact(usize::MAX).unwrap_err();
        let error = super::triangle_allocation_error(17, failure);
        assert_eq!(error.class, crate::FailureClass::Invalid);
        assert_eq!(error.operation, "triangle color call plan");
        assert!(
            error.cause.contains("17 groups") && error.cause.contains("expected available storage")
        );
    }
}
