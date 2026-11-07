use super::*;

impl SimulationResultsEdit {
    /// Borrow the immutable original parent; e.g. `edit.source().fields()`.
    pub fn source(&self) -> &SimulationResultsProperty {
        &self.source
    }

    /// Borrow the complete validated result; e.g. `edit.archive().to_bytes()`.
    pub fn archive(&self) -> &SimulationResultsProperty {
        &self.archive
    }

    /// Retain the requested original occurrence independently; e.g. `edit.parent().ordinal()`.
    pub fn parent(&self) -> SimulationResultsFieldId {
        self.parent.clone()
    }

    /// Retain the requested result occurrence independently; e.g. `edit.result().ordinal()`.
    pub fn result(&self) -> SimulationResultsFieldId {
        self.result.clone()
    }

    /// Borrow the complete supplied replacement parent; e.g. `edit.replacement()`.
    pub fn replacement(&self) -> &SimulationResultsValue {
        &self.replacement
    }

    /// Return validated status, coverage, diagnostics and achieved pattern error; e.g. `edit.coverage().completion`.
    pub fn coverage(&self) -> SimulationResultsCoverage {
        self.coverage
    }

    /// Borrow every outer occurrence relation in physical order; e.g. `edit.history()`.
    pub fn history(&self) -> &[SimulationResultsHistory] {
        &self.history
    }

    /// Borrow composed nested source/replacement relations; e.g. `edit.nested_history()`.
    pub fn nested_history(&self) -> &[SimulationResultsNestedHistory] {
        &self.nested_history
    }
}
