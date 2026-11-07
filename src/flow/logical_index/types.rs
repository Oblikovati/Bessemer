use super::*;
/// Selection policy applied after complete physical framing has been validated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowIndexPolicy {
    /// Adopt a recognized modern strict writer profile; scan only on a recognition miss.
    PreferTerminalIndex,
    /// Treat every control payload as opaque and construct maps from physical records.
    ForcePhysicalScan,
}
/// Independent bounds for distinct map keys and decoded control bodies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowIndexOptions {
    /// Maximum distinct active identifiers, including the terminal's self entry.
    pub maximum_active_entries: usize,
    /// Maximum distinct saved-map keys; zero admits an empty map.
    pub maximum_saved_entries: usize,
    /// Strict control limits, consulted only when a control payload is decoded.
    pub controls: FlowControlOptions,
}
/// Raw absolute carrier span; ordinary terminal targets have not been validated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowIndexedSpan {
    /// Raw unsigned absolute record start.
    pub record_start: u64,
    /// Raw unsigned record length including the physical header.
    pub record_span: u64,
}
/// Exact replay path, distinct from validation of the resulting ordinary targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowIndexMethod {
    /// Last physical occurrence wins; displaced and deleted spans populate the saved map.
    PhysicalScan,
    /// Strict eight-byte-footer terminal entries win; a selected saved map is loaded exactly.
    TerminalIndex,
}
/// Immutable fresh-map projection retaining its complete physical and control provenance.
#[derive(Clone, Debug)]
pub struct FlowLogicalIndex {
    pub(super) source: FlowCarrier,
    pub(super) policy: FlowIndexPolicy,
    pub(super) options: FlowIndexOptions,
    pub(super) active: Arc<BTreeMap<u32, FlowIndexedSpan>>,
    pub(super) saved: Arc<BTreeMap<u64, u64>>,
    pub(super) terminal: Option<FlowTerminalIndex>,
    pub(super) saved_control: Option<FlowSavedSpans>,
}
/// Exact replay coverage, with payload interpretation and target validation excluded explicitly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowIndexCoverage {
    /// Actual replay path.
    pub method: FlowIndexMethod,
    /// Complete physically framed carrier bytes, not decoded-payload credit.
    pub physical_wire_bytes: usize,
    /// Every physical record, including controls and duplicate identifiers.
    pub physical_records: usize,
    /// Distinct active identifiers after replay.
    pub active_entries: usize,
    /// Distinct saved-map keys after replay.
    pub saved_entries: usize,
    /// Decoded bytes in the selected terminal control, or zero for scanning.
    pub terminal_decoded_bytes: usize,
    /// Decoded bytes in the selected saved control, or zero if absent.
    pub saved_decoded_bytes: usize,
    /// Always false: this operation does not decode ordinary payloads.
    pub ordinary_payloads_decoded: bool,
    /// Always false: raw ordinary terminal targets remain unchecked.
    pub indexed_ordinary_targets_validated: bool,
    /// Always false: replay does not cover the complete document schema.
    pub complete_document_schema: bool,
    /// Zero for exact serialized replay; no geometric error claim is made.
    pub achieved_byte_error: usize,
}
/// Qualifications retained alongside every completed replay.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowIndexDiagnostic {
    /// Maps start empty; existing native map state is not emulated.
    FreshEmptyMapsAssumed,
    /// Ordinary payload bytes have not been decoded.
    OrdinaryPayloadsNotDecoded,
    /// Ordinary indexed targets have not been checked against physical records.
    OrdinaryTargetsNotValidated,
    /// Strict writer-profile decoding does not emulate every native recovery path.
    NativeRecoveryNotReproduced,
    /// Installed samples exercise scanning rather than populated terminal adoption.
    InstalledTerminalPayloadNotVerified,
    /// The complete document schema has not been covered.
    CompleteDocumentSchemaNotChecked,
    /// Native application acceptance has not been executed.
    NativeApplicationNotChecked,
}
