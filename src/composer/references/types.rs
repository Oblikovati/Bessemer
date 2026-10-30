use super::{ComposerGeodeServer, ComposerReferenceText};
use crate::archive_codec::ArchiveRevision;

/// Immutable identity of one complete supplied-state snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerServerRevision(pub(super) ArchiveRevision);

/// Exact stored-text projection; file text is not a resolved resource key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerReferenceSplit {
    /// Stored reference before any branch.
    pub original: ComposerReferenceText,
    /// First marker position in UTF-16 units; absent means plain reference.
    pub marker: Option<usize>,
    /// First global hash position, considered only in the external branch.
    pub separator: Option<usize>,
    /// Unresolved external file text; None means no output assignment.
    pub file: Option<ComposerReferenceText>,
    /// Exact remaining GUID units.
    pub guid: ComposerReferenceText,
}

/// Explicit allocation bounds for the complete supplied maps.
#[derive(Debug, Clone, Copy)]
pub struct ComposerServerLimits {
    /// Maximum distinct normalized file keys.
    pub maximum_files: usize,
    /// Maximum primary plus override entries.
    pub maximum_entries: usize,
    /// Cumulative ASCII bytes of file and GUID keys.
    pub maximum_key_bytes: usize,
}

/// Explicit map input; duplicate keys within either tier are rejected.
#[derive(Debug, Clone)]
pub struct ComposerSuppliedFile {
    /// Explicit runtime file-map key under the ASCII C-locale profile.
    pub file: ComposerReferenceText,
    /// Explicit primary entries, including null tokens.
    pub primary: Vec<(ComposerReferenceText, u64)>,
    /// Explicit override entries; null shadows primary.
    pub overrides: Vec<(ComposerReferenceText, u64)>,
}

/// Native lookup precedence, distinct from pointer truthiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerGeodeTier {
    /// Searched before primary, even for null tokens.
    Override,
    /// Searched only when override lacks the GUID.
    Primary,
}

/// A miss retains the explicitly supplied initial opaque pointer token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerGeodeLookup {
    /// Key presence independently of pointer truthiness.
    pub found: bool,
    /// Opaque u64 pointer identity; zero means null.
    pub pointer: u64,
    /// Matched map tier, or None on a miss.
    pub tier: Option<ComposerGeodeTier>,
}

/// Exact registration branch on supplied maps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerRegistrationBranch {
    /// Existing non-null result retains maps and requests incoming-parent detach.
    ExistingNonnull,
    /// Null public result reaches an insertion helper that retains both maps.
    InnerUnchanged,
    /// Incoming token was inserted into the indicated tier.
    Inserted(ComposerGeodeTier),
}

/// Local-map validation scope, never native loading or graph acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerReferenceCheck {
    /// Every supplied key and map entry passed this profile.
    ExactSuppliedState,
}

/// A completed final lookup block may legitimately return a null token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerReferenceCompletion {
    /// All ordered attempts required by the declared supplied-state profile ran.
    Completed,
}

/// Exact local map semantics, independent of external/native acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerReferenceFidelity {
    /// Every supplied key, token, precedence and branch was retained exactly.
    ExactSuppliedState,
}

/// Complete validation counts for a published server snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerServerCoverage {
    /// Declared exact supplied-state validation scope.
    pub mode: ComposerReferenceCheck,
    /// Number of normalized file maps.
    pub files: usize,
    /// Primary plus override entry count.
    pub entries: usize,
    /// Cumulative file and GUID key bytes.
    pub key_bytes: usize,
}

/// Required unchecked parts of native reference resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerReferenceDiagnostic {
    /// The running CRT locale was not measured.
    NativeLocaleNotMeasured,
    /// External resources were not discovered or loaded.
    FileResolutionNotPerformed,
    /// No parent detachment, clone or merge was executed.
    GraphSideEffectsNotPerformed,
    /// The empty-GUID merge pipeline is outside this operation.
    EmptyGuidMergingNotSupported,
    /// The native application did not accept or validate this result.
    NativeApplicationNotChecked,
}

/// Isolated map operation, with requested detach reported but never executed.
#[derive(Debug, Clone)]
pub struct ComposerRegistration {
    /// Whether the isolated supplied maps changed.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Exact local semantics achieved by this operation.
    pub fidelity: ComposerReferenceFidelity,
    /// Isolated resulting immutable snapshot.
    pub server: ComposerGeodeServer,
    /// Exact public/helper registration branch taken.
    pub branch: ComposerRegistrationBranch,
    /// Opaque u64 pointer identity; zero means null.
    pub pointer: u64,
    /// Requested native detach; graph mutation is not executed.
    pub detach_incoming_first_parent_if_present: bool,
    /// Input snapshot identity.
    pub parent: ComposerServerRevision,
    /// Result snapshot identity.
    pub result: ComposerServerRevision,
    /// Complete local supplied-map validation counts.
    pub coverage: ComposerServerCoverage,
    /// Required native responsibilities left unchecked.
    pub diagnostics: [ComposerReferenceDiagnostic; 5],
}

/// One attempt, retained even when its file key duplicates an earlier key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerLookupAttempt {
    /// Lexical candidate for this attempt, including duplicates.
    pub file_key: ComposerReferenceText,
    /// Found status, pointer token and tier for this attempt.
    pub result: ComposerGeodeLookup,
}

/// Complete nonempty-GUID final lookup block for explicit context.
#[derive(Debug, Clone)]
pub struct ComposerReferenceLookup {
    /// Completion of the declared final block, including a completed miss.
    pub completion: ComposerReferenceCompletion,
    /// Exact local semantics achieved by this operation.
    pub fidelity: ComposerReferenceFidelity,
    /// Opaque u64 pointer identity; zero means null.
    pub pointer: u64,
    /// All attempted candidates in native order.
    pub attempts: Vec<ComposerLookupAttempt>,
    /// Snapshot queried by every attempt.
    pub revision: ComposerServerRevision,
    /// Complete local supplied-map validation counts.
    pub coverage: ComposerServerCoverage,
    /// Required native responsibilities left unchecked.
    pub diagnostics: [ComposerReferenceDiagnostic; 5],
}
