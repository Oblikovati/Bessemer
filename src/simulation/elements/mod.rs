mod classification;
mod legacy;
mod selection;
mod topology;
use super::{Result, SimulationGenArchive, invalid, require};
use crate::archive_codec::ArchiveRevision;
pub use classification::*;
pub use legacy::*;
pub use selection::*;
pub use topology::*;

/// Identity of one immutable complete element-record snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElementRecordId(ArchiveRevision);

/// Complete retained 140-byte record, without inferred engineering enums.
#[derive(Debug, Clone)]
pub struct SimulationElementRecord {
    bytes: [u8; 140],
    id: SimulationElementRecordId,
}

impl SimulationElementRecord {
    /// Retain exactly one record, e.g. `SimulationElementRecord::from_bytes(bytes)?`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let bytes = bytes.try_into().map_err(|_| {
            invalid(
                "Simulation element record",
                format!("{} bytes; expected exactly 140", bytes.len()),
            )
        })?;
        Ok(Self {
            bytes,
            id: SimulationElementRecordId(ArchiveRevision::default()),
        })
    }

    /// Complete unchanged stored bytes, e.g. `record.to_bytes()`.
    pub fn to_bytes(&self) -> [u8; 140] {
        self.bytes
    }

    /// Snapshot identity, e.g. `record.id()`; cloning preserves it.
    pub fn id(&self) -> &SimulationElementRecordId {
        &self.id
    }
}

impl SimulationGenArchive {
    /// Snapshot a complete stored record without imposing the accessor's count guard, e.g. `gen.element_record(0)?`.
    pub fn element_record(&self, index: usize) -> Result<SimulationElementRecord> {
        let block = self.block(6)?;
        let records = block.as_chunks::<140>().0;
        let bytes = records.get(index).ok_or_else(|| {
            invalid(
                "GEN element record",
                format!("index {index}; expected less than {}", records.len()),
            )
        })?;
        SimulationElementRecord::from_bytes(bytes)
    }
}

/// Explicit limits of the inspected projections, distinct from file validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElementDiagnostic {
    /// No complete mapping to engineering names or API element enums is bound.
    EngineeringNamesUnassigned,
    /// Native runtime/postprocessing and its caller gates were not executed.
    NativeRuntimeNotExecuted,
    /// Geometry, topology consistency, intersections and tolerances were not validated.
    GeometryNotValidated,
    /// The supplied legacy context does not prove native loop visitation.
    LoopVisitationNotProven,
}

fn projection_diagnostics() -> [SimulationElementDiagnostic; 3] {
    [
        SimulationElementDiagnostic::EngineeringNamesUnassigned,
        SimulationElementDiagnostic::NativeRuntimeNotExecuted,
        SimulationElementDiagnostic::GeometryNotValidated,
    ]
}
