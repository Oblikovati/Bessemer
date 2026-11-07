use super::packet::Cursor;
use bessemer::simulation::*;

pub fn row(raw: &[u8]) -> SimulationAuxiliaryBodyRow {
    let profile = SimulationAuxiliaryBodyProfile::from_stride(raw.len()).unwrap();
    SimulationAuxiliaryBodyRow::from_bytes(raw, profile).unwrap()
}

pub fn append_words(output: &mut Vec<u8>, values: &[u32]) {
    output.extend(values.iter().flat_map(|v| v.to_le_bytes()));
}

pub fn append_span(output: &mut Vec<u8>, bytes: &[u8]) {
    append_words(output, &[bytes.len() as u32]);
    output.extend_from_slice(bytes);
}

pub fn name_bytes(row: &SimulationAuxiliaryBodyRow) -> Vec<u8> {
    let view = row.name_view();
    let mut actual = Vec::new();
    append_words(&mut actual, &name_header(row, &view));
    append_name_payload(&mut actual, &view);
    let units: Vec<_> = view
        .stored_units
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect();
    assert_eq!(units, view.name_buffer);
    assert!(!view.native_trim_right_applied);
    actual
}

fn name_header(row: &SimulationAuxiliaryBodyRow, view: &SimulationAuxiliaryNameView) -> [u32; 9] {
    let fields = row.fields();
    let offset = if row.profile().stride() == 284 {
        20
    } else {
        24
    };
    [
        row.profile().stride() as u32,
        fields.body_id.pattern(),
        view.vector_count as u32,
        fields.stored_low as u32,
        fields.stored_high as u32,
        offset,
        view.prefix_units as u32,
        u32::from(view.source_nul_index.is_some()),
        view.source_nul_index.unwrap_or(0) as u32,
    ]
}

fn append_name_payload(output: &mut Vec<u8>, view: &SimulationAuxiliaryNameView) {
    append_span(output, &view.opaque_prefix);
    append_span(output, &view.name_buffer);
    let prefix: Vec<_> = view
        .bounded_units()
        .iter()
        .flat_map(|u| u.to_le_bytes())
        .collect();
    append_span(output, &prefix);
    append_span(output, view.unconsumed_name_bytes());
    append_words(output, &view.trailing_words);
}

pub fn name_store(
    row: &SimulationAuxiliaryBodyRow,
    units: &[u16],
    golden: &[u8],
    copied: usize,
) -> SimulationAuxiliaryNameStore {
    let result = row.project_name_store(units).unwrap();
    assert_eq!(result.row.to_bytes(), golden);
    assert_name_identity(row, &result, golden);
    assert_name_coverage(&result, row.profile().stride(), copied);
    result
}

fn assert_name_identity(
    row: &SimulationAuxiliaryBodyRow,
    result: &SimulationAuxiliaryNameStore,
    golden: &[u8],
) {
    assert_eq!(result.parent, *row.revision());
    assert_eq!(result.result, *result.row.revision());
    assert_eq!(result.row.gen_parent(), row.gen_parent());
    let unchanged = row.to_bytes() == golden;
    assert_eq!(result.parent == result.result, unchanged);
    let no_change = bessemer::display_lists::DisplayEditCompletion::Unchanged;
    assert_eq!(result.completion == no_change, unchanged);
}

fn assert_name_coverage(result: &SimulationAuxiliaryNameStore, length: usize, copied: usize) {
    assert_eq!(result.requested_pattern_error_bits, 0);
    assert_eq!(result.coverage.copied_units, copied);
    assert_eq!(result.coverage.written_bytes, (copied + 1) * 2);
    assert_eq!(result.coverage.row_bytes, length);
    assert!(!result.coverage.native_in_place_row_update_proven);
    let mode = SimulationAuxiliaryNameCheckMode::ExactBoundedCopyAndResidualPreservation;
    assert_eq!(result.coverage.mode, mode);
    let fidelity = SimulationAuxiliaryNameFidelity::ExactOutsideCopiedPrefixAndTerminator;
    assert_eq!(result.fidelity, fidelity);
    assert!(!result.tail_cleared);
    assert!(!result.native_trim_right_applied);
}

pub fn plan_bytes(plan: &SimulationAuxiliaryAllocationPlan) -> Vec<u8> {
    assert_plan_contract(plan);
    let mut actual = Vec::new();
    append_words(&mut actual, &plan_words(plan));
    actual
}

