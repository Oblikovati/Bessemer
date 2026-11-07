use super::{
    SimulationLegendFormatInputs, SimulationLegendFormatPlan, SimulationLegendFormatRuntime,
    SimulationLegendFormatSelection, constructed_format, required_format_word,
};
use crate::{BessemerError, FailureClass, Result};

impl SimulationLegendFormatInputs {
    /// Construct the bounded primary string without rendering numeric values; e.g. `inputs.primary_format()?.format_string()`.
    pub fn primary_format(self) -> Result<SimulationLegendFormatPlan> {
        let mode = required_format_word(self.mode_raw, "legend.word_b8")?;
        let precision = required_format_word(self.precision_raw, "legend.word_bc")?;
        Ok(SimulationLegendFormatPlan::complete(
            self,
            None,
            SimulationLegendFormatSelection::Primary,
            Some(constructed_format(mode, precision)),
            3,
        ))
    }

    /// Select the native alternate branch under explicit FP assumptions; e.g. `inputs.alternate_format_masked_exceptions_daz_off(runtime)?`.
    pub fn alternate_format_masked_exceptions_daz_off(
        self,
        runtime: SimulationLegendFormatRuntime,
    ) -> Result<SimulationLegendFormatPlan> {
        if let Some((selection, mask)) = self.alternate_bypass(runtime)? {
            return Ok(SimulationLegendFormatPlan::complete(
                self,
                Some(runtime),
                selection,
                None,
                mask,
            ));
        }
        let selector = required_format_word(self.alternate_selector_raw, "legend.word_c8")?;
        let precision = required_format_word(self.precision_raw, "legend.word_bc")?;
        let mode = if selector == 0 { 1 } else { 2 };
        Ok(SimulationLegendFormatPlan::complete(
            self,
            Some(runtime),
            SimulationLegendFormatSelection::Alternate,
            Some(constructed_format(mode, precision)),
            15,
        ))
    }

    fn alternate_bypass(
        self,
        runtime: SimulationLegendFormatRuntime,
    ) -> Result<Option<(SimulationLegendFormatSelection, u8)>> {
        use SimulationLegendFormatSelection::{AlternateDisabled, ModeNotZero, OutputPointerNull};
        if !required_format_pointer(runtime.output_pointer_present)? {
            return Ok(Some((OutputPointerNull, 0)));
        }
        if required_format_word(self.mode_raw, "legend.word_b8")? != 0 {
            return Ok(Some((ModeNotZero, 1)));
        }
        if required_format_word(self.alternate_enabled_raw, "legend.word_c4")? == 0 {
            return Ok(Some((AlternateDisabled, 5)));
        }
        alternate_magnitude_bypass(runtime.value_bits)
    }
}

fn required_format_pointer(pointer: Option<bool>) -> Result<bool> {
    pointer.ok_or_else(|| BessemerError::new(
        "legend number format", "output string pointer", FailureClass::Missing,
        "None; expected Some(true) for a present output pointer or Some(false) for a null pointer",
    ))
}

fn alternate_magnitude_bypass(
    value_bits: Option<u32>,
) -> Result<Option<(SimulationLegendFormatSelection, u8)>> {
    let bits = value_bits.ok_or_else(|| {
        BessemerError::new(
            "legend number format",
            "input value",
            FailureClass::Missing,
            "None; expected Some(raw binary32 input bits) after the reached enable guard",
        )
    })?;
    // These are the first/last positive binary32 values within [binary64 0.001, binary32 1000].
    // Integer ordering rejects both NaN classes without depending on this process's FP state.
    let magnitude = bits & 0x7fff_ffff;
    if (0x3a83_126f..=0x447a_0000).contains(&magnitude) {
        return Ok(None);
    }
    Ok(Some((
        SimulationLegendFormatSelection::MagnitudeOutsideInterval,
        5,
    )))
}
