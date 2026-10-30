use crate::archive_codec::ArchiveRevision;
use std::sync::Arc;

/// Explicit budgets for stored reports and their ordered projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationOutLimits {
    /// Maximum complete wrapped stream bytes.
    pub maximum_bytes: usize,
    /// Maximum retained lines, including empty lines.
    pub maximum_lines: usize,
    /// Maximum numeric occurrences, including repeated symbols.
    pub maximum_fields: usize,
}

/// Immutable identity of a complete report snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutRevision(pub(super) ArchiveRevision);

/// Normalized ordered occurrence bound to one report revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutFieldId {
    pub(super) revision: SimulationOutRevision,
    pub(super) index: usize,
}

/// Exact decimal coefficient and power, preserving trailing zeros and signed zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutDecimal {
    pub(super) negative: bool,
    pub(super) digits: Arc<[u8]>,
    pub(super) exponent: i64,
}

impl SimulationOutDecimal {
    /// Stored sign, including negative zero, e.g. `value.is_negative()`.
    pub fn is_negative(&self) -> bool {
        self.negative
    }

    /// Decimal digits 0..=9 without leading zeros, e.g. `value.digits()`.
    pub fn digits(&self) -> &[u8] {
        &self.digits
    }

    /// Exact base-ten exponent, e.g. `value.exponent()`; value is signed digits times ten to this power.
    pub fn exponent(&self) -> i64 {
        self.exponent
    }
}

/// All source bytes of one line for complete image construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutLineInput {
    /// Seven-bit content, including embedded NUL and tab; no CR or LF.
    pub content: Arc<[u8]>,
    /// Whether this line ends in CRLF; only the final line may omit it.
    pub crlf: bool,
}

/// One stored line and its derived occurrence associations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutLine {
    /// Complete stored content and ending.
    pub input: SimulationOutLineInput,
    /// Offset including the four-byte transfer prefix.
    pub stream_offset: usize,
    /// NUL positions relative to content, without text repair.
    pub nul_positions: Arc<[usize]>,
    /// Ordered numeric occurrence on this line, if recognized.
    pub field_index: Option<usize>,
    /// Preceding field occurrence associated with this printed legend.
    pub legend_field_index: Option<usize>,
}

/// One printed legend; it is not a universal solver enum definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutLegend {
    /// Exact mathematical signed integer in canonical decimal text.
    pub value: Arc<str>,
    /// Trimmed seven-bit description, retaining NUL if present.
    pub description: Arc<str>,
    /// Zero-based line in this report.
    pub line_index: usize,
    /// Complete stream offset of the original line.
    pub stream_offset: usize,
}

/// A distinct ordered numeric occurrence, even when another occurrence has the same symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutField {
    /// Snapshot-bound ordered identity.
    pub id: SimulationOutFieldId,
    /// Normalized display label, with dot leaders and whitespace combined.
    pub label: Arc<str>,
    /// Printed uppercase symbol, which need not be unique.
    pub symbol: Arc<str>,
    /// Exact numeric spelling, including Fortran exponents and signed zero.
    pub numeric_lexeme: Arc<str>,
    /// Exact decimal representation, with no binary floating conversion.
    pub value: SimulationOutDecimal,
    /// Zero-based original line.
    pub line_index: usize,
    /// Original line's full stream offset.
    pub stream_offset: usize,
    /// Printed legends associated with this occurrence in stored order.
    pub legends: Arc<[SimulationOutLegend]>,
    pub(super) token_offset: usize,
}
