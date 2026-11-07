/// Explicit enabled-state assumption required by the native owner, without reconstructing it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockExtrasEnabled {
    /// Caller explicitly selects the enabled record-408 branch.
    AssumeEnabled,
}

/// Complete physical scope, retaining all parent ordinals without assigning tree order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockExtraEntity {
    /// Complete selected record, including version.
    Record,
    /// Physical leaf ordinal.
    Block(usize),
    /// Present body for a physical leaf.
    Body(usize),
    /// First-list item and its complete nested fields.
    First {
        /// Physical leaf ordinal.
        block: usize,
        /// Physical item ordinal within its list or row.
        index: usize,
    },
    /// First-item mixed table.
    FirstMixed {
        /// Physical leaf ordinal.
        block: usize,
        /// Physical item ordinal within its list or row.
        index: usize,
    },
    /// Second-list item.
    Second {
        /// Physical leaf ordinal.
        block: usize,
        /// Physical item ordinal within its list or row.
        index: usize,
    },
    /// Third-list item.
    Third {
        /// Physical leaf ordinal.
        block: usize,
        /// Physical item ordinal within its list or row.
        index: usize,
    },
    /// Fourth count, wide words and bytes.
    Fourth(usize),
    /// One nested row selected by its fourth-list ordinal.
    NestedRow {
        /// Physical leaf ordinal.
        block: usize,
        /// Existing fourth-list row ordinal.
        row: usize,
    },
    /// Nested entry, retaining its row and local ordinal.
    NestedEntry {
        /// Physical leaf ordinal.
        block: usize,
        /// Existing fourth-list row ordinal.
        row: usize,
        /// Physical item ordinal within its list or row.
        index: usize,
    },
    /// Present final integer tail.
    Final(usize),
    /// Present additional mixed tail.
    Extra(usize),
    /// Mixed table in an additional tail.
    ExtraMixed(usize),
}

/// Exact stored field role. Counts and versions are structural and cannot be edited.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockExtraFieldKind {
    /// Legacy version.
    Version,
    /// Signed leaf token.
    Token,
    /// Raw version-three-plus body flag; any nonzero value selects a body.
    BodyFlag,
    /// First-list byte count.
    FirstCount,
    /// Second-list byte count.
    SecondCount,
    /// Third-list byte count.
    ThirdCount,
    /// Two raw item bytes.
    PairBytes,
    /// Five signed first-item words.
    FirstWords,
    /// Signed mixed-element count.
    MixedCount,
    /// Signed mixed words, parallel or strided.
    MixedWords,
    /// Raw mixed wide patterns, parallel or strided.
    MixedWide,
    /// Two signed legacy or raw wide reference words.
    ReferenceWords,
    /// Byte-array length.
    ByteArrayCount,
    /// Exact raw byte array, including present empty arrays.
    ByteArray,
    /// Conditional additional-array flag; any nonzero value selects the array.
    AdditionalFlag,
    /// Three signed words per preceding byte when selected.
    AdditionalWords,
    /// Raw second-item tag.
    TagByte,
    /// Signed third-item word.
    ThirdWord,
    /// Four raw third-item wide patterns.
    ThirdWide,
    /// Fourth-list byte count.
    FourthCount,
    /// Three raw wide patterns per fourth item.
    FourthWide,
    /// Raw fourth-item bytes.
    FourthBytes,
    /// Outer presence flag; only one selects nested/final flags.
    OuterFlag,
    /// Nested presence flag; only one selects rows.
    NestedFlag,
    /// Version-selected byte or signed row count.
    NestedCount,
    /// Byte count of nested words.
    NestedElementCount,
    /// Stored version-six-plus nested index.
    NestedIndex,
    /// Signed nested words.
    NestedWords,
    /// Three raw wide patterns per nested word.
    NestedWide,
    /// Final presence flag; only one selects final fields.
    FinalFlag,
    /// Signed final word.
    FinalWord,
    /// Signed final array count.
    FinalCount,
    /// Signed final array.
    FinalWords,
    /// Additional tail flag; any nonzero value selects extra fields.
    ExtraFlag,
    /// Signed additional-tail word.
    ExtraWord,
    /// Raw version-four-plus final byte.
    LastByte,
}

/// Exact field storage encoding, without engineering coercion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockExtraEncoding {
    /// Raw one-byte value.
    Byte,
    /// Signed little-endian four-byte word.
    Signed32,
    /// Raw little-endian eight-byte pattern.
    WideBits,
}

/// Exact value variant required for an existing field element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockExtraValue {
    /// Raw byte.
    Byte(u8),
    /// Signed four-byte word.
    Signed32(i32),
    /// Unnormalized wide pattern.
    WideBits(u64),
}

impl FlowBlockExtraFieldKind {
    /// Reports structural fields that cannot be edited; e.g. `let fixed = field.kind().is_structural()`.
    pub fn is_structural(self) -> bool {
        use FlowBlockExtraFieldKind::*;
        matches!(
            self,
            Version
                | FirstCount
                | SecondCount
                | ThirdCount
                | MixedCount
                | ByteArrayCount
                | FourthCount
                | NestedCount
                | NestedElementCount
                | FinalCount
        )
    }
}

impl FlowBlockExtraEncoding {
    /// Returns the exact element width; e.g. `let width = field.encoding().element_bytes()`.
    pub fn element_bytes(self) -> usize {
        match self {
            Self::Byte => 1,
            Self::Signed32 => 4,
            Self::WideBits => 8,
        }
    }
}
