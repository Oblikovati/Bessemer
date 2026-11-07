use super::{Result, SimulationGuidArchive, SimulationGuidCoverage, SimulationGuidFieldId};

/// One stored-field derivation, without inventing a semantic GUID identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationGuidFieldHistory {
    /// Original frame identity.
    pub parent: SimulationGuidFieldId,
    /// Derived corresponding frame identity.
    pub result: SimulationGuidFieldId,
    /// Whether its complete stored content/null state changed.
    pub replaced: bool,
}

/// Required acceptance limits beyond known string framing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationGuidDiagnostic {
    /// All four roles and any GUID syntax/identity meaning remain unassigned.
    FieldRolesUnassigned,
    /// No native allocation, COM stream operation or failure path was executed.
    NativeRuntimeNotExecuted,
    /// The native application did not load this output.
    NativeApplicationNotChecked,
}

/// Isolated complete stored-field replacement; any unknown tail remains explicitly incomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationGuidFidelity {
    /// Every other stored frame and all tail bytes remain exact; frame offsets are derived from the replaced content.
    ExactOtherFramesAndTail,
}

/// Isolated stored-field replacement with explicit whole-stream coverage status.
#[derive(Debug, Clone)]
pub struct SimulationGuidEdit {
    /// Immutable preserved result, including original opaque tail.
    pub archive: SimulationGuidArchive,
    /// Byte-change completion, separate from whole-stream coverage completion.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Exact stored-byte preservation scope.
    pub fidelity: SimulationGuidFidelity,
    /// Every original field's derivation in stored order.
    pub history: [SimulationGuidFieldHistory; 4],
    /// Known-frame coverage and explicit incomplete-tail state.
    pub coverage: SimulationGuidCoverage,
    /// Requested UTF-16 pattern error, zero; no semantic identity error was established.
    pub requested_pattern_error_bits: u32,
    /// Required unchecked responsibilities.
    pub diagnostics: [SimulationGuidDiagnostic; 3],
}

impl SimulationGuidArchive {
    /// Replace complete stored units/null state while preserving every other field and tail, e.g. `archive.edit_field(&id, Some(&[0xd800, 0]))?`.
    pub fn edit_field(
        &self,
        id: &SimulationGuidFieldId,
        units: Option<&[u16]>,
    ) -> Result<SimulationGuidEdit> {
        let field = self.field(id)?;
        let archive = if field.stored_units() == units {
            self.clone()
        } else {
            self.replaced_field(field.index, units)?
        };
        Ok(edit_result(self, archive))
    }

    fn replaced_field(&self, index: usize, units: Option<&[u16]>) -> Result<Self> {
        super::codec::field_length(units)?;
        let mut parts = self.parts();
        parts.fields[index] = units.map(Into::into);
        Self::from_parts(parts, self.maximum)
    }

    /// Explicit unassigned-role and runtime exclusions, e.g. `archive.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationGuidDiagnostic; 3] {
        [
            SimulationGuidDiagnostic::FieldRolesUnassigned,
            SimulationGuidDiagnostic::NativeRuntimeNotExecuted,
            SimulationGuidDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

fn edit_result(
    parent: &SimulationGuidArchive,
    archive: SimulationGuidArchive,
) -> SimulationGuidEdit {
    let history = std::array::from_fn(|index| SimulationGuidFieldHistory {
        parent: parent.fields[index].id.clone(),
        result: archive.fields[index].id.clone(),
        replaced: parent.fields[index].stored_units() != archive.fields[index].stored_units(),
    });
    SimulationGuidEdit {
        fidelity: SimulationGuidFidelity::ExactOtherFramesAndTail,
        completion: if parent.revision == archive.revision {
            crate::display_lists::DisplayEditCompletion::Unchanged
        } else {
            crate::display_lists::DisplayEditCompletion::Changed
        },
        coverage: archive.coverage(),
        diagnostics: archive.diagnostics(),
        history,
        requested_pattern_error_bits: 0,
        archive,
    }
}
