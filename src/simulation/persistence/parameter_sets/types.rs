use super::*;
use crate::archive_codec::ArchiveRevision;

/// Immutable complete ParameterSets identity, including its explicit version/options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationParameterRevision(pub(super) ArchiveRevision);

/// Once-normalized physical primitive/text field tied to its original archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationParameterFieldId {
    pub(super) revision: SimulationParameterRevision,
    pub(super) ordinal: usize,
}

/// Exact primitive role; ordinals do not assign CAD or physical meanings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationParameterRole {
    /// One of the original five header words.
    HeaderWord(usize),
    /// Parent word above version 700013.
    Gate700013,
    /// Parent word above version 700018.
    Gate700018,
    /// Original signed record count; structural changes require reconstruction.
    RecordCount,
    /// Integer table entry.
    TableWord(usize),
    /// Complete counted text of an existing record.
    RecordText(usize),
    /// First signed record word.
    RecordSignedWord(usize),
    /// One of the next two unsigned record words.
    RecordUnsignedWord {
        /// Original zero-based record.
        record: usize,
        /// Normalized unsigned-word position, zero or one.
        component: usize,
    },
    /// Record word above version 700012.
    RecordConditionalWord(usize),
    /// Raw record double above version 700013.
    RecordConditionalDouble(usize),
    /// One complete raw eight-byte table value.
    RecordValue {
        /// Original zero-based record.
        record: usize,
        /// Original zero-based table column.
        column: usize,
    },
    /// Unconditional trailing parent word.
    Tail,
    /// Parent word above version 820015.
    Gate820015,
    /// Parent word above version 830009.
    Gate830009,
}

/// Complete typed primitive pattern or one original persisted text frame.
#[derive(Debug, Clone)]
pub enum SimulationParameterValue {
    /// Signed four-byte primitive.
    Signed(i32),
    /// Unsigned four-byte primitive.
    Unsigned(u32),
    /// Raw eight-byte double pattern, without floating-point conversion.
    DoubleBits(u64),
    /// Complete counted raw text with its entire immutable parent.
    Text(SimulationPersistText),
}

impl SimulationParameterValue {
    /// Exact full-width pattern, e.g. `SimulationParameterValue::DoubleBits(1).to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.append_to(&mut bytes);
        bytes
    }
    pub(super) fn append_to(&self, bytes: &mut Vec<u8>) {
        match self {
            Self::Signed(value) => bytes.extend_from_slice(&value.to_le_bytes()),
            Self::Unsigned(value) => bytes.extend_from_slice(&value.to_le_bytes()),
            Self::DoubleBits(value) => bytes.extend_from_slice(&value.to_le_bytes()),
            Self::Text(text) => bytes.extend_from_slice(text.encoded()),
        }
    }
    pub(super) fn kind(&self) -> u8 {
        match self {
            Self::Signed(_) => 0,
            Self::Unsigned(_) => 1,
            Self::DoubleBits(_) => 2,
            Self::Text(_) => 3,
        }
    }
}

/// One complete original primitive/text field in total physical order.
#[derive(Debug, Clone)]
pub struct SimulationParameterField {
    /// Original revision-bound identity.
    pub id: SimulationParameterFieldId,
    /// Exact documented primitive role.
    pub role: SimulationParameterRole,
    /// Original physical byte position.
    pub stream_offset: usize,
    /// Complete immutable typed pattern.
    pub value: SimulationParameterValue,
}

/// Complete records versus completed empty-record framing and exact edit status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationParameterCompletion {
    /// Every required field was checked, with zero complete records.
    CompletedEmptyRecords,
    /// Every required field and complete declared record was checked.
    CompletedRecords,
    /// The requested complete pattern already matched; original identities survived.
    Unchanged,
    /// One isolated primitive/text pattern changed on a complete validated archive.
    Changed,
}

/// Exact byte validation scope, independent of native application or geometry acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationParameterCheckMode {
    /// Selected version, budgets, every field and exact EOF.
    ExactSelectedProfile,
    /// Complete selected profile and literal preservation outside one requested field.
    ExactSelectedProfileAndPreservation,
}

/// Explicit exclusions from bare ParameterSets acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationParameterDiagnostic {
    /// No installed loader, writer, storage enumeration or COM operation executed.
    NativeApplicationNotExecuted,
    /// Caller context does not prove the application's version dispatch.
    RuntimeVersionNotInferred,
    /// No guessed BasicStudy or alternate storage grammar was attempted.
    LegacyFallbackNotUsed,
    /// Complete wire entries survived independently of runtime retention limits.
    NativeRetentionNotApplied,
    /// Raw primitives do not establish physical/domain meanings or geometric validity.
    FieldMeaningsUnassigned,
}

/// Completion, exact check scope, coverage, exclusions and achieved pattern error together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationParameterCoverage {
    /// Once-normalized explicit caller context.
    pub version: SimulationPersistVersion,
    /// Exact caller budgets used for construction/read/edit.
    pub limits: SimulationParameterLimits,
    /// Complete framing or isolated edit status.
    pub completion: SimulationParameterCompletion,
    /// Exact selected validation scope.
    pub check_mode: SimulationParameterCheckMode,
    /// Every original/result byte checked.
    pub checked_bytes: usize,
    /// Every complete primitive/text field checked.
    pub checked_fields: usize,
    /// Entire wire table count, without native retention truncation.
    pub table_count: usize,
    /// Every complete declared record checked.
    pub record_count: usize,
    /// Every original counted text frame checked.
    pub checked_text_frames: usize,
    /// Native, runtime and semantic exclusions.
    pub diagnostics: [SimulationParameterDiagnostic; 5],
    /// Achieved requested full-width pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

/// Relation between each original/result physical field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationParameterRelation {
    /// Complete original pattern survived byte for byte.
    Copied,
    /// Field derives from its original position and retained replacement pattern.
    Derived,
}

/// One original/result relation per primitive/text field, in wire order.
#[derive(Debug, Clone)]
pub struct SimulationParameterHistory {
    /// Original physical field.
    pub parent: SimulationParameterFieldId,
    /// Validated result physical field.
    pub result: SimulationParameterFieldId,
    /// Exact copy or derivation relation.
    pub relation: SimulationParameterRelation,
}

/// Complete isolated change retaining source/replacement parents and every relation.
#[derive(Debug, Clone)]
pub struct SimulationParameterEdit {
    /// Original complete immutable archive.
    pub source: SimulationParameterSets,
    /// Complete validated isolated result.
    pub archive: SimulationParameterSets,
    /// Requested original field.
    pub parent: SimulationParameterFieldId,
    /// Requested result field.
    pub result: SimulationParameterFieldId,
    /// Entire supplied typed pattern and any original text parent.
    pub replacement: SimulationParameterValue,
    /// Exact completion, check scope, exclusions and achieved error.
    pub coverage: SimulationParameterCoverage,
    /// Every original/result field in total wire order.
    pub history: Arc<[SimulationParameterHistory]>,
}
