use super::*;

/// Positive signed-width logical position, normalized independently of stored identifier bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxiliaryLogicalIndex(u32);

impl SimulationAuxiliaryLogicalIndex {
    /// Normalize a one-based position once, e.g. `SimulationAuxiliaryLogicalIndex::new(1)?`.
    pub fn new(one_based: u32) -> Result<Self> {
        require(
            (1..=i32::MAX as u32).contains(&one_based),
            "auxiliary logical index",
            format!("{one_based}; expected a positive signed-i32 one-based position"),
        )?;
        Ok(Self(one_based))
    }
    /// Exact one-based caller position, e.g. `index.one_based()`.
    pub fn one_based(self) -> u32 {
        self.0
    }
}

/// Typed root to which the allocation plan's stored fields belong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationAuxiliaryPlanParent {
    /// Complete stored row, which may use either profile.
    StoredRow(SimulationAuxiliaryBodyRevision),
    /// Complete normalized memory image.
    Workspace(SimulationAuxiliaryWorkspaceRevision),
}

/// Exact ID-gated request plan; no allocation, read or pointer use occurred.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryAllocationPlan {
    /// Exact ID/count validation scope.
    pub check_mode: SimulationAuxiliaryAllocationCheckMode,
    /// Request versus skipped gate, distinct from zero byte count.
    pub disposition: SimulationAuxiliaryAllocationDisposition,
    /// Stored-field source identity.
    pub parent: SimulationAuxiliaryPlanParent,
    /// Normalized supplied logical position.
    pub logical_index: SimulationAuxiliaryLogicalIndex,
    /// Original full identifier pattern, not converted to a portable pointer.
    pub identifier_pattern: u32,
    /// Signed stored count, retained even on a skipped mismatched row.
    pub vector_count: i32,
    /// Exact equality gate; length zero does not suppress it.
    pub selected: bool,
    /// Native allocation-call path would be entered; not an executed allocation.
    pub allocation_call_required: bool,
    /// Exact request, including Some(0) for a selected zero-length row.
    pub allocation_bytes: Option<u32>,
    /// Exact delegated read count, including Some(0) on the same selected path.
    pub read_bytes: Option<u32>,
    /// Always false: stored slot bits are not portable addresses.
    pub stored_pointer_usable: bool,
    /// Native operations not executed by this plan.
    pub diagnostics: [SimulationAuxiliaryRuntimeDiagnostic; 4],
}

impl SimulationAuxiliaryBodyRow {
    /// Plan only the inspected ID/count path, e.g. `row.allocation_plan(index)?`; mismatched negative counts remain unconsumed.
    pub fn allocation_plan(
        &self,
        index: SimulationAuxiliaryLogicalIndex,
    ) -> Result<SimulationAuxiliaryAllocationPlan> {
        allocation_plan(
            &self.bytes,
            index,
            SimulationAuxiliaryPlanParent::StoredRow(self.revision.clone()),
        )
    }
}

impl SimulationAuxiliaryWorkspace {
    /// Plan requests without using stored slot bits, e.g. `workspace.allocation_plan(index)?`.
    pub fn allocation_plan(
        &self,
        index: SimulationAuxiliaryLogicalIndex,
    ) -> Result<SimulationAuxiliaryAllocationPlan> {
        allocation_plan(
            &self.bytes,
            index,
            SimulationAuxiliaryPlanParent::Workspace(self.revision.clone()),
        )
    }

    /// Project explicit result bits into a selected memory slot, e.g. `workspace.project_pointer_binding(index, Some(0))?`; no native allocation/read/write executes.
    pub fn project_pointer_binding(
        &self,
        index: SimulationAuxiliaryLogicalIndex,
        result: Option<u64>,
    ) -> Result<SimulationAuxiliaryPointerBinding> {
        let plan = self.allocation_plan(index)?;
        let applied = required_result(&plan, result)?;
        let mut bytes = self.to_bytes();
        if let Some(bits) = applied {
            bytes[8..16].copy_from_slice(&bits.to_le_bytes());
        }
        let workspace = if bytes.as_slice() == self.bytes.as_ref() {
            self.clone()
        } else {
            derived_workspace(self, &bytes)?
        };
        Ok(binding_result(self, workspace, plan, applied))
    }
}

fn allocation_plan(
    bytes: &[u8],
    index: SimulationAuxiliaryLogicalIndex,
    parent: SimulationAuxiliaryPlanParent,
) -> Result<SimulationAuxiliaryAllocationPlan> {
    let identifier = read_pattern(bytes, 0);
    let count = read_pattern(bytes, 4) as i32;
    let length = request_length(identifier, index, count)?;
    Ok(plan_result(identifier, count, index, parent, length))
}

fn request_length(
    identifier: u32,
    index: SimulationAuxiliaryLogicalIndex,
    count: i32,
) -> Result<Option<u32>> {
    let selected = identifier == index.0;
    require(
        !selected || (0..=0x1fffffff).contains(&count),
        "auxiliary selected vector count",
        format!(
            "identifier {identifier:#010x}, logical index {}, count {count}; expected selected count 0..=536870911 for signed-width four-byte reads",
            index.0
        ),
    )?;
    Ok(selected.then(|| count as u32 * 4))
}

fn plan_result(
    identifier: u32,
    count: i32,
    index: SimulationAuxiliaryLogicalIndex,
    parent: SimulationAuxiliaryPlanParent,
    length: Option<u32>,
) -> SimulationAuxiliaryAllocationPlan {
    let selected = length.is_some();
    SimulationAuxiliaryAllocationPlan {
        check_mode: SimulationAuxiliaryAllocationCheckMode::ExactIdentifierAndSignedCount,
        disposition: plan_disposition(selected),
        parent,
        logical_index: index,
        identifier_pattern: identifier,
        vector_count: count,
        selected,
        allocation_call_required: selected,
        allocation_bytes: length,
        read_bytes: length,
        stored_pointer_usable: false,
        diagnostics: outcome::runtime_diagnostics(),
    }
}

