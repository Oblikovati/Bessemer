use super::*;

/// Shared explicit raw/zlib owner transport.
pub type FlowCountTransport = FlowGroupCompression;

/// Bounded complete transport decoding with twelve known prefix bytes.
#[derive(Clone, Copy, Debug)]
pub struct FlowCountPrefixOptions {
    /// Caller-selected compression context.
    pub compression: FlowCountTransport,
    /// Maximum complete decoded prefix/body bytes, at least twelve.
    pub maximum_decoded_bytes: usize,
}

/// Documented native branch selected by the positive version, without decoding its body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCountBodyBranch {
    /// Legacy body versions one through seventeen.
    Legacy1To17,
    /// Delegated body versions eighteen and later.
    Restored18Plus,
}

/// Exact prefix scope with explicitly unchecked body bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCountPrefixCheckMode {
    /// Raw payload with exact known prefix and retained opaque tail.
    ExactRawPrefixOpaqueBody,
    /// Complete checksum-valid RFC 1950 stream with exact prefix and opaque tail.
    ExactZlibPrefixOpaqueBody,
}

/// Serialized transport/prefix coverage; no body validation credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowCountPrefixCoverage {
    /// Explicit scope of this prefix-only operation.
    pub mode: FlowCountPrefixCheckMode,
    /// Every stored payload byte.
    pub stored_bytes: usize,
    /// Every decoded payload byte.
    pub decoded_bytes: usize,
    /// Exactly twelve known version/count bytes.
    pub known_prefix_bytes: usize,
    /// Every retained body byte, without interpretation.
    pub opaque_body_bytes: usize,
    /// Exact known-prefix byte error, without approximation.
    pub achieved_prefix_error: usize,
}

/// Explicit exclusions from reading the known prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCountPrefixDiagnostic {
    /// Tail bytes are not a validated legacy or delegated body.
    OpaqueBodyNotValidated,
    /// Prefix totals alone do not establish record 433's runtime count.
    Record433ContextNotProven,
    /// The compression context was supplied without reconstructing its owner.
    OwnerCompressionContextNotReconstructed,
    /// No native context execution or application acceptance.
    NativeApplicationNotChecked,
}

/// Immutable known prefix and all original transport/body bytes.
#[derive(Clone, Debug)]
pub struct FlowContextCounts {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) decoded: Arc<[u8]>,
    pub(super) version: i32,
    pub(super) counts: GlobalCountPair,
    pub(super) options: FlowCountPrefixOptions,
    pub(super) coverage: FlowCountPrefixCoverage,
}

/// Required explicit native initialization branch assumption.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowAbsentMapInitialization {
    /// Assume an absent map is being initialized; arbitrary reused maps are excluded.
    InitializeAbsentMap,
}

/// Native eight-byte pointer layout without allocating or populating any pointers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowInitializedPointerLayout {
    /// Native pointer slot size, eight bytes.
    pub pointer_slot_bytes: u8,
    /// Mathematical complete pointer array extent.
    pub pointer_array_bytes: u64,
    /// Fluid partition starts at slot zero.
    pub fluid_start_index: u32,
    /// Solid partition starts at the validated G.Fluid count.
    pub solid_start_index: u32,
    /// Fluid partition starts at byte zero.
    pub fluid_pointer_byte_offset: u64,
    /// Solid partition byte offset, without native pointer population.
    pub solid_pointer_byte_offset: u64,
}

/// Conditional count provenance with the original complete count-prefix checkpoint.
#[derive(Clone, Debug)]
pub struct FlowInitializedCountBinding {
    pub(super) counts: FlowContextCounts,
    pub(super) count: u32,
    pub(super) assumption: FlowAbsentMapInitialization,
}

/// Independent array allocation limits; the bound count cannot be overridden here.
#[derive(Clone, Copy, Debug)]
pub struct FlowBoundArrayLimits {
    /// Maximum complete record-433 decoded bytes, at least eight.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum elements per array.
    pub maximum_elements: usize,
}

/// Complete record-433 arrays with retained conditional count provenance.
#[derive(Clone, Debug)]
pub struct FlowBoundMapArrays {
    pub(super) binding: FlowInitializedCountBinding,
    pub(super) arrays: FlowMapArrays,
}
