use super::*;
impl std::fmt::Debug for ZipEntryTimestamps {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ZipEntryTimestamps")
            .field("file_bytes", &self.raw.len())
            .field("entries", &self.entries.len())
            .finish()
    }
}
impl ZipEntryTimestamps {
    /// Borrows the complete original checkpoint; e.g. `let raw = times.as_bytes()`.
    pub fn as_bytes(&self) -> &[u8] {
        &self.raw
    }
    /// Returns the admitted input bound; e.g. `let bound = times.maximum_file_bytes()`.
    pub fn maximum_file_bytes(&self) -> usize {
        self.maximum
    }
    /// Enumerates central-directory order; e.g. `let count = times.entries().len()`.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = ZipTimestampEntry<'_>> {
        self.entries
            .iter()
            .enumerate()
            .map(|(ordinal, span)| ZipTimestampEntry {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Resolves an exact case-sensitive stream name; e.g. `let entry = times.entry_named("PreviewPNG")?`.
    pub fn entry_named(&self, name: &str) -> Result<ZipTimestampEntry<'_>> {
        self.entries()
            .find(|entry| entry.name() == name)
            .ok_or_else(|| {
                failure(
                    name,
                    FailureClass::Missing,
                    format!("stream {name:?}; expected an existing case-sensitive entry"),
                )
            })
    }
    /// Resolves a revision-bound entry; e.g. `let entry = times.entry(&id)?`.
    pub fn entry(&self, id: &ZipTimestampEntryId) -> Result<ZipTimestampEntry<'_>> {
        if id.revision != self.revision {
            return Err(failure(
                "entry identity",
                FailureClass::Invalid,
                format!("foreign entry {id:?}; expected this immutable checkpoint"),
            ));
        }
        self.entries().nth(id.ordinal).ok_or_else(|| {
            failure(
                "entry identity",
                FailureClass::Invalid,
                format!(
                    "entry ordinal {}; expected below {}",
                    id.ordinal,
                    self.entries.len()
                ),
            )
        })
    }
    /// Reports all structurally checked headers; e.g. `let proof = times.coverage()`.
    pub fn coverage(&self) -> ZipTimestampCoverage {
        ZipTimestampCoverage {
            mode: ZipTimestampCheckMode::ExactContainerHeaders,
            file_bytes: self.raw.len(),
            entries: self.entries.len(),
            timestamp_copies: self.entries.len() * 2,
            payload_crc_checked: false,
            achieved_error: 0,
        }
    }
    /// Borrows explicit native and payload exclusions; e.g. `let notes = times.diagnostics()`.
    pub fn diagnostics(&self) -> &[ZipTimestampDiagnostic] {
        &[
            ZipTimestampDiagnostic::PayloadsNotValidated,
            ZipTimestampDiagnostic::NativeMetadataProvenanceNotReproduced,
            ZipTimestampDiagnostic::NativeApplicationNotChecked,
        ]
    }
}
impl ZipTimestampEntryId {
    /// Returns central-directory position; e.g. `let index = id.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}
impl<'a> ZipTimestampEntry<'a> {
    /// Returns the exact source identity; e.g. `let id = entry.id()`.
    pub fn id(self) -> ZipTimestampEntryId {
        ZipTimestampEntryId {
            revision: self.owner.revision.clone(),
            ordinal: self.ordinal,
        }
    }
    /// Borrows the original case-sensitive name; e.g. `let name = entry.name()`.
    pub fn name(self) -> &'a str {
        &self.span.name
    }
    /// Returns all stored timestamp bits without calendar interpretation; e.g. `let raw = entry.packed_word()`.
    pub fn packed_word(self) -> u32 {
        self.span.packed
    }
    /// Returns the two complete four-byte intervals; e.g. `let (local, central) = entry.timestamp_intervals()`.
    pub fn timestamp_intervals(self) -> ((usize, usize), (usize, usize)) {
        (self.span.local, self.span.central)
    }
}
