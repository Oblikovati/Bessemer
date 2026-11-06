use super::*;

/// Independent complete raw payload and actual stored-row limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowObjectRecordOptions {
    /// Complete raw bytes, at least eight.
    pub maximum_payload_bytes: usize,
    /// Actual stored rows; zero admits stored-empty versions.
    pub maximum_rows: usize,
}
/// Immutable complete raw version-0..7 object rows and conditional legacy trailer.
#[derive(Clone, Debug)]
pub struct FlowObjectRecords {
    pub(super) wire: TableWire,
    pub(super) options: FlowObjectRecordOptions,
    pub(super) count: usize,
    pub(super) stride: usize,
}
/// Borrowed physically stored row; discarded values remain observable.
#[derive(Clone, Copy, Debug)]
pub struct FlowObjectRow<'a> {
    pub(super) owner: &'a FlowObjectRecords,
    pub(super) ordinal: usize,
    pub(super) start: usize,
}
/// Borrowed uncounted legacy trailer, separate from stored rows.
#[derive(Clone, Copy, Debug)]
pub struct FlowObjectTrailer<'a> {
    pub(super) owner: &'a FlowObjectRecords,
    pub(super) start: usize,
}
/// Serialized row field selection; indexed variants retain their physical bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowObjectField {
    /// First signed member in every stored row.
    Member10,
    /// One of the four physically serialized legacy words.
    DiscardedWord(usize),
    /// One of the 16 opaque block bytes.
    BlockByte(usize),
    /// Additional signed member in versions 4 through 7.
    Member7C,
    /// Raw state byte in versions 6 and 7.
    StateByte,
    /// One of the 56 unresolved version-7 bytes.
    ExtensionByte(usize),
}
/// Serialized legacy-trailer selection; it does not address counted rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowObjectTrailerField {
    /// Trailer's signed member, separate from counted rows.
    Member10,
    /// One of the trailer's 16 opaque block bytes.
    BlockByte(usize),
}
/// Revision-bound physical scalar identity with a checked signed-word or byte encoding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowObjectFieldId {
    pub(super) record: FlowRecordId,
    pub(super) offset: usize,
    pub(super) width: usize,
}
/// Exact framing coverage; no runtime population or geometric check is implied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowObjectRecordCoverage {
    /// Complete raw framing validation mode.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete validated payload extent.
    pub payload_bytes: usize,
    /// Actual stored row count, excluding any trailer.
    pub stored_rows: usize,
    /// Physical trailer presence, independent of the native append predicate.
    pub legacy_trailer: bool,
    /// Zero for exact byte and framing checks; no geometric accuracy is asserted.
    pub achieved_error: u64,
}
/// Explicit boundaries of complete serialized-field interoperability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowObjectRecordDiagnostic {
    /// Native identity-list contents and bindings were not verified.
    NativeIdentityBindingNotVerified,
    /// No native runtime objects were populated.
    NativeRuntimePopulationNotReproduced,
    /// Native legacy-flag lifetime was not reconstructed.
    LegacyFlagLifecycleNotReproduced,
    /// Serialized members have no assigned engineering roles.
    EngineeringRolesNotResolved,
    /// Results were not opened by SolidWorks.
    NativeApplicationNotChecked,
}
