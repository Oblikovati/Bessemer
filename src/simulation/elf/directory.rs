use super::{
    Result, SimulationElfIndexContext, SimulationElfRecordId, SimulationElfScalarRecord,
    SimulationElfSelector,
};
use std::sync::Arc;

/// Exact selected native traversal result, without whole populated-file acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfDirectoryCompletion {
    /// The selected pair declares no records; its raw first-record word remains available.
    Empty,
    /// Every declared record was visited without a negative identifier.
    DeclaredCountExhausted,
    /// A complete negative-identifier record was visited and terminated the getter.
    StoppedOnNegative,
}

/// Exact selected record check scope; unvisited source bytes are preserved separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfDirectoryCheckMode {
    /// Exact copied framing, observed index context and every native-selected complete record.
    ExactSelectedTraversal,
}

/// Quantified selected-record coverage, separate from unvisited source acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElfDirectoryCoverage {
    /// Explicit exact selected check mode.
    pub mode: SimulationElfDirectoryCheckMode,
    /// Actual complete records visited, including a terminating negative record.
    pub visited_records: usize,
    /// Actual complete selected record bytes checked.
    pub visited_bytes: usize,
    /// Complete original bytes retained, including unvisited content.
    pub source_bytes_preserved: usize,
    /// Always false: selected traversal does not establish whole populated-file acceptance.
    pub whole_populated_image_checked: bool,
}

/// One complete visited physical record with its immutable source identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElfVisitedRecord {
    /// Revision-bound normalized physical address.
    pub id: SimulationElfRecordId,
    /// Complete immutable identifier and sixty scalar patterns.
    pub record: SimulationElfScalarRecord,
}

/// Immutable selected-directory projection derived from an actual complete source context.
#[derive(Debug, Clone)]
pub struct SimulationElfDirectory {
    pub(super) context: SimulationElfIndexContext,
    pub(super) selector: SimulationElfSelector,
    pub(super) first: i32,
    pub(super) declared_count: u32,
    pub(super) records: Arc<[SimulationElfVisitedRecord]>,
    pub(super) cursor: Option<usize>,
    pub(super) completion: SimulationElfDirectoryCompletion,
}

impl SimulationElfDirectory {
    /// The actual retained source context, e.g. `directory.context().to_bytes()`.
    pub fn context(&self) -> &SimulationElfIndexContext {
        &self.context
    }

    /// Selected normalized one-based directory entry, e.g. `directory.selector()`.
    pub fn selector(&self) -> SimulationElfSelector {
        self.selector
    }

    /// Exact signed first-record word, even for an empty pair, e.g. `directory.first_record()`.
    pub fn first_record(&self) -> i32 {
        self.first
    }

    /// Exact validated nonnegative selected count, e.g. `directory.declared_count()`.
    pub fn declared_count(&self) -> u32 {
        self.declared_count
    }

    /// Every complete visited record in native visit order, e.g. `directory.records()`.
    pub fn records(&self) -> &[SimulationElfVisitedRecord] {
        &self.records
    }

    /// Full-stream pair address, e.g. `directory.pair_stream_offset()`.
    pub fn pair_stream_offset(&self) -> usize {
        self.selector.pair_body_offset() + 4
    }

    /// Full-stream cursor after the final visited record, or none for an empty pair, e.g. `directory.cursor_stream_offset()`.
    pub fn cursor_stream_offset(&self) -> Option<usize> {
        self.cursor.map(|offset| offset + 4)
    }

    /// Empty, exhausted or negative-stop completion, e.g. `directory.completion()`.
    pub fn completion(&self) -> SimulationElfDirectoryCompletion {
        self.completion
    }

    /// Quantify exact selected coverage, e.g. `directory.coverage().visited_records`.
    pub fn coverage(&self) -> SimulationElfDirectoryCoverage {
        SimulationElfDirectoryCoverage {
            mode: SimulationElfDirectoryCheckMode::ExactSelectedTraversal,
            visited_records: self.records.len(),
            visited_bytes: self.records.len() * 244,
            source_bytes_preserved: self.context.bytes.len(),
            whole_populated_image_checked: false,
        }
    }
}

impl SimulationElfIndexContext {
    /// Visit the actual selected directory with native stop/skip rules, e.g. `context.directory(selector)?`.
    pub fn directory(&self, selector: SimulationElfSelector) -> Result<SimulationElfDirectory> {
        super::traversal::read(self, selector)
    }
}
