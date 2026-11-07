mod codec;
mod types;
use super::{Result, invalid, require};
use crate::archive_codec::ArchiveRevision;
pub use types::*;

/// Complete immutable observed MAS table image, separate from its paired GEN.
#[derive(Debug, Clone)]
pub struct SimulationMasterArchive {
    parts: SimulationMasterParts,
    revision: SimulationMasterRevision,
}

impl SimulationMasterArchive {
    /// Decode all records and prove exact reconstruction, e.g. `SimulationMasterArchive::from_bytes(mas)?`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let archive = Self::from_parts(codec::decode(bytes)?)?;
        require(
            archive.to_bytes() == bytes,
            "MAS",
            "reconstructed image differs; expected all 28,420 bytes including padding",
        )?;
        Ok(archive)
    }

    /// Construct a complete image from all fields, e.g. `SimulationMasterArchive::from_parts(parts)?`.
    pub fn from_parts(parts: SimulationMasterParts) -> Result<Self> {
        codec::validate(&parts)?;
        Ok(Self {
            parts,
            revision: SimulationMasterRevision(ArchiveRevision::default()),
        })
    }

    /// Complete immutable image components, e.g. `master.parts().directory`.
    pub fn parts(&self) -> &SimulationMasterParts {
        &self.parts
    }

    /// Complete exact stored image, e.g. `master.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        codec::encode(&self.parts)
    }

    /// Identity used by GEN context and cell IDs, e.g. `master.revision()`.
    pub fn revision(&self) -> &SimulationMasterRevision {
        &self.revision
    }

    /// Directory-selected signed version at logical row 21 / field 3, e.g. `master.version()`.
    pub fn version(&self) -> i32 {
        let physical = self.parts.directory[20] as usize - 1;
        self.parts.physical_rows[physical].fields[2] as i32
    }

    /// Validate one-based coordinates once, e.g. `master.cell_id(21, 3)?`.
    pub fn cell_id(
        &self,
        logical_row: usize,
        field_number: usize,
    ) -> Result<SimulationMasterCellId> {
        require(
            (1..=70).contains(&logical_row) && (1..=81).contains(&field_number),
            "MAS cell",
            format!(
                "logical row {logical_row}, field {field_number}; expected rows 1..=70 and fields 1..=81"
            ),
        )?;
        Ok(SimulationMasterCellId {
            revision: self.revision.clone(),
            row: logical_row - 1,
            field: field_number - 1,
        })
    }

    /// Read exact field bits through the logical directory, e.g. `master.cell(&id)?`.
    pub fn cell(&self, id: &SimulationMasterCellId) -> Result<u32> {
        self.require_cell(id)?;
        let physical = self.parts.directory[id.row] as usize - 1;
        Ok(self.parts.physical_rows[physical].fields[id.field])
    }

    /// Replace one field on an isolated complete image, e.g. `master.edit_cell(&id, bits)?`.
    pub fn edit_cell(
        &self,
        id: &SimulationMasterCellId,
        bits: u32,
    ) -> Result<SimulationMasterEdit> {
        let changed = self.cell(id)? != bits;
        let archive = if changed {
            self.replaced_cell(id, bits)?
        } else {
            self.clone()
        };
        let result = archive.cell_id(id.row + 1, id.field + 1)?;
        Ok(SimulationMasterEdit {
            coverage: archive.coverage(),
            field_pattern_error_bits: 0,
            diagnostics: super::study::diagnostics(),
            archive,
            completion: if changed {
                crate::display_lists::DisplayEditCompletion::Changed
            } else {
                crate::display_lists::DisplayEditCompletion::Unchanged
            },
            parent: id.clone(),
            result,
        })
    }

    fn replaced_cell(&self, id: &SimulationMasterCellId, bits: u32) -> Result<Self> {
        let mut parts = self.parts.clone();
        let mut rows = parts.physical_rows.to_vec();
        let physical = parts.directory[id.row] as usize - 1;
        rows[physical].fields[id.field] = bits;
        parts.physical_rows = rows.into();
        Self::from_parts(parts)
    }

    fn require_cell(&self, id: &SimulationMasterCellId) -> Result<()> {
        if id.revision == self.revision && id.row < 70 && id.field < 81 {
            return Ok(());
        }
        Err(invalid(
            "MAS cell",
            format!(
                "row {}, field {} from foreign/missing revision; expected this MAS image's logical cell",
                id.row + 1,
                id.field + 1
            ),
        ))
    }

    /// Complete fixed-image check counts, e.g. `master.coverage().fields`.
    pub fn coverage(&self) -> SimulationMasterCoverage {
        SimulationMasterCoverage {
            rows: 70,
            fields: 5670,
            opaque_padding_bytes: 4772,
            reconstruction_checked: true,
        }
    }
}
