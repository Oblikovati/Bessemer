pub use super::super::primary_table::FlowPrimaryFieldLayout as FlowLoadFieldLayout;
use super::*;

/// Explicit caller assumption, not a reconstructed native owner mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLoadMode {
    /// Select the reference's true branch, omitting the final mode tail.
    AssumeTrue,
    /// Select the false branch, retaining the final tail for nonempty records.
    AssumeFalse,
}
impl FlowLoadMode {
    /// Returns the selected native boolean branch; e.g. `let mode = state.mode().value()`.
    pub fn value(self) -> bool {
        self == Self::AssumeTrue
    }
}

/// Complete physical ancestry, retaining empty strings and the selected tail.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLoadScope {
    /// Complete raw payload.
    Record,
    /// Physical item ordinal.
    Item(usize),
    /// Physical item and name ordinals; the name ordinal is zero or one.
    Name(usize, u8),
    /// Description belonging to one physical item.
    Description(usize),
    /// Present final word/wide arrays under false mode.
    ModeTail,
}

/// Exact stored role; native member tuples retain their documented physical order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLoadFieldKind {
    /// Version18 prefix.
    Version,
    /// Nonnegative item count.
    ItemCount,
    /// Signed root member at native offset0x130.
    Member130,
    /// Stored width shared by per-item and root arrays.
    ArrayWidth,
    /// UTF16 code-unit count.
    TextUnitCount,
    /// Full raw UTF16 units, including hidden suffixes and unpaired surrogates.
    TextUnits,
    /// Nine signed item words in documented native-offset order.
    MemberWords,
    /// Two signed states at offsets0xf0/0xf4.
    States,
    /// Raw wide pattern at native offset0xf8.
    WideF8,
    /// Signed member at native offset0x1f8.
    Member1F8,
    /// Two raw wide patterns at offsets0xa0/0xb0.
    WideA0B0,
    /// One of four per-item wide arrays.
    ItemWideArray(u8),
    /// Independent extra word count.
    ExtraCount,
    /// Raw extra words.
    ExtraWords,
    /// One of four shared arrays: word, word, wide, wide.
    SharedArray(u8),
    /// Two signed root members at offsets0x128/0x12c.
    FinalMembers,
    /// Sixteen raw four-byte patterns in the mode tail.
    ModeWords,
    /// Sixteen raw eight-byte patterns in the mode tail.
    ModeWide,
}
impl FlowLoadFieldKind {
    /// Reports immutable framing; e.g. `let framing = field.kind().is_structural()`.
    pub fn is_structural(self) -> bool {
        matches!(
            self,
            Self::Version
                | Self::ItemCount
                | Self::ArrayWidth
                | Self::TextUnitCount
                | Self::ExtraCount
        )
    }
}

/// Storage encoding without extending existing public scalar enums.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLoadEncoding {
    /// Existing exact scalar encoding.
    Scalar(FlowPrimaryEncoding),
    /// One raw little-endian UTF16 unit.
    Utf16Unit,
}
impl FlowLoadEncoding {
    /// Returns the exact serialized width; e.g. `let width = field.encoding().width()`.
    pub fn width(self) -> usize {
        match self {
            Self::Scalar(encoding) => encoding.width(),
            Self::Utf16Unit => 2,
        }
    }
}

/// Exact existing-field value, keeping UTF16 distinct from generic numeric words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLoadValue {
    /// Existing raw scalar value.
    Scalar(FlowPrimaryValue),
    /// One raw UTF16 unit, including surrogates and zero.
    Utf16Unit(u16),
}

/// Documented native member offsets, normalized once to the stored nine-word order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FlowLoadMemberOffset {
    /// Native offset0x08, stored slot0.
    At08 = 8,
    /// Native offset0x18, stored slot1.
    At18 = 24,
    /// Native offset0x0c, stored slot2.
    At0C = 12,
    /// Native offset0x10, stored slot3.
    At10 = 16,
    /// Native offset0x14, stored slot4.
    At14 = 20,
    /// Native offset0x1c, stored slot5.
    At1C = 28,
    /// Native offset0x28, stored slot6.
    At28 = 40,
    /// Native offset0x24, stored slot7.
    At24 = 36,
    /// Native offset0x2c, stored slot8.
    At2C = 44,
}
impl TryFrom<u32> for FlowLoadMemberOffset {
    type Error = BessemerError;
    fn try_from(value: u32) -> Result<Self> {
        match value {
            8 => Ok(Self::At08),
            24 => Ok(Self::At18),
            12 => Ok(Self::At0C),
            16 => Ok(Self::At10),
            20 => Ok(Self::At14),
            28 => Ok(Self::At1C),
            40 => Ok(Self::At28),
            36 => Ok(Self::At24),
            44 => Ok(Self::At2C),
            _ => Err(load_invalid(format!(
                "native member offset {value}; expected one of8,24,12,16,20,28,40,36,44"
            ))),
        }
    }
}
impl FlowLoadMemberOffset {
    /// Returns the native byte offset; e.g. `let offset = member.value()`.
    pub fn value(self) -> u32 {
        self as u32
    }
    /// Returns its physical signed-word slot; e.g. `let edit = state.replace_field(&id, member.slot(), value)?`.
    pub fn slot(self) -> usize {
        match self {
            Self::At08 => 0,
            Self::At18 => 1,
            Self::At0C => 2,
            Self::At10 => 3,
            Self::At14 => 4,
            Self::At1C => 5,
            Self::At28 => 6,
            Self::At24 => 7,
            Self::At2C => 8,
        }
    }
}

/// Exact framing check with explicit caller mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLoadCheckMode {
    /// Complete raw version18 framing through exact EOF under the declared mode.
    ExactRawDeclaredLoadMode,
}

/// Required exclusions from complete serialized acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLoadDiagnostic {
    /// Native owner mode was supplied, not reconstructed.
    LoadModeContextAssumed,
    /// Native runtime transformations were not executed.
    RuntimeTransformationsNotReproduced,
    /// Engineering meanings/units remain unresolved.
    EngineeringRolesNotResolved,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
