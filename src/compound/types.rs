use super::CompoundFile;
use crate::archive_codec::ArchiveRevision;
use std::sync::Arc;

/// Explicit input, output, payload, directory and hierarchy budgets.
#[derive(Debug, Clone, Copy)]
pub struct CompoundLimits {
    /// Maximum complete input or constructed carrier size.
    pub maximum_file_bytes: usize,
    /// Maximum logical bytes in one user stream.
    pub maximum_stream_bytes: usize,
    /// Maximum cumulative user-stream bytes.
    pub maximum_total_stream_bytes: usize,
    /// Maximum directory slots, including unused records.
    pub maximum_directory_entries: usize,
    /// Maximum non-root directory hierarchy depth.
    pub maximum_storage_depth: usize,
}

/// Immutable identity of one complete compound-file snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundRevision(pub(super) ArchiveRevision);

/// Directory identity bound to one snapshot; indices are original directory slots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundEntryId {
    pub(super) revision: CompoundRevision,
    pub(super) slot: usize,
}

impl CompoundEntryId {
    /// Original slot without path or case normalization, e.g. `id.slot()`.
    pub fn slot(&self) -> usize {
        self.slot
    }

    /// Required snapshot identity, e.g. `id.revision()`.
    pub fn revision(&self) -> &CompoundRevision {
        &self.revision
    }
}

/// Stored CFB object category, independently of its stream payload format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundEntryKind {
    /// Unallocated directory record; excluded from published active entries.
    Unused,
    /// Storage object with direct child entries.
    Storage,
    /// User-defined logical byte stream.
    Stream,
    /// Root storage; its allocation fields describe the internal mini stream.
    Root,
}

/// Active directory record with exact stored metadata and raw UTF-16 name.
#[derive(Debug, Clone)]
pub struct CompoundEntry {
    pub(super) id: CompoundEntryId,
    pub(super) parent: Option<CompoundEntryId>,
    pub(super) name: Arc<[u16]>,
    pub(super) kind: CompoundEntryKind,
    pub(super) size: usize,
    pub(super) raw: [u8; 128],
}

impl CompoundEntry {
    /// Snapshot-bound directory identity, e.g. `entry.id()`.
    pub fn id(&self) -> &CompoundEntryId {
        &self.id
    }

    /// Root-owned parent or None for the root, e.g. `entry.parent()`.
    pub fn parent(&self) -> Option<&CompoundEntryId> {
        self.parent.as_ref()
    }

    /// Exact UTF-16 name units, e.g. `entry.name_units()`; invalid pairs are retained.
    pub fn name_units(&self) -> &[u16] {
        &self.name
    }

    /// Stored object role, e.g. `entry.kind()`.
    pub fn kind(&self) -> CompoundEntryKind {
        self.kind
    }

    /// Logical size with version-3 high-DWORD compatibility, e.g. `entry.logical_size()`.
    pub fn logical_size(&self) -> usize {
        self.size
    }

    /// All 128 stored bytes, including opaque metadata, e.g. `entry.directory_record()`.
    pub fn directory_record(&self) -> &[u8; 128] {
        &self.raw
    }
}

/// Exact structural checks performed before publication; not complete payload acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundCheckMode {
    /// Complete chains, exclusive ownership, extents and directory reachability.
    ExactAllocationAndDirectoryLinks,
}

/// Quantified coverage of the complete carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompoundCoverage {
    /// Declared exact check scope.
    pub mode: CompoundCheckMode,
    /// Stored major version, 3 or 4.
    pub major: u16,
    /// Complete file size, including header and slack.
    pub file_bytes: usize,
    /// Physical sectors after the header sector.
    pub sectors: usize,
    /// Directory slots, including inactive records.
    pub directory_slots: usize,
    /// Active root/storage/stream objects.
    pub active_entries: usize,
    /// User-defined streams.
    pub streams: usize,
    /// Total logical user-stream bytes.
    pub logical_stream_bytes: usize,
    /// Physical sectors with an established owner.
    pub claimed_sectors: usize,
    /// Mini sectors with an established user-stream owner.
    pub claimed_mini_sectors: usize,
    /// Every logical stream and active directory metadata survived forced reconstruction.
    pub reconstruction_checked: bool,
}

/// Responsibilities outside the declared structural check scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundDiagnostic {
    /// Full Unicode CFB name ordering and red-black balance were not established.
    DirectoryCollationAndBalanceNotChecked,
    /// User-stream formats remain the responsibility of their separate codecs.
    PayloadSemanticsNotChecked,
    /// The native application did not load this output.
    NativeApplicationNotChecked,
}

/// Exact logical content and metadata; physical allocation may be rebuilt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompoundEditFidelity {
    /// All user bytes and directory metadata apart from allocation/size fields.
    ExactLogicalStreamsAndDirectoryMetadata,
}

/// One identity derivation; every active entry has exactly one parent.
#[derive(Debug, Clone)]
pub struct CompoundEntryHistory {
    /// Identity in the input snapshot.
    pub parent: CompoundEntryId,
    /// Corresponding identity in the result snapshot.
    pub result: CompoundEntryId,
    /// Whether this entry's user-stream content changed.
    pub replaced_stream: bool,
}

/// Complete snapshot and directory identity history, ordered by original slot.
#[derive(Debug, Clone)]
pub struct CompoundHistory {
    /// Source snapshot.
    pub parent: CompoundRevision,
    /// Result snapshot.
    pub result: CompoundRevision,
    /// Every active root/storage/stream derivation.
    pub entries: Arc<[CompoundEntryHistory]>,
}

/// Validated isolated existing-stream replacement and its declared acceptance limits.
#[derive(Debug, Clone)]
pub struct CompoundEdit {
    /// Immutable resulting carrier.
    pub file: CompoundFile,
    /// Whether bytes in the selected logical stream changed.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Exact logical preservation achieved by reconstruction.
    pub fidelity: CompoundEditFidelity,
    /// Complete snapshot and directory derivation.
    pub history: CompoundHistory,
    /// Structural checks and forced reconstruction coverage.
    pub coverage: CompoundCoverage,
    /// Required unchecked responsibilities.
    pub diagnostics: [CompoundDiagnostic; 3],
}
