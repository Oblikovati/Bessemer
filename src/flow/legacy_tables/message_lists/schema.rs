/// Version-normalized stored text representation, independent of engineering interpretation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMessageTextEncoding {
    /// Version one transfers exactly N raw bytes, without interpreting their text.
    LegacyBytes,
    /// Other supported versions transfer N UTF16 units with a final NUL when nonempty.
    Utf16Units,
}
impl FlowMessageTextEncoding {
    /// Returns one stored element's byte width; e.g. `let width = text.encoding().element_width()`.
    pub fn element_width(self) -> usize {
        match self {
            Self::LegacyBytes => 1,
            Self::Utf16Units => 2,
        }
    }
}
/// Complete physical ancestry of one stored primary or list string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMessageTextRole {
    /// Primary text of one physical item.
    Primary(usize),
    /// Item and first-list local ordinals.
    FirstList(usize, usize),
    /// Item and second-list local ordinals.
    SecondList(usize, usize),
}
/// Required qualifications from exact serialized message-list acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMessageDiagnostic {
    /// Mode zero was supplied explicitly; native owner context was not reconstructed.
    NativeStringModeAssumed,
    /// Legacy raw bytes have no reproduced native text interpretation.
    LegacyStringInterpretationNotReproduced,
    /// Native runtime objects and destination capacity were not reconstructed.
    NativeRuntimeObjectsNotReconstructed,
    /// No engineering roles or enum meanings were assigned to stored words/lists.
    EngineeringRolesNotResolved,
    /// No acceptance check was performed in SolidWorks.
    NativeApplicationNotChecked,
}
