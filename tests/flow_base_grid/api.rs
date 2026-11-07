use bessemer::flow::*;
#[path = "bounds.rs"]
mod bounds;
#[path = "../flow_carriers/fixture.rs"]
mod native_fixture;

pub fn options(compressed: bool) -> FlowGridOptions {
    FlowGridOptions {
        compression: if compressed {
            FlowGridCompression::Zlib
        } else {
            FlowGridCompression::Raw
        },
        maximum_decoded_bytes: 100_000,
        maximum_boundary_words: 10_000,
    }
}

pub fn carrier(payload: &[u8], maximum: usize) -> FlowCarrier {
    let mut wire = 7_u16.to_le_bytes().to_vec();
    for (identifier, bytes) in [
        (101_u32, &b"unselected"[..]),
        (101, payload),
        (999, &b"opaque\0"[..]),
    ] {
        wire.extend_from_slice(&identifier.to_le_bytes());
        wire.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        wire.extend_from_slice(bytes);
    }
    FlowCarrier::from_bytes(
        &wire,
        FlowLimits {
            maximum_wire_bytes: maximum + 100,
            maximum_payload_bytes: maximum,
            maximum_records: 3,
        },
    )
    .unwrap()
}

pub fn parse(payload: &[u8], compressed: bool) -> FlowBaseGrid {
    let source = carrier(payload, 100_000);
    source
        .record_101_grid(
            &source.latest(101).unwrap().unwrap().id,
            options(compressed),
        )
        .unwrap()
}

pub fn check_case(compressed: bool, stored: &[u8], decoded: &[u8], dimensions: [i32; 3]) {
    let grid = parse(stored, compressed);
    assert_eq!(grid.dimensions(), dimensions);
    assert_eq!(grid.decoded_bytes(), decoded);
    assert_eq!(
        grid.default_tree_bounds().pairs(),
        &dimensions.map(|n| [0, n - 1])
    );
    assert_eq!(
        grid.root_grid_count(),
        dimensions.into_iter().map(|n| n as u128).product()
    );
    check_coverage(&grid, stored.len(), decoded.len());
    check_arrays(&grid, decoded);
    bounds::check_prefixes_and_limits(stored, compressed, decoded.len());
}

fn check_coverage(grid: &FlowBaseGrid, stored: usize, decoded: usize) {
    assert_eq!(grid.options().maximum_boundary_words, 10_000);
    assert_eq!(grid.record().ordinal(), 1);
    assert_eq!(grid.coverage().stored_bytes, stored);
    assert_eq!(grid.coverage().decoded_bytes, decoded);
    assert_eq!(grid.coverage().boundary_words, (decoded - 12) / 8);
    assert_eq!(grid.coverage().achieved_error, 0);
    let expected_mode = if grid.options().compression == FlowGridCompression::Raw {
        FlowGridCheckMode::ExactRawBoundaryArrays
    } else {
        FlowGridCheckMode::ExactZlibBoundaryArrays
    };
    assert_eq!(grid.coverage().mode, expected_mode);
    assert_eq!(grid.diagnostics().len(), 5);
    assert!(grid.boundary(3).is_none());
}

fn check_arrays(grid: &FlowBaseGrid, decoded: &[u8]) {
    let mut start = 12;
    for array in 0..3 {
        let boundary = grid.boundary(array).unwrap();
        let count = grid.dimensions()[array] as usize + 1;
        assert_eq!(boundary.word_count(), count);
        assert_eq!(boundary.payload_interval(), (start, start + count * 8));
        assert_eq!(boundary.as_bytes(), &decoded[start..start + count * 8]);
        assert_eq!(boundary.raw_bits(count), None);
        assert_eq!(boundary.raw_bits(usize::MAX), None);
        assert_eq!(boundary.value(count), None);
        for (index, bytes) in boundary.as_bytes().as_chunks::<8>().0.iter().enumerate() {
            let bits = u64::from_le_bytes(*bytes);
            assert_eq!(boundary.raw_bits(index), Some(bits));
            assert_eq!(boundary.value(index).unwrap().to_bits(), bits);
            check_edit(grid, array, index, bits);
        }
        start += count * 8;
    }
}