fn plan_words(plan: &SimulationAuxiliaryAllocationPlan) -> [u32; 8] {
    [
        u32::from(plan.selected),
        u32::from(plan.allocation_call_required),
        plan.vector_count as u32,
        u32::from(plan.allocation_bytes.is_some()),
        plan.allocation_bytes.unwrap_or(0),
        u32::from(plan.read_bytes.is_some()),
        plan.read_bytes.unwrap_or(0),
        u32::from(plan.stored_pointer_usable),
    ]
}

fn assert_plan_contract(plan: &SimulationAuxiliaryAllocationPlan) {
    assert!(plan.logical_index.one_based() > 0);
    assert_eq!(
        plan.selected,
        plan.identifier_pattern == plan.logical_index.one_based()
    );
    let disposition = if plan.selected {
        SimulationAuxiliaryAllocationDisposition::SelectedRequest
    } else {
        SimulationAuxiliaryAllocationDisposition::SkippedIdentifierMismatch
    };
    assert_eq!(plan.disposition, disposition);
    let mode = SimulationAuxiliaryAllocationCheckMode::ExactIdentifierAndSignedCount;
    assert_eq!(plan.check_mode, mode);
}

pub fn normalization(
    row: &SimulationAuxiliaryBodyRow,
    prior: &SimulationAuxiliaryWorkspace,
    expected: &[u8],
    discarded: Option<&[u8]>,
) -> SimulationAuxiliaryNormalization {
    let result = row.normalize_workspace(Some(prior)).unwrap();
    assert_eq!(result.workspace.to_bytes(), expected);
    assert_norm_sources(row, prior, &result, discarded);
    assert_norm_coverage(&result);
    assert_copy_history(&result, &result.copied_spans, None, expected);
    result
}

fn assert_norm_sources(
    row: &SimulationAuxiliaryBodyRow,
    prior: &SimulationAuxiliaryWorkspace,
    result: &SimulationAuxiliaryNormalization,
    discarded: Option<&[u8]>,
) {
    assert_eq!(result.source.revision(), row.revision());
    assert_eq!(result.source.to_bytes(), row.to_bytes());
    assert_eq!(result.workspace.gen_parent(), row.gen_parent());
    let omitted = result.legacy_word_discarded.as_ref().map(|v| v.as_slice());
    assert_eq!(omitted, discarded);
    let legacy = row.profile().stride() == 284;
    assert_eq!(result.initial_workspace.is_some(), legacy);
    if legacy {
        assert_prior_slot(prior, result);
    }
}

fn assert_prior_slot(
    prior: &SimulationAuxiliaryWorkspace,
    result: &SimulationAuxiliaryNormalization,
) {
    let captured = result.initial_workspace.as_ref().unwrap();
    assert_eq!(captured.revision(), prior.revision());
    assert_eq!(
        result.workspace.pointer_slot_bits(),
        prior.pointer_slot_bits()
    );
    assert_eq!(captured.to_bytes(), prior.to_bytes());
}

fn assert_norm_coverage(result: &SimulationAuxiliaryNormalization) {
    let legacy = result.initial_workspace.is_some();
    assert_eq!(result.copied_pattern_error_bits, 0);
    assert!(result.coverage.memory_only);
    assert_eq!(result.coverage.destination_bytes, 288);
    let mode = SimulationAuxiliaryNormalizationCheckMode::ExactStoredSpansAndSuppliedSlot;
    assert_eq!(result.coverage.mode, mode);
    assert_eq!(
        result.coverage.prior_workspace_bytes,
        if legacy { 8 } else { 0 }
    );
    assert_eq!(
        result.coverage.uncopied_stored_bytes,
        if legacy { 4 } else { 0 }
    );
    let completion = if legacy {
        SimulationAuxiliaryNormalizationCompletion::ConvertedLegacySpans
    } else {
        SimulationAuxiliaryNormalizationCompletion::CopiedModernImage
    };
    assert_eq!(result.completion, completion);
}

