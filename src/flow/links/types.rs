use super::*;

/// Caller-supplied native result classification; version 7/8 does not imply a type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowResultType {
    /// Native type 1, extension cpt.
    Cpt,
    /// Native type 2, extension fld.
    Fld,
    /// Native type 4, extension cfld; lookup also accepts CPT candidates.
    StaticSettings,
}

impl TryFrom<u32> for FlowResultType {
    type Error = BessemerError;
    fn try_from(value: u32) -> Result<Self> {
        match value {
            1 => Ok(Self::Cpt),
            2 => Ok(Self::Fld),
            4 => Ok(Self::StaticSettings),
            _ => Err(BessemerError::new(
                "Flow GUID lookup",
                "result type",
                FailureClass::Unsupported,
                format!("result type {value}; expected 1 (CPT), 2 (FLD) or 4 (static settings)"),
            )),
        }
    }
}

impl FlowResultType {
    /// Returns the native word; e.g. `assert_eq!(FlowResultType::Cpt.native_value(), 1)`.
    pub fn native_value(self) -> u32 {
        match self {
            Self::Cpt => 1,
            Self::Fld => 2,
            Self::StaticSettings => 4,
        }
    }
    /// Returns the native lowercase extension without a dot; e.g. `let suffix = kind.extension()`.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Cpt => "cpt",
            Self::Fld => "fld",
            Self::StaticSettings => "cfld",
        }
    }
    pub(super) fn key_index(self) -> usize {
        match self {
            Self::Cpt => 0,
            Self::Fld => 1,
            Self::StaticSettings => 2,
        }
    }
    pub(super) fn accepts(self, candidate: Self) -> bool {
        self == candidate || self == Self::StaticSettings && candidate == Self::Cpt
    }
}

/// Immutable caller-classified candidate, retaining its literal path and full source.
#[derive(Clone, Debug)]
pub struct FlowLinkCandidate {
    pub(super) kind: FlowResultType,
    pub(super) path: Arc<str>,
    pub(super) carrier: FlowCarrier,
}

/// Complete ordered lookup, including completed absence and the checked prefix.
#[derive(Clone, Debug)]
pub struct FlowGuidLookup {
    pub(super) candidates: Arc<[FlowLinkCandidate]>,
    pub(super) kind: FlowResultType,
    pub(super) query: FlowGuid,
    pub(super) selected: Option<usize>,
    pub(super) visited: usize,
    pub(super) eligible: usize,
}

/// Exact scope of candidate selection, separate from filesystem enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGuidLookupCheckMode {
    /// Every visited eligible candidate key checked until first match or list end.
    ExactOrderedCandidatePrefix,
}

/// Selection counts make short-circuiting explicit without implying full-list validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowGuidLookupCoverage {
    /// Exact prefix check scope.
    pub mode: FlowGuidLookupCheckMode,
    /// Total caller-supplied candidates retained as checkpoints.
    pub supplied_candidates: usize,
    /// Candidates visited, including ineligible types.
    pub visited_candidates: usize,
    /// Eligible candidate headers checked in that prefix.
    pub checked_eligible_candidates: usize,
    /// Exact selected-key byte disagreement, zero for completed lookups.
    pub achieved_byte_error: usize,
}

/// Why the related-settings lookup did or did not execute.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowRelatedSettingsStage {
    /// No type-1 CPT candidate matched the seed's second GUID.
    NoRelatedCpt,
    /// Matching CPT candidate supplied an empty literal path.
    EmptyCptPath,
    /// Seed's settings GUID was absent or nil.
    EmptySettingsKey,
    /// Settings lookup executed; its completed result may still be empty.
    Searched,
}

/// Retained seed, ordered inputs, CPT lookup and conditionally executed settings lookup.
#[derive(Clone, Debug)]
pub struct FlowRelatedCarriers {
    pub(super) seed: FlowCarrier,
    pub(super) cpt: FlowGuidLookup,
    pub(super) settings: Option<FlowGuidLookup>,
    pub(super) stage: FlowRelatedSettingsStage,
}

/// Work outside the pure GUID-selection profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGuidLookupDiagnostic {
    /// Candidate order, classification and literal paths are caller context, not discovered.
    FilesystemAndPathValidationNotPerformed,
    /// Only fresh result-info keys are modeled, without stale reused strings.
    ReusedResultInfoNotModeled,
    /// No native application was executed and selected payloads were not loaded.
    NativeApplicationNotChecked,
}
