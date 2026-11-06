pub use super::super::primary_table::FlowPrimaryEncoding as FlowConditionalEncoding;
pub use super::super::primary_table::FlowPrimaryFieldLayout as FlowConditionalFieldLayout;
pub use super::super::primary_table::FlowPrimaryValue as FlowConditionalValue;

/// Supported complete wire grammar, selected by a typed carrier entry point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowConditionalFormat {
    /// Raw version-one parallel arrays or version-two three-dimensional items.
    Record126,
    /// Whole-payload zlib version four with one-dimensional items.
    Record315,
    /// Raw version six/seven followed by a zlib block-vector body.
    Record128,
}

impl FlowConditionalFormat {
    pub(super) fn dimension(self) -> i32 {
        if self == Self::Record126 { 3 } else { 1 }
    }

    /// Returns the physical record identifier; e.g. `let id = items.format().identifier()`.
    pub fn identifier(self) -> u32 {
        match self {
            Self::Record126 => 126,
            Self::Record315 => 315,
            Self::Record128 => 128,
        }
    }
}

/// Physical scope kind; parent IDs and local ordinals retain the complete ancestry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowConditionalScopeKind {
    /// Complete decoded record or decoded body for record128.
    Record,
    /// One of the two record315 headers, or record128's sole header.
    HeaderGroup,
    /// One physical presence position, including an absent item.
    Entry,
    /// Present item, including an inactive item.
    Item,
    /// Legacy fixed group or modern counted item group.
    ItemGroup,
    /// Legacy pair member or one modern group's subrecord.
    Subrecord,
}

/// Exact stored role, with raw members kept separate from structural counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowConditionalFieldKind {
    /// Outer, header or item version.
    Version,
    /// Shared parallel-array or presence-list count.
    Count,
    /// Signed record126 inner word, constrained below one.
    InnerWord,
    /// Signed record126 member.
    MemberA0Signed,
    /// Header mode, constrained to zero through two.
    Mode,
    /// Required item/header dimension.
    Dimension,
    /// Conditional signed record126 gate.
    GateSigned,
    /// Independent leading record315 vector count.
    LegacyArrayCount(u8),
    /// Leading wide vector, or record126's parallel array.
    LegacyArrayValues(u8),
    /// Independent common-vector count.
    CommonCount,
    /// Common wide vector.
    CommonValues,
    /// Independent record315 extra-vector count.
    ExtraCount,
    /// Signed extra mode present only for a positive extra count.
    ExtraMode,
    /// Extra raw wide patterns.
    ExtraValues,
    /// Raw unsigned header member.
    MemberA0Pattern,
    /// Required protocol word equal to one.
    Protocol,
    /// Conditional unsigned header gate.
    GatePattern,
    /// Presence byte; every nonzero value selects an item.
    Presence,
    /// Activity byte; every nonzero value selects the active body.
    Active,
    /// Signed legacy item word.
    LegacyWord,
    /// Signed item member.
    Member84,
    /// Stored mode byte with a nonzero native projection.
    ItemMode,
    /// Modern group count.
    GroupCount,
    /// Logical wide-vector count, before record128's block multiplier.
    WideCount,
    /// Individual wide patterns in a logical wide vector.
    WideValues,
    /// Independent byte-vector count.
    BytesCount,
    /// Complete raw byte vector, including embedded zeros.
    Bytes,
    /// Version-one legacy word-vector count.
    LegacyVectorCount(u8),
    /// Version-one legacy word vector.
    LegacyVectorValues(u8),
    /// Arbitrary signed subrecord kind; two alone selects an extra vector.
    SubKind,
    /// First signed subrecord member.
    MemberC,
    /// Second signed subrecord member.
    Member10,
    /// First subrecord word-vector count.
    FirstCount,
    /// First raw subrecord word vector.
    FirstValues,
    /// Conditional extra subrecord word-vector count.
    SubExtraCount,
    /// Conditional extra raw subrecord word vector.
    SubExtraValues,
}

impl FlowConditionalFieldKind {
    /// Identifies immutable version/count framing; e.g. `let framing = field.kind().is_structural()`.
    pub fn is_structural(self) -> bool {
        use FlowConditionalFieldKind::*;
        matches!(
            self,
            Version
                | Count
                | LegacyArrayCount(_)
                | CommonCount
                | ExtraCount
                | GroupCount
                | WideCount
                | BytesCount
                | LegacyVectorCount(_)
                | FirstCount
                | SubExtraCount
        )
    }
}

/// Exact framing scope, retaining the raw-prefix/body-codec distinction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowConditionalCheckMode {
    /// Complete raw payload through EOF.
    ExactRawPayload,
    /// Complete checksum-valid single zlib stream and decoded EOF.
    ExactZlibPayload,
    /// Four raw version bytes and one exact zlib body through decoded EOF.
    ExactRawVersionZlibBody,
}

/// Required exclusions from exact serialized acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowConditionalDiagnostic {
    /// Native allocation/conversion state is not reproduced.
    NativeRuntimeNotReproduced,
    /// Native inverse maps remain unresolved.
    NativeInverseMapsNotReproduced,
    /// Raw values retain no assigned engineering units or meaning.
    EngineeringRolesNotResolved,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
    /// Changed compressed streams are reencoded; retained source checkpoints preserve originals.
    CompressedRepresentationReencodedOnChange,
}