fn check_edit(grid: &FlowBaseGrid, array: usize, index: usize, bits: u64) {
    let noop = grid.replace_boundary_word(array, index, bits).unwrap();
    assert_eq!(noop.result().record(), grid.record());
    assert_eq!(noop.result().source().as_bytes(), grid.source().as_bytes());
    check_history(noop.carrier_edit(), false);
    let edit = grid
        .replace_boundary_word(array, index, bits ^ 0x8000000000000001)
        .unwrap();
    let offset = grid.boundary(array).unwrap().payload_interval().0 + index * 8;
    let mut expected = grid.decoded_bytes().to_vec();
    expected[offset..offset + 8].copy_from_slice(&(bits ^ 0x8000000000000001).to_le_bytes());
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().record(), grid.record());
    assert_eq!(edit.source().source().as_bytes(), grid.source().as_bytes());
    check_history(edit.carrier_edit(), true);
    check_reverse(grid, edit.result(), array, index, bits);
}

fn check_reverse(
    source: &FlowBaseGrid,
    changed: &FlowBaseGrid,
    array: usize,
    index: usize,
    bits: u64,
) {
    let reverse = changed.replace_boundary_word(array, index, bits).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), source.decoded_bytes());
    if source.options().compression == FlowGridCompression::Raw {
        assert_eq!(
            reverse.result().source().as_bytes(),
            source.source().as_bytes()
        );
    }
}

fn check_history(edit: &FlowCarrierEdit, changed: bool) {
    assert_eq!(edit.history().len(), 3);
    for (ordinal, (before, after)) in edit
        .source()
        .records()
        .zip(edit.result().records())
        .enumerate()
    {
        let relation = &edit.history()[ordinal];
        assert_eq!(relation.source, before.id);
        assert_eq!(relation.result, after.id);
        let expected = if ordinal == 1 && changed {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        };
        assert_eq!(relation.relation, expected);
        check_unselected(ordinal, &before, &after);
    }
}

fn check_unselected(ordinal: usize, before: &FlowRecord<'_>, after: &FlowRecord<'_>) {
    if ordinal == 1 {
        return;
    }
    assert_eq!(before.payload, after.payload);
    assert_eq!(before.identifier, after.identifier);
}

#[test]
fn all_public_grid_types_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FlowBaseGrid>();
    assert_send_sync::<FlowGridBoundary<'static>>();
    assert_send_sync::<FlowGridOptions>();
    assert_send_sync::<FlowGridCompression>();
    assert_send_sync::<FlowGridCoverage>();
    assert_send_sync::<FlowGridCheckMode>();
    assert_send_sync::<FlowGridDiagnostic>();
    assert_send_sync::<FlowGridEdit>();
}

#[test]
fn complete_native_carriers_preserve_grid_absences_and_actual_tree_bounds() {
    let mut present = 0;
    for case in native_fixture::cases().into_iter().take(3) {
        let source = FlowCarrier::from_bytes(&case.raw, native_fixture::limits()).unwrap();
        assert_eq!(
            source.latest(101).unwrap().is_some(),
            case.records.iter().any(|r| r.0 == 101)
        );
        let Some(record) = source.latest(101).unwrap() else {
            continue;
        };
        let grid = source.record_101_grid(&record.id, options(false)).unwrap();
        assert_eq!(grid.dimensions(), [22, 6, 6], "{}", case.name);
        assert_eq!(grid.root_grid_count(), 792);
        assert_eq!(grid.source().as_bytes(), case.raw);
        check_native_tree_bounds(&source, &grid);
        present += 1;
    }
    assert_eq!(present, 1);
}

fn check_native_tree_bounds(source: &FlowCarrier, grid: &FlowBaseGrid) {
    let id = source.latest(400).unwrap().unwrap().id;
    let options = FlowTreeOptions {
        compression: FlowTreeCompression::Zlib,
        legacy_bounds: None,
        maximum_decoded_bytes: 100_000,
        maximum_roots: 1_000,
        maximum_nodes: 10_000,
        maximum_depth: 256,
    };
    let trees = source.record_400_trees(&id, options).unwrap();
    assert_eq!(trees.bounds(), grid.default_tree_bounds());
    assert_eq!(trees.roots().unwrap().len() as u128, grid.root_grid_count());
}
