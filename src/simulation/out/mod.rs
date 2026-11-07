mod decimal;
mod edit;
mod framing;
mod outcome;
mod projection;
mod study;
mod syntax;
mod totals;
mod types;
use crate::archive_codec::ArchiveRevision;
use crate::simulation::{Result, invalid, require};
pub use outcome::*;
use std::sync::Arc;
pub use study::*;
pub use totals::*;
pub use types::*;

const SI: &[u8] = b" International System of Units, SI, is used for all input/output data.";

/// Complete immutable seven-bit OUT report syntax, distinct from solver acceptance.
#[derive(Debug, Clone)]
pub struct SimulationOutArchive {
    bytes: Arc<[u8]>,
    lines: Arc<[SimulationOutLine]>,
    fields: Arc<[SimulationOutField]>,
    revision: SimulationOutRevision,
    limits: SimulationOutLimits,
}

impl SimulationOutArchive {
    /// Validate complete report framing and ordered syntax, e.g. `SimulationOutArchive::from_bytes(out, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: SimulationOutLimits) -> Result<Self> {
        let inputs = framing::decode(bytes, limits)?;
        require(
            framing::encode(&inputs, limits)? == bytes,
            "OUT reconstruction",
            format!(
                "{} stored bytes differ from complete line construction; expected exact wrapped image",
                bytes.len()
            ),
        )?;
        let revision = SimulationOutRevision(ArchiveRevision::default());
        let (lines, fields) = projection::project(inputs, &revision, limits)?;
        Ok(Self {
            bytes: bytes.into(),
            lines: lines.into(),
            fields: fields.into(),
            revision,
            limits,
        })
    }

    /// Construct every report byte from complete source lines, e.g. `SimulationOutArchive::from_lines(&lines, limits)?`.
    pub fn from_lines(
        lines: &[SimulationOutLineInput],
        limits: SimulationOutLimits,
    ) -> Result<Self> {
        let archive = Self::from_bytes(&framing::encode(lines, limits)?, limits)?;
        require(
            archive.lines.len() == lines.len(),
            "OUT construction",
            format!(
                "{} decoded lines, supplied {}; expected no phantom empty unterminated line",
                archive.lines.len(),
                lines.len()
            ),
        )?;
        Ok(archive)
    }

    /// Exact complete original stream, e.g. `report.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    /// Every original line with exact ending and offset, e.g. `report.lines()`.
    pub fn lines(&self) -> &[SimulationOutLine] {
        &self.lines
    }

    /// Every distinct numeric occurrence in source order, e.g. `report.fields()`.
    pub fn fields(&self) -> &[SimulationOutField] {
        &self.fields
    }

    /// Immutable report identity, e.g. `report.revision()`.
    pub fn revision(&self) -> &SimulationOutRevision {
        &self.revision
    }

    /// Exact report-local SI declaration line, e.g. `report.si_declaration_line()`; no GEN units are inferred.
    pub fn si_declaration_line(&self) -> Option<usize> {
        self.lines
            .iter()
            .position(|line| line.input.content.as_ref() == SI)
    }

    /// Resolve a revision-bound occurrence, e.g. `report.field(&field.id)?`.
    pub fn field(&self, id: &SimulationOutFieldId) -> Result<&SimulationOutField> {
        require(
            id.revision == self.revision,
            "OUT occurrence",
            format!("revision {:?}; expected {:?}", id.revision, self.revision),
        )?;
        self.fields.get(id.index).ok_or_else(|| {
            invalid(
                "OUT occurrence",
                format!(
                    "index {}; expected less than {}",
                    id.index,
                    self.fields.len()
                ),
            )
        })
    }
}
