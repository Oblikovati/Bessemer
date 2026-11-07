use super::*;

impl SimulationThermalEdit {
    /// Borrow the immutable original parent; e.g. `edit.source().fields()`.
    pub fn source(&self) -> &SimulationThermalCollection {
        &self.source
    }

    /// Borrow the complete validated result; e.g. `edit.archive().to_bytes()`.
    pub fn archive(&self) -> &SimulationThermalCollection {
        &self.archive
    }

    /// Retain the requested original occurrence independently; e.g. `edit.parent().ordinal()`.
    pub fn parent(&self) -> SimulationThermalFieldId {
        self.parent.clone()
    }

    /// Retain the requested result occurrence independently; e.g. `edit.result().ordinal()`.
    pub fn result(&self) -> SimulationThermalFieldId {
        self.result.clone()
    }

    /// Borrow the complete supplied replacement parent; e.g. `edit.replacement()`.
    pub fn replacement(&self) -> &SimulationThermalValue {
        &self.replacement
    }

    /// Return validated status, coverage, diagnostics and achieved pattern error; e.g. `edit.coverage().completion`.
    pub fn coverage(&self) -> SimulationThermalCoverage {
        self.coverage
    }

    /// Borrow every outer occurrence relation in physical order; e.g. `edit.history()`.
    pub fn history(&self) -> &[SimulationThermalHistory] {
        &self.history
    }

    /// Borrow composed nested source/replacement relations; e.g. `edit.nested_history()`.
    pub fn nested_history(&self) -> &[SimulationThermalNestedHistory] {
        &self.nested_history
    }
}
