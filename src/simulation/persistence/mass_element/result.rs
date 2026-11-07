use super::*;

impl SimulationMassElementEdit {
    /// Borrow the immutable original parent; e.g. `edit.source().fields()`.
    pub fn source(&self) -> &SimulationMassElementProperty {
        &self.source
    }

    /// Borrow the complete validated result; e.g. `edit.archive().to_bytes()`.
    pub fn archive(&self) -> &SimulationMassElementProperty {
        &self.archive
    }

    /// Retain the requested original occurrence independently; e.g. `edit.parent().ordinal()`.
    pub fn parent(&self) -> SimulationMassElementFieldId {
        self.parent.clone()
    }

    /// Retain the requested result occurrence independently; e.g. `edit.result().ordinal()`.
    pub fn result(&self) -> SimulationMassElementFieldId {
        self.result.clone()
    }

    /// Borrow the complete supplied replacement parent; e.g. `edit.replacement()`.
    pub fn replacement(&self) -> &SimulationMassElementValue {
        &self.replacement
    }

    /// Return validated status, coverage, diagnostics and achieved pattern error; e.g. `edit.coverage().completion`.
    pub fn coverage(&self) -> SimulationMassElementCoverage {
        self.coverage
    }

    /// Borrow every outer occurrence relation in physical order; e.g. `edit.history()`.
    pub fn history(&self) -> &[SimulationMassElementHistory] {
        &self.history
    }

    /// Borrow composed nested source/replacement relations; e.g. `edit.nested_history()`.
    pub fn nested_history(&self) -> &[SimulationMassElementNestedHistory] {
        &self.nested_history
    }
}
