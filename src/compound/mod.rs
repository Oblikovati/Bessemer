//! MS-CFB version-3/4 carriers with exact logical streams and isolated replacements.
mod address;
mod directory;
mod header;
mod layout;
mod read;
mod tables;
mod types;
mod verify;
mod write;

use crate::archive_codec::ArchiveRevision;
use crate::{BessemerError, FailureClass, Result};
use read::CompoundModel;
use std::sync::Arc;
pub use types::*;

/// Immutable complete compound carrier, separate from the newer SolidWorks container.
#[derive(Debug, Clone)]
pub struct CompoundFile {
    original: Arc<[u8]>,
    model: Arc<CompoundModel>,
    entries: Arc<[CompoundEntry]>,
    revision: CompoundRevision,
    limits: CompoundLimits,
}

impl CompoundFile {
    /// Read and force full logical reconstruction, e.g. `CompoundFile::from_bytes(bytes, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: CompoundLimits) -> Result<Self> {
        let model = read::decode(bytes, limits).map_err(compound_error)?;
        verify::reconstruct(&model, &model.payloads, limits).map_err(compound_error)?;
        Ok(Self::publish(bytes, model, limits))
    }

    fn publish(bytes: &[u8], model: CompoundModel, limits: CompoundLimits) -> Self {
        let revision = CompoundRevision(ArchiveRevision::default());
        let entries = active_entries(&model, &revision);
        Self {
            original: bytes.into(),
            model: Arc::new(model),
            entries: entries.into(),
            revision,
            limits,
        }
    }

    /// Active entries in original slot order, e.g. `file.entries()`.
    pub fn entries(&self) -> &[CompoundEntry] {
        &self.entries
    }

    /// Snapshot identity required by every entry, e.g. `file.revision()`.
    pub fn revision(&self) -> &CompoundRevision {
        &self.revision
    }

    /// Read a revision-bound active entry, e.g. `file.entry(&id)?`.
    pub fn entry(&self, id: &CompoundEntryId) -> Result<&CompoundEntry> {
        if id.revision == self.revision
            && let Ok(position) = self
                .entries
                .binary_search_by_key(&id.slot, |entry| entry.id.slot)
        {
            return Ok(&self.entries[position]);
        }
        Err(invalid(format!(
            "entry slot {} from foreign/missing revision; expected an active entry in this snapshot",
            id.slot
        )))
    }

    /// Resolve an exact stored UTF-16 path from Unicode, e.g. `file.entry_at_path("Storage/Stream")?`.
    pub fn entry_at_path(&self, path: &str) -> Result<&CompoundEntry> {
        let mut slot = 0;
        if path.is_empty() {
            return self.entry(&self.id(0));
        }
        budget(
            "requested path depth",
            path.split('/').count(),
            self.limits.maximum_storage_depth,
        )?;
        for component in path.split('/') {
            slot = self.child_slot(slot, component, path)?;
        }
        self.entry(&self.id(slot))
    }

    /// Read complete user-stream bytes, e.g. `file.stream(entry.id())?`.
    pub fn stream(&self, id: &CompoundEntryId) -> Result<&[u8]> {
        let entry = self.entry(id)?;
        self.model
            .payloads
            .get(&entry.id.slot)
            .map(AsRef::as_ref)
            .ok_or_else(|| {
                invalid(format!(
                    "entry slot {} kind {:?}; expected a user stream",
                    id.slot, entry.kind
                ))
            })
    }

    /// Exact current file bytes, including original slack on a no-op, e.g. `file.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.original.to_vec()
    }

    /// Rebuild allocation and verify every logical stream, e.g. `file.reconstructed_bytes()?`.
    pub fn reconstructed_bytes(&self) -> Result<Vec<u8>> {
        verify::reconstruct(&self.model, &self.model.payloads, self.limits)
            .map(|(bytes, _)| bytes)
            .map_err(compound_error)
    }

    /// Replace an existing stream on an isolated snapshot, e.g. `file.edit_stream(&id, replacement)?`.
    pub fn edit_stream(&self, id: &CompoundEntryId, replacement: &[u8]) -> Result<CompoundEdit> {
        let original = self.stream(id)?;
        if original == replacement {
            return Ok(self.edit_outcome(self.clone(), id, false));
        }
        self.replacement_budget(original.len(), replacement.len())?;
        let mut payloads = self.model.payloads.clone();
        payloads.insert(id.slot, replacement.into());
        let (bytes, model) =
            verify::reconstruct(&self.model, &payloads, self.limits).map_err(compound_error)?;
        Ok(self.edit_outcome(Self::publish(&bytes, model, self.limits), id, true))
    }

    fn replacement_budget(&self, previous: usize, replacement: usize) -> Result<()> {
        budget(
            "replacement stream bytes",
            replacement,
            self.limits.maximum_stream_bytes,
        )?;
        budget(
            "replacement bytes in carrier",
            replacement,
            self.limits.maximum_file_bytes,
        )?;
        let retained = self.coverage().logical_stream_bytes - previous;
        let total = retained.checked_add(replacement).ok_or_else(|| invalid(format!("retained stream bytes {retained} plus {replacement}; expected a representable aggregate")))?;
        budget(
            "replacement aggregate bytes",
            total,
            self.limits.maximum_total_stream_bytes,
        )
    }

    /// Exact structural scope and reconstruction counts, e.g. `file.coverage().streams`.
    pub fn coverage(&self) -> CompoundCoverage {
        CompoundCoverage {
            mode: CompoundCheckMode::ExactAllocationAndDirectoryLinks,
            major: self.model.header.major,
            file_bytes: self.original.len(),
            sectors: self.model.header.sectors,
            directory_slots: self.model.directory.records.len(),
            active_entries: self.entries.len(),
            streams: self.model.payloads.len(),
            logical_stream_bytes: self
                .model
                .payloads
                .values()
                .map(|payload| payload.len())
                .sum(),
            claimed_sectors: self.model.normal_claims,
            claimed_mini_sectors: self.model.mini_claims,
            reconstruction_checked: true,
        }
    }

    /// Unchecked directory collation, stream semantics and native acceptance, e.g. `file.diagnostics()`.
    pub fn diagnostics(&self) -> [CompoundDiagnostic; 3] {
        DIAGNOSTICS
    }

    fn child_slot(&self, slot: usize, component: &str, path: &str) -> Result<usize> {
        budget("requested name UTF-8 bytes", component.len(), 124)?;
        let units: Vec<u16> = component.encode_utf16().collect();
        budget("requested name units", units.len(), 31)?;
        require(
            !units.is_empty() && !units.contains(&0),
            format!("path component {component:?}; expected a nonempty NUL-free name"),
        )?;
        self.model.directory.children[slot].iter().copied()
            .find(|child| self.model.directory.records[*child].name.as_ref() == units)
            .ok_or_else(|| BessemerError::new("compound file", path, FailureClass::Missing, format!("component {component:?} below slot {slot}; expected an exact stored child name")))
    }

    fn id(&self, slot: usize) -> CompoundEntryId {
        CompoundEntryId {
            revision: self.revision.clone(),
            slot,
        }
    }

    fn entry_relations(
        &self,
        file: &Self,
        changed: &CompoundEntryId,
        replaced: bool,
    ) -> Vec<CompoundEntryHistory> {
        self.entries
            .iter()
            .map(|entry| CompoundEntryHistory {
                parent: entry.id.clone(),
                result: file.id(entry.id.slot),
                replaced_stream: replaced && entry.id == *changed,
            })
            .collect()
    }

    fn edit_outcome(&self, file: Self, changed: &CompoundEntryId, replaced: bool) -> CompoundEdit {
        let relations = self.entry_relations(&file, changed, replaced);
        CompoundEdit {
            history: CompoundHistory {
                parent: self.revision.clone(),
                result: file.revision.clone(),
                entries: relations.into(),
            },
            completion: if replaced {
                crate::display_lists::DisplayEditCompletion::Changed
            } else {
                crate::display_lists::DisplayEditCompletion::Unchanged
            },
            fidelity: CompoundEditFidelity::ExactLogicalStreamsAndDirectoryMetadata,
            coverage: file.coverage(),
            file,
            diagnostics: DIAGNOSTICS,
        }
    }
}

