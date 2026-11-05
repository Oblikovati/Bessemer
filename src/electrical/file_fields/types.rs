use super::*;
use crate::sqlite_image::SqliteValue;

/// Reader requested by the observed native field descriptor; not a SQLite coercion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalFileReaderKind {
    /// Signed native Long request.
    Long,
    /// Native Boolean request.
    Bool,
    /// Native floating-point request without inferred physical units.
    Double,
    /// Native wide-string request without inferred normalization.
    Str,
    /// Variant/date conversion request without inferred date format or timezone.
    Date,
}

/// Native getter output location, distinct from the later copy/virtual-setter effects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalFileReadDestination {
    /// Byte offset relative to the runtime EwAdFile object, never a SQLite offset.
    ObjectByteOffset(u32),
    /// Separate ID temporary initially -1, later passed to a virtual setter.
    IdentityTemporary,
    /// Reused native Long temporary initially zero.
    SharedLongTemporary,
    /// Reused native Boolean temporary initially zero.
    SharedBoolTemporary,
    /// Reused native double temporary initially zero.
    SharedDoubleTemporary,
}

/// One verified descriptor binding paired with its uncoerced stored SQLite value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalFileRead {
    /// Exact descriptor name in the verified native build.
    pub column: &'static str,
    /// Requested provider reader kind.
    pub reader: ElectricalFileReaderKind,
    /// Destination supplied to that getter call.
    pub destination: ElectricalFileReadDestination,
    /// Original dynamic SQLite value; NULL stays distinct from absence.
    pub value: SqliteValue,
}

/// Immutable complete declared direct-read map with the full input row retained.
#[derive(Clone, Debug)]
pub struct ElectricalFileReadMap {
    pub(super) record: ElectricalRecord,
    pub(super) reads: Arc<[ElectricalFileRead]>,
}

/// Executed file-field check scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalFileReadCheckMode {
    /// Every observed direct descriptor resolves to a present stored column.
    DeclaredNativeTransport,
}

/// Completed descriptor/name mapping, separate from provider or application validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalFileReadCoverage {
    /// Selected explicit transport scope.
    pub mode: ElectricalFileReadCheckMode,
    /// Complete native-order descriptor count, 38 for this observed build.
    pub mapped_reads: usize,
    /// All input columns, including unclassified fields.
    pub preserved_columns: usize,
}

/// Exclusions accompanying a complete declared direct-read map.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalFileReadDiagnostic {
    /// No typed getter, NULL/default rule or date conversion was applied.
    ProviderConversionsNotApplied,
    /// Offsets describe an observed runtime object, not stored file byte positions.
    RuntimeOffsetsAreNotFileOffsets,
    /// Failed reads can leave a previous temporary value; no per-column zero was guessed.
    TemporaryReuseHasNoIndependentDefaults,
    /// Unmatched columns retain their values without an ignored/unimportant classification.
    UnmappedColumnsUnclassified,
    /// The file ID goes through a virtual setter whose final member offset is not established.
    VirtualIdentityDestinationUnresolved,
    /// Native file loader, delegate, migration and application acceptance were not executed.
    NativeLoaderNotExecuted,
}
