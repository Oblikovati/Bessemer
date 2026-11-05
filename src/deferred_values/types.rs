use super::{Result, unsupported};

/// Caller-selected state of the installed writer's mutable byte-order control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeferredValueProfile {
    /// Control0 reverses installed x64 native bytes into big-endian payloads.
    BigEndianControl0,
    /// Control1 retains installed x64 little-endian payloads.
    LittleEndianControl1,
}

impl DeferredValueProfile {
    /// Parses the control once without inferring runtime state; e.g. `DeferredValueProfile::from_control(0)?`.
    pub fn from_control(control: u8) -> Result<Self> {
        match control {
            0 => Ok(Self::BigEndianControl0),
            1 => Ok(Self::LittleEndianControl1),
            _ => Err(unsupported(format!(
                "byte-order control {control}; expected 0 or 1"
            ))),
        }
    }
}

/// Explicit memory/traversal budgets; root depth is zero and the host recursion ceiling is64.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeferredValueLimits {
    /// Maximum complete encoded blob length.
    pub maximum_bytes: usize,
    /// Maximum visited values, including object keys and container roots.
    pub maximum_values: usize,
    /// Maximum child depth, from0 through64.
    pub maximum_depth: usize,
}

/// Persisted value forms shared by all17 installed concrete writers.
/// Integer constructor widths are not recoverable from identical minimal encodings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeferredValue {
    /// Null tag.
    Null,
    /// Encoded false/true; original nonzero runtime Boolean bytes are not serialized.
    Boolean(bool),
    /// Negative signed64 value; nonnegative constructor inputs are rejected.
    NegativeInteger(i64),
    /// Nonnegative value, including positive signed-writer outputs.
    UnsignedInteger(u64),
    /// Raw binary32 bits, including NaN payloads and signed zero.
    Float32Bits(u32),
    /// Raw binary64 bits, including NaN payloads and signed zero.
    Float64Bits(u64),
    /// Stored byte string, without UTF-8 or first-NUL normalization.
    StringBytes(Vec<u8>),
    /// Opaque binary payload.
    BinaryBytes(Vec<u8>),
    /// Ordered recursive elements.
    Array(Vec<DeferredValue>),
    /// Encoded recursive pairs in order; duplicate keys remain distinct.
    ObjectPairs(Vec<(DeferredValue, DeferredValue)>),
    /// Raw extension discriminator and payload.
    Extension {
        /// Entire stored type byte.
        type_byte: u8,
        /// Uninterpreted extension bytes.
        bytes: Vec<u8>,
    },
}

/// Explicit syntax coverage, independent of deferred-object or database meanings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeferredValueCoverage {
    /// Complete blob bytes checked through EOF against the selected writer form.
    pub encoded_bytes: usize,
    /// All visited roots, child values and object keys.
    pub values: usize,
    /// Deepest parsed child depth.
    pub deepest_depth: usize,
}
