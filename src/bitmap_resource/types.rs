use super::BitmapResourceArray;
use std::ops::Range;

/// Completion of an isolated logical-array edit, including unchanged empty arrays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapResourceCompletion {
    /// The complete encoded array differs from its source checkpoint.
    Changed,
    /// The complete encoded array equals its source checkpoint.
    Unchanged,
}

/// Declared validation scope; no image or geometry interpretation is implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapResourceCheckMode {
    /// Every count and payload byte was checked through EOF.
    ExactCountAndPayload,
}

/// Exact coverage of the complete logical resource, after container decompression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitmapResourceCoverage {
    /// Complete count/payload parsing through EOF.
    pub mode: BitmapResourceCheckMode,
    /// Entire logical archive length.
    pub wire_bytes: usize,
    /// Two-byte or six-byte compact-count prefix.
    pub count_prefix_bytes: usize,
    /// Opaque payload extent.
    pub payload_bytes: usize,
    /// Measured payload disagreement after reparse; successful results require zero.
    pub achieved_byte_error: usize,
    /// Headerless arrays add no MFC class or object slots.
    pub object_map_slots_added: usize,
}

/// Explicit semantic exclusions retained with every successful edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapResourceDiagnostic {
    /// Syntax validation does not decode image or pixel meanings.
    OpaquePayloadNotInterpreted,
    /// Native dimension-dependent buffer resizing is not simulated.
    RuntimeResizingNotSimulated,
    /// The caller selects the stream; no universal storage owner is inferred.
    StorageOwnerNotInferred,
}

/// Logical fields whose identities persist even when the payload is empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapResourceField {
    /// Compact element count, including its selected prefix width.
    Count,
    /// Opaque bytes following the compact count.
    Payload,
}

/// Complete field relation to the operation's immutable source snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapResourceRelation {
    /// The entire field encoding equals the source field encoding.
    Copied,
    /// The field encoding was produced from the source field.
    Derived,
}

/// Logical field history; empty payload ranges still identify a present field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitmapResourceHistory {
    /// Count or opaque payload field.
    pub field: BitmapResourceField,
    /// Whether this logical field was retained or derived.
    pub relation: BitmapResourceRelation,
    /// All source byte ranges consumed by the field relation.
    pub parents: Vec<Range<usize>>,
    /// Complete result field extent.
    pub result: Range<usize>,
}

/// Published isolated array result with completion, exact coverage and full history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitmapResourceEdit {
    pub(super) source: BitmapResourceArray,
    pub(super) resource: BitmapResourceArray,
    pub(super) completion: BitmapResourceCompletion,
    pub(super) coverage: BitmapResourceCoverage,
    pub(super) history: [BitmapResourceHistory; 2],
}

impl BitmapResourceEdit {
    /// Anchors all history parents to the immutable source; e.g. `edit.source().to_bytes()?`.
    pub fn source(&self) -> &BitmapResourceArray {
        &self.source
    }
    /// Borrows the validated immutable result; e.g. `edit.resource().to_bytes()?`.
    pub fn resource(&self) -> &BitmapResourceArray {
        &self.resource
    }
    /// Distinguishes changed and unchanged success; e.g. `let completion = edit.completion();`.
    pub fn completion(&self) -> BitmapResourceCompletion {
        self.completion
    }
    /// Copies the exact completed check coverage; e.g. `let coverage = edit.coverage();`.
    pub fn coverage(&self) -> BitmapResourceCoverage {
        self.coverage
    }
    /// Borrows both logical field relations; e.g. `let payload = &edit.history()[1];`.
    pub fn history(&self) -> &[BitmapResourceHistory; 2] {
        &self.history
    }
    /// Lists semantic exclusions; e.g. `let limits = edit.diagnostics();`.
    pub fn diagnostics(&self) -> &[BitmapResourceDiagnostic] {
        use BitmapResourceDiagnostic::*;
        &[
            OpaquePayloadNotInterpreted,
            RuntimeResizingNotSimulated,
            StorageOwnerNotInferred,
        ]
    }
}