fn active_entries(model: &CompoundModel, revision: &CompoundRevision) -> Vec<CompoundEntry> {
    model
        .directory
        .records
        .iter()
        .enumerate()
        .filter(|(_, record)| record.kind != CompoundEntryKind::Unused)
        .map(|(slot, record)| active_entry(model, revision, slot, record))
        .collect()
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "compound file",
        "allocation/directory",
        FailureClass::Invalid,
        cause,
    )
}

fn require(condition: bool, cause: impl Into<String>) -> Result<()> {
    if condition {
        return Ok(());
    }
    Err(invalid(cause))
}

fn budget(label: &str, actual: usize, maximum: usize) -> Result<()> {
    require(
        actual <= maximum,
        format!("{label} {actual}; expected at most {maximum}"),
    )
}

fn compound_error(mut error: BessemerError) -> BessemerError {
    error.operation = "compound file";
    error
}

const DIAGNOSTICS: [CompoundDiagnostic; 3] = [
    CompoundDiagnostic::DirectoryCollationAndBalanceNotChecked,
    CompoundDiagnostic::PayloadSemanticsNotChecked,
    CompoundDiagnostic::NativeApplicationNotChecked,
];

fn active_entry(
    model: &CompoundModel,
    revision: &CompoundRevision,
    slot: usize,
    record: &directory::DirectoryRecord,
) -> CompoundEntry {
    CompoundEntry {
        id: CompoundEntryId {
            revision: revision.clone(),
            slot,
        },
        parent: model.directory.parents[slot].map(|slot| CompoundEntryId {
            revision: revision.clone(),
            slot,
        }),
        name: record.name.clone(),
        kind: record.kind,
        size: record.size,
        raw: record.raw,
    }
}
