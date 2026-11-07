use super::*;
#[test]
fn every_exact_input_and_aggregate_limit_passes_and_each_used_one_below_fails() {
    for row in cases() {
        let expected = component(&row).coverage().counts;
        let exact = exact_options(row.raw.len(), expected);
        let parsed = bounded(&row, exact).unwrap();
        assert_eq!(parsed.options(), exact);
        assert_eq!(parsed.coverage().counts, expected);
        for below in one_below(exact) {
            let error = bounded(&row, below).unwrap_err();
            assert_eq!(error.operation, "Flow logical checkpoint components");
            assert_eq!(error.class, bessemer::FailureClass::Invalid);
            assert!(!error.entity.is_empty() && !error.cause.is_empty());
        }
    }
}
fn exact_options(bytes: usize, counts: FlowCheckpointCounts) -> FlowCheckpointOptions {
    FlowCheckpointOptions {
        maximum_component_bytes: bytes,
        maximum_row_positions: counts.row_positions,
        maximum_indices: counts.indices,
        maximum_slots: counts.slots,
        maximum_containers: counts.containers,
        maximum_wide_patterns: counts.wide_patterns,
        maximum_chunks: counts.chunks,
        maximum_encoded_bytes: counts.encoded_bytes,
    }
}
fn one_below(exact: FlowCheckpointOptions) -> Vec<FlowCheckpointOptions> {
    let totals = [
        exact.maximum_component_bytes,
        exact.maximum_row_positions,
        exact.maximum_indices,
        exact.maximum_slots,
        exact.maximum_containers,
        exact.maximum_wide_patterns,
        exact.maximum_chunks,
        exact.maximum_encoded_bytes,
    ];
    totals
        .into_iter()
        .enumerate()
        .filter(|(_, count)| *count > 0)
        .map(|(index, count)| lowered(exact, index, count - 1))
        .collect()
}
fn lowered(mut exact: FlowCheckpointOptions, index: usize, value: usize) -> FlowCheckpointOptions {
    match index {
        0 => exact.maximum_component_bytes = value,
        1 => exact.maximum_row_positions = value,
        2 => exact.maximum_indices = value,
        3 => exact.maximum_slots = value,
        4 => exact.maximum_containers = value,
        5 => exact.maximum_wide_patterns = value,
        6 => exact.maximum_chunks = value,
        7 => exact.maximum_encoded_bytes = value,
        _ => panic!("limit index {index}; expected 0..8"),
    }
    exact
}
fn bounded(
    row: &ComponentCase,
    options: FlowCheckpointOptions,
) -> bessemer::Result<FlowCheckpointComponent> {
    FlowCheckpointComponent::from_bytes(
        &row.raw,
        FlowCheckpointContext::LogicalVersionSix,
        kind(row.kind),
        options,
    )
}
#[test]
fn all_small_prefixes_extensions_negative_dimensions_and_hostile_counts_reject_without_partial_results()
 {
    let mut prefixes = 0;
    for row in cases() {
        prefixes += rejected_prefixes(&row);
        let mut extended = row.raw.clone();
        extended.push(0);
        rejected(&extended, kind(row.kind));
    }
    assert_eq!(prefixes, 130257);
    hostile_headers();
}
fn rejected_prefixes(row: &ComponentCase) -> usize {
    if row.raw.len() > 128 {
        return 0;
    }
    for end in 0..row.raw.len() {
        rejected(&row.raw[..end], kind(row.kind));
    }
    row.raw.len()
}

fn rejected(raw: &[u8], kind: FlowCheckpointKind) {
    let error = FlowCheckpointComponent::from_bytes(
        raw,
        FlowCheckpointContext::LogicalVersionSix,
        kind,
        options(),
    )
    .unwrap_err();
    assert_eq!(error.operation, "Flow logical checkpoint components");
    assert!(matches!(
        error.class,
        bessemer::FailureClass::Invalid | bessemer::FailureClass::Unsupported
    ));
    assert!(!error.entity.is_empty() && !error.cause.is_empty());
}
fn hostile_headers() {
    for count in [i32::MIN, -1, i32::MAX] {
        let mut grid = vec![0; 24];
        grid[..4].copy_from_slice(&count.to_le_bytes());
        rejected(&grid, FlowCheckpointKind::Graph2);
        rejected(&grid, FlowCheckpointKind::Auxiliary);
        let mut trailing = vec![0; 12];
        trailing[..4].copy_from_slice(&count.to_le_bytes());
        rejected(&trailing, FlowCheckpointKind::TrailingContainers);
        let mut chunks = vec![0; 16];
        chunks[4..8].copy_from_slice(&count.to_le_bytes());
        rejected(&chunks, FlowCheckpointKind::ChunkBuffer);
    }
    rejected(&[0; 16], FlowCheckpointKind::ChunkBuffer);
}
