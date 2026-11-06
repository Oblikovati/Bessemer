use super::*;

/// External legacy member count, admitted once to the native nonnegative signed domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSimpleLegacyCount(pub(super) u32);

impl TryFrom<u32> for FlowSimpleLegacyCount {
    type Error = BessemerError;
    fn try_from(value: u32) -> Result<Self> {
        if value <= i32::MAX as u32 {
            return Ok(Self(value));
        }
        Err(BessemerError::new(
            "Flow simple table context",
            "legacy member count",
            FailureClass::Invalid,
            format!("count {value}; expected zero through {}", i32::MAX),
        ))
    }
}

impl FlowSimpleLegacyCount {
    /// Returns the admitted count; e.g. `let count = legacy.value()`.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Explicit transport and bounds for the two parallel groups.
#[derive(Clone, Copy, Debug)]
pub struct FlowParallelTableOptions {
    /// Caller-selected raw/zlib transport.
    pub compression: FlowLegacyTableTransport,
    /// Complete decoded byte limit, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive total wide-word limit for present arrays only.
    pub maximum_wide_words: usize,
}

/// Required leaf context and independent physical row bounds.
#[derive(Clone, Copy, Debug)]
pub struct FlowLeafPairOptions {
    /// Caller-selected raw/zlib transport.
    pub compression: FlowLegacyTableTransport,
    /// Explicit leaf count, without inferred tree ordering.
    pub leaf_count: FlowLeafCount,
    /// Complete decoded byte limit, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum number of wire rows.
    pub maximum_wire_rows: usize,
}

/// Explicit optional legacy count and bounds for actual five-word wire rows.
#[derive(Clone, Copy, Debug)]
pub struct FlowFiveWordTableOptions {
    /// Caller-selected raw/zlib transport.
    pub compression: FlowLegacyTableTransport,
    /// Optional in versions zero through two, required in three, forbidden in four and later.
    pub legacy_count: Option<FlowSimpleLegacyCount>,
    /// Complete decoded byte limit, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum actual wire rows; inactive external counts do not allocate rows.
    pub maximum_wire_rows: usize,
}

/// Explicit legacy byte count and independent actual byte bound.
#[derive(Clone, Copy, Debug)]
pub struct FlowRawByteTableOptions {
    /// Caller-selected raw/zlib transport.
    pub compression: FlowLegacyTableTransport,
    /// Required in version zero, forbidden in positive versions.
    pub legacy_count: Option<FlowSimpleLegacyCount>,
    /// Complete decoded byte limit, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum actual raw byte count.
    pub maximum_raw_bytes: usize,
}
