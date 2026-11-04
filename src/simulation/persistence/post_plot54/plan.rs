use super::super::{SimulationRgbaColor, rgba::cap_alpha_one_masked_daz_ftz_off};
use super::types::*;
use crate::{BessemerError, FailureClass, Result};

/// Immutable local color plan with original arguments, runtime facts and achieved scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPlot54Plan {
    inputs: SimulationPlot54Inputs,
    runtime: SimulationPlot54Runtime,
    stage: SimulationPlot54Stage,
    copied: Option<SimulationPlot54Colors>,
    read: Option<SimulationRgbaColor>,
    projected: Option<SimulationRgbaColor>,
}

impl SimulationPlot54Inputs {
    /// Plan local normal-flow colors with masked exceptions and DAZ/FTZ off; e.g. `inputs.plan_masked_daz_ftz_off(runtime)?`.
    pub fn plan_masked_daz_ftz_off(
        self,
        runtime: SimulationPlot54Runtime,
    ) -> Result<SimulationPlot54Plan> {
        let (stage, copied) = plot54_admission(self, runtime)?;
        let read = admitted_plot54_read(stage, copied)?;
        let projected = read.map(plot54_project_color);
        Ok(SimulationPlot54Plan {
            inputs: self,
            runtime,
            stage,
            copied,
            read,
            projected,
        })
    }
}

fn admitted_plot54_read(
    stage: SimulationPlot54Stage,
    copied: Option<SimulationPlot54Colors>,
) -> Result<Option<SimulationRgbaColor>> {
    if stage != SimulationPlot54Stage::ColorReadAdmitted {
        return Ok(None);
    }
    let pair = known_plot54_fact(
        copied,
        "copied color pair",
        "Some(complete original2AC/2BC RGBA)",
    )?;
    Ok(Some(pair.color_2ac))
}

fn known_plot54_fact<T: Copy + std::fmt::Debug>(
    value: Option<T>,
    entity: &str,
    expected: &str,
) -> Result<T> {
    value.ok_or_else(|| {
        BessemerError::new(
            "plot54 color plan",
            entity,
            FailureClass::Missing,
            format!("runtime/file fact {value:?}; expected {expected} for the reached local stage"),
        )
    })
}

fn plot54_admission(
    inputs: SimulationPlot54Inputs,
    runtime: SimulationPlot54Runtime,
) -> Result<(SimulationPlot54Stage, Option<SimulationPlot54Colors>)> {
    let mode = known_plot54_fact(
        runtime.mode_2c68_raw,
        "runtime2C68",
        "Some(raw signed32 mode)",
    )?;
    if !matches!(mode, 3 | 4) {
        return Ok((SimulationPlot54Stage::ModeBypassed, None));
    }
    let plot = known_plot54_fact(
        inputs.plot_type_raw_code,
        "parent170",
        "Some(raw signed32 plot code)",
    )?;
    if plot != 54 {
        return Ok((SimulationPlot54Stage::PlotBypassed, None));
    }
    plot54_copied_admission(inputs.colors, runtime)
}

fn plot54_copied_admission(
    pair: Option<SimulationPlot54Colors>,
    runtime: SimulationPlot54Runtime,
) -> Result<(SimulationPlot54Stage, Option<SimulationPlot54Colors>)> {
    let colors = known_plot54_fact(
        pair,
        "stored color pair",
        "Some(complete original2AC/2BC RGBA)",
    )?;
    Ok((plot54_helper_stage(runtime)?, Some(colors)))
}

fn plot54_helper_stage(runtime: SimulationPlot54Runtime) -> Result<SimulationPlot54Stage> {
    let core = known_plot54_fact(
        runtime.core_object_present,
        "core2520",
        "Some(bool core presence)",
    )?;
    if !core {
        return Ok(SimulationPlot54Stage::CoreRejected);
    }
    let callback = known_plot54_fact(
        runtime.callback_key10_present,
        "callback key10",
        "Some(bool callback presence)",
    )?;
    if !callback {
        return Ok(SimulationPlot54Stage::CallbackRejected);
    }
    Ok(SimulationPlot54Stage::ColorReadAdmitted)
}

fn plot54_project_color(color: SimulationRgbaColor) -> SimulationRgbaColor {
    let mut bits = color.bits();
    bits[3] = cap_alpha_one_masked_daz_ftz_off(bits[3]);
    SimulationRgbaColor::from_bits(bits)
}

impl SimulationPlot54Plan {
    /// Inspect canonical original file facts, including missing groups; e.g. `plan.source_inputs()`.
    pub fn source_inputs(&self) -> SimulationPlot54Inputs {
        self.inputs
    }
    /// Inspect copied explicit runtime facts; e.g. `plan.runtime_context()`.
    pub fn runtime_context(&self) -> SimulationPlot54Runtime {
        self.runtime
    }
    /// Inspect the furthest admitted local stage; e.g. `plan.stage()`.
    pub fn stage(&self) -> SimulationPlot54Stage {
        self.stage
    }
    /// Inspect both caller arguments even when the helper rejected a known null; e.g. `plan.copied_arguments()`.
    pub fn copied_arguments(&self) -> Option<SimulationPlot54Colors> {
        self.copied
    }
    /// Inspect exact2AC RGBA at the admitted read boundary; e.g. `plan.renderer_read_color()`.
    pub fn renderer_read_color(&self) -> Option<SimulationRgbaColor> {
        self.read
    }
    /// Inspect the separate post-comparison alpha projection; e.g. `plan.renderer_projected_color()`.
    pub fn renderer_projected_color(&self) -> Option<SimulationRgbaColor> {
        self.projected
    }
    /// Inspect local exact coverage and floating-point assumptions; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationPlot54Coverage {
        SimulationPlot54Coverage {
            copied_color_arguments: 2 * usize::from(self.copied.is_some()),
            renderer_components_read: 4 * usize::from(self.read.is_some()),
            local_color_only: true,
            achieved_pattern_error_bits: 0,
            exceptions_masked: true,
            daz_enabled: false,
            ftz_enabled: false,
        }
    }
    /// Distinguish completed empty local reads from admitted projections; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationPlot54Completion {
        if self.read.is_none() {
            return SimulationPlot54Completion::CompletedEmpty;
        }
        SimulationPlot54Completion::CompletedColorProjection
    }
    /// Keep native, renderer-state, callback and purpose exclusions alongside completion; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationPlot54Diagnostic; 4] {
        [
            SimulationPlot54Diagnostic::NativeNotExecuted,
            SimulationPlot54Diagnostic::RendererStateNotModeled,
            SimulationPlot54Diagnostic::CallbackGeometryUnknown,
            SimulationPlot54Diagnostic::Forwarded2bcPurposeUnknown,
        ]
    }
}