pub fn composed(
    row: &SimulationAuxiliaryBodyRow,
    prior: &SimulationAuxiliaryWorkspace,
    index: SimulationAuxiliaryLogicalIndex,
    bits: Option<u64>,
    expected: &[u8],
) -> SimulationAuxiliaryBoundProjection {
    let result = row.normalize_and_bind(Some(prior), index, bits).unwrap();
    assert_eq!(result.binding.workspace.to_bytes(), expected);
    assert_eq!(
        result.binding.parent,
        *result.normalization.workspace.revision()
    );
    assert_eq!(
        result.binding.source.revision(),
        result.normalization.workspace.revision()
    );
    assert_copy_history(
        &result.normalization,
        &result.final_spans,
        result.binding.allocation_result,
        expected,
    );
    assert_binding_contract(&result.binding);
    result
}

pub fn assert_binding_contract(result: &SimulationAuxiliaryPointerBinding) {
    assert_binding_identity(result);
    assert_eq!(result.checked_image_bytes, 288);
    assert!(result.memory_only);
    assert_eq!(result.slot_pattern_error_bits, 0);
    let mode = SimulationAuxiliaryPointerCheckMode::ExactImageWithOnlySlotReplacement;
    assert_eq!(result.check_mode, mode);
    let fidelity = SimulationAuxiliaryPointerFidelity::ExactOutsideEightByteSlot;
    assert_eq!(result.fidelity, fidelity);
    assert!(!result.plan.stored_pointer_usable);
}

fn assert_binding_identity(result: &SimulationAuxiliaryPointerBinding) {
    assert_eq!(result.parent, *result.source.revision());
    assert_eq!(result.result, *result.workspace.revision());
    assert_eq!(result.source.gen_parent(), result.workspace.gen_parent());
    assert_eq!(
        result.source.trial_source_offset(),
        result.workspace.trial_source_offset()
    );
    let unchanged = result.source.to_bytes() == result.workspace.to_bytes();
    assert_eq!(result.parent == result.result, unchanged);
    let expected = if !result.plan.selected {
        SimulationAuxiliaryBindingCompletion::SkippedIdentifierMismatch
    } else if unchanged {
        SimulationAuxiliaryBindingCompletion::ProjectedResultUnchanged
    } else {
        SimulationAuxiliaryBindingCompletion::ProjectedResultChanged
    };
    assert_eq!(result.completion, expected);
}

fn assert_copy_history(
    normalization: &SimulationAuxiliaryNormalization,
    spans: &[SimulationAuxiliarySpanCopy],
    bits: Option<u64>,
    expected: &[u8],
) {
    let row = normalization.source.to_bytes();
    let prior = normalization
        .initial_workspace
        .as_ref()
        .map(|p| p.to_bytes());
    let allocation = bits.map(u64::to_le_bytes);
    let parents = ReplayParents {
        row: &row,
        prior: prior.as_deref(),
        allocation: allocation.as_ref(),
    };
    let mut coverage = [0u8; 288];
    let mut replay = [0u8; 288];
    for span in spans {
        replay_span(&parents, *span, &mut coverage, &mut replay);
    }
    assert!(coverage.iter().all(|v| *v == 1));
    assert_eq!(replay.as_slice(), expected);
}

struct ReplayParents<'a> {
    row: &'a [u8],
    prior: Option<&'a [u8]>,
    allocation: Option<&'a [u8; 8]>,
}

fn replay_source<'a>(
    parents: &ReplayParents<'a>,
    source: SimulationAuxiliaryCopySource,
) -> &'a [u8] {
    match source {
        SimulationAuxiliaryCopySource::StoredRow => parents.row,
        SimulationAuxiliaryCopySource::InitialWorkspace => parents.prior.unwrap(),
        SimulationAuxiliaryCopySource::SuppliedAllocationResult => parents.allocation.unwrap(),
    }
}

fn replay_span(
    parents: &ReplayParents<'_>,
    span: SimulationAuxiliarySpanCopy,
    coverage: &mut [u8; 288],
    replay: &mut [u8; 288],
) {
    let source = replay_source(parents, span.source);
    for (index, value) in source[span.source_offset..span.source_offset + span.length]
        .iter()
        .enumerate()
    {
        coverage[span.destination_offset + index] += 1;
        replay[span.destination_offset + index] = *value;
    }
}

pub fn caller_units(cursor: &mut Cursor<'_>) -> Vec<u16> {
    cursor
        .span()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|b| u16::from_le_bytes(*b))
        .collect()
}
