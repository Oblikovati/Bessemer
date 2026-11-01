//! Static C-878/C-880 vector model; numeric control meanings and fourth-word semantics remain qualified (#166).
mod dispatch;
mod tables;
mod views;

/// Origin of the exact four-word output, without assigning engineering meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationComponentVectorSource {
    /// DGXPOST's explicit selector/loaded-text conjunction.
    LocalFixed,
    /// A verified provider table entry.
    Table,
    /// Provider constant branch or initialized default.
    ConstantOrDefault,
}

/// Exact output and dispatch provenance of the bounded static model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationComponentVectorView {
    /// Four authoritative raw words; the fourth word's physical role is unresolved.
    pub output_bits: [u32; 4],
    /// Selected output origin.
    pub source: SimulationComponentVectorSource,
    /// Selected table RVA, retained even when an out-of-range component takes its fallback.
    pub table_rva: Option<u32>,
}

/// Explicit signed32 arguments to dgxPostCommonGetComponentVectorDir, in native order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationComponentVectorInputs {
    /// Raw analysis control A, from dialog0C.
    pub analysis: i32,
    /// Raw plot control P, from dialog10.
    pub plot_type: i32,
    /// Raw component control C, from dialog18.
    pub component: i32,
    /// Raw fourth argument D, from dialog14; zero/nonzero selection is preserved.
    pub fourth_word: i32,
}

impl SimulationComponentVectorInputs {
    /// Evaluate the complete static provider model over signed32 inputs; e.g. `inputs.component_vector()`.
    pub fn component_vector(self) -> SimulationComponentVectorView {
        dispatch::provider_view(self)
    }
}

/// Composed dialog inputs with explicit loaded CString presence rather than inferred wire text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationVectorDirectionInputs {
    /// Exact provider arguments, kept even when the local branch takes precedence.
    pub provider: SimulationComponentVectorInputs,
    /// Signed local selector at dialog570.
    pub word_570: i32,
    /// Explicit caller knowledge of loaded CString length being nonzero.
    pub loaded_text_nonempty: bool,
}

impl SimulationVectorDirectionInputs {
    /// Apply the local fixed branch before provider dispatch; e.g. `inputs.vector_direction()`.
    pub fn vector_direction(self) -> SimulationComponentVectorView {
        if self.word_570 >= 0 && self.loaded_text_nonempty {
            return SimulationComponentVectorView {
                output_bits: [0x3f800000, 0x3f800000, 0x3f800000, 0],
                source: SimulationComponentVectorSource::LocalFixed,
                table_rva: None,
            };
        }
        self.provider.component_vector()
    }
}
