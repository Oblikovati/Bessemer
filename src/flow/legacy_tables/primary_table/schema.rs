/// Explicit primary-class selection; bytes alone do not establish native allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPrimaryTableProfile {
    /// Complete raw primary-class versions zero through three.
    PrimaryClass,
}

/// Physical ancestry, preserving empty scopes and both group positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPrimaryScope {
    /// Complete version-prefixed record.
    Record,
    /// Physical item ordinal.
    Item(usize),
    /// Item and group ordinals.
    Group(usize, usize),
    /// Present optional body, identified by item and group.
    Body(usize, usize),
    /// Item, group and typed-element ordinals.
    Element(usize, usize, usize),
    /// Item, group, element and row ordinals.
    Row(usize, usize, usize, usize),
    /// Item, group, element and wide-block ordinals.
    WideBlock(usize, usize, usize, usize),
}

/// Exact serialized role; counts and version are immutable framing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPrimaryFieldKind {
    /// Signed version prefix.
    Version,
    /// Nonnegative item count.
    ItemCount,
    /// Raw kind; two and three select typed lists.
    Kind,
    /// Raw factory tag, admitted only when its typed list is populated.
    ElementKind,
    /// Version-three mode byte.
    Mode,
    /// Two signed group members.
    Members,
    /// Body selection uses any nonzero stored byte.
    Presence,
    /// First vector count.
    FirstCount,
    /// First raw word vector, including an empty vector.
    FirstWords,
    /// Independent second vector count.
    SecondCount,
    /// Second raw word vector.
    SecondWords,
    /// Three signed body members.
    Triplet,
    /// Six raw wide patterns.
    WideSix,
    /// Three raw wide patterns.
    WideThree,
    /// Version-zero legacy wide patterns.
    LegacyWideNine,
    /// Version-two-plus flag; native projection is true only for exactly one.
    StoredFlag,
    /// Typed-list element count, independent of optional body presence.
    ElementCount,
    /// Ragged row count.
    RowCount,
    /// Entry count in a row.
    EntryCount,
    /// First word column in every typed layout.
    RowFirst,
    /// Second word column in factory two.
    RowSecond,
    /// Wide column in factories three and four.
    RowWide,
    /// Final word column in factory four.
    RowLast,
    /// Independent sixteen-pattern block count.
    WideBlockCount,
    /// One complete sixteen-pattern block.
    WideBlock,
}

impl FlowPrimaryFieldKind {
    /// Identifies immutable framing; e.g. `let framing = field.kind().is_structural()`.
    pub fn is_structural(self) -> bool {
        use FlowPrimaryFieldKind::*;
        matches!(
            self,
            Version
                | ItemCount
                | FirstCount
                | SecondCount
                | ElementCount
                | RowCount
                | EntryCount
                | WideBlockCount
        )
    }
}

/// Exact storage representation without numeric or engineering coercion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPrimaryValue {
    /// One stored byte.
    Byte(u8),
    /// Signed little-endian member or count.
    Signed32(i32),
    /// Raw four-byte pattern.
    WordBits(u32),
    /// Raw eight-byte pattern.
    WideBits(u64),
}

/// Width and interpretation of one stored element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPrimaryEncoding {
    /// One raw byte.
    Byte,
    /// Signed four-byte member.
    Signed32,
    /// Raw four-byte pattern.
    WordBits,
    /// Raw eight-byte pattern.
    WideBits,
}

impl FlowPrimaryEncoding {
    /// Returns serialized width without native alignment; e.g. `let bytes = field.encoding().width()`.
    pub fn width(self) -> usize {
        match self {
            Self::Byte => 1,
            Self::Signed32 | Self::WordBits => 4,
            Self::WideBits => 8,
        }
    }
}

/// Required exclusions accompanying exact physical coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPrimaryDiagnostic {
    /// Caller selected the documented primary class.
    PrimaryClassAssumed,
    /// Populated native examples have not been accepted.
    PopulatedNativeNotChecked,
    /// Native allocation and runtime state are not reproduced.
    NativeRuntimeNotReproduced,
    /// Raw patterns retain no assigned engineering meaning.
    EngineeringRolesNotResolved,
    /// Native application acceptance has not been executed.
    NativeApplicationNotChecked,
}
