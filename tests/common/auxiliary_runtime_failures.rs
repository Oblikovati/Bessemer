use super::checks;
use bessemer::simulation::*;

fn raw_row(stride: usize, identifier: u32, count: i32) -> Vec<u8> {
    let mut raw = vec![0x5au8; stride];
    raw[..4].copy_from_slice(&identifier.to_le_bytes());
    raw[4..8].copy_from_slice(&count.to_le_bytes());
    let start = if stride == 284 { 20 } else { 24 };
    raw[start..start + 256].fill(0);
    raw
}

fn supplied_prior() -> SimulationAuxiliaryWorkspace {
    let raw: Vec<_> = (0..288).map(|i| (i * 11 + 97) as u8).collect();
    SimulationAuxiliaryWorkspace::from_bytes(&raw).unwrap()
}

#[test]
fn workspace_and_logical_index_boundaries_reject_incomplete_shapes_and_unassigned_positions() {
    for size in 0..288 {
        assert!(SimulationAuxiliaryWorkspace::from_bytes(&vec![0; size]).is_err());
    }
    for size in [289, 576] {
        assert!(SimulationAuxiliaryWorkspace::from_bytes(&vec![0; size]).is_err());
    }
    for index in [0, 0x80000000, u32::MAX] {
        assert!(SimulationAuxiliaryLogicalIndex::new(index).is_err());
    }
    assert_eq!(
        SimulationAuxiliaryLogicalIndex::new(i32::MAX as u32)
            .unwrap()
            .one_based(),
        i32::MAX as u32
    );
}

#[test]
fn legacy_requires_prior_bytes_while_modern_ignores_them_and_preserves_full_image() {
    let legacy = checks::row(&raw_row(284, 1, 0));
    assert!(legacy.normalize_workspace(None).is_err());
    let modern = checks::row(&raw_row(288, 1, 0));
    let prior = supplied_prior();
    for initial in [None, Some(&prior)] {
        let normalized = modern.normalize_workspace(initial).unwrap();
        assert_eq!(normalized.workspace.to_bytes(), modern.to_bytes());
        assert!(normalized.initial_workspace.is_none());
        assert!(normalized.legacy_word_discarded.is_none());
        assert_eq!(normalized.coverage.prior_workspace_bytes, 0);
    }
}

#[test]
fn legacy_word_is_discarded_without_assigning_or_zeroing_the_supplied_eight_byte_slot() {
    let mut raw = raw_row(284, 1, 0);
    raw[8..12].copy_from_slice(&0x12345678u32.to_le_bytes());
    let row = checks::row(&raw);
    let prior = supplied_prior();
    let before = prior.to_bytes();
    let normalized = row.normalize_workspace(Some(&prior)).unwrap();
    assert_eq!(&normalized.workspace.to_bytes()[8..16], &before[8..16]);
    assert_eq!(
        normalized.legacy_word_discarded,
        Some(0x12345678u32.to_le_bytes())
    );
    assert_ne!(normalized.workspace.pointer_slot_bits(), 0);
    assert_eq!(normalized.source.to_bytes(), raw);
    assert_eq!(prior.to_bytes(), before);
}

#[test]
fn selected_zero_count_requires_explicit_result_even_when_the_slot_does_not_change() {
    let row = checks::row(&raw_row(288, 1, 0));
    let workspace = row.normalize_workspace(None).unwrap().workspace;
    let index = SimulationAuxiliaryLogicalIndex::new(1).unwrap();
    let plan = workspace.allocation_plan(index).unwrap();
    assert!(plan.selected);
    assert!(plan.allocation_call_required);
    assert_eq!(plan.allocation_bytes, Some(0));
    assert_eq!(plan.read_bytes, Some(0));
    let original = workspace.to_bytes();
    assert!(workspace.project_pointer_binding(index, None).is_err());
    assert_eq!(workspace.to_bytes(), original);
    let result = workspace
        .project_pointer_binding(index, Some(workspace.pointer_slot_bits()))
        .unwrap();
    assert_eq!(
        result.completion,
        SimulationAuxiliaryBindingCompletion::ProjectedResultUnchanged
    );
    assert_eq!(result.parent, result.result);
    checks::assert_binding_contract(&result);
}

