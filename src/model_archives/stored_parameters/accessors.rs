use super::*;

impl ModelParameterName {
    /// Borrow the physical name pointer field; e.g. `name.pointer().index()`.
    pub fn pointer(&self) -> &ModelFieldHandle {
        &self.pointer
    }

    /// Borrow the complete retained object tag and target identity; e.g. `name.tag()`.
    pub fn tag(&self) -> &ModelObjectTag {
        &self.tag
    }

    /// Borrow exact stored name text, or null; e.g. `name.text().map(|text| text.text())`.
    pub fn text(&self) -> Option<&ArchiveString> {
        self.text.as_ref()
    }

    /// Borrow the text definition field, possibly in the inherited configuration; e.g. `name.text_field()`.
    pub fn text_field(&self) -> Option<&ModelFieldHandle> {
        self.text_field.as_ref()
    }
}

impl ModelParameterStoredState {
    /// Borrow the complete immutable source; e.g. `state.source()`.
    pub fn source(&self) -> &ModelParameterSource {
        &self.source
    }

    /// Copy the physical parameter class; e.g. `state.kind()`.
    pub fn kind(&self) -> ModelParameterKind {
        self.kind
    }

    /// Borrow the inherited node-name pointer and text; e.g. `state.node_name().text()`.
    pub fn node_name(&self) -> &ModelParameterName {
        &self.names[0]
    }

    /// Borrow the independent parameter-name pointer and text; e.g. `state.parameter_name().text()`.
    pub fn parameter_name(&self) -> &ModelParameterName {
        &self.names[1]
    }

    /// Borrow the exact primary value without alternate evaluation; e.g. `state.primary()`.
    pub fn primary(&self) -> &ModelParameterValue {
        &self.primary
    }

    /// Borrow a stored alternate or override, when serialized; e.g. `state.alternate()`.
    pub fn alternate(&self) -> Option<&ModelParameterValue> {
        self.alternate.as_ref()
    }

    /// Copy the full presence byte, absent for string parameters; e.g. `state.alternate_presence()`.
    pub fn alternate_presence(&self) -> Option<u8> {
        self.presence
    }

    /// Borrow the physical parameter definition selector; e.g. `source.stored_parameter(state.definition())?`.
    pub fn definition(&self) -> &ModelFieldHandle {
        &self.owner
    }

    /// Borrow copied physical field relations, including shared-name dependencies; e.g. `state.history()`.
    pub fn history(&self) -> &[ModelParameterFieldCopy] {
        &self.copies
    }

    /// Return complete unchanged-read completion; e.g. `state.completion()`.
    pub fn completion(&self) -> ModelEditCompletion {
        ModelEditCompletion::Unchanged
    }

    /// Return complete source-schema coverage; e.g. `state.coverage()`.
    pub fn coverage(&self) -> ModelArchiveCoverage {
        self.source.coverage()
    }

    /// Declare exact stored-field fidelity; e.g. `state.fidelity()`.
    pub fn fidelity(&self) -> ModelEditFidelity {
        ModelEditFidelity::ExactStoredFields
    }

    /// Return zero disagreement in projected stored values; e.g. `state.achieved_stored_value_error()`.
    pub fn achieved_stored_value_error(&self) -> usize {
        0
    }

    /// Borrow semantic exclusions for this stored operation; e.g. `state.diagnostics()`.
    pub fn diagnostics(&self) -> &[ModelParameterDiagnostic; 3] {
        &PARAMETER_DIAGNOSTICS
    }
}

const PARAMETER_DIAGNOSTICS: [ModelParameterDiagnostic; 3] = [
    ModelParameterDiagnostic::EffectiveValueAndUnitsNotEvaluated,
    ModelParameterDiagnostic::EquationsAndNativeSettersNotExecuted,
    ModelParameterDiagnostic::GeometryAndExternalDocumentsNotChecked,
];
