/// Complete empty framing remains distinct from a complete populated collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcFinalCompletion {
    /// All required counts and exact EOF were checked, with no records.
    CompletedEmpty,
    /// Every declared record belongs to a complete checked image.
    CompletedRecords,
}

/// Exact declared check scope, independent of native name matching or map insertion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcFinalCheckMode {
    /// Signed count, every original text frame and exact EOF, without a leading selection string.
    ExactNameOrder,
    /// Complete name-order frame and literal preservation outside one requested text.
    ExactNameOrderAndPreservation,
    /// Exactly three signed-count legacy groups, every four-word record and EOF.
    ExactLegacyMap,
    /// Complete legacy framing and literal preservation outside one stored word.
    ExactLegacyMapAndPreservation,
}

/// Responsibilities outside bare final-LBC framing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcFinalDiagnostic {
    /// No installed reader/writer, storage open or application operation executed.
    NativeApplicationNotExecuted,
    /// No name normalization, matching or runtime reordering is inferred.
    NameMatchingNotInferred,
    /// No legacy insertion, persistent-ID grammar or availability fallback is inferred.
    MapInsertionNotInferred,
    /// Raw stored positions do not establish physical or CAD meanings.
    FieldMeaningsUnassigned,
}

/// Exact framing scope, completion, diagnostics and achieved byte-pattern error together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLbcFinalCoverage {
    /// Complete empty versus complete populated image.
    pub completion: SimulationLbcFinalCompletion,
    /// Exact selected check mode.
    pub check_mode: SimulationLbcFinalCheckMode,
    /// All original/result bytes checked.
    pub checked_bytes: usize,
    /// Every signed count word checked.
    pub checked_count_words: usize,
    /// Complete preserved text frames, without normalizing their content.
    pub checked_text_frames: usize,
    /// Complete legacy map groups checked; zero for a name list.
    pub checked_map_groups: usize,
    /// Every complete legacy four-word record checked; zero for a name list.
    pub checked_map_records: usize,
    /// Native and semantic exclusions.
    pub diagnostics: [SimulationLbcFinalDiagnostic; 4],
    /// Achieved requested pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

pub(super) fn name_coverage(bytes: usize, count: usize) -> SimulationLbcFinalCoverage {
    SimulationLbcFinalCoverage {
        completion: if count == 0 {
            SimulationLbcFinalCompletion::CompletedEmpty
        } else {
            SimulationLbcFinalCompletion::CompletedRecords
        },
        check_mode: SimulationLbcFinalCheckMode::ExactNameOrder,
        checked_bytes: bytes,
        checked_count_words: 1,
        checked_text_frames: count,
        checked_map_groups: 0,
        checked_map_records: 0,
        diagnostics: [
            SimulationLbcFinalDiagnostic::NativeApplicationNotExecuted,
            SimulationLbcFinalDiagnostic::NameMatchingNotInferred,
            SimulationLbcFinalDiagnostic::MapInsertionNotInferred,
            SimulationLbcFinalDiagnostic::FieldMeaningsUnassigned,
        ],
        requested_pattern_error_bits: 0,
    }
}

pub(super) fn map_coverage(bytes: usize, records: usize) -> SimulationLbcFinalCoverage {
    let mut coverage = name_coverage(bytes, records);
    coverage.check_mode = SimulationLbcFinalCheckMode::ExactLegacyMap;
    coverage.checked_count_words = 3;
    coverage.checked_text_frames = 0;
    coverage.checked_map_groups = 3;
    coverage.checked_map_records = records;
    coverage
}