#[test]
fn mismatched_negative_counts_skip_requests_and_ignore_supplied_result_bits() {
    let row = checks::row(&raw_row(288, 99, i32::MIN));
    let workspace = row.normalize_workspace(None).unwrap().workspace;
    let index = SimulationAuxiliaryLogicalIndex::new(1).unwrap();
    for result in [None, Some(0), Some(u64::MAX)] {
        let bound = workspace.project_pointer_binding(index, result).unwrap();
        assert_eq!(bound.workspace.to_bytes(), workspace.to_bytes());
        assert_eq!(
            bound.completion,
            SimulationAuxiliaryBindingCompletion::SkippedIdentifierMismatch
        );
        assert!(bound.allocation_result.is_none());
        assert_eq!(bound.plan.allocation_bytes, None);
        assert_eq!(bound.plan.read_bytes, None);
        assert!(!bound.plan.allocation_call_required);
    }
}

#[test]
fn selected_read_width_stops_preserve_source_and_the_exact_upper_bound_is_supported() {
    let index = SimulationAuxiliaryLogicalIndex::new(1).unwrap();
    for count in [i32::MIN, -1, 0x20000000, i32::MAX] {
        let raw = raw_row(288, 1, count);
        let row = checks::row(&raw);
        let workspace = row.normalize_workspace(None).unwrap().workspace;
        assert!(row.allocation_plan(index).is_err());
        assert!(workspace.project_pointer_binding(index, Some(0)).is_err());
        assert_eq!(workspace.to_bytes(), raw);
        assert_eq!(row.to_bytes(), raw);
    }
    let row = checks::row(&raw_row(288, 1, 0x1fffffff));
    assert_eq!(
        row.allocation_plan(index).unwrap().read_bytes,
        Some(0x7ffffffc)
    );
}

#[test]
fn unit_128_is_not_scanned_and_truncation_retains_split_surrogates_without_trimming() {
    let mut raw = raw_row(288, 1, 0);
    for unit in raw[24..280].as_chunks_mut::<2>().0 {
        unit.copy_from_slice(&65u16.to_le_bytes());
    }
    raw[278..280].fill(0);
    let row = checks::row(&raw);
    let view = row.name_view();
    assert_eq!(view.prefix_units, 127);
    assert_eq!(view.source_nul_index, None);
    assert_eq!(view.unconsumed_name_bytes(), [0, 0]);
    let mut caller = vec![65u16; 126];
    caller.extend([0xd800, 0xdc00]);
    let stored = row.project_name_store(&caller).unwrap();
    assert_eq!(stored.coverage.copied_units, 127);
    let view = stored.row.name_view();
    assert_eq!(view.bounded_units()[126], 0xd800);
    assert_eq!(view.stored_units[127], 0);
    assert!(!view.native_trim_right_applied);
    let spaces = row.project_name_store(&[65, 32, 32]).unwrap();
    assert_eq!(spaces.row.name_view().bounded_units(), [65, 32, 32]);
}

#[test]
fn name_view_accessors_remain_bounded_after_caller_metadata_mutation() {
    let row = checks::row(&raw_row(288, 1, 0));
    let mut view = row.name_view();
    view.prefix_units = usize::MAX;
    assert_eq!(view.bounded_units().len(), 127);
    assert_eq!(view.unconsumed_name_bytes().len(), 2);
    assert_eq!(row.name_view().prefix_units, 0);
    assert_eq!(row.name_view().source_nul_index, Some(0));
}

#[test]
fn mutation_drop_and_failure_leave_published_rows_and_workspace_results_isolated() {
    let mut raw = raw_row(284, 1, 0);
    let row = checks::row(&raw);
    let prior = supplied_prior();
    let prior_bytes = prior.to_bytes();
    let mut caller = vec![65u16, 0xd800, 32];
    let named = row.project_name_store(&caller).unwrap();
    raw.fill(0);
    caller.fill(0);
    assert_eq!(named.row.name_view().bounded_units(), [65, 0xd800, 32]);
    assert_ne!(named.row.to_bytes(), raw);
    let index = SimulationAuxiliaryLogicalIndex::new(1).unwrap();
    assert!(row.normalize_and_bind(Some(&prior), index, None).is_err());
    let result = row
        .normalize_and_bind(Some(&prior), index, Some(u64::MAX))
        .unwrap();
    let expected = result.binding.workspace.to_bytes();
    drop(prior);
    drop(row);
    assert_dropped_result(&result, &expected, &prior_bytes);
}

