//! Bounded named renderer decisions over explicit inputs, specification 14.490–14.491 (#166).
mod views;

/// Inputs for the named Layer0_RenderBody entry conjunction; this does not prove drawing occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendLayer0Gate {
    /// Original signed A8 suppression word; nonzero values are not normalized.
    pub suppression_raw: i32,
    /// Explicit caller knowledge of the runtime plot pointer.
    pub plot_object_present: bool,
}

impl SimulationLegendLayer0Gate {
    /// Model only the named body-entry gate; e.g. `gate.enters_body()`.
    pub fn enters_body(self) -> bool {
        self.plot_object_present && self.suppression_raw == 0
    }
}

/// Explicit loaded words for NeedModelTess, without inferred initialization or actual mesh generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationModelTessellationInputs {
    /// Stored PostStyle override at 0x21c.
    pub style_21c: i32,
    /// Named hidden-parts option at style 0x1ec.
    pub hidden_option: i32,
    /// Unresolved dialog control at 0x518.
    pub dialog_518: i32,
    /// Stored style control interpreted as unsigned for the bounded 0/1 test.
    pub style_1f0: i32,
    /// Raw numeric plot type at dialog 0x10.
    pub plot_type: i32,
    /// Raw dialog word at 0x18; numeric branch meaning remains qualified.
    pub word_18: i32,
}

impl SimulationModelTessellationInputs {
    /// Reproduce the named request predicate over supplied signed32 words; e.g. `inputs.needs_model_tessellation()`.
    pub fn needs_model_tessellation(self) -> bool {
        if self.style_21c != 0 {
            return true;
        }
        if self.hidden_option != 0 && self.dialog_518 == 0 && self.style_1f0 as u32 <= 1 {
            return true;
        }
        matches!(self.plot_type, 4 | 65) || self.plot_type == 53 && self.word_18 == 2
    }
}
