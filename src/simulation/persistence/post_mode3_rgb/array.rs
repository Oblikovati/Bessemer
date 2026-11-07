use super::{
    SimulationMode3RgbPlan, SimulationMode3RgbTriples, mode3_failure, mode3_required, types::*,
};
use crate::{FailureClass, Result};

/// Immutable isolated fixed-slot update with both color and destination parents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationMode3RgbUpdate {
    plan: SimulationMode3RgbPlan,
    context: SimulationMode3RgbArrayContext,
    destination: Option<SimulationMode3RgbTriples>,
    history: Option<SimulationMode3RgbWriteHistory>,
}
impl SimulationMode3RgbPlan {
    /// Apply an admitted request to a copied explicit destination; e.g. `plan.apply_local_array(&context)?`.
    pub fn apply_local_array(
        &self,
        context: &SimulationMode3RgbArrayContext,
    ) -> Result<SimulationMode3RgbUpdate> {
        let Some(rgb) = self.array_request_rgb() else {
            return Ok(mode3_empty_update(self, context));
        };
        let (parent, source) = mode3_array_parent(context)?;
        let (destination, before) = source.patched(rgb)?;
        Ok(SimulationMode3RgbUpdate {
            plan: self.clone(),
            context: context.clone(),
            destination: Some(destination),
            history: Some(SimulationMode3RgbWriteHistory {
                parent,
                index: 5,
                before,
                after: rgb,
            }),
        })
    }
}
fn mode3_array_parent(
    context: &SimulationMode3RgbArrayContext,
) -> Result<(SimulationMode3RgbArrayParent, &SimulationMode3RgbTriples)> {
    let capacity = mode3_required(
        context.capacity_raw,
        "array capacity",
        "Some(raw signed32 capacity)",
    )?;
    if capacity > 5 {
        let source = mode3_required(
            context.existing.as_ref(),
            "existing destination",
            "Some(explicit complete destination prefix)",
        )?;
        return Ok((SimulationMode3RgbArrayParent::Existing, source));
    }
    mode3_grown_parent(context)
}
impl SimulationMode3RgbUpdate {
    /// Inspect retained selected-color parents and every reached stage; e.g. `update.source_plan()`.
    pub fn source_plan(&self) -> &SimulationMode3RgbPlan {
        &self.plan
    }
    /// Inspect both original destination facts, including unused/unknown values; e.g. `update.source_context()`.
    pub fn source_context(&self) -> &SimulationMode3RgbArrayContext {
        &self.context
    }
    /// Read the isolated updated destination; e.g. `update.destination()`.
    pub fn destination(&self) -> Option<&SimulationMode3RgbTriples> {
        self.destination.as_ref()
    }
    /// Inspect the exact selected destination parent and changed cell; e.g. `update.history()`.
    pub fn history(&self) -> Option<SimulationMode3RgbWriteHistory> {
        self.history
    }
    /// Distinguish completed empty from a bounded exact write; e.g. `update.completion()`.
    pub fn completion(&self) -> SimulationMode3RgbCompletion {
        if self.destination.is_some() {
            return SimulationMode3RgbCompletion::CompletedArrayWrite;
        }
        SimulationMode3RgbCompletion::CompletedEmpty
    }
    /// Inspect local exact copy/bounds coverage; e.g. `update.coverage()`.
    pub fn coverage(&self) -> SimulationMode3RgbCoverage {
        let mut coverage = self.plan.coverage();
        coverage.written_words = 3 * usize::from(self.destination.is_some());
        coverage.destination_bounds_checked = self.destination.is_some();
        coverage
    }
    /// Retain native allocator and whole-scene exclusions; e.g. `update.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationMode3RgbDiagnostic; 4] {
        self.plan.diagnostics()
    }
}

fn mode3_empty_update(
    plan: &SimulationMode3RgbPlan,
    context: &SimulationMode3RgbArrayContext,
) -> SimulationMode3RgbUpdate {
    SimulationMode3RgbUpdate {
        plan: plan.clone(),
        context: context.clone(),
        destination: None,
        history: None,
    }
}
fn mode3_grown_parent(
    context: &SimulationMode3RgbArrayContext,
) -> Result<(SimulationMode3RgbArrayParent, &SimulationMode3RgbTriples)> {
    let result = mode3_required(
        context.growth_result_raw,
        "growth result",
        "Some(raw signed32 external growth result)",
    )?;
    if result != 0 {
        return Err(mode3_failure(
            "growth result",
            FailureClass::Invalid,
            format!(
                "external growth result {result}; expected exactly0 before the local copy (native status4 path otherwise)"
            ),
        ));
    }
    let source = mode3_required(
        context.after_growth.as_ref(),
        "grown destination",
        "Some(explicit post-growth destination prefix)",
    )?;
    Ok((SimulationMode3RgbArrayParent::AfterGrowth, source))
}
