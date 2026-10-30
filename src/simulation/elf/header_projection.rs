use super::{
    Result, SimulationElfDiagnostic, SimulationElfIndexContext, SimulationElfIndexRevision,
    SimulationElfSelector, require, wire,
};
use std::sync::Arc;

/// Completion is limited to proposed header bytes; advertised data records were not written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfIndexProjectionCompletion {
    /// A complete header/index write proposal, never a completed populated file.
    HeaderOnlyNoDataRecords,
}

/// Exact portable projection check scope, never whole populated-file acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfProjectionCheckMode {
    /// Every proposed header/index byte, retained word and cleared span.
    ExactHeaderWriteProposal,
    /// Every selected copy, normalized target and visit-order overwrite.
    ExactSelectedRecordCopies,
}

/// Complete proposed header-byte coverage and explicit missing data writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElfHeaderProjectionCoverage {
    /// Explicit exact proposal check mode.
    pub mode: SimulationElfProjectionCheckMode,
    /// Complete proposed body-byte extent, without a transfer wrapper.
    pub header_body_bytes: usize,
    /// Header-record bytes after the seven retained words that are cleared.
    pub cleared_header_padding_bytes: usize,
    /// All directory pairs except the selected replacement that are cleared.
    pub cleared_other_directory_pairs: usize,
    /// Trailing index-record padding bytes that are cleared.
    pub cleared_index_padding_bytes: usize,
    /// Always false: this proposal writes no complete populated file.
    pub complete_populated_file_written: bool,
}

/// Exact proposed header/index write with explicit incomplete-file status.
#[derive(Debug, Clone)]
pub struct SimulationElfIndexProjection {
    header_body: Arc<[u8]>,
    /// Original immutable source identity from which retained words were derived.
    pub parent_revision: SimulationElfIndexRevision,
    /// Normalized selected entry; all other entries are cleared.
    pub selector: SimulationElfSelector,
    /// Wrapped stream offset at which these header-body bytes would be written.
    pub write_stream_offset: usize,
    /// Full-stream address of the selected directory pair.
    pub pair_stream_offset: usize,
    /// Original marker advertised as the first record of the proposed block.
    pub first_record: u32,
    /// Advertised data-record count; none of these records are supplied by this proposal.
    pub count: u32,
    /// Marker after checked signed-range advancement.
    pub next_record_marker: u32,
    /// Explicit header-only completion scope.
    pub completion: SimulationElfIndexProjectionCompletion,
    /// Exact header-byte check scope, including the uncompleted file responsibility.
    pub coverage: SimulationElfHeaderProjectionCoverage,
    /// Actual data records written by this operation, always zero.
    pub data_records_written: usize,
    /// Achieved requested header-pattern error, zero; no physical error was established.
    pub header_pattern_error_bits: u32,
    /// Required unchecked responsibilities.
    pub diagnostics: [SimulationElfDiagnostic; 4],
}

impl SimulationElfIndexProjection {
    /// The exact 1,220 proposed body bytes, without a transfer wrapper, e.g. `proposal.header_body_bytes()`.
    pub fn header_body_bytes(&self) -> &[u8] {
        &self.header_body
    }
}

impl SimulationElfIndexContext {
    /// Project only the documented clearing header/index write, e.g. `context.project_index_update(selector)?`; no populated file is published.
    pub fn project_index_update(
        &self,
        selector: SimulationElfSelector,
    ) -> Result<SimulationElfIndexProjection> {
        let next = checked_next_marker(self.header[0], self.header[5])?;
        let header_body = proposed_body(self.header, selector, next);
        Ok(SimulationElfIndexProjection {
            header_body: header_body.into(),
            parent_revision: self.revision.clone(),
            selector,
            write_stream_offset: 4,
            pair_stream_offset: selector.pair_body_offset() + 4,
            first_record: self.header[0],
            count: self.header[5],
            next_record_marker: next,
            completion: SimulationElfIndexProjectionCompletion::HeaderOnlyNoDataRecords,
            coverage: header_coverage(),
            data_records_written: 0,
            header_pattern_error_bits: 0,
            diagnostics: super::initial_edit::diagnostics(),
        })
    }
}

fn header_coverage() -> SimulationElfHeaderProjectionCoverage {
    SimulationElfHeaderProjectionCoverage {
        mode: SimulationElfProjectionCheckMode::ExactHeaderWriteProposal,
        header_body_bytes: 1220,
        cleared_header_padding_bytes: 216,
        cleared_other_directory_pairs: 99,
        cleared_index_padding_bytes: 176,
        complete_populated_file_written: false,
    }
}

fn proposed_body(mut header: [u32; 7], selector: SimulationElfSelector, next: u32) -> Vec<u8> {
    let marker = header[0];
    header[0] = next;
    if (header[4] as i32) < selector.one_based() as i32 {
        header[4] = selector.one_based() as u32;
    }
    let mut body = Vec::with_capacity(1220);
    wire::append_patterns(&mut body, &header);
    body.resize(1220, 0);
    let offset = selector.pair_body_offset();
    body[offset..offset + 4].copy_from_slice(&marker.to_le_bytes());
    body[offset + 4..offset + 8].copy_from_slice(&header[5].to_le_bytes());
    body
}

fn checked_next_marker(marker: u32, count: u32) -> Result<u32> {
    let next = u64::from(marker) + u64::from(count);
    require(
        marker >= 6 && next <= i32::MAX as u64,
        "ELF header proposal marker",
        format!(
            "marker {marker} plus count {count}; expected marker at least six and sum at most {}",
            i32::MAX
        ),
    )?;
    Ok(next as u32)
}
