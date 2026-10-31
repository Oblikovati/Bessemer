use super::{
    Result, SimulationElfInitialArchive, SimulationElfInitialEdit, SimulationElfParameterId,
    require,
};
use crate::compound::{CompoundEdit, CompoundLimits};
use crate::simulation::SimulationStudy;

/// Complete CWR with a validated complete native ELF initial image.
#[derive(Debug, Clone)]
pub struct SimulationElfInitialStudy {
    paired: SimulationStudy,
    initial: SimulationElfInitialArchive,
}

/// Initializer edit composed with all carrier derivations and preservation checks.
#[derive(Debug, Clone)]
pub struct SimulationElfInitialStudyEdit {
    /// Resulting complete CWR study.
    pub study: SimulationElfInitialStudy,
    /// Exact complete initializer replacement and parameter derivation.
    pub initial_edit: SimulationElfInitialEdit,
    /// Complete all-entry carrier history, logical fidelity and structural coverage.
    pub carrier_edit: CompoundEdit,
}

impl SimulationElfInitialStudy {
    /// Read a complete CWR and its native ELF initializer, e.g. `SimulationElfInitialStudy::from_bytes(cwr, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: CompoundLimits) -> Result<Self> {
        let paired = SimulationStudy::from_bytes(bytes, limits)?;
        let entry = paired.carrier().entry_at_path("ELF")?;
        let initial =
            SimulationElfInitialArchive::from_bytes(paired.carrier().stream(entry.id())?)?;
        Ok(Self { paired, initial })
    }

    /// Actual paired binary context and complete carrier, e.g. `study.paired().carrier()`.
    pub fn paired(&self) -> &SimulationStudy {
        &self.paired
    }

    /// Complete immutable initializer, e.g. `study.initial().parameters()`.
    pub fn initial(&self) -> &SimulationElfInitialArchive {
        &self.initial
    }

    /// Replace one initial parameter and reopen the complete result, e.g. `study.edit_parameter(&id, bits)?`.
    pub fn edit_parameter(
        &self,
        id: &SimulationElfParameterId,
        bits: u32,
    ) -> Result<SimulationElfInitialStudyEdit> {
        let initial_edit = self.initial.edit_parameter(id, bits)?;
        let entry = self.paired.carrier().entry_at_path("ELF")?;
        let carrier_edit = self
            .paired
            .carrier()
            .edit_stream(entry.id(), &initial_edit.archive.to_bytes())?;
        let paired = self.paired.with_carrier(carrier_edit.file.clone())?;
        verify_reopened_initial(&paired, &initial_edit.archive)?;
        let study = Self {
            paired,
            initial: initial_edit.archive.clone(),
        };
        Ok(SimulationElfInitialStudyEdit {
            study,
            initial_edit,
            carrier_edit,
        })
    }
}

fn verify_reopened_initial(
    paired: &SimulationStudy,
    expected: &SimulationElfInitialArchive,
) -> Result<()> {
    let entry = paired.carrier().entry_at_path("ELF")?;
    let reopened = SimulationElfInitialArchive::from_bytes(paired.carrier().stream(entry.id())?)?;
    require(
        reopened.to_bytes() == expected.to_bytes(),
        "CWR ELF initial replacement",
        "reopened ELF differs; expected exact complete native initializer",
    )
}