#[test]
fn name_workspace_plan_and_history_types_are_send_sync_and_parallel_bytes_equal_serial() {
    assert_send_sync_names();
    assert_send_sync_workspaces();
    let row = checks::row(&raw_row(284, 1, 0));
    let prior = supplied_prior();
    let index = SimulationAuxiliaryLogicalIndex::new(1).unwrap();
    let (serial, serial_name) = serial_results(&row, &prior, index);
    std::thread::scope(|scope| {
        let tasks: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat(&row, &prior, index, &serial, &serial_name)))
            .collect();
        for task in tasks {
            task.join().unwrap();
        }
    });
}

fn repeat(
    row: &SimulationAuxiliaryBodyRow,
    prior: &SimulationAuxiliaryWorkspace,
    index: SimulationAuxiliaryLogicalIndex,
    serial: &[u8],
    serial_name: &[u8],
) {
    for _ in 0..32 {
        let result = checks::composed(row, prior, index, Some(u64::MAX), serial);
        assert_eq!(result.normalization.source.revision(), row.revision());
        let store = row.project_name_store(&[65, 0xd800, 32]).unwrap();
        assert_eq!(store.row.to_bytes(), serial_name);
        assert_eq!(store.row.name_view().bounded_units(), [65, 0xd800, 32]);
        assert_eq!(
            prior.pointer_slot_bits(),
            result
                .normalization
                .initial_workspace
                .as_ref()
                .unwrap()
                .pointer_slot_bits()
        );
    }
}

fn assert_send_sync_names() {
    fn check<T: Send + Sync>() {}
    check::<SimulationAuxiliaryNameView>();
    check::<SimulationAuxiliaryNameCoverage>();
    check::<SimulationAuxiliaryNameStore>();
    check::<SimulationAuxiliaryNameCheckMode>();
    check::<SimulationAuxiliaryNameFidelity>();
    check::<SimulationAuxiliaryNameDiagnostic>();
}

fn assert_send_sync_workspaces() {
    fn check<T: Send + Sync>() {}
    check::<SimulationAuxiliaryWorkspaceRevision>();
    check::<SimulationAuxiliaryWorkspace>();
    check::<SimulationAuxiliaryCopySource>();
    check::<SimulationAuxiliarySpanCopy>();
    check::<SimulationAuxiliaryRuntimeDiagnostic>();
    check::<SimulationAuxiliaryNormalizationCoverage>();
    check::<SimulationAuxiliaryNormalizationCheckMode>();
    check::<SimulationAuxiliaryNormalizationCompletion>();
    check::<SimulationAuxiliaryNormalization>();
    check::<SimulationAuxiliaryLogicalIndex>();
    check::<SimulationAuxiliaryPlanParent>();
    check::<SimulationAuxiliaryAllocationPlan>();
    check::<SimulationAuxiliaryBindingCompletion>();
    check::<SimulationAuxiliaryPointerBinding>();
    check::<SimulationAuxiliaryAllocationCheckMode>();
    check::<SimulationAuxiliaryAllocationDisposition>();
    check::<SimulationAuxiliaryPointerCheckMode>();
    check::<SimulationAuxiliaryPointerFidelity>();
    check::<SimulationAuxiliaryBoundProjection>();
}

fn assert_dropped_result(
    result: &SimulationAuxiliaryBoundProjection,
    expected: &[u8],
    prior_bytes: &[u8],
) {
    let mut returned = result.binding.workspace.to_bytes();
    returned.fill(0);
    assert_eq!(result.binding.workspace.to_bytes(), expected);
    assert_ne!(expected, returned);
    let captured = result.normalization.initial_workspace.as_ref().unwrap();
    assert_eq!(captured.to_bytes(), prior_bytes);
}

fn serial_results(
    row: &SimulationAuxiliaryBodyRow,
    prior: &SimulationAuxiliaryWorkspace,
    index: SimulationAuxiliaryLogicalIndex,
) -> (Vec<u8>, Vec<u8>) {
    let result = row
        .normalize_and_bind(Some(prior), index, Some(u64::MAX))
        .unwrap();
    let serial = result.binding.workspace.to_bytes();
    let named = row.project_name_store(&[65, 0xd800, 32]).unwrap();
    (serial, named.row.to_bytes())
}
