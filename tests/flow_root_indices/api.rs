use super::*;

fn limits() -> FlowRootIndexLimits {
    FlowRootIndexLimits {
        maximum_roots: 10_000,
        maximum_leaves: 100_000,
        maximum_path_bytes: 10_000_000,
    }
}

pub fn check_projection(case: &fixture::RootCase, context: &FlowDerivedLeafCount) {
    let projection = context.project_root_indices(limits()).unwrap();
    check_checkpoints(&projection, context);
    assert_eq!(
        projection.limits().maximum_path_bytes,
        limits().maximum_path_bytes
    );
    assert_eq!(projection.roots().len(), case.roots.len());
    assert!(projection.root(case.roots.len()).is_none());
    assert!(projection.root(usize::MAX).is_none());
    for (index, (actual, expected)) in projection.roots().zip(&case.roots).enumerate() {
        check_root(actual, expected);
        check_root(projection.root(index).unwrap(), expected);
    }
    check_coverage(&projection, case);
    check_bounds(context, case);
}

fn check_root(actual: FlowProjectedRoot<'_>, expected: &fixture::ExpectedRoot) {
    assert_eq!(actual.indices(), expected.indices);
    assert_eq!(actual.storage_index().value(), expected.storage_index);
    assert_eq!(actual.storage_index().byte_offset(), expected.byte_offset);
    assert_eq!(actual.leaves().len(), expected.leaves.len());
    for (leaf, frozen) in actual.leaves().zip(&expected.leaves) {
        assert_eq!(leaf.local_leaf_ordinal(), frozen.ordinal as usize);
        assert_eq!(leaf.child_path(), frozen.path);
        assert_eq!(
            leaf.wire_interval(),
            (frozen.start as usize, frozen.end as usize)
        );
    }
}

fn check_coverage(projection: &FlowRootIndexProjection, case: &fixture::RootCase) {
    let coverage = projection.coverage();
    let leaves: Vec<_> = case.roots.iter().flat_map(|root| &root.leaves).collect();
    assert_eq!(
        coverage.mode,
        FlowRootIndexCheckMode::ExactRootStorageAndLocalPaths
    );
    assert_eq!(coverage.checked_roots, case.roots.len());
    assert_eq!(coverage.checked_leaves, case.leaf_count as usize);
    assert_eq!(
        coverage.child_path_bytes,
        leaves.iter().map(|leaf| leaf.path.len()).sum()
    );
    assert_eq!(
        coverage.maximum_path_depth as usize,
        leaves.iter().map(|leaf| leaf.path.len()).max().unwrap_or(0)
    );
    assert_eq!(coverage.achieved_error, 0);
    check_diagnostics(projection);
}

fn check_bounds(context: &FlowDerivedLeafCount, case: &fixture::RootCase) {
    let path_bytes: usize = case
        .roots
        .iter()
        .flat_map(|root| &root.leaves)
        .map(|leaf| leaf.path.len())
        .sum();
    let exact = FlowRootIndexLimits {
        maximum_roots: case.roots.len().max(1),
        maximum_leaves: (case.leaf_count as usize).max(1),
        maximum_path_bytes: path_bytes.max(1),
    };
    assert!(context.project_root_indices(exact).is_ok());
    check_short_limits(
        context,
        exact,
        [case.roots.len(), case.leaf_count as usize, path_bytes],
    );
}

#[test]
fn zero_limits_reject_without_changing_source_context() {
    let case = fixture::cases().remove(0);
    let (grid, trees) = parsed_inputs(&case);
    let context =
        derive_fresh_flow_leaf_count(&grid, &trees, FlowFreshContextAssumption::AssumeFresh)
            .unwrap();
    for axis in 0..3 {
        let mut invalid = limits();
        match axis {
            0 => invalid.maximum_roots = 0,
            1 => invalid.maximum_leaves = 0,
            _ => invalid.maximum_path_bytes = 0,
        }
        let error = context.project_root_indices(invalid).unwrap_err();
        assert_eq!(error.operation, "Flow root indices");
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
    }
    assert_eq!(context.grid().record(), grid.record());
    assert_eq!(context.trees().record(), trees.record());
}

#[test]
fn all_public_projection_types_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<FlowRootIndexLimits>();
    assert_traits::<FlowRootStorageIndex>();
    assert_traits::<FlowRootIndexProjection>();
    assert_traits::<FlowProjectedRoot<'static>>();
    assert_traits::<FlowProjectedLeaf<'static>>();
    assert_traits::<FlowRootIndexCoverage>();
    assert_traits::<FlowRootIndexCheckMode>();
    assert_traits::<FlowRootIndexDiagnostic>();
}

#[test]
fn full_depth_paths_survive_input_mutation_and_eight_worker_projection() {
    let mut cases = fixture::cases();
    let mut case = cases.pop().unwrap();
    let (grid, trees) = parsed_inputs(&case);
    let context =
        derive_fresh_flow_leaf_count(&grid, &trees, FlowFreshContextAssumption::AssumeFresh)
            .unwrap();
    let projection = context.project_root_indices(limits()).unwrap();
    case.grid.fill(255);
    case.trees.fill(255);
    let expected = projection_packet(&projection);
    check_workers(&context, &expected);
    assert_eq!(projection_packet(&projection), expected);
    assert_eq!(projection.coverage().maximum_path_depth, 256);
}

fn check_workers(context: &FlowDerivedLeafCount, expected: &[u8]) {
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    let result = context.project_root_indices(limits()).unwrap();
                    assert_eq!(projection_packet(&result), expected);
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

fn projection_packet(projection: &FlowRootIndexProjection) -> Vec<u8> {
    let mut bytes = Vec::new();
    for root in projection.roots() {
        bytes.extend_from_slice(&root.storage_index().value().to_le_bytes());
        bytes.extend_from_slice(&root.storage_index().byte_offset().to_le_bytes());
        for leaf in root.leaves() {
            bytes.extend_from_slice(&(leaf.local_leaf_ordinal() as u64).to_le_bytes());
            bytes.extend_from_slice(&(leaf.child_path().len() as u64).to_le_bytes());
            bytes.extend_from_slice(leaf.child_path());
            let (start, end) = leaf.wire_interval();
            bytes.extend_from_slice(&(start as u64).to_le_bytes());
            bytes.extend_from_slice(&(end as u64).to_le_bytes());
        }
    }
    bytes
}

fn check_checkpoints(projection: &FlowRootIndexProjection, context: &FlowDerivedLeafCount) {
    assert_eq!(
        projection.source_context().grid().record(),
        context.grid().record()
    );
    assert_eq!(
        projection.source_context().trees().record(),
        context.trees().record()
    );
}

fn check_diagnostics(projection: &FlowRootIndexProjection) {
    assert_eq!(
        projection.diagnostics(),
        &[
            FlowRootIndexDiagnostic::NativeContextNotExecuted,
            FlowRootIndexDiagnostic::GlobalBlockOrdinalsNotAssigned,
            FlowRootIndexDiagnostic::ParallelScanOffsetsNotValidated,
            FlowRootIndexDiagnostic::NativeApplicationNotChecked
        ]
    );
}

fn check_short_limits(
    context: &FlowDerivedLeafCount,
    exact: FlowRootIndexLimits,
    counts: [usize; 3],
) {
    for (axis, count) in counts.into_iter().enumerate() {
        if count == 0 {
            continue;
        }
        let mut short = exact;
        match axis {
            0 => short.maximum_roots = count - 1,
            1 => short.maximum_leaves = count - 1,
            _ => short.maximum_path_bytes = count - 1,
        }
        assert!(context.project_root_indices(short).is_err());
    }
}
