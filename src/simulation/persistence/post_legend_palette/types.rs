use super::super::SimulationThermalRelation;

/// Exact signed controls; older absent words remain absent without initialization defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPaletteInputs {
    /// Raw object8; exactly4 reaches plot-dependent shell dispatch.
    pub mapping_mode_raw: Option<i32>,
    /// Raw objectC; every nonzero value reverses the consumer's base copy order.
    pub reverse_base_palette_raw: Option<i32>,
    /// Raw object10; it never resizes the fixed serialized color area.
    pub base_palette_count_raw: Option<i32>,
    /// Raw objectD4; every nonzero value reaches the gray helper first.
    pub gray_above_yield_raw: Option<i32>,
}

impl SimulationLegendPaletteInputs {
    /// Inspect the raw zero/nonzero reversal view without rewriting it; e.g. `inputs.reverse_base_palette()`.
    pub fn reverse_base_palette(self) -> Option<bool> {
        self.reverse_base_palette_raw.map(|word| word != 0)
    }

    /// Inspect the enable test without asserting gray output; e.g. `inputs.gray_above_yield_enabled()`.
    pub fn gray_above_yield_enabled(self) -> Option<bool> {
        self.gray_above_yield_raw.map(|word| word != 0)
    }

    /// Compare with nine serialized slots, without native validity claims; e.g. `inputs.base_count_within_nine_slots()`.
    pub fn base_count_within_nine_slots(self) -> Option<bool> {
        self.base_palette_count_raw
            .map(|count| (0..=9).contains(&count))
    }
}

/// Explicit local call facts; unconsumed supplied values remain retained parents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPaletteRuntime {
    /// Return value of the reached gray helper, not an inferred output color.
    pub gray_helper_return_raw: Option<i32>,
    /// None is unknown; Some(false) cannot supply the mode4 dereference.
    pub plot_pointer_present: Option<bool>,
    /// Raw signed plot word18, required only through a known present mode4 pointer.
    pub plot_word18_raw: Option<i32>,
}

/// Completed local dispatcher choice; destination algorithms are not executed or emulated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPaletteDispatch {
    /// The supplied nonzero gray-helper return causes the dispatcher to return.
    ReturnedAfterGrayHelper,
    /// A mode other than4 selects the regular mapper call.
    RegularMapper,
    /// Mode4 with runtime plot word18 equal3 selects the material tail target.
    ShellMaterialMapper,
    /// Other known plot word18 values select the thickness tail target.
    ShellThicknessMapper,
}

/// Local selection completion remains distinct from completion of a mapper algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPaletteCompletion {
    /// Completed the local early-return selection.
    CompletedEarlyReturnSelection,
    /// Completed selection of the regular mapper call.
    CompletedRegularSelection,
    /// Completed selection of one shell tail target.
    CompletedTailSelection,
}

/// Complete local dependency coverage, without any mapped-color validation claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPaletteCoverage {
    /// Bits0..3 identify mapping mode, reversal, base count and gray flag reads.
    pub consumed_member_mask: u8,
    /// Whether the supplied gray-helper return was reached and consumed.
    pub gray_helper_result_read: bool,
    /// Whether runtime pointer presence was reached and consumed.
    pub plot_pointer_read: bool,
    /// Whether plot word18 was reached and consumed.
    pub plot_word_read: bool,
    /// True; every reached integer equality/zero test was checked exactly.
    pub exact_local_dispatch: bool,
    /// False; neither a raw count nor the slot diagnostic proves native count safety.
    pub native_count_validated: bool,
    /// False; no destination mapper or its output is modeled here.
    pub selected_mapper_modeled: bool,
    /// False; the projection never edits wire bytes or resizes the color area.
    pub wire_edited: bool,
    /// Exact local selection has zero achieved pattern error.
    pub achieved_pattern_error_bits: u32,
}

/// All supplied source parents survive the local derivation, including unused facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPaletteHistory {
    /// Original four optional raw controls.
    pub inputs: SimulationLegendPaletteInputs,
    /// Original supplied runtime facts, without filling unknown values.
    pub runtime: SimulationLegendPaletteRuntime,
    /// Derived local dispatcher choice with all parents retained.
    pub relation: SimulationThermalRelation,
}

/// Unresolved native/runtime boundaries accompany every completed local choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPaletteDiagnostic {
    /// SolidWorks target code was not executed.
    NativeNotExecuted,
    /// Interpolation and all regular/gray/shell destination algorithms remain unmodeled.
    MappingAlgorithmsNotModeled,
    /// Reaching a gray helper does not establish actual gray color output.
    GrayOutputNotEstablished,
    /// Actual runtime plot creation and helper invocation are not inferred.
    RuntimeProductionNotInferred,
    /// The0..9 slot diagnostic is not native runtime count validation.
    NativeCountValidityUnknown,
}