fn plan_disposition(selected: bool) -> SimulationAuxiliaryAllocationDisposition {
    if selected {
        SimulationAuxiliaryAllocationDisposition::SelectedRequest
    } else {
        SimulationAuxiliaryAllocationDisposition::SkippedIdentifierMismatch
    }
}

fn required_result(
    plan: &SimulationAuxiliaryAllocationPlan,
    result: Option<u64>,
) -> Result<Option<u64>> {
    if !plan.selected {
        return Ok(None);
    }
    result.map(Some).ok_or_else(||invalid("auxiliary allocation result",format!("selected logical row {}, vector count {}, allocation result None; expected explicit u64 result bits even for zero-length requests",plan.logical_index.0,plan.vector_count)))
}

fn derived_workspace(
    parent: &SimulationAuxiliaryWorkspace,
    bytes: &[u8],
) -> Result<SimulationAuxiliaryWorkspace> {
    let mut workspace = SimulationAuxiliaryWorkspace::from_bytes(bytes)?;
    workspace.gen_parent = parent.gen_parent.clone();
    workspace.trial_source_offset = parent.trial_source_offset;
    Ok(workspace)
}

/// Memory-image completion distinct from allocation/native-I/O success.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryBindingCompletion {
    /// Identifier mismatch skipped binding and required no allocation result.
    SkippedIdentifierMismatch,
    /// Selected result bits were applied and matched the original bytes.
    ProjectedResultUnchanged,
    /// Selected result bits changed only the eight-byte slot.
    ProjectedResultChanged,
}

/// Exact slot replacement with source/history, plan and runtime exclusions together.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryPointerBinding {
    /// Exact complete-image and changed-slot check scope.
    pub check_mode: SimulationAuxiliaryPointerCheckMode,
    /// All bytes outside the slot survive exactly.
    pub fidelity: SimulationAuxiliaryPointerFidelity,
    /// Complete immutable source memory image.
    pub source: SimulationAuxiliaryWorkspace,
    /// Complete resulting memory image, never published as a file.
    pub workspace: SimulationAuxiliaryWorkspace,
    /// Original source identity.
    pub parent: SimulationAuxiliaryWorkspaceRevision,
    /// Result identity, reused when bytes are unchanged.
    pub result: SimulationAuxiliaryWorkspaceRevision,
    /// Exact stored-field request plan.
    pub plan: SimulationAuxiliaryAllocationPlan,
    /// Applied supplied bits; mismatch does not consume them.
    pub allocation_result: Option<u64>,
    /// Skip versus authentic selected no-op/change.
    pub completion: SimulationAuxiliaryBindingCompletion,
    /// All 288 bytes checked; only 8..16 may differ.
    pub checked_image_bytes: usize,
    /// Exact achieved slot-pattern error, zero.
    pub slot_pattern_error_bits: u32,
    /// Always true: no native allocation, read or file write executed.
    pub memory_only: bool,
    /// Explicit nonportable/runtime responsibilities.
    pub diagnostics: [SimulationAuxiliaryRuntimeDiagnostic; 4],
}

fn binding_result(
    source: &SimulationAuxiliaryWorkspace,
    workspace: SimulationAuxiliaryWorkspace,
    plan: SimulationAuxiliaryAllocationPlan,
    result: Option<u64>,
) -> SimulationAuxiliaryPointerBinding {
    let completion = binding_completion(source, &workspace, plan.selected);
    SimulationAuxiliaryPointerBinding {
        check_mode: SimulationAuxiliaryPointerCheckMode::ExactImageWithOnlySlotReplacement,
        fidelity: SimulationAuxiliaryPointerFidelity::ExactOutsideEightByteSlot,
        source: source.clone(),
        parent: source.revision.clone(),
        result: workspace.revision.clone(),
        workspace,
        plan,
        allocation_result: result,
        completion,
        checked_image_bytes: 288,
        slot_pattern_error_bits: 0,
        memory_only: true,
        diagnostics: outcome::runtime_diagnostics(),
    }
}

/// Exact scope of a request plan, never a completed allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryAllocationCheckMode {
    /// Validated identifier equality and signed count when selected.
    ExactIdentifierAndSignedCount,
}

/// Explicit gate disposition, including selected zero-byte requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryAllocationDisposition {
    /// ID equality requires an allocation call, even with count zero.
    SelectedRequest,
    /// Mismatched ID skips count consumption and allocation/read paths.
    SkippedIdentifierMismatch,
}

/// Exact scope of a supplied-result slot projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryPointerCheckMode {
    /// Every image byte is retained except the exact selected 8..16 replacement.
    ExactImageWithOnlySlotReplacement,
}

/// Complete memory-image preservation scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryPointerFidelity {
    /// Bytes outside the eight-byte numeric slot remain exact.
    ExactOutsideEightByteSlot,
}

fn binding_completion(
    source: &SimulationAuxiliaryWorkspace,
    result: &SimulationAuxiliaryWorkspace,
    selected: bool,
) -> SimulationAuxiliaryBindingCompletion {
    if !selected {
        return SimulationAuxiliaryBindingCompletion::SkippedIdentifierMismatch;
    }
    if source.revision == result.revision {
        return SimulationAuxiliaryBindingCompletion::ProjectedResultUnchanged;
    }
    SimulationAuxiliaryBindingCompletion::ProjectedResultChanged
}
