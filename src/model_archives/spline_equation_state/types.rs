use crate::archive_codec::ArchiveString;
use crate::model_archives::{ModelFieldHandle, PartResolvedFeaturesArchive};

/// Saved CString slots bound to the selected equation getter, without API conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelSplineEquationTextRole {
    /// Saved1B0, associated with XExpression.
    XExpression,
    /// Saved1B8, associated with YExpression.
    YExpression,
    /// Saved1C0, associated with ZExpression.
    ZExpression,
    /// Saved1C8 declared-character text, not an API getter output.
    DeclaredCharacters,
    /// Saved1D0 range-start text, without expression/double conversion.
    RangeStart,
    /// Saved1D8 range-end text, without expression/double conversion.
    RangeEnd,
}

impl ModelSplineEquationTextRole {
    pub(super) fn slot(self) -> usize {
        match self {
            Self::XExpression => 0,
            Self::YExpression => 1,
            Self::ZExpression => 2,
            Self::DeclaredCharacters => 3,
            Self::RangeStart => 4,
            Self::RangeEnd => 5,
        }
    }
}

/// Exact saved-value request; no native setter cascade or effective geometry operation.
#[derive(Debug, Clone)]
pub enum ModelSplineEquationStoredEdit {
    /// Replace only32C mask0x10, preserving every other stored bit.
    LockStart(bool),
    /// Replace only32C mask0x20, preserving every other stored bit.
    LockEnd(bool),
    /// Replace the full signed saved1E0 option without Boolean canonicalization.
    SavedOption(i32),
    /// Replace one complete CString with its explicit encoding/framing.
    Text {
        /// Selected saved slot, independent of converted BSTR request ordering.
        role: ModelSplineEquationTextRole,
        /// Complete admitted CString value.
        value: ArchiveString,
    },
}

/// Copied physical field relation for the unchanged stored-state projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSplineEquationFieldCopy {
    /// Every source parent; this projection copies one physical occurrence.
    pub parents: [ModelFieldHandle; 1],
    /// Same-revision result occurrence.
    pub result: ModelFieldHandle,
}

/// Semantic exclusions carried with each complete stored-state projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelSplineEquationDiagnostic {
    /// Saved expressions and range strings were not evaluated or converted to API outputs.
    ExpressionsAndApiConversionsNotEvaluated,
    /// No native setter, update trigger, resolver or provider was executed.
    NativeSetterAndProvidersNotExecuted,
    /// Geometry, physical units and external documents remain outside the operation.
    GeometryUnitsAndExternalDocumentsNotChecked,
}

/// Immutable equation fields, complete source, coverage and copied physical provenance.
#[derive(Debug, Clone)]
pub struct ModelSplineEquationStoredState {
    pub(super) source: PartResolvedFeaturesArchive,
    pub(super) flags: u32,
    pub(super) strings: [ArchiveString; 6],
    pub(super) compatibility: i32,
    pub(super) option: i32,
    pub(super) fields: [ModelFieldHandle; 9],
    pub(super) copies: [ModelSplineEquationFieldCopy; 9],
}
