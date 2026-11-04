use super::super::SimulationRgbaColor;
use super::types::*;
use crate::{BessemerError, FailureClass, Result};

/// Immutable local read with original file/runtime provenance and explicit exclusions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationSegmentColorPlan {
    inputs: SimulationSegmentColorInputs,
    runtime: SimulationSegmentColorRuntime,
    stage: SimulationSegmentColorStage,
    read: Option<[u32; 3]>,
}

impl SimulationSegmentColorInputs {
    /// Project the guarded normal-flow RGB read; e.g. `inputs.plan_rgb_read(runtime)?`.
    pub fn plan_rgb_read(
        self,
        runtime: SimulationSegmentColorRuntime,
    ) -> Result<SimulationSegmentColorPlan> {
        let stage = segment_color_stage(runtime)?;
        let read = segment_color_read(stage, self.color_1bc)?;
        Ok(SimulationSegmentColorPlan {
            inputs: self,
            runtime,
            stage,
            read,
        })
    }
}

fn segment_required_fact<T: Copy + std::fmt::Debug>(
    value: Option<T>,
    entity: &str,
    expected: &str,
) -> Result<T> {
    value.ok_or_else(|| {
        BessemerError::new(
            "element-segment RGB read",
            entity,
            FailureClass::Missing,
            format!("fact {value:?}; expected {expected} at the reached local stage"),
        )
    })
}

fn segment_color_stage(
    runtime: SimulationSegmentColorRuntime,
) -> Result<SimulationSegmentColorStage> {
    let mode = segment_required_fact(
        runtime.mode_2c68_raw,
        "runtime2C68",
        "Some(raw signed32 mode)",
    )?;
    if !matches!(mode, 3 | 4) {
        return Ok(SimulationSegmentColorStage::ModeBypassed);
    }
    segment_index_stage(runtime)
}

fn segment_index_stage(
    runtime: SimulationSegmentColorRuntime,
) -> Result<SimulationSegmentColorStage> {
    let first = segment_required_fact(
        runtime.first_element_274c,
        "first274C",
        "Some(raw signed32 first index)",
    )?;
    if first <= 0 {
        return Ok(SimulationSegmentColorStage::FirstIndexRejected);
    }
    let last = segment_required_fact(
        runtime.last_element_2750,
        "last2750",
        "Some(raw signed32 last index)",
    )?;
    if last <= 0 {
        return Ok(SimulationSegmentColorStage::LastIndexRejected);
    }
    segment_ordered_range(first, last, runtime.flags_da4_raw)
}

fn segment_ordered_range(
    first: i32,
    last: i32,
    flags: Option<u32>,
) -> Result<SimulationSegmentColorStage> {
    if first > last {
        return Ok(SimulationSegmentColorStage::RangeRejected);
    }
    segment_flags_stage(flags)
}

fn segment_flags_stage(flags: Option<u32>) -> Result<SimulationSegmentColorStage> {
    let flags = segment_required_fact(flags, "flagsDA4", "Some(raw unsigned32 flags)")?;
    if flags != 0 && flags & 4 == 0 {
        return Ok(SimulationSegmentColorStage::FlagsRejected);
    }
    Ok(SimulationSegmentColorStage::RgbReadAdmitted)
}

fn segment_color_read(
    stage: SimulationSegmentColorStage,
    color: Option<SimulationRgbaColor>,
) -> Result<Option<[u32; 3]>> {
    if stage != SimulationSegmentColorStage::RgbReadAdmitted {
        return Ok(None);
    }
    let bits = segment_required_fact(color, "stored1BC", "Some(complete original RGBA)")?.bits();
    Ok(Some([bits[0], bits[1], bits[2]]))
}

impl SimulationSegmentColorPlan {
    /// Inspect canonical original storage, including older absence; e.g. `plan.source_inputs()`.
    pub fn source_inputs(&self) -> SimulationSegmentColorInputs {
        self.inputs
    }
    /// Inspect copied runtime facts, without inferring invocation; e.g. `plan.runtime_context()`.
    pub fn runtime_context(&self) -> SimulationSegmentColorRuntime {
        self.runtime
    }
    /// Inspect the furthest local caller stage; e.g. `plan.stage()`.
    pub fn stage(&self) -> SimulationSegmentColorStage {
        self.stage
    }
    /// Inspect three exact raw words, preserving all float patterns; e.g. `plan.rgb_bits()`.
    pub fn rgb_bits(&self) -> Option<[u32; 3]> {
        self.read
    }
    /// Inspect achieved local coverage; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationSegmentColorCoverage {
        SimulationSegmentColorCoverage {
            components_read: 3 * usize::from(self.read.is_some()),
            geometry_validated: false,
            achieved_pattern_error_bits: 0,
        }
    }
    /// Inspect completed empty versus completed local read; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationSegmentColorCompletion {
        if self.read.is_some() {
            return SimulationSegmentColorCompletion::CompletedRgbRead;
        }
        SimulationSegmentColorCompletion::CompletedEmpty
    }
    /// Retain unresolved scope even on bypassed callers; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationSegmentColorDiagnostic; 4] {
        [
            SimulationSegmentColorDiagnostic::NativeNotExecuted,
            SimulationSegmentColorDiagnostic::PreReadStateUnmodeled,
            SimulationSegmentColorDiagnostic::EndpointGeometryUnknown,
            SimulationSegmentColorDiagnostic::StoredAlphaPurposeUnknown,
        ]
    }
}
