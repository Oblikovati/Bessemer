use super::{
    SimulationLegendPaletteDispatch, SimulationLegendPaletteInputs, SimulationLegendPalettePlan,
    SimulationLegendPaletteRuntime, required_palette_word,
};
use crate::{BessemerError, FailureClass, Result};

impl SimulationLegendPaletteInputs {
    /// Select only documented dispatcher control flow from explicit facts; e.g. `inputs.dispatch_palette(runtime)?`.
    pub fn dispatch_palette(
        self,
        runtime: SimulationLegendPaletteRuntime,
    ) -> Result<SimulationLegendPalettePlan> {
        let gray = required_palette_word(self.gray_above_yield_raw, "legend.word_d4")?;
        if gray != 0
            && required_palette_word(runtime.gray_helper_return_raw, "gray helper return")? != 0
        {
            return Ok(SimulationLegendPalettePlan::complete(
                self,
                runtime,
                SimulationLegendPaletteDispatch::ReturnedAfterGrayHelper,
            ));
        }
        let mode = required_palette_word(self.mapping_mode_raw, "legend.word_8")?;
        let dispatch = if mode == 4 {
            shell_palette_dispatch(runtime)?
        } else {
            SimulationLegendPaletteDispatch::RegularMapper
        };
        Ok(SimulationLegendPalettePlan::complete(
            self, runtime, dispatch,
        ))
    }
}

fn shell_palette_dispatch(
    runtime: SimulationLegendPaletteRuntime,
) -> Result<SimulationLegendPaletteDispatch> {
    require_palette_plot_pointer(runtime.plot_pointer_present)?;
    let word = required_palette_word(runtime.plot_word18_raw, "plot.word_18")?;
    if word == 3 {
        return Ok(SimulationLegendPaletteDispatch::ShellMaterialMapper);
    }
    Ok(SimulationLegendPaletteDispatch::ShellThicknessMapper)
}

fn require_palette_plot_pointer(present: Option<bool>) -> Result<()> {
    match present {
        Some(true) => Ok(()),
        Some(false) => Err(BessemerError::new(
            "legend palette dispatch",
            "plot pointer",
            FailureClass::Unsupported,
            "Some(false); expected a known present plot pointer for the mode4 dereference; null execution is outside this local projection",
        )),
        None => Err(BessemerError::new(
            "legend palette dispatch",
            "plot pointer",
            FailureClass::Missing,
            "None; expected Some(true) for the reached mode4 plot dereference",
        )),
    }
}
