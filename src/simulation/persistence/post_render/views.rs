use super::super::{
    SimulationPostDialog, SimulationPostStyle, load_snapshot::BinaryLoadSnapshot,
    thermal::SimulationThermalValue,
};
use super::{SimulationLegendLayer0Gate, SimulationModelTessellationInputs};
use crate::{BessemerError, FailureClass, Result};

impl SimulationPostStyle {
    /// Project stored suppression with explicit runtime plot presence; e.g. `style.legend_layer0_gate(true)?.enters_body()`.
    pub fn legend_layer0_gate(
        &self,
        plot_object_present: bool,
    ) -> Result<SimulationLegendLayer0Gate> {
        gate_from_snapshot(self, self.version().value(), plot_object_present)
    }
}

impl SimulationPostDialog {
    /// Project the embedded legend gate without inferring runtime pointers; e.g. `dialog.legend_layer0_gate(false)?`.
    pub fn legend_layer0_gate(
        &self,
        plot_object_present: bool,
    ) -> Result<SimulationLegendLayer0Gate> {
        gate_from_snapshot(self, self.version().value(), plot_object_present)
    }

    /// Extract all six stored controls or fail on missing older runtime state; e.g. `dialog.model_tessellation_inputs()?.needs_model_tessellation()`.
    pub fn model_tessellation_inputs(&self) -> Result<SimulationModelTessellationInputs> {
        Ok(SimulationModelTessellationInputs {
            style_21c: render_word(self, "style.word_21c")?,
            hidden_option: render_word(self, "style.word_1ec")?,
            dialog_518: render_word(self, "fourteen_words0c_28_30_3c_518_51c[12]")?,
            style_1f0: render_word(self, "style.word_1f0")?,
            plot_type: render_word(self, "fourteen_words0c_28_30_3c_518_51c[1]")?,
            word_18: render_word(self, "fourteen_words0c_28_30_3c_518_51c[3]")?,
        })
    }
}

fn gate_from_snapshot<T: BinaryLoadSnapshot>(
    snapshot: &T,
    version: i32,
    plot_object_present: bool,
) -> Result<SimulationLegendLayer0Gate> {
    let path = if version < 700006 {
        "v7.legend.word_a0"
    } else {
        "legend.word_a8"
    };
    Ok(SimulationLegendLayer0Gate {
        suppression_raw: render_word(snapshot, path)?,
        plot_object_present,
    })
}

pub(in crate::simulation::persistence) fn render_word<T: BinaryLoadSnapshot>(
    snapshot: &T,
    path: &str,
) -> Result<i32> {
    let value = snapshot
        .fields()
        .iter()
        .find(|field| field.path.as_ref() == path)
        .map(|field| &field.value);
    match value {
        Some(SimulationThermalValue::Signed(word)) => Ok(*word),
        Some(SimulationThermalValue::Unsigned(bits)) => Ok(*bits as i32),
        observed => Err(render_input_failure(path, observed)),
    }
}

fn render_input_failure(path: &str, observed: Option<&SimulationThermalValue>) -> BessemerError {
    let class = if observed.is_none() {
        FailureClass::Missing
    } else {
        FailureClass::Invalid
    };
    BessemerError::new(
        "post rendering policy input",
        path,
        class,
        format!(
            "{observed:?}; expected an explicitly stored raw32 control, without initialization defaults"
        ),
    )
}
