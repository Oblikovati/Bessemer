//! Independent storage arithmetic and exact child-path replay before the projection API.
#[path = "flow_root_indices/api.rs"]
mod api;
#[path = "flow_root_indices/fixture.rs"]
mod fixture;
use bessemer::flow::*;

#[test]
fn all_233_root_projections_replay_exact_leaf_paths_and_storage_arithmetic() {
    for case in fixture::cases() {
        let (grid, trees) = parsed_inputs(&case);
        assert_eq!(case.flags, [true, false, false, false]);
        let context =
            derive_fresh_flow_leaf_count(&grid, &trees, FlowFreshContextAssumption::AssumeFresh)
                .unwrap();
        assert_eq!(context.count().value(), case.leaf_count, "{}", case.name);
        assert_eq!(case.roots.len(), trees.roots().unwrap().len());
        assert_eq!(case.roots.len() as u128, grid.root_grid_count());
        let mut leaves = 0;
        for (ordinal, (expected, root)) in case.roots.iter().zip(trees.roots().unwrap()).enumerate()
        {
            check_root_arithmetic(expected, ordinal, grid.dimensions());
            assert_eq!(expected.indices, root.indices());
            check_leaf_paths(expected, &trees, root.node_index().unwrap());
            leaves += expected.leaves.len();
        }
        assert_eq!(leaves, case.leaf_count as usize);
        api::check_projection(&case, &context);
    }
}

fn parsed_inputs(case: &fixture::RootCase) -> (FlowBaseGrid, FlowBlockTrees) {
    let source = root_carrier(case);
    let grid_id = source.latest(101).unwrap().unwrap().id;
    let grid = source
        .record_101_grid(
            &grid_id,
            FlowGridOptions {
                compression: FlowGridCompression::Raw,
                maximum_decoded_bytes: 100_000,
                maximum_boundary_words: 100_000,
            },
        )
        .unwrap();
    let tree_id = source.latest(400).unwrap().unwrap().id;
    let trees = source
        .record_400_trees(&tree_id, tree_options(case))
        .unwrap();
    (grid, trees)
}

fn tree_options(case: &fixture::RootCase) -> FlowTreeOptions {
    FlowTreeOptions {
        compression: if case.compressed {
            FlowTreeCompression::Zlib
        } else {
            FlowTreeCompression::Raw
        },
        legacy_bounds: case.legacy.map(FlowTreeBounds::from_pairs),
        maximum_decoded_bytes: 100_000,
        maximum_roots: 10_000,
        maximum_nodes: 10_000,
        maximum_depth: case.maximum_depth,
    }
}

fn check_root_arithmetic(root: &fixture::ExpectedRoot, ordinal: usize, dimensions: [i32; 3]) {
    let [first, second, third] = root.indices.map(i128::from);
    let [n0, n1, n2] = dimensions.map(i128::from);
    assert!(first >= 0 && first < n0 && second >= 0 && second < n1 && third >= 0 && third < n2);
    let index = (third * n1 + second) * n0 + first;
    assert_eq!(index, i128::from(root.storage_index));
    assert_eq!(index, ordinal as i128);
    assert_eq!(root.byte_offset as i128, index * 56);
}

fn check_leaf_paths(root: &fixture::ExpectedRoot, trees: &FlowBlockTrees, first_node: usize) {
    let mut visited = std::collections::BTreeSet::new();
    for (ordinal, expected) in root.leaves.iter().enumerate() {
        assert_eq!(expected.ordinal, ordinal as u32);
        let mut index = first_node;
        for child in &expected.path {
            assert!(*child < 8);
            index = trees.nodes()[index].children()[*child as usize].node_index();
        }
        let node = &trees.nodes()[index];
        assert_ne!(node.marker_byte(), 255);
        assert_eq!(node.wire_depth() as usize, expected.path.len());
        assert_eq!(
            node.payload_interval(),
            (expected.start as usize, expected.end as usize)
        );
        assert!(visited.insert(index));
    }
    check_leaf_completeness(root, trees, first_node, &visited);
}

fn check_leaf_completeness(
    root: &fixture::ExpectedRoot,
    trees: &FlowBlockTrees,
    first: usize,
    visited: &std::collections::BTreeSet<usize>,
) {
    let span = trees.nodes()[first].payload_interval();
    let leaves: std::collections::BTreeSet<_> = trees
        .nodes()
        .iter()
        .enumerate()
        .filter(|(_, node)| {
            let part = node.payload_interval();
            node.marker_byte() != 255 && part.0 >= span.0 && part.1 <= span.1
        })
        .map(|(index, _)| index)
        .collect();
    assert_eq!(&leaves, visited);
    assert!(
        root.leaves
            .windows(2)
            .all(|pair| pair[0].path < pair[1].path && pair[0].start < pair[1].start)
    );
}

fn root_carrier(case: &fixture::RootCase) -> FlowCarrier {
    let mut wire = 7_u16.to_le_bytes().to_vec();
    for (id, raw) in [(101_u32, &case.grid), (400, &case.trees)] {
        wire.extend_from_slice(&id.to_le_bytes());
        wire.extend_from_slice(&(raw.len() as u64).to_le_bytes());
        wire.extend_from_slice(raw);
    }
    FlowCarrier::from_bytes(
        &wire,
        FlowLimits {
            maximum_wire_bytes: 100_000,
            maximum_records: 2,
            maximum_payload_bytes: 100_000,
        },
    )
    .unwrap()
}
