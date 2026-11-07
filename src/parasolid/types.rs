use super::{NodeId, NodeReference, SchemaKey, StoredDouble};
use std::collections::BTreeMap;

/// Binary format marker and resulting numeric byte order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransmitFormat {
    /// `PS`, big-endian.
    Neutral,
    /// `B`, little-endian.
    Bare,
}

/// Allocation limits applied before publishing a decoded transmit file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransmitLimits {
    /// Maximum records across all initial and modified lists.
    pub maximum_records: usize,
    /// Maximum element count in any one field, additionally bounded by remaining bytes.
    pub maximum_field_elements: usize,
}

/// One transmitted primitive field kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    /// `d`, signed 32-bit integer.
    Integer,
    /// `p`, encoded node reference.
    Pointer,
    /// `u`, raw unsigned byte.
    UnsignedByte,
    /// `c`, raw character byte.
    Character,
    /// `l`, raw logical byte.
    Logical,
    /// `n`, signed 16-bit integer.
    Short,
    /// `w`, UTF-16 code unit, including an unpaired surrogate.
    WideUnit,
    /// `f`, IEEE-754 double bits.
    Real,
    /// `i`, two doubles.
    Interval,
    /// `v`, three doubles.
    Vector,
    /// `h`, three partition doubles or eleven delta doubles.
    HVector,
    /// `b`, six doubles.
    Box,
    /// `q`, retained in base-schema declarations; has no documented transmitted encoding.
    SchemaOnlyQ,
    /// `t`, retained in base-schema declarations; has no documented transmitted encoding.
    SchemaOnlyT,
}

/// Scalar, fixed-array, or record-length-controlled field shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldExtent {
    /// One element.
    Scalar,
    /// Record's stored u32 element count.
    Variable,
    /// At least two elements; descriptor encodings additionally require <= 65534.
    Fixed(u32),
}

/// Complete transmitted field definition, in layout order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDefinition {
    /// Latin-1 field name.
    pub name: String,
    /// Primitive wire kind.
    pub kind: FieldKind,
    /// Declared pointer class; zero for nonpointer fields.
    pub pointer_class: u16,
    /// Element-count rule.
    pub extent: FieldExtent,
}

/// Sequential edit of the base type's transmitted field list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutEdit {
    /// `C`, copy the next base field.
    Copy,
    /// `D`, discard the next base field.
    Drop,
    /// `I`, insert an explicit field definition.
    Insert(FieldDefinition),
    /// `A`, append an explicit field definition.
    Append(FieldDefinition),
}

/// First-record layout descriptor; edited layouts end with an implicit `Z`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutDefinition {
    /// `0xFF`, transmitted base layout.
    Base,
    /// Explicit operations against an existing base type.
    Edited(Vec<LayoutEdit>),
    /// Complete definition of a type absent from the base schema.
    Defined {
        /// Latin-1 node-type name.
        name: String,
        /// Latin-1 description.
        description: String,
        /// Transmitted fields.
        fields: Vec<FieldDefinition>,
        /// Retained variable-layout byte, separate from each field's extent.
        variable: u8,
    },
}

/// A pointer and its optional uninterpreted delta flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransmitPointer {
    /// Null or referenced node.
    pub reference: NodeReference,
    /// Present exactly in delta records.
    pub delta_flag: Option<u8>,
}

/// Typed field elements; scalar fields use exactly one element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValues {
    /// Signed 32-bit integers.
    Integers(Vec<i32>),
    /// References, including raw per-pointer delta flags.
    Pointers(Vec<TransmitPointer>),
    /// Raw unsigned, character, or logical bytes, distinguished by the layout.
    Bytes(Vec<u8>),
    /// Signed 16-bit integers.
    Shorts(Vec<i16>),
    /// UTF-16 code units without text normalization.
    WideUnits(Vec<u16>),
    /// Doubles.
    Reals(Vec<StoredDouble>),
    /// Intervals.
    Intervals(Vec<[StoredDouble; 2]>),
    /// Vectors.
    Vectors(Vec<[StoredDouble; 3]>),
    /// Partition hvecs.
    PartitionHVectors(Vec<[StoredDouble; 3]>),
    /// Delta hvecs, including all eight additional raw doubles.
    DeltaHVectors(Vec<[StoredDouble; 11]>),
    /// Boxes.
    Boxes(Vec<[StoredDouble; 6]>),
}

/// A complete record with positional typed fields matching its type's layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeRecord {
    /// Node type; 1 is reserved for list terminators.
    pub node_type: u16,
    /// Null is retained if stored; ordinary records use a node identity.
    pub index: NodeReference,
    /// Present exactly when the effective layout has a variable field.
    pub length: Option<u32>,
    /// All fields in effective layout order, including unknown-meaning fields.
    pub fields: Vec<FieldValues>,
}

/// A record list and its exact terminator index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordList {
    /// Records in stored order; delta records may repeat identities.
    pub records: Vec<NodeRecord>,
    /// Reference following node type 1.
    pub terminator: NodeReference,
}

/// A delta's declared new node and type pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewNode {
    /// Actual node identity.
    pub index: NodeId,
    /// Declared type.
    pub node_type: u16,
}

/// A nonfinal delta section's new-node declarations and modified records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaChanges {
    /// Pairs before the null index terminator.
    pub new_nodes: Vec<NewNode>,
    /// Raw stored count; never used to delimit the modified record list.
    pub count: u32,
    /// Every record is encoded as a delta, including types 3 and 4 here.
    pub modified: RecordList,
}

/// One delta section; only the final section omits changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaSection {
    /// Initial list; types 3 and 4 are plain, all others are delta records.
    pub records: RecordList,
    /// Absent only in the last section at file end.
    pub changes: Option<DeltaChanges>,
}

/// Complete partition or rollback-deltas body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransmitBody {
    /// One plain list and no trailing payload.
    Partition(RecordList),
    /// Nonempty sequence ending with a section without changes.
    Deltas(Vec<DeltaSection>),
}

/// Owned contents validated before immutable publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransmitContents {
    /// Numeric byte order and marker.
    pub format: TransmitFormat,
    /// Latin-1 title; its `(partition)` or `(deltas)` marker selects the body.
    pub title: String,
    /// Normalized schema identity.
    pub schema_key: SchemaKey,
    /// Writer's node-type table size; unrelated to encountered layout count.
    pub n_types: u16,
    /// Explicit layouts, one for every encountered type.
    pub layouts: BTreeMap<u16, LayoutDefinition>,
    /// All records and framing.
    pub body: TransmitBody,
}
