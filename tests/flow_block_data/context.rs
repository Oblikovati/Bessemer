use super::*;
#[path = "../flow_carriers/fixture.rs"]
mod native_fixture;

fn grid_from(source: &FlowCarrier) -> FlowBaseGrid {
    let id = source.latest(101).unwrap().unwrap().id;
    source
        .record_101_grid(
            &id,
            FlowGridOptions {
                compression: FlowGridCompression::Raw,
                maximum_decoded_bytes: 1_000_000,
                maximum_boundary_words: 100_000,
            },
        )
        .unwrap()
}

fn trees_from(source: &FlowCarrier, compressed: bool) -> FlowBlockTrees {
    let id = source.latest(400).unwrap().unwrap().id;
    source
        .record_400_trees(
            &id,
            FlowTreeOptions {
                compression: if compressed {
                    FlowTreeCompression::Zlib
                } else {
                    FlowTreeCompression::Raw
                },
                legacy_bounds: None,
                maximum_decoded_bytes: 100_000,
                maximum_roots: 10_000,
                maximum_nodes: 10_000,
                maximum_depth: 256,
            },
        )
        .unwrap()
}

fn synthetic_context(
    dimensions: [i32; 3],
    version: i32,
    bounds: [[i32; 2]; 3],
    body: &[u8],
) -> (FlowBaseGrid, FlowBlockTrees) {
    let mut grid: Vec<u8> = dimensions.into_iter().flat_map(i32::to_le_bytes).collect();
    grid.resize(
        12 + dimensions
            .into_iter()
            .map(|n| n as usize + 1)
            .sum::<usize>()
            * 8,
        0,
    );
    let mut tree = version.to_le_bytes().to_vec();
    tree.extend(bounds.into_iter().flatten().flat_map(i32::to_le_bytes));
    tree.extend_from_slice(body);
    let mut wire = 7_u16.to_le_bytes().to_vec();
    for (id, raw) in [(101_u32, grid), (400, tree)] {
        wire.extend_from_slice(&id.to_le_bytes());
        wire.extend_from_slice(&(raw.len() as u64).to_le_bytes());
        wire.extend_from_slice(&raw);
    }
    let source = FlowCarrier::from_bytes(&wire, native_fixture::limits()).unwrap();
    (grid_from(&source), trees_from(&source, false))
}

#[test]
fn derived_fresh_count_requires_complete_full_grid_and_retains_both_checkpoints() {
    let (grid, trees) = synthetic_context([1; 3], 3, [[0, 0]; 3], &[1, 0]);
    let result =
        derive_fresh_flow_leaf_count(&grid, &trees, FlowFreshContextAssumption::AssumeFresh)
            .unwrap();
    check_derived_checkpoints(&result, &grid, &trees);
    for (version, bounds, body) in [
        (3, [[0, 0]; 3], vec![0]),
        (3, [[0, 1], [0, 0], [0, 0]], vec![1, 0, 1, 0]),
        (4, [[0, 0]; 3], vec![0]),
    ] {
        let (grid, trees) = synthetic_context([1; 3], version, bounds, &body);
        assert!(
            derive_fresh_flow_leaf_count(&grid, &trees, FlowFreshContextAssumption::AssumeFresh)
                .is_err()
        );
    }
}

#[test]
fn zero_grid_derives_completed_empty_count_but_absent_roots_do_not() {
    let (grid, trees) = synthetic_context([0, 1, 1], 3, [[0, -1], [0, 0], [0, 0]], &[]);
    assert_eq!(
        derive_fresh_flow_leaf_count(&grid, &trees, FlowFreshContextAssumption::AssumeFresh)
            .unwrap()
            .count()
            .value(),
        0
    );
    let (grid, trees) = synthetic_context([2, 1, 1], 3, [[0, 1], [0, 0], [0, 0]], &[1, 0, 0]);
    assert!(
        derive_fresh_flow_leaf_count(&grid, &trees, FlowFreshContextAssumption::AssumeFresh)
            .is_err()
    );
}

#[test]
fn complete_native_carriers_use_only_actual_cpt_record_with_explicit_fresh_assumption() {
    let mut present = 0;
    for case in native_fixture::cases().into_iter().take(3) {
        let source = FlowCarrier::from_bytes(&case.raw, native_fixture::limits()).unwrap();
        assert_eq!(
            source.latest(406).unwrap().is_some(),
            case.records.iter().any(|record| record.0 == 406)
        );
        let Some(record) = source.latest(406).unwrap() else {
            continue;
        };
        let blocks = native_blocks(&source, &record.id, &case.name);
        assert_eq!(blocks.coverage().checked_entries, 5952);
        assert_eq!(blocks.source().as_bytes(), case.raw);
        assert!(blocks.entries_equal_global_total());
        present += 1;
    }
    assert_eq!(present, 1);
}

fn check_derived_checkpoints(
    result: &FlowDerivedLeafCount,
    grid: &FlowBaseGrid,
    trees: &FlowBlockTrees,
) {
    assert_eq!(result.count().value(), 1);
    assert_eq!(result.grid().record(), grid.record());
    assert_eq!(result.trees().record(), trees.record());
    assert_eq!(result.assumption(), FlowFreshContextAssumption::AssumeFresh);
    assert!(!result.native_context_executed());
    assert!(!result.block_to_tree_order_assigned());
}

fn native_blocks(source: &FlowCarrier, id: &FlowRecordId, name: &str) -> FlowLegacyBlockData {
    let context = derive_fresh_flow_leaf_count(
        &grid_from(source),
        &trees_from(source, true),
        FlowFreshContextAssumption::AssumeFresh,
    )
    .unwrap();
    assert_eq!(context.count().value(), 3984, "{}", name);
    let mut configured = options(&fixture::cases()[0]);
    configured.leaf_count = context.count();
    source.record_406_blocks(id, configured).unwrap()
}
