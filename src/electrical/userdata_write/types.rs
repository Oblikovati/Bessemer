use super::*;
use crate::sqlite_image::SqliteValue;

/// Validated userdata text slot 0 through 19.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataSlot(pub(super) u8);

impl ElectricalUserdataSlot {
    /// Parses the bounded native slot once; e.g. `let slot = ElectricalUserdataSlot::new(3)?`.
    pub fn new(index: usize) -> Result<Self> {
        if index >= 20 {
            return Err(BessemerError::new(
                "Electrical userdata write",
                "slot",
                FailureClass::Invalid,
                format!("slot {index}; expected integer 0..19"),
            ));
        }
        Ok(Self(index as u8))
    }
    /// Returns the checked slot index; e.g. `assert_eq!(slot.index(), 3)`.
    pub fn index(self) -> u8 {
        self.0
    }
}

/// Immutable committed cell edit with exact source checkpoint and composed carrier history.
#[derive(Clone, Debug)]
pub struct ElectricalUserdataWrite {
    pub(super) carrier: ElectricalArchiveEdit,
    pub(super) history: ElectricalUserdataCellHistory,
    pub(super) coverage: ElectricalUserdataWriteCoverage,
}

/// Exact unique-key logical cell parent/result relation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataCellHistory {
    /// Selected source member occurrence.
    pub source_member: ElectricalMemberId,
    /// Corresponding result member occurrence.
    pub result_member: ElectricalMemberId,
    /// Unchanged signed two-part SQL key.
    pub key: ElectricalUserdataKey,
    /// Selected bounded text slot.
    pub slot: ElectricalUserdataSlot,
    /// Complete original dynamic SQLite value.
    pub before: SqliteValue,
    /// Exact verified UTF-8 TEXT result.
    pub after: SqliteValue,
    /// Copied for a no-op, otherwise derived from this complete source cell.
    pub relation: ElectricalMemberRelation,
}

/// Required stored-image mutation check mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalUserdataWriteCheckMode {
    /// Complete image/schema/header metadata, foreign keys and every ordinary stored table.
    ExactTextAndStoredDatabase,
}

/// Completed unique-cell mutation coverage, separate from application acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalUserdataWriteCoverage {
    /// Required check scope.
    pub mode: ElectricalUserdataWriteCheckMode,
    /// Complete ordinary tables checked on both sides, counted once.
    pub checked_tables: usize,
    /// Complete source rows checked, with result shape required equal.
    pub checked_rows: usize,
    /// Complete source cells checked against unchanged or requested values.
    pub checked_cells: usize,
    /// Shared logical snapshot/output accounting, not peak engine memory.
    pub accounted_bytes: usize,
    /// Exact requested TEXT-byte disagreement; successful writes require zero.
    pub achieved_byte_error: usize,
}

/// Explicit exclusions accompanying a complete stored SQL mutation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalUserdataWriteDiagnostic {
    /// SQLite may change page framing/slack beyond the logical target cell.
    PhysicalImageMayBeRewritten,
    /// Typed recordset conversions, defaults and native string normalization were not applied.
    NativeProviderConversionsNotApplied,
    /// Native loader, migration, drawing and geometry acceptance were not checked.
    NativeApplicationNotChecked,
    /// Fresh opposite edits need not restore original bytes; the retained source checkpoint does.
    FreshReverseWireIdentityNotGuaranteed,
}
