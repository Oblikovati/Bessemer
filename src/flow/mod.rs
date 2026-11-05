//! Bounded ordinary physical Flow carriers, specification 14.29; payload schemas remain separate.
mod decode;
mod edit;
mod types;
use crate::{BessemerError, FailureClass, Result};
use std::sync::Arc;
pub use types::*;

#[derive(Debug)]
struct FlowRevision {
    wire: Arc<[u8]>,
    spans: Vec<FlowSpan>,
    limits: FlowLimits,
    version: u16,
}

#[derive(Clone, Copy, Debug)]
struct FlowSpan {
    identifier: u32,
    start: usize,
    end: usize,
}

/// Complete immutable ordinary physical partition with revision-bound record identities.
#[derive(Clone, Debug)]
pub struct FlowCarrier {
    revision: Arc<FlowRevision>,
}

impl FlowCarrier {
    /// Checks complete physical EOF; e.g. `let carrier = FlowCarrier::from_bytes(&raw, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: FlowLimits) -> Result<Self> {
        decode::read(bytes, limits)
    }
    /// Borrows exact carrier bytes; e.g. `assert_eq!(carrier.as_bytes(), raw)`.
    pub fn as_bytes(&self) -> &[u8] {
        &self.revision.wire
    }
    /// Returns the observed version/variant word; e.g. `let variant = carrier.version()`.
    pub fn version(&self) -> u16 {
        self.revision.version
    }
    /// Enumerates every physical record, including duplicates; e.g. `let count = carrier.records().len()`.
    pub fn records(&self) -> impl ExactSizeIterator<Item = FlowRecord<'_>> {
        self.revision
            .spans
            .iter()
            .enumerate()
            .map(|(ordinal, span)| self.record(ordinal, span))
    }
    /// Selects the last ordinary occurrence; e.g. `let header = carrier.latest(123)?`.
    pub fn latest(&self, identifier: u32) -> Result<Option<FlowRecord<'_>>> {
        if matches!(identifier, 0xffffff9b | 0xffffff9d) {
            return Err(failure(
                "lookup",
                FailureClass::Unsupported,
                format!(
                    "identifier {identifier:#x}; expected ordinary ID excluding reserved -99/-101"
                ),
            ));
        }
        Ok(self
            .revision
            .spans
            .iter()
            .enumerate()
            .rev()
            .find(|(_, span)| span.identifier == identifier)
            .map(|(ordinal, span)| self.record(ordinal, span)))
    }
    /// Reports exact physical framing checks; e.g. `let checked = carrier.coverage()`.
    pub fn coverage(&self) -> FlowCoverage {
        let framing_bytes = 2 + self.revision.spans.len() * 12;
        FlowCoverage {
            mode: FlowCheckMode::ExactOrdinaryPhysicalCarrier,
            wire_bytes: self.as_bytes().len(),
            checked_records: self.revision.spans.len(),
            framing_bytes,
            opaque_payload_bytes: self.as_bytes().len() - framing_bytes,
            achieved_byte_error: 0,
        }
    }
    /// Borrows explicit semantic exclusions; e.g. `let exclusions = carrier.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowDiagnostic] {
        &[
            FlowDiagnostic::PayloadSchemasNotChecked,
            FlowDiagnostic::AlternateIndexNotInterpreted,
            FlowDiagnostic::NativeApplicationNotChecked,
        ]
    }
    fn record(&self, ordinal: usize, span: &FlowSpan) -> FlowRecord<'_> {
        FlowRecord {
            id: FlowRecordId {
                revision: self.revision.clone(),
                ordinal,
            },
            identifier: span.identifier,
            header_offset: span.start,
            payload: &self.revision.wire[span.start + 12..span.end],
        }
    }
}

fn failure(entity: &str, class: FailureClass, cause: impl Into<String>) -> BessemerError {
    BessemerError::new("Flow physical carrier", entity, class, cause)
}
