use super::*;
#[path = "../flow_carriers/fixture.rs"]
mod native_fixture;

#[test]
fn all_81_profiles_match_complete_source_forests_raw_fields_and_context() {
    for case in fixture::cases() {
        let parsed = trees(&case);
        check_header(&parsed, &case);
        assert_eq!(parsed.nodes().len(), case.nodes.len());
        for (actual, expected) in parsed.nodes().iter().zip(&case.nodes) {
            check_node(actual, expected);
        }
        check_roots(&parsed, &case);
        check_coverage(&parsed, &case);
        assert!(!case.native_order_proven);
    }
}

fn check_header(parsed: &FlowBlockTrees, case: &fixture::TreeCase) {
    assert_eq!(parsed.source().as_bytes(), wire(&case.stored));
    assert_eq!(
        parsed.record(),
        &parsed.source().latest(400).unwrap().unwrap().id
    );
    assert_eq!(parsed.version(), case.version);
    assert_eq!(parsed.decoded_bytes(), case.decoded, "{}", case.name);
    assert_eq!(*parsed.bounds().pairs(), case.bounds);
    assert_eq!(parsed.bounds().grid_count(), case.grid_count);
    assert_eq!(parsed.bounds_stored(), case.bounds_stored);
    assert_eq!(parsed.body_present(), case.body_present);
    assert_eq!(parsed.body_presence_byte(), case.body_presence);
    assert_eq!(parsed.options().maximum_depth, case.maximum_depth);
    assert_eq!(
        parsed.options().legacy_bounds.map(|bounds| *bounds.pairs()),
        case.legacy_bounds
    );
    assert_eq!(parsed.diagnostics().len(), 4);
}

fn check_node(actual: &FlowTreeNode, expected: &fixture::ExpectedNode) {
    assert_eq!(actual.payload_interval(), (expected.start, expected.end));
    assert_eq!(actual.marker_byte(), expected.marker);
    assert_eq!(actual.wire_depth(), expected.depth);
    assert_eq!(actual.native_level_byte(), expected.level);
    assert_eq!(actual.discarded_leaf_word(), expected.word);
    let children: Vec<(Option<u8>, usize)> = actual
        .children()
        .iter()
        .map(|child| (child.discarded_prefix_byte(), child.node_index()))
        .collect();
    assert_eq!(children, expected.children);
}

fn check_roots(parsed: &FlowBlockTrees, case: &fixture::TreeCase) {
    assert_eq!(parsed.roots().is_some(), case.roots.is_some());
    let Some(expected) = &case.roots else {
        return;
    };
    let actual = parsed.roots().unwrap();
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected) {
        assert_eq!(actual.payload_interval(), (expected.start, expected.end));
        assert_eq!(actual.indices(), expected.indices);
        assert_eq!(actual.presence_byte(), expected.presence);
        assert_eq!(actual.node_index(), expected.node);
    }
}

fn check_coverage(parsed: &FlowBlockTrees, case: &fixture::TreeCase) {
    let coverage = parsed.coverage();
    let mode = if case.compressed {
        FlowTreeCheckMode::ExactZlibForestFrames
    } else {
        FlowTreeCheckMode::ExactRawForestFrames
    };
    assert_eq!(coverage.mode, mode);
    assert_eq!(coverage.stored_bytes, case.stored.len());
    assert_eq!(coverage.decoded_bytes, case.decoded.len());
    assert_eq!(
        coverage.checked_roots,
        case.roots.as_ref().map_or(0, Vec::len)
    );
    assert_eq!(coverage.checked_nodes, case.nodes.len());
    let splits = case.nodes.iter().filter(|node| node.marker == 255).count();
    assert_eq!(coverage.checked_splits, splits);
    assert_eq!(coverage.checked_leaves, case.nodes.len() - splits);
    check_discarded(coverage, case);
    assert_eq!(coverage.achieved_byte_error, 0);
}

fn check_discarded(coverage: FlowTreeCoverage, case: &fixture::TreeCase) {
    let prefixes = case
        .nodes
        .iter()
        .flat_map(|node| &node.children)
        .filter(|child| child.0.is_some())
        .count();
    assert_eq!(coverage.discarded_child_bytes, prefixes);
    assert_eq!(
        coverage.discarded_leaf_word_bytes,
        case.nodes.iter().filter(|node| node.word.is_some()).count() * 4
    );
    assert_eq!(
        coverage.maximum_wire_depth,
        case.nodes.iter().map(|node| node.depth).max().unwrap_or(0)
    );
}

#[test]
fn three_complete_native_carriers_retain_only_the_actual_cpt_forest() {
    let mut present = 0;
    for case in native_fixture::cases().into_iter().take(3) {
        let source = FlowCarrier::from_bytes(&case.raw, native_fixture::limits()).unwrap();
        assert_eq!(
            source.latest(400).unwrap().is_some(),
            case.records.iter().any(|record| record.0 == 400)
        );
        let Some(record) = source.latest(400).unwrap() else {
            continue;
        };
        let configured = options(&fixture::cases()[0]);
        let parsed = source.record_400_trees(&record.id, configured).unwrap();
        assert_eq!(parsed.nodes().len(), 4440, "{}", case.name);
        assert_eq!(parsed.bounds().grid_count(), 792);
        assert_eq!(parsed.source().as_bytes(), case.raw);
        present += 1;
    }
    assert_eq!(present, 1);
}

#[test]
fn all_published_tree_types_are_send_sync() {
    assert_traits::<FlowTreeCompression>();
    assert_traits::<FlowTreeBounds>();
    assert_traits::<FlowTreeOptions>();
    assert_traits::<FlowTreeChild>();
    assert_traits::<FlowTreeNode>();
    assert_traits::<FlowTreeRoot>();
    assert_traits::<FlowBlockTrees>();
    assert_traits::<FlowTreeCheckMode>();
    assert_traits::<FlowTreeCoverage>();
    assert_traits::<FlowTreeDiagnostic>();
    assert_traits::<FlowTreeFieldEdit>();
    assert_traits::<FlowTreeEdit>();
}

fn assert_traits<T: Send + Sync>() {}
