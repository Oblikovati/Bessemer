use crate::{Result, simulation::SimulationPersistVersion};

/// Explicit hierarchy and reader/writer dialect; no stream basename or version inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationResultsContext {
    /// Standalone root reader; e.g. `SimulationResultsContext::RootReader(version)`.
    RootReader(SimulationPersistVersion),
    /// Standalone category reader, independent of its parent's opening gate.
    CategoryReader(SimulationPersistVersion),
    /// Standalone plot reader, preserving the original first word.
    PlotReader(SimulationPersistVersion),
    /// Documented unconditional six-field current root writer.
    RootCurrentWriter,
    /// Documented unconditional two-word current category writer.
    CategoryCurrentWriter,
}

impl SimulationResultsContext {
    pub(super) fn reader_version(self) -> Result<SimulationPersistVersion> {
        match self {
            Self::RootReader(version)
            | Self::CategoryReader(version)
            | Self::PlotReader(version) => Ok(version),
            Self::RootCurrentWriter => SimulationPersistVersion::new(890605),
            Self::CategoryCurrentWriter => SimulationPersistVersion::new(810010),
        }
    }
}
