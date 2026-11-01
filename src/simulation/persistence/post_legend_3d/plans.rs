use super::super::{SimulationLegendHeightInputs, SimulationThermalRelation};
use super::{
    SimulationLegend3DBackgroundHistory, SimulationLegend3DBackgroundRuntime,
    SimulationLegend3DBackgroundValues, SimulationLegend3DBodyHistory,
    SimulationLegend3DBodyRuntime, SimulationLegend3DBodyValues, SimulationLegend3DCompletion,
    SimulationLegend3DCoverage, SimulationLegend3DDiagnostic,
};

/// Immutable local body requests without allocated buffers or rendered output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegend3DBodyPlan {
    history: SimulationLegend3DBodyHistory,
    values: SimulationLegend3DBodyValues,
}

impl SimulationLegend3DBodyPlan {
    pub(super) fn complete(
        inputs: SimulationLegendHeightInputs,
        runtime: SimulationLegend3DBodyRuntime,
        values: SimulationLegend3DBodyValues,
    ) -> Self {
        Self {
            history: SimulationLegend3DBodyHistory {
                inputs,
                runtime,
                relation: SimulationThermalRelation::Derived,
            },
            values,
        }
    }

    /// Inspect the entry snapshot and actual later facts; e.g. `plan.history().runtime`.
    pub fn history(&self) -> SimulationLegend3DBodyHistory {
        self.history
    }

    /// Inspect raw requests without native safety claims; e.g. `plan.values().mapping_request_count_raw`.
    pub fn values(&self) -> SimulationLegend3DBodyValues {
        self.values
    }

    /// Distinguish completed zero/nonzero mapping requests; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegend3DCompletion {
        if self.values.mapping_request_count_raw == 0 {
            return SimulationLegend3DCompletion::CompletedZeroMappingRequest;
        }
        SimulationLegend3DCompletion::CompletedNonzeroMappingRequest
    }

    /// Inspect exact integer coverage and renderer exclusions; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationLegend3DCoverage {
        local_3d_coverage(1, 2, true, false)
    }

    /// Retain explicit later-count and rendering qualifications; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegend3DDiagnostic; 5] {
        use SimulationLegend3DDiagnostic::*;
        [
            NativeNotExecuted,
            AllocationAndGeometryNotValidated,
            RendererHelpersNotModeled,
            LaterFactsExplicit,
            StoredArgumentIdentityNotInferred,
        ]
    }
}

/// Immutable local constructor/factor/lighting projection without background geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegend3DBackgroundPlan {
    history: SimulationLegend3DBackgroundHistory,
    values: SimulationLegend3DBackgroundValues,
}

impl SimulationLegend3DBackgroundPlan {
    pub(super) fn complete(
        runtime: SimulationLegend3DBackgroundRuntime,
        values: SimulationLegend3DBackgroundValues,
    ) -> Self {
        Self {
            history: SimulationLegend3DBackgroundHistory {
                runtime,
                relation: SimulationThermalRelation::Derived,
            },
            values,
        }
    }

    /// Inspect every original supplied fact; e.g. `plan.history().runtime.precision_at_width_raw`.
    pub fn history(&self) -> SimulationLegend3DBackgroundHistory {
        self.history
    }

    /// Inspect the constructed color and raw width/lighting decisions; e.g. `plan.values()`.
    pub fn values(&self) -> SimulationLegend3DBackgroundValues {
        self.values
    }

    /// Inspect complete local background-input evaluation; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegend3DCompletion {
        SimulationLegend3DCompletion::CompletedBackgroundInputs
    }

    /// Inspect exact local coverage without floating-dimension claims; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationLegend3DCoverage {
        local_3d_coverage(0, 3, false, true)
    }

    /// Retain packed-argument, float-getter and GL-state boundaries; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegend3DDiagnostic; 5] {
        use SimulationLegend3DDiagnostic::*;
        [
            NativeNotExecuted,
            LaterFactsExplicit,
            StoredArgumentIdentityNotInferred,
            BackgroundFloatingOutputNotModeled,
            CompleteGlStateNotEstablished,
        ]
    }
}

fn local_3d_coverage(
    stored: usize,
    runtime: usize,
    allocations: bool,
    constructor: bool,
) -> SimulationLegend3DCoverage {
    SimulationLegend3DCoverage {
        stored_snapshot_words_read: stored,
        runtime_scalars_read: runtime,
        exact_integer_patterns: true,
        allocation_requests_computed: allocations,
        nearest_even_constructor: constructor,
        actual_allocation_performed: false,
        renderer_modeled: false,
        pointer_geometry_validated: false,
        achieved_pattern_error_bits: 0,
    }
}
