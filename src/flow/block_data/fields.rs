use super::*;

/// Raw field value with explicit stored/logical width; no engineering conversion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockFieldValue {
    /// Full unsigned byte.
    Byte(u8),
    /// Full unsigned sixteen-bit word.
    Word16(u16),
    /// Signed field or sign/zero-extended versioned second-array count.
    Signed32(i32),
    /// Full raw unsigned array word.
    Word32(u32),
}

/// Existing block header fields; negative counts remain raw stored words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockHeaderField {
    /// Stored byte, not the native runtime result of its operation.
    BlockByte,
    /// Native member 10 unsigned word.
    Word10,
    /// Native member 12 raw word with a signed-i16 count view.
    Count12,
}

/// Existing entry fields, with version-one member 20 retaining its wire alias.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockEntryField {
    /// Signed identifier without an inferred domain.
    Identifier,
    /// Stored legacy index before version twelve.
    LegacyIndex,
    /// Member 20, aliased to the legacy index in version one.
    Member20,
    /// Signed member 28.
    Member28,
    /// Discarded prefix in versions eleven through thirteen.
    DiscardedPrefix,
    /// Full state byte.
    State,
    /// Full flag byte present from version five.
    Flag2,
    /// Full flag byte present from version seventeen.
    Flag4,
    /// One of seven descriptors; index six also stores the first-array length.
    Descriptor {
        /// Existing descriptor index zero through six.
        index: usize,
    },
    /// A raw first-array word.
    FirstWord {
        /// Existing word index.
        index: usize,
    },
    /// Versioned second-array count as a signed i32 view.
    SecondCount,
    /// A raw second-array word.
    SecondWord {
        /// Existing word index.
        index: usize,
    },
    /// Discarded suffix present in versions sixteen and seventeen.
    DiscardedSuffix,
}

/// Existing global count label; neither label establishes the leaf count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockGlobalCount {
    /// G.Fluid prefix word.
    Fluid,
    /// G.Solid prefix word.
    Solid,
}

/// One fixed-width field replacement; structural array/cardinality changes are rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockFieldEdit {
    /// Nonnegative global count replacement retaining the physical bitmap length.
    GlobalCount {
        /// Prefix label.
        field: FlowBlockGlobalCount,
        /// New signed nonnegative count.
        count: i32,
    },
    /// Existing bitmap byte replacement.
    BitmapByte {
        /// Existing byte offset within the bitmap.
        index: usize,
        /// Complete replacement byte.
        byte: u8,
    },
    /// Existing block header replacement.
    Block {
        /// Physical block ordinal.
        index: usize,
        /// Header field.
        field: FlowBlockHeaderField,
        /// Matching typed raw value.
        value: FlowBlockFieldValue,
    },
    /// Existing entry field replacement.
    Entry {
        /// Flat physical entry ordinal across all blocks.
        index: usize,
        /// Entry field.
        field: FlowBlockEntryField,
        /// Matching typed raw value.
        value: FlowBlockFieldValue,
    },
}

#[derive(Clone, Copy, Debug)]
pub(in crate::flow) enum ScalarKind {
    Byte,
    Word16,
    Signed32,
    Word32,
    Signed8,
    UnsignedCount16,
    NarrowWord16,
}

#[derive(Clone, Copy)]
pub(in crate::flow) struct FieldSelection {
    pub(in crate::flow) offset: usize,
    pub(in crate::flow) kind: ScalarKind,
}

impl FieldSelection {
    pub(in crate::flow) fn read(self, raw: &[u8]) -> FlowBlockFieldValue {
        let bytes = &raw[self.offset..];
        match self.kind {
            ScalarKind::Byte => FlowBlockFieldValue::Byte(bytes[0]),
            ScalarKind::NarrowWord16 => FlowBlockFieldValue::Word16(u16::from(bytes[0])),
            ScalarKind::Signed8 => FlowBlockFieldValue::Signed32(i32::from(bytes[0] as i8)),
            ScalarKind::Word16 => {
                FlowBlockFieldValue::Word16(u16::from_le_bytes([bytes[0], bytes[1]]))
            }
            ScalarKind::UnsignedCount16 => {
                FlowBlockFieldValue::Signed32(i32::from(u16::from_le_bytes([bytes[0], bytes[1]])))
            }
            ScalarKind::Signed32 => FlowBlockFieldValue::Signed32(u32_word(bytes) as i32),
            ScalarKind::Word32 => FlowBlockFieldValue::Word32(u32_word(bytes)),
        }
    }
}

pub(super) fn u32_word(raw: &[u8]) -> u32 {
    u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]])
}

pub(in crate::flow) fn encode(kind: ScalarKind, value: FlowBlockFieldValue) -> Result<Vec<u8>> {
    use FlowBlockFieldValue::*;
    let bytes = match (kind, value) {
        (ScalarKind::Byte, Byte(byte)) => vec![byte],
        (ScalarKind::NarrowWord16, Word16(word)) if word <= u8::MAX.into() => vec![word as u8],
        (ScalarKind::Word16, Word16(word)) => word.to_le_bytes().to_vec(),
        (ScalarKind::Signed32, Signed32(word)) => word.to_le_bytes().to_vec(),
        (ScalarKind::Word32, Word32(word)) => word.to_le_bytes().to_vec(),
        (ScalarKind::Signed8, Signed32(word)) if (-128..=127).contains(&word) => {
            vec![word as i8 as u8]
        }
        (ScalarKind::UnsignedCount16, Signed32(word)) if (0..=65535).contains(&word) => {
            (word as u16).to_le_bytes().to_vec()
        }
        _ => return Err(encoding_error(kind, value)),
    };
    Ok(bytes)
}

fn encoding_error(kind: ScalarKind, value: FlowBlockFieldValue) -> BessemerError {
    invalid_block(format!(
        "field value {value:?}; expected matching stored kind and representable width {kind:?}"
    ))
}
