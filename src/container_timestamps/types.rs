use super::*;
/// Explicit unsigned native inputs, without a clock or calendar normalization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZipTimestampRequest {
    /// Seconds, minutes, hours, day, zero-based month and year; absent calendar input yields zero unless overridden.
    pub calendar_words: Option<[u32; 6]>,
    /// A nonzero raw packed word takes precedence over every calendar word.
    pub packed_override: u32,
}
/// Immutable whole-file checkpoint with validated existing-entry timestamp locations.
#[derive(Clone)]
pub struct ZipEntryTimestamps {
    pub(super) revision: ArchiveRevision,
    pub(super) raw: Arc<[u8]>,
    pub(super) entries: Arc<[TimestampSpan]>,
    pub(super) maximum: usize,
}
#[derive(Clone, Debug)]
pub(super) struct TimestampSpan {
    pub name: Arc<str>,
    pub local: (usize, usize),
    pub central: (usize, usize),
    pub packed: u32,
}
/// Entry identity bound to one immutable checkpoint and central-directory ordinal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZipTimestampEntryId {
    pub(super) revision: ArchiveRevision,
    pub(super) ordinal: usize,
}
/// Borrowed existing entry; reading takes no lock and never inflates payloads.
#[derive(Clone, Copy, Debug)]
pub struct ZipTimestampEntry<'a> {
    pub(super) owner: &'a ZipEntryTimestamps,
    pub(super) ordinal: usize,
    pub(super) span: &'a TimestampSpan,
}
/// Exact framing scope of a timestamp checkpoint; payload semantics remain separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZipTimestampCheckMode {
    /// Complete container directory/local-header consistency and all timestamp pairs.
    ExactContainerHeaders,
}
/// Complete structural counts, independent of payload CRC or application acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZipTimestampCoverage {
    /// Always complete structural header validation.
    pub mode: ZipTimestampCheckMode,
    /// Complete immutable checkpoint length.
    pub file_bytes: usize,
    /// Every parsed central-directory entry.
    pub entries: usize,
    /// Both stored timestamp copies for every entry.
    pub timestamp_copies: usize,
    /// Always false: this operation does not inflate and CRC-check payloads.
    pub payload_crc_checked: bool,
    /// Always zero: stored-bit arithmetic and byte edits are exact.
    pub achieved_error: usize,
}
/// Responsibilities excluded from exact timestamp framing and arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZipTimestampDiagnostic {
    /// Compressed payload CRC and model contents were not validated.
    PayloadsNotValidated,
    /// Native uninitialized metadata, timezones and wall-clock state were not reconstructed.
    NativeMetadataProvenanceNotReproduced,
    /// Native application loading was not executed.
    NativeApplicationNotChecked,
}
/// Relation for one complete interval with its retained source parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZipTimestampByteRelation {
    /// This interval is byte-identical in both checkpoints.
    Copied,
    /// This stored timestamp word changed on an isolated copy.
    Derived,
}
/// Complete ordered byte partition; enclosing edit retains both parent/result identities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZipTimestampByteHistory {
    /// Half-open source interval.
    pub source_interval: (usize, usize),
    /// Half-open corresponding result interval.
    pub result_interval: (usize, usize),
    /// Copied bytes or an explicitly changed stored word.
    pub relation: ZipTimestampByteRelation,
}
/// Completed isolated edit, with immutable checkpoints, coverage, diagnostics and history.
#[derive(Clone, Debug)]
pub struct ZipTimestampEdit {
    pub(super) source: ZipEntryTimestamps,
    pub(super) result: ZipEntryTimestamps,
    pub(super) ordinal: usize,
    pub(super) history: Vec<ZipTimestampByteHistory>,
}
