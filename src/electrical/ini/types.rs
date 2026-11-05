use super::*;
pub use crate::database_hash::{
    HashBufferHistory as ElectricalByteHistory, HashBufferRelation as ElectricalByteRelation,
};
use std::ops::Range;

/// Independent byte and record bounds for an ASCII lexical view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalIniLimits {
    /// Maximum complete payload bytes, including whitespace and line endings.
    pub maximum_bytes: usize,
    /// Maximum complete lexical records.
    pub maximum_records: usize,
}

/// Lexical classification without interpreting application field meanings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalIniKind {
    /// Comments, empty records and uninterpreted lexical lines.
    Other,
    /// A literal bracketed section, including an empty name.
    Section,
    /// A first-equals assignment, including an empty key/value.
    Field,
}

/// Immutable spans into one complete source lexical document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectricalIniRecord {
    pub(super) raw: Range<usize>,
    pub(super) section: Option<Range<usize>>,
    pub(super) kind: ElectricalIniKind,
    pub(super) key: Option<Range<usize>>,
    pub(super) value: Option<Range<usize>>,
}

impl ElectricalIniRecord {
    /// Reports lexical class; e.g. `let kind = record.kind()`.
    pub fn kind(&self) -> ElectricalIniKind {
        self.kind
    }
    /// Returns its complete raw line interval; e.g. `let line = &document.bytes()[record.raw_range()]`.
    pub fn raw_range(&self) -> Range<usize> {
        self.raw.clone()
    }
    /// Returns the current section name interval; e.g. `let section = record.section_range()`.
    pub fn section_range(&self) -> Option<Range<usize>> {
        self.section.clone()
    }
    /// Returns its exact assignment key interval; e.g. `let key = record.key_range()`.
    pub fn key_range(&self) -> Option<Range<usize>> {
        self.key.clone()
    }
    /// Returns its exact assignment value interval; e.g. `let value = record.value_range()`.
    pub fn value_range(&self) -> Option<Range<usize>> {
        self.value.clone()
    }
}

/// Explicit ASCII lexical validation scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalIniCheckMode {
    /// Every supplied byte belongs to a classified raw lexical record.
    ExactAsciiLexicalRecords,
}

/// Exact lexical accounting without application-value interpretation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectricalIniCoverage {
    /// Declared lexical check mode.
    pub mode: ElectricalIniCheckMode,
    /// Complete interpreted lexical byte extent.
    pub buffer_bytes: usize,
    /// Raw lexical records, including comments and empty records.
    pub records: usize,
    /// Complete assignment occurrences, including duplicate keys.
    pub fields: usize,
    /// Reparse disagreement; successful edits require zero.
    pub achieved_byte_error: usize,
}

/// Lexical interpretation excludes numeric domains and native project loading.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectricalIniDiagnostic {
    /// Keys, values, cardinalities and duplicate-key policy have no application validation.
    StoredValuesNotApplicationValidated,
    /// No native Electrical load, migration or project evaluation was checked.
    NativeLoadNotChecked,
}

/// Immutable lexical operation with exact source/result byte correspondence.
#[derive(Clone, Debug)]
pub struct ElectricalIniEdit {
    pub(super) source: ElectricalIniDocument,
    pub(super) result: ElectricalIniDocument,
    pub(super) completion: ElectricalEditCompletion,
    pub(super) history: [ElectricalByteHistory; 3],
}
