use super::*;
/// Independent complete decoded-body and actual entry bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowControlOptions {
    /// Complete decoded bytes; at least 16 for saved spans and four for an index.
    pub maximum_decoded_bytes: usize,
    /// Actual stored entries; zero admits a complete empty table.
    pub maximum_entries: usize,
}
/// Explicit carrier length/footer context; four bytes does not enable another physical carrier format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowControlLengthWidth {
    /// Four-byte unsigned backward distance, supported by synthetic writer evidence only.
    FourBytes,
    /// Eight-byte unsigned backward distance.
    EightBytes,
}
impl FlowControlLengthWidth {
    /// Returns the declared distance-field width; e.g. `assert_eq!(FlowControlLengthWidth::EightBytes.byte_width(), 8)`.
    pub fn byte_width(self) -> usize {
        match self {
            Self::FourBytes => 4,
            Self::EightBytes => 8,
        }
    }
}
#[derive(Clone, Debug)]
pub(super) struct ControlWire {
    pub source: FlowCarrier,
    pub record: FlowRecordId,
    pub decoded: Arc<[u8]>,
    pub stored_bytes: usize,
    pub identifier: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct IndexFooter {
    pub start: usize,
    pub distance: u64,
}
/// Immutable complete saved-span writer body with ordered raw pairs, not an adopted map.
#[derive(Clone, Debug)]
pub struct FlowSavedSpans {
    pub(super) wire: ControlWire,
    pub(super) options: FlowControlOptions,
    pub(super) count: usize,
}
/// Borrowed physical raw pair; duplicate keys retain separate ordinals.
#[derive(Clone, Copy, Debug)]
pub struct FlowSavedSpan<'a> {
    pub(super) owner: &'a FlowSavedSpans,
    pub(super) ordinal: usize,
    pub(super) start: usize,
}
/// Immutable complete strict index payload and verified footer under explicit length context.
#[derive(Clone, Debug)]
pub struct FlowTerminalIndex {
    pub(super) wire: ControlWire,
    pub(super) options: FlowControlOptions,
    pub(super) width: FlowControlLengthWidth,
    pub(super) footer: IndexFooter,
    pub(super) count: usize,
}
/// Borrowed raw index entry without physical target validation or deduplication.
#[derive(Clone, Copy, Debug)]
pub struct FlowTerminalIndexEntry<'a> {
    pub(super) owner: &'a FlowTerminalIndex,
    pub(super) ordinal: usize,
    pub(super) start: usize,
}
/// Exact writer framing scope; no universal native-reader claim is made.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowControlCheckMode {
    /// Zero decoded marker, unsigned count, complete pairs and one exact zlib stream.
    ExactSavedSpanWriter,
    /// Zero raw marker, exact zlib body, signed count and exact checked footer.
    ExactTerminalIndexWriter,
}
/// Complete writer-profile framing with transport and entry extents kept separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowControlCoverage {
    /// Exact documented writer-profile checks.
    pub mode: FlowControlCheckMode,
    /// Explicit footer context for indexes; absent for saved spans.
    pub length_width: Option<FlowControlLengthWidth>,
    /// Entire stored physical payload.
    pub stored_bytes: usize,
    /// Entire decoded body through EOF.
    pub decoded_bytes: usize,
    /// Actual ordered entries, including duplicates.
    pub stored_entries: usize,
    /// Decoded marker/count prefix bytes.
    pub decoded_prefix_bytes: usize,
    /// Raw decoded entry bytes.
    pub entry_bytes: usize,
    /// Entire checksum-valid compressed member.
    pub compressed_bytes: usize,
    /// Raw stored marker prefix bytes, outside compression.
    pub raw_prefix_bytes: usize,
    /// Raw checked footer bytes, outside compression.
    pub footer_bytes: usize,
    /// Zero for exact serialized checks; no geometric accuracy is asserted.
    pub achieved_error: usize,
}
/// Native, semantic and cross-record exclusions accompanying strict payload results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowControlDiagnostic {
    /// The strict zero-marker writer profile is assumed, not universal reader acceptance.
    WriterProfileAssumed,
    /// No installed native control-payload fixture has been verified.
    InstalledNativePayloadNotVerified,
    /// Native writer or reader execution was not reproduced.
    NativeRuntimeNotReproduced,
    /// Raw saved-span key/value roles remain unresolved.
    SavedSpanMeaningsNotAssigned,
    /// Index entries were not checked against physical target records.
    IndexTargetsNotValidated,
    /// Parsing did not adopt an index into the physical carrier.
    IndexAdoptionNotPerformed,
    /// Opaque control records elsewhere in the carrier were not repaired after edits.
    OtherControlRecordsNotRewritten,
    /// Four-byte footer evidence does not establish another physical carrier format.
    FourByteCarrierFormatNotEstablished,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
