use super::{Result, require};
use crate::archive_codec::ArchiveRevision;

/// Exact named bare scalar grammar, normalized before reading any payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationScalarKind {
    /// Supplied model-document update stamp; no wall-clock meaning is assigned.
    TimeStamp,
    /// Stored state pattern with no complete enum assigned.
    RunState,
}

impl SimulationScalarKind {
    /// Normalize an exact bound stream name once, e.g. `SimulationScalarKind::from_stream_name("TimeStamp")?`.
    pub fn from_stream_name(name: &str) -> Result<Self> {
        match name {
            "TimeStamp" => Ok(Self::TimeStamp),
            "RunState" => Ok(Self::RunState),
            _ => Err(crate::simulation::unsupported(
                "scalar stream name",
                format!("{name:?}; expected TimeStamp or RunState"),
            )),
        }
    }

    /// Exact bound root stream name, e.g. `kind.stream_name()`.
    pub fn stream_name(self) -> &'static str {
        match self {
            Self::TimeStamp => "TimeStamp",
            Self::RunState => "RunState",
        }
    }

    pub(super) fn index(self) -> usize {
        match self {
            Self::TimeStamp => 0,
            Self::RunState => 1,
        }
    }
}

/// Build-local meaning established for the scalar, independently of its numeric value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationScalarMeaning {
    /// Current inspected binding to the supplied model-document GetUpdateStamp value.
    ModelDocumentUpdateStamp,
    /// No complete RunState enum or physical meaning has been assigned.
    Unassigned,
}

/// Immutable scalar snapshot identity, bound to its normalized stream grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationScalarId {
    pub(super) revision: ArchiveRevision,
    pub(super) kind: SimulationScalarKind,
}

/// Complete immutable bare four-byte stream, distinct from a transfer envelope.
#[derive(Debug, Clone)]
pub struct SimulationScalarArchive {
    pub(super) id: SimulationScalarId,
    pattern: u32,
}

impl SimulationScalarArchive {
    /// Read exactly four bare bytes, e.g. `SimulationScalarArchive::from_bytes(kind, scalar)?`.
    pub fn from_bytes(kind: SimulationScalarKind, bytes: &[u8]) -> Result<Self> {
        require(
            bytes.len() == 4,
            kind.stream_name(),
            format!(
                "{} bytes; expected exactly four bare scalar bytes without a transfer prefix",
                bytes.len()
            ),
        )?;
        let pattern = crate::simulation::wire::patterns::<1>(bytes, kind.stream_name())?[0];
        Ok(Self::from_pattern(kind, pattern))
    }

    /// Construct all stored bits without assigning an enum, e.g. `SimulationScalarArchive::from_pattern(kind, 0xffffffff)`.
    pub fn from_pattern(kind: SimulationScalarKind, pattern: u32) -> Self {
        Self {
            id: SimulationScalarId {
                revision: ArchiveRevision::default(),
                kind,
            },
            pattern,
        }
    }

    /// Construct a supplied signed update/state value, e.g. `SimulationScalarArchive::from_signed(kind, -1)`.
    pub fn from_signed(kind: SimulationScalarKind, value: i32) -> Self {
        Self::from_pattern(kind, value as u32)
    }

    /// Exact bare bytes, e.g. `scalar.to_bytes()`.
    pub fn to_bytes(&self) -> [u8; 4] {
        self.pattern.to_le_bytes()
    }

    /// Exact unsigned pattern, e.g. `scalar.pattern()`.
    pub fn pattern(&self) -> u32 {
        self.pattern
    }

    /// Exact signed interpretation, e.g. `scalar.signed_value()`.
    pub fn signed_value(&self) -> i32 {
        self.pattern as i32
    }

    /// Normalized bound stream grammar, e.g. `scalar.kind()`.
    pub fn kind(&self) -> SimulationScalarKind {
        self.id.kind
    }

    /// Immutable scalar identity, e.g. `scalar.id()`.
    pub fn id(&self) -> &SimulationScalarId {
        &self.id
    }

    /// Explicit known binding, e.g. `scalar.meaning()`; no clock unit or state enum is inferred.
    pub fn meaning(&self) -> SimulationScalarMeaning {
        match self.kind() {
            SimulationScalarKind::TimeStamp => SimulationScalarMeaning::ModelDocumentUpdateStamp,
            SimulationScalarKind::RunState => SimulationScalarMeaning::Unassigned,
        }
    }

    /// Isolate a complete bit-pattern replacement, e.g. `scalar.edit_pattern(bits)`.
    pub fn edit_pattern(&self, pattern: u32) -> super::SimulationScalarEdit {
        let archive = if pattern == self.pattern {
            self.clone()
        } else {
            Self::from_pattern(self.kind(), pattern)
        };
        super::scalar_outcome::edit_result(self, archive)
    }
}

/// Successful inspected helper's proposed bytes; native I/O and command-line conversion are not executed.
#[derive(Debug, Clone)]
pub struct SimulationScalarHelperProjection {
    /// Fixed caller state pattern one, without an assigned complete enum.
    pub run_state: SimulationScalarArchive,
    /// Explicit supplied signed model update stamp.
    pub time_stamp: SimulationScalarArchive,
    /// Always false: no native stream operation was executed.
    pub native_io_executed: bool,
}

impl SimulationScalarHelperProjection {
    /// Reproduce the two inspected callers' scalar words, e.g. `SimulationScalarHelperProjection::from_update_stamp(-1)`.
    pub fn from_update_stamp(stamp: i32) -> Self {
        Self {
            run_state: SimulationScalarArchive::from_pattern(SimulationScalarKind::RunState, 1),
            time_stamp: SimulationScalarArchive::from_signed(
                SimulationScalarKind::TimeStamp,
                stamp,
            ),
            native_io_executed: false,
        }
    }
}
