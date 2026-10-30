use crate::archive_codec::ArchiveRevision;

/// Explicit caller-established transport; no transfer prefix is inferred or stripped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationEgrProfile {
    /// Verified constructor-bound flat MFC CFile route at byte zero.
    ConstructorBoundFlatCFile,
}

/// Allocation bounds applied before directory or record ownership is published.
#[derive(Debug, Clone, Copy)]
pub struct SimulationEgrLimits {
    /// Maximum logical capacity, including holes and unallocated capacity.
    pub maximum_rows: usize,
}

/// Identity of one immutable complete conditional EGR file image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationEgrRevision(pub(super) ArchiveRevision);

/// Loaded descriptor; the tag is raw, without an inferred type or unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationEgrLoadedField {
    /// Original signed first descriptor word.
    pub tag: i32,
    /// Validated positive loaded byte width.
    pub width: usize,
    /// Start within a complete physical record.
    pub row_offset: usize,
    /// Exclusive end within the same record.
    pub row_end: usize,
}

/// One ordered deleted-record link, without a live solver-row interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationEgrFreeRecord {
    /// One-based physical record number.
    pub record_number: u32,
    /// Original signed next-link word, zero at the checked chain endpoint.
    pub next_record_number: i32,
}

#[derive(Debug, Clone)]
pub(super) struct EgrHeader {
    pub words: [i32; 9],
    pub capacity: usize,
    pub field_count: usize,
    pub live_count: usize,
    pub free_count: usize,
    pub allocated: usize,
    pub directory_start: usize,
    pub rows_start: usize,
    pub expected_bytes: usize,
}

#[derive(Debug, Clone)]
pub(super) struct EgrLayout {
    pub header: EgrHeader,
    pub fields: Vec<SimulationEgrLoadedField>,
    pub directory: Vec<i32>,
    pub free_records: Vec<SimulationEgrFreeRecord>,
}
