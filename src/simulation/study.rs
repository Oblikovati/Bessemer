use super::{Result, SimulationGenArchive, SimulationMasterArchive, SimulationNodeId, require};
use crate::compound::{CompoundEdit, CompoundFile, CompoundLimits};

/// Immutable paired MAS/GEN study inside a complete validated CWR carrier.
#[derive(Debug, Clone)]
pub struct SimulationStudy {
    carrier: CompoundFile,
    master: SimulationMasterArchive,
    image: SimulationGenArchive,
}

/// Complete isolated study edit, retaining carrier history and framing exclusions.
#[derive(Debug, Clone)]
pub struct SimulationStudyEdit {
    /// Resulting complete paired study.
    pub study: SimulationStudy,
    /// All-entry carrier derivation, exact logical fidelity and structural coverage.
    pub carrier_edit: CompoundEdit,
    /// Exact coordinate replacement and node derivation within the paired GEN.
    pub image_edit: super::SimulationGenEdit,
    /// Required semantic checks that were not performed.
    pub diagnostics: [SimulationDiagnostic; 3],
}

/// Explicit acceptance limits for complete stored Simulation images.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationDiagnostic {
    /// Unknown stored fields and solver semantics were not assigned or validated.
    SolverAndUnknownFieldSemanticsNotChecked,
    /// Stored mesh accessors do not establish geometry, topology or tolerances.
    GeometryAndTopologyNotChecked,
    /// SolidWorks did not load the output.
    NativeApplicationNotChecked,
}

impl SimulationStudy {
    /// Read complete root MAS/GEN streams, e.g. `SimulationStudy::from_bytes(cwr, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: CompoundLimits) -> Result<Self> {
        Self::from_carrier(CompoundFile::from_bytes(bytes, limits)?)
    }

    fn from_carrier(carrier: CompoundFile) -> Result<Self> {
        let mas = carrier.entry_at_path("MAS")?;
        let master = SimulationMasterArchive::from_bytes(carrier.stream(mas.id())?)?;
        let gen_entry = carrier.entry_at_path("GEN")?;
        let image = SimulationGenArchive::from_bytes(carrier.stream(gen_entry.id())?, &master)?;
        Ok(Self {
            carrier,
            master,
            image,
        })
    }

    /// Complete immutable compound carrier, e.g. `study.carrier().entries()`.
    pub fn carrier(&self) -> &CompoundFile {
        &self.carrier
    }

    /// Actual paired master image, e.g. `study.master().version()`.
    pub fn master(&self) -> &SimulationMasterArchive {
        &self.master
    }

    /// Complete GEN image and stored accessors, e.g. `study.image().node_id(0)?`.
    pub fn image(&self) -> &SimulationGenArchive {
        &self.image
    }

    /// All unexecuted semantic acceptance responsibilities, e.g. `study.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationDiagnostic; 3] {
        diagnostics()
    }

    /// Replace coordinate patterns and reopen the complete result, e.g. `study.edit_node_coordinates(&id, bits)?`.
    pub fn edit_node_coordinates(
        &self,
        id: &SimulationNodeId,
        bits: [u32; 3],
    ) -> Result<SimulationStudyEdit> {
        let image_edit = self.image.edit_node_coordinates(id, bits)?;
        let image = &image_edit.archive;
        let entry = self.carrier.entry_at_path("GEN")?;
        let carrier_edit = self.carrier.edit_stream(entry.id(), &image.to_bytes())?;
        verify_reopened(&carrier_edit.file, image)?;
        let study = Self {
            carrier: carrier_edit.file.clone(),
            master: self.master.clone(),
            image: image.clone(),
        };
        Ok(SimulationStudyEdit {
            diagnostics: study.diagnostics(),
            study,
            carrier_edit,
            image_edit,
        })
    }
}

pub(super) fn diagnostics() -> [SimulationDiagnostic; 3] {
    [
        SimulationDiagnostic::SolverAndUnknownFieldSemanticsNotChecked,
        SimulationDiagnostic::GeometryAndTopologyNotChecked,
        SimulationDiagnostic::NativeApplicationNotChecked,
    ]
}

fn verify_reopened(carrier: &CompoundFile, image: &SimulationGenArchive) -> Result<()> {
    let reopened = SimulationStudy::from_carrier(carrier.clone())?;
    require(
        reopened.image.to_bytes() == image.to_bytes(),
        "CWR GEN edit",
        "reopened GEN differs from replacement; expected exact stored image",
    )?;
    Ok(())
}
