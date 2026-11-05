use super::*;

/// Positive physical read/edit bounds, independent of payload semantic validation.
#[derive(Clone, Copy, Debug)]
pub struct FlowLimits {
    /// Maximum complete encoded bytes.
    pub maximum_wire_bytes: usize,
    /// Maximum retained physical records.
    pub maximum_records: usize,
    /// Maximum bytes in any one opaque payload.
    pub maximum_payload_bytes: usize,
}

/// Identity of one exact occurrence in one immutable revision.
#[derive(Clone, Debug)]
pub struct FlowRecordId {
    pub(super) revision: Arc<FlowRevision>,
    pub(super) ordinal: usize,
}

impl PartialEq for FlowRecordId {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.revision, &other.revision) && self.ordinal == other.ordinal
    }
}
impl Eq for FlowRecordId {}

impl FlowRecordId {
    /// Returns the physical ordinal, not the native numeric identifier; e.g. `let index = id.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}

/// Exact physical record view, retaining unknown and reserved payloads without interpretation.
#[derive(Clone, Debug)]
pub struct FlowRecord<'a> {
    /// Revision-bound occurrence identity.
    pub id: FlowRecordId,
    /// Unchanged native lookup word.
    pub identifier: u32,
    /// Absolute twelve-byte header offset.
    pub header_offset: usize,
    /// Complete stored payload, including zero bytes or unknown extensions.
    pub payload: &'a [u8],
}

/// Complete exact physical framing coverage; no decoded-payload semantic credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCheckMode {
    /// Every ordinary physical record partitions the full carrier through exact EOF.
    ExactOrdinaryPhysicalCarrier,
}

/// Complete exact physical framing coverage; no decoded-payload semantic credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowCoverage {
    /// Required scope, distinct from native alternate-index or payload validation.
    pub mode: FlowCheckMode,
    /// Complete encoded input/output extent.
    pub wire_bytes: usize,
    /// Every retained physical record, including reserved records.
    pub checked_records: usize,
    /// Two version bytes and twelve bytes per record.
    pub framing_bytes: usize,
    /// All payload bytes retained without semantic interpretation.
    pub opaque_payload_bytes: usize,
    /// Exact byte disagreement; completed physical operations report zero.
    pub achieved_byte_error: usize,
}

/// Explicit exclusions from physical carrier acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowDiagnostic {
    /// Record payload schemas, compression and GUID roles were not checked.
    PayloadSchemasNotChecked,
    /// Reserved records and alternate compressed terminal indexes remain opaque.
    AlternateIndexNotInterpreted,
    /// Solver/viewer application and geometry acceptance were not checked.
    NativeApplicationNotChecked,
}

/// Payload identity relation, independent of changed physical offsets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPayloadRelation {
    /// Exact payload bytes copied from the source occurrence.
    Copied,
    /// Complete replacement payload derived from the source occurrence.
    Derived,
}

/// One source/result occurrence relation in physical order.
#[derive(Clone, Debug)]
pub struct FlowRecordHistory {
    /// Exact source occurrence.
    pub source: FlowRecordId,
    /// Corresponding result occurrence.
    pub result: FlowRecordId,
    /// Complete payload relation.
    pub relation: FlowPayloadRelation,
}

/// Completed physical edit, retaining exact input checkpoint and all occurrence parents.
#[derive(Clone, Debug)]
pub struct FlowCarrierEdit {
    pub(super) source: FlowCarrier,
    pub(super) result: FlowCarrier,
    pub(super) history: Vec<FlowRecordHistory>,
}

impl FlowCarrierEdit {
    /// Borrows the exact source checkpoint; e.g. `let original = edit.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the checked complete output; e.g. `let output = edit.result().as_bytes()`.
    pub fn result(&self) -> &FlowCarrier {
        &self.result
    }
    /// Borrows every source/result payload parent; e.g. `let parents = edit.history()`.
    pub fn history(&self) -> &[FlowRecordHistory] {
        &self.history
    }
}
