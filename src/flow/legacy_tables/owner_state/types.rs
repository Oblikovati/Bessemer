use super::*;
/// Bound on the complete selected raw payload; zero admits a completed absent selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowOwnerStateOptions {
    /// Actual selected raw bytes; legacy requires 200 and modern requires 29.
    pub maximum_payload_bytes: usize,
}
/// Typed serialized layout, independent of engineering owner interpretation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowOwnerStateKind {
    /// Fifty raw words, without a version prefix.
    Legacy122,
    /// Version zero/one, six signed members and a state byte.
    Modern434,
}
/// Immutable complete fixed-layout owner-state record and recomputed update projection.
#[derive(Clone, Debug)]
pub struct FlowOwnerState {
    pub(super) wire: TableWire,
    pub(super) kind: FlowOwnerStateKind,
    pub(super) options: FlowOwnerStateOptions,
}
/// Native member-offset selector; these names do not assign engineering roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowOwnerMember {
    /// Native member at offset 0x8.
    Member08,
    /// Native member at offset 0xc.
    Member0C,
    /// Native member at offset 0x10.
    Member10,
    /// Native member at offset 0x14.
    Member14,
    /// Native member at offset 0x18.
    Member18,
    /// Native member at offset 0x1c.
    Member1C,
    /// Native member at offset 0x20, including the unsigned modern state byte.
    Member20,
}
/// Pure seven-member update projection; no native object is mutated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowOwnerUpdates {
    pub(super) members: [i32; 7],
}
/// Exact physical framing and documented projection coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowOwnerStateCoverage {
    /// Complete raw framing check mode.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete raw payload bytes.
    pub payload_bytes: usize,
    /// All stored legacy words or modern signed member words.
    pub raw_words: usize,
    /// Whether the documented update projection is enabled.
    pub updates_enabled: bool,
    /// Seven enabled updates or zero disabled updates.
    pub projected_updates: usize,
    /// Zero for exact serialized checks, without geometric accuracy claims.
    pub achieved_error: usize,
}
