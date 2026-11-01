use super::{
    SimulationLegendPreferenceEffect, SimulationLegendPreferenceInputs,
    SimulationLegendPreferencePlan, SimulationLegendPreferencePlot,
    SimulationLegendPreferenceRoute, SimulationLegendPreferenceRuntime,
    SimulationLegendPreferenceSource,
};
use crate::{BessemerError, FailureClass, Result};

impl SimulationLegendPreferenceInputs {
    /// Derive only preference word0 while preserving original A8; e.g. `inputs.project_preference_word()?.effect()`.
    pub fn project_preference_word(self) -> Result<SimulationLegendPreferencePlan> {
        let word = require_inverse_word(
            self.stored_a8_raw,
            "legend preference zero-test export",
            "actual legend A8",
        )?;
        Ok(SimulationLegendPreferencePlan::complete(
            SimulationLegendPreferenceSource::StoredA8(self),
            SimulationLegendPreferenceRoute::ExportA8,
            SimulationLegendPreferenceEffect::PreferenceWord0(inverse_preference_word(word)),
        ))
    }
}

impl SimulationLegendPreferenceRuntime {
    /// Select only the guarded local A8 effect, not the whole setter; e.g. `runtime.project_local_a8_assignment()?.effect()`.
    pub fn project_local_a8_assignment(self) -> Result<SimulationLegendPreferencePlan> {
        let context = require_inverse_plot(self.plot_context)?;
        let route = inverse_assignment_route(context);
        let effect = inverse_assignment_effect(route, self.preference_word_1528_raw)?;
        Ok(SimulationLegendPreferencePlan::complete(
            SimulationLegendPreferenceSource::RuntimePreference(self),
            route,
            effect,
        ))
    }
}

fn require_inverse_plot(
    context: Option<SimulationLegendPreferencePlot>,
) -> Result<SimulationLegendPreferencePlot> {
    context.ok_or_else(|| {
        BessemerError::new(
            "legend local preference assignment",
            "actual plot context",
            FailureClass::Missing,
            "None; expected Some(NullPlot) or Some(PresentType(original signed32 plot type))",
        )
    })
}

fn inverse_assignment_effect(
    route: SimulationLegendPreferenceRoute,
    source: Option<i32>,
) -> Result<SimulationLegendPreferenceEffect> {
    if route == SimulationLegendPreferenceRoute::SkipPlotType {
        return Ok(SimulationLegendPreferenceEffect::NoAssignment);
    }
    let word = require_inverse_word(
        source,
        "legend local preference assignment",
        "actual preference1528",
    )?;
    Ok(SimulationLegendPreferenceEffect::LegendWordA8(
        inverse_preference_word(word),
    ))
}

fn inverse_assignment_route(
    context: SimulationLegendPreferencePlot,
) -> SimulationLegendPreferenceRoute {
    match context {
        SimulationLegendPreferencePlot::PresentType(22 | 57 | 73) => {
            SimulationLegendPreferenceRoute::SkipPlotType
        }
        SimulationLegendPreferencePlot::PresentType(70) => {
            SimulationLegendPreferenceRoute::ApplyType70
        }
        _ => SimulationLegendPreferenceRoute::ApplyOrdinary,
    }
}

fn inverse_preference_word(word: i32) -> i32 {
    i32::from(word == 0)
}

fn require_inverse_word(word: Option<i32>, operation: &'static str, entity: &str) -> Result<i32> {
    word.ok_or_else(|| BessemerError::new(operation, entity, FailureClass::Missing,
        "None; expected Some(original signed32 source word); no constructor or preference default inferred"))
}
