//! Versioned record-404 groups with explicit compression and bit-preserving wire-field edits.
mod decode;
mod edit;
mod fields;
mod types;
mod view;
use super::*;
pub use types::*;

impl FlowCarrier {
    /// Reads one exact occurrence under explicit context; e.g. `let groups = carrier.record_404_groups(&id, options)?`.
    pub fn record_404_groups(
        &self,
        id: &FlowRecordId,
        options: FlowGroupOptions,
    ) -> Result<FlowGroupedRecord> {
        decode::read(self, id, options)
    }
}

impl FlowGroupedRecord {
    /// Borrows the original complete carrier; e.g. `let bytes = record.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.0.source
    }
    /// Borrows the selected occurrence identity; e.g. `let id = record.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.0.record
    }
    /// Borrows exact decoded bytes; e.g. `let raw = record.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.0.decoded
    }
    /// Returns the normalized payload dialect; e.g. `let version = record.version().wire_value()`.
    pub fn version(&self) -> FlowGroupVersion {
        self.0.version
    }
    /// Returns the explicitly supplied profile; e.g. `let mode = record.compression()`.
    pub fn compression(&self) -> FlowGroupCompression {
        self.0.options.compression
    }
    /// Enumerates all groups including empty ones; e.g. `let count = record.groups().len()`.
    pub fn groups(&self) -> impl ExactSizeIterator<Item = FlowGroupView<'_>> {
        self.0
            .spans
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowGroupView {
                revision: &self.0,
                span,
                ordinal,
            })
    }
    /// Reports exact structural checks without semantic credit; e.g. `let checked = record.coverage()`.
    pub fn coverage(&self) -> FlowGroupCoverage {
        let mode = match self.compression() {
            FlowGroupCompression::Raw => FlowGroupCheckMode::ExactRawGroupedFrames,
            FlowGroupCompression::Zlib => FlowGroupCheckMode::ExactZlibGroupedFrames,
        };
        FlowGroupCoverage {
            mode,
            stored_bytes: self.0.stored_bytes,
            decoded_bytes: self.0.decoded.len(),
            checked_groups: self.0.spans.len(),
            checked_members: self.0.members,
            achieved_byte_error: 0,
        }
    }
    /// Borrows explicit owner/semantic/application exclusions; e.g. `let exclusions = record.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowGroupDiagnostic] {
        &[
            FlowGroupDiagnostic::MemberSemanticsNotEstablished,
            FlowGroupDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowGroupDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowGroupEdit {
    /// Borrows the exact original checkpoint; e.g. `let original = edit.source().source().as_bytes()`.
    pub fn source(&self) -> &FlowGroupedRecord {
        &self.source
    }
    /// Borrows the completely reparsed result; e.g. `let checked = edit.result().coverage()`.
    pub fn result(&self) -> &FlowGroupedRecord {
        &self.result
    }
    /// Borrows complete record-parent history; e.g. `let parents = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier
    }
    /// Borrows exact selected-field history; e.g. `let original = edit.history().before`.
    pub fn history(&self) -> &FlowGroupFieldHistory {
        &self.history
    }
    /// Borrows wire-reversal exclusions for the selected encoding; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowGroupEditDiagnostic] {
        match self.source.compression() {
            FlowGroupCompression::Raw => &[],
            FlowGroupCompression::Zlib => {
                &[FlowGroupEditDiagnostic::FreshReverseWireIdentityNotGuaranteed]
            }
        }
    }
}

fn group_failure(entity: &str, class: FailureClass, cause: impl Into<String>) -> BessemerError {
    BessemerError::new("Flow record 404", entity, class, cause)
}
