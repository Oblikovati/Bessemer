//! Independent flat-forest reconstruction preserves discarded bytes and explicit root ordering.
#[path = "flow_block_trees/fixture.rs"]
mod fixture;
use bessemer::flow::*;
use std::io::Read;

enum Task {
    Node(usize, u16),
    Prefix(u8),
    End(usize),
}

#[test]
fn all_81_forests_rebuild_every_wire_byte_without_recursive_test_storage() {
    for case in fixture::cases() {
        check_transport(&case);
        let rebuilt = rebuild(&case);
        assert_eq!(rebuilt, case.decoded, "{}", case.name);
        check_carrier(&case);
        check_forest_balance(&case);
    }
}

fn check_transport(case: &fixture::TreeCase) {
    if !case.compressed {
        assert_eq!(case.stored, case.decoded);
        return;
    }
    let mut decoded = Vec::new();
    flate2::read::ZlibDecoder::new(case.stored.as_slice())
        .take(100_001)
        .read_to_end(&mut decoded)
        .unwrap();
    assert_eq!(decoded, case.decoded);
}

fn rebuild(case: &fixture::TreeCase) -> Vec<u8> {
    let mut output = case.version.to_le_bytes().to_vec();
    assert_eq!(case.bounds_stored, case.version >= 2);
    if case.bounds_stored {
        output.extend(
            case.bounds
                .iter()
                .flatten()
                .flat_map(|value| value.to_le_bytes()),
        );
    } else {
        assert_eq!(case.legacy_bounds, Some(case.bounds));
    }
    if !case.body_present {
        append_inactive(&mut output, case);
        return output;
    }
    assert!(case.version < 4);
    assert_eq!(case.body_presence, None);
    append_roots(&mut output, case);
    output
}

fn append_roots(output: &mut Vec<u8>, case: &fixture::TreeCase) {
    let roots = case.roots.as_ref().unwrap();
    assert_eq!(roots.len() as u128, case.grid_count);
    let mut visited = vec![false; case.nodes.len()];
    for (ordinal, root) in roots.iter().enumerate() {
        assert_eq!(output.len(), root.start);
        check_indices(case.bounds, ordinal, root.indices);
        assert_eq!(root.presence.is_some(), case.version >= 2);
        if let Some(byte) = root.presence {
            output.push(byte);
        }
        if let Some(node) = root.node {
            append_tree(output, case, node, &mut visited);
        } else {
            assert_eq!(root.presence, Some(0));
        }
        assert_eq!(output.len(), root.end);
    }
    assert!(visited.into_iter().all(|value| value));
}

fn check_indices(bounds: [[i32; 2]; 3], ordinal: usize, actual: [i32; 3]) {
    let widths = bounds.map(|[lo, hi]| u128::try_from(i64::from(hi) - i64::from(lo) + 1).unwrap());
    let ordinal = ordinal as u128;
    let steps = [
        ordinal % widths[0],
        (ordinal / widths[0]) % widths[1],
        ordinal / (widths[0] * widths[1]),
    ];
    let expected: [i32; 3] = std::array::from_fn(|index| {
        i32::try_from(i128::from(bounds[index][0]) + steps[index] as i128).unwrap()
    });
    assert_eq!(actual, expected);
}

fn append_tree(output: &mut Vec<u8>, case: &fixture::TreeCase, first: usize, visited: &mut [bool]) {
    let mut pending = vec![Task::Node(first, 0)];
    while let Some(task) = pending.pop() {
        match task {
            Task::Prefix(byte) => output.push(byte),
            Task::End(end) => assert_eq!(output.len(), end),
            Task::Node(index, depth) => {
                append_node(output, case, index, depth, visited, &mut pending)
            }
        }
    }
}

fn append_node(
    output: &mut Vec<u8>,
    case: &fixture::TreeCase,
    index: usize,
    depth: u16,
    visited: &mut [bool],
    pending: &mut Vec<Task>,
) {
    assert!(!visited[index]);
    visited[index] = true;
    let node = &case.nodes[index];
    assert_eq!(output.len(), node.start);
    assert_eq!(node.depth, depth);
    assert!(depth <= case.maximum_depth);
    assert_eq!(node.level, (depth & 255) as u8);
    output.push(node.marker);
    assert_eq!(node.word.is_some(), node.marker != 255 && case.version == 2);
    if node.marker == 255 {
        push_children(pending, node, case.version);
        return;
    }
    assert!(node.children.is_empty());
    if let Some(word) = node.word {
        output.extend(word.to_le_bytes());
    }
    assert_eq!(output.len(), node.end);
}

fn push_children(pending: &mut Vec<Task>, node: &fixture::ExpectedNode, version: i32) {
    assert_eq!(node.children.len(), 8);
    pending.push(Task::End(node.end));
    for (prefix, index) in node.children.iter().rev() {
        assert_eq!(prefix.is_some(), matches!(version, 2 | 3));
        pending.push(Task::Node(*index, node.depth + 1));
        if let Some(byte) = prefix {
            pending.push(Task::Prefix(*byte));
        }
    }
}

fn check_forest_balance(case: &fixture::TreeCase) {
    let widths = case
        .bounds
        .map(|[lo, hi]| (i64::from(hi) - i64::from(lo) + 1).max(0) as u128);
    assert_eq!(case.grid_count, widths.into_iter().product::<u128>());
    let roots = case.roots.as_ref().map_or(0, |roots| {
        roots.iter().filter(|root| root.node.is_some()).count()
    });
    let splits = case.nodes.iter().filter(|node| node.marker == 255).count();
    assert_eq!(case.nodes.len(), roots + 8 * splits);
    assert_eq!(case.nodes.len() - splits, roots + 7 * splits);
    assert!(!case.native_order_proven);
}

fn check_carrier(case: &fixture::TreeCase) {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    wire.extend(400_u32.to_le_bytes());
    wire.extend((case.stored.len() as u64).to_le_bytes());
    wire.extend(&case.stored);
    let limits = FlowLimits {
        maximum_wire_bytes: 100_000,
        maximum_records: 1,
        maximum_payload_bytes: 100_000,
    };
    let carrier = FlowCarrier::from_bytes(&wire, limits).unwrap();
    assert_eq!(carrier.latest(400).unwrap().unwrap().payload, case.stored);
    assert_eq!(carrier.as_bytes(), wire);
}

#[test]
fn native_counts_and_depth_wrap_have_independent_closed_forms() {
    let cases = fixture::cases();
    assert_eq!(cases[0].decoded.len(), 8908);
    assert_eq!(cases[0].stored.len(), 340);
    assert_eq!(cases[0].grid_count, 792);
    assert_eq!(cases[0].nodes.len(), 4440);
    assert_eq!(
        cases[0]
            .nodes
            .iter()
            .filter(|node| node.marker == 255)
            .count(),
        456
    );
    assert!(cases.iter().any(|case| case.grid_count == 1_u128 << 96));
    assert!(cases.iter().any(|case| {
        case.nodes
            .iter()
            .any(|node| node.depth == 256 && node.level == 0)
    }));
}

fn append_inactive(output: &mut Vec<u8>, case: &fixture::TreeCase) {
    assert!(case.version >= 4);
    assert!(case.roots.is_none());
    assert_eq!(case.body_presence, Some(0));
    output.push(0);
}
