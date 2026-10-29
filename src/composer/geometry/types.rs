use super::{ComposerGeometryArchive, ComposerSceneValue};
use crate::archive_codec::ArchiveRevision;
use std::sync::Arc;

/// Explicit native appearance/decal import context for version-7.9 strip records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerSceneContext {
    /// `SWImportAppearance OR SWImportDecals`, never inferred by trying alignments.
    pub texture_import_enabled: bool,
}

/// Operation-local wire, allocation and traversal budgets.
#[derive(Debug, Clone, Copy)]
pub struct ComposerSceneLimits {
    /// Maximum gzip input/output bytes.
    pub maximum_encoded_bytes: usize,
    /// Maximum complete decoded geometry bytes.
    pub maximum_decoded_bytes: usize,
    /// Maximum stored fields, including empty documented extensions.
    pub maximum_fields: usize,
    /// Maximum geode and drawable records together.
    pub maximum_records: usize,
    /// Maximum cumulative UTF-16 content code units, excluding terminators.
    pub maximum_text_units: usize,
}

/// Immutable complete-stream revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerSceneRevision(pub(super) ArchiveRevision);

/// Revision-bound field occurrence; paths alone do not confer identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerSceneFieldId {
    pub(super) revision: ComposerSceneRevision,
    pub(super) index: usize,
}

/// Canonical ordered stored field; derived defaults are not emitted as extra fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerSceneField {
    /// Unique canonical path assigned by the complete grammar.
    pub path: Arc<str>,
    /// Exact typed stored representation.
    pub value: ComposerSceneValue,
}

/// Whole-stream framing validation, separate from geometric/application acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerSceneCheckMode {
    /// Complete decoded EOF, canonical typed reconstruction and all bounded spans.
    ExactSceneWire,
}

/// Entire accepted wire scope and explicit sized-extension exclusions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerSceneCoverage {
    /// Explicit full-wire check mode.
    pub mode: ComposerSceneCheckMode,
    /// Complete decoded wire bytes.
    pub decoded_bytes: usize,
    /// Ordered stored fields.
    pub fields: usize,
    /// Geode records.
    pub geodes: usize,
    /// Drawable records.
    pub drawables: usize,
    /// Explicit sized future/unknown bytes, never an unexplained scene suffix.
    pub sized_extension_bytes: usize,
    /// The accepted input also passed strict gzip deflate/CRC/size checks.
    pub input_gzip_integrity_checked: bool,
}

/// Checks outside a complete scene wire operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerSceneDiagnostic {
    /// Scene/XML/hierarchy relationships are outside this grammar.
    ProjectSemanticsNotChecked,
    /// Topology, geometry, self-intersection and tolerances were not validated.
    GeometryNotChecked,
    /// Runtime transforms, rendering and native application acceptance were not checked.
    NativeApplicationNotChecked,
}

/// Zero stored-bit error for requested fields, with bounded sizes derived on a copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerSceneEditFidelity {
    /// Exact typed replacement and unchanged unrelated stored fields.
    ExactTypedFields,
}

/// Whole-stream derivation, independent of application naming policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerSceneHistory {
    /// Original complete-stream revision.
    pub parent: ComposerSceneRevision,
    /// Published complete-stream revision (same for a completed no-op).
    pub result: ComposerSceneRevision,
}

/// Complete isolated edit outcome, including empty/no-op completion and explicit exclusions.
#[derive(Debug, Clone)]
pub struct ComposerSceneEdit {
    /// Immutable accepted result.
    pub archive: ComposerGeometryArchive,
    /// Completed changed/unchanged state.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Exact achieved stored-field fidelity.
    pub fidelity: ComposerSceneEditFidelity,
    /// Whole-stream provenance.
    pub history: ComposerSceneHistory,
    /// Complete wire coverage.
    pub coverage: ComposerSceneCoverage,
    /// Required distinctions from geometry/application checks.
    pub diagnostics: [ComposerSceneDiagnostic; 3],
}

#[derive(Debug, Clone)]
pub(super) struct SizeRule {
    pub size_field: usize,
    pub start_field: usize,
    pub end_field: usize,
    pub omit_text_terminators: bool,
}

#[derive(Default)]
pub(super) struct SceneLedger {
    pub fields: Vec<ComposerSceneField>,
    pub rules: Vec<SizeRule>,
    pub text_units: usize,
    pub geodes: usize,
    pub drawables: usize,
    pub extension_bytes: usize,
}
