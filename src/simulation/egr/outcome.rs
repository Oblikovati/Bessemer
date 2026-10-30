use super::{SimulationEgrArchive, SimulationEgrProfile};

/// Successful profile framing distinguishes absence of live rows from failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationEgrReadCompletion {
    /// The complete image has no live rows; deleted or unallocated capacity may remain.
    CompletedWithoutLiveRows,
    /// Every complete stored region and live/deleted partition was checked.
    CompletedLiveRows,
}

/// Exact declared check scope, without inferred native cache or save behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationEgrCheckMode {
    /// Entire header, loaded widths, directory, high-water word, chain and physical partition.
    ExactMetadataAndAllocatedPhysicalPartition,
    /// Complete existing live row and exact loaded-width inclusive field interval.
    ExactExistingLiveFieldRange,
    /// Complete resulting framing and every byte outside the supplied field edit.
    ExactLiveFieldEditAndPreservedOtherBytes,
    /// Ten assigned conditional integer words and all retained row bytes.
    ExactConditionalFourByteRowValues,
    /// Stored group identifier checked against explicit caller ordinal; no line selected.
    ExactConditionalGroupIdentifierGate,
    /// Wrapping sum and parameter permutation, without a completed name/output.
    ExactConditionalParametersWithIncompleteName,
    /// Complete logical ASCII EGROUP line for an established known name.
    ExactConditionalKnownNameLine,
}

/// Conditional semantics outside complete stored-region framing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationEgrDiagnostic {
    /// Widths and raw bytes do not establish meanings for every tag or value.
    CompleteEgrSemanticsUnassigned,
    /// No native cache lookup, error path or whole-file producer executed.
    NativeCacheAndSaveNotExecuted,
}

/// Complete file-profile coverage with loaded row states kept distinct.
#[derive(Debug, Clone, Copy)]
pub struct SimulationEgrCoverage {
    /// Empty-live-set versus populated successful completion.
    pub completion: SimulationEgrReadCompletion,
    /// Exact metadata and physical partition check mode.
    pub check_mode: SimulationEgrCheckMode,
    /// Caller-established flat transport profile.
    pub profile: SimulationEgrProfile,
    /// Entire supplied file extent.
    pub checked_bytes: usize,
    /// All directory slots, including zero holes.
    pub logical_capacity: usize,
    /// All positive loaded field widths.
    pub loaded_fields: usize,
    /// Complete active logical mappings.
    pub live_rows: usize,
    /// Complete ordered deleted-record chain.
    pub deleted_records: usize,
    /// All physically present records, live and deleted.
    pub allocated_records: usize,
    /// Capacity that has no physically present record.
    pub unallocated_capacity: usize,
    /// Always false: complete file-profile framing is not a complete semantic schema.
    pub semantics_complete: bool,
    /// Native and semantic exclusions.
    pub diagnostics: [SimulationEgrDiagnostic; 2],
}

impl SimulationEgrArchive {
    /// Exact complete profile scope, e.g. `table.coverage()`; no native acceptance is inferred.
    pub fn coverage(&self) -> SimulationEgrCoverage {
        let header = &self.layout.header;
        SimulationEgrCoverage {
            completion: if header.live_count == 0 {
                SimulationEgrReadCompletion::CompletedWithoutLiveRows
            } else {
                SimulationEgrReadCompletion::CompletedLiveRows
            },
            check_mode: SimulationEgrCheckMode::ExactMetadataAndAllocatedPhysicalPartition,
            profile: self.profile,
            checked_bytes: self.bytes.len(),
            logical_capacity: header.capacity,
            loaded_fields: header.field_count,
            live_rows: header.live_count,
            deleted_records: header.free_count,
            allocated_records: header.allocated,
            unallocated_capacity: header.capacity - header.allocated,
            semantics_complete: false,
            diagnostics: EGR_DIAGNOSTICS,
        }
    }
}

const EGR_DIAGNOSTICS: [SimulationEgrDiagnostic; 2] = [
    SimulationEgrDiagnostic::CompleteEgrSemanticsUnassigned,
    SimulationEgrDiagnostic::NativeCacheAndSaveNotExecuted,
];
