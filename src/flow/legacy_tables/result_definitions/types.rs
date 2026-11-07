use super::*;
/// Independent inflated-body, physical-object and total raw-value bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowResultDefinitionOptions {
    /// Complete inflated body, at least 21 bytes for five empty collections.
    pub maximum_decoded_bytes: usize,
    /// Physical primary objects, including duplicate identifiers; zero admits empty collections.
    pub maximum_objects: usize,
    /// Raw words across every shared array; zero admits present-empty arrays.
    pub maximum_values: usize,
}
/// Normalized admitted IMS definition version; independent of the enclosing byte version.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowImsDefinitionVersion {
    /// Version zero, with no arrays or allocation byte.
    V0,
    /// Version one, with only the first shared array.
    V1,
    /// Version two, with two arrays and an allocation byte.
    V2,
    /// Version three, with two additional arrays after the allocation byte.
    V3,
}
impl FlowImsDefinitionVersion {
    pub(super) fn presence(self) -> [bool; 4] {
        match self {
            Self::V0 => [false; 4],
            Self::V1 => [true, false, false, false],
            Self::V2 => [true, true, false, false],
            Self::V3 => [true; 4],
        }
    }
}
/// Physical shared-array position within one IMS definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowImsArrayKind {
    /// First array, present from IMS version one.
    First,
    /// Second array, present from IMS version two.
    Second,
    /// First extra array, present only in IMS version three.
    FirstExtra,
    /// Second extra array, present only in IMS version three.
    SecondExtra,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DefinitionSpan {
    pub start: usize,
    pub end: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ImsSpan {
    pub identifier_start: usize,
    pub guid_start: usize,
    pub body: DefinitionSpan,
    pub version: FlowImsDefinitionVersion,
    pub allocation: Option<usize>,
    pub arrays: [Option<DefinitionSpan>; 4],
}
/// Immutable complete profile with physical duplicate order and exact array presence.
#[derive(Clone, Debug)]
pub struct FlowResultDefinitions {
    pub(super) wire: TableWire,
    pub(super) options: FlowResultDefinitionOptions,
    pub(super) collections: [DefinitionSpan; 5],
    pub(super) objects: Arc<[ImsSpan]>,
    pub(super) values: usize,
}
/// Borrowed physical definition; identifiers are serialized values, not unique keys.
#[derive(Clone, Copy, Debug)]
pub struct FlowImsDefinition<'a> {
    pub(super) owner: &'a FlowResultDefinitions,
    pub(super) ordinal: usize,
    pub(super) span: &'a ImsSpan,
}
/// Exact complete compressed/body framing with separate object and array counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowResultDefinitionCoverage {
    /// Exact checksum-valid single-stream and decoded-EOF validation scope.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete stored payload extent.
    pub stored_bytes: usize,
    /// Complete inflated extent through all five collections.
    pub decoded_bytes: usize,
    /// Physical primary definitions; duplicate identifiers retain separate ordinals.
    pub stored_objects: usize,
    /// Physically present shared arrays, including empty arrays.
    pub stored_arrays: usize,
    /// Present arrays with zero values, distinct from absent version-dependent arrays.
    pub empty_arrays: usize,
    /// Total raw 32-bit patterns.
    pub raw_words: usize,
    /// Zero for exact serialized checks; no geometric accuracy is asserted.
    pub achieved_error: usize,
}
