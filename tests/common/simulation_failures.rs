use super::reference::{Cursor, owned};
use bessemer::simulation::*;

fn pair() -> (Vec<u8>, Vec<u8>) {
    let packet = owned();
    let mut cursor = Cursor(&packet);
    cursor.take(8);
    (cursor.span().to_vec(), cursor.span().to_vec())
}

#[test]
fn master_rejects_truncation_extensions_wrong_headers_widths_and_directories() {
    let (mas, _) = pair();
    for size in [0, 3, 40, 388, 1036, 1156, 1436, 1540, 28419] {
        assert!(SimulationMasterArchive::from_bytes(&mas[..size]).is_err());
    }
    for (offset, value) in [
        (0, 2_u32),
        (4, 383),
        (392, 8),
        (1156, 0),
        (1156, 71),
        (1160, 1),
    ] {
        let mut changed = mas.clone();
        changed[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        assert!(
            SimulationMasterArchive::from_bytes(&changed).is_err(),
            "offset {offset}, value {value}"
        );
    }
    let mut extra = mas.clone();
    extra.push(0);
    assert!(SimulationMasterArchive::from_bytes(&extra).is_err());
}

#[test]
fn gen_rejects_truncated_blocks_counts_delegated_collections_and_trailer_mismatch() {
    let (mas, raw) = pair();
    let master = SimulationMasterArchive::from_bytes(&mas).unwrap();
    for size in [0, 4, 603, 1003, raw.len() - 1] {
        assert!(SimulationGenArchive::from_bytes(&raw[..size], &master).is_err());
    }
    for (offset, value) in [
        (0, 0_u32),
        (404, 1),
        (416, u32::MAX),
        (420, i32::MAX as u32),
        (raw.len() - 48, 1),
    ] {
        let mut changed = raw.clone();
        changed[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        assert!(
            SimulationGenArchive::from_bytes(&changed, &master).is_err(),
            "offset {offset}, value {value}"
        );
    }
    let mut extra = raw;
    extra.push(0);
    assert!(SimulationGenArchive::from_bytes(&extra, &master).is_err());
}

#[test]
fn typed_boundaries_foreign_contexts_and_accessor_failures_are_isolated() {
    let (mas, raw) = pair();
    let master = SimulationMasterArchive::from_bytes(&mas).unwrap();
    let image = SimulationGenArchive::from_bytes(&raw, &master).unwrap();
    for (row, field) in [(0, 1), (71, 1), (1, 0), (1, 82), (usize::MAX, usize::MAX)] {
        assert!(master.cell_id(row, field).is_err());
    }
    assert!(image.node_id(usize::MAX).is_err());
    assert!(image.element(usize::MAX).is_err());
    assert!(image.block(10).is_err());
    assert!(image.vector(usize::MAX).is_err());
    let other = SimulationGenArchive::from_bytes(&raw, &master).unwrap();
    assert!(
        image
            .edit_node_coordinates(&other.node_id(0).unwrap(), [0; 3])
            .is_err()
    );
    let mut parts = image.parts();
    let mut elements = parts.blocks[6].to_vec();
    elements[23] = 12;
    parts.blocks[6] = elements.into();
    let invalid_accessor = SimulationGenArchive::from_parts(parts, &master).unwrap();
    assert!(invalid_accessor.element(0).is_err());
    assert_eq!(image.to_bytes(), raw);
}

#[test]
fn all_published_simulation_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationMasterArchive>();
    check::<SimulationMasterParts>();
    check::<SimulationMasterRow>();
    check::<SimulationMasterRevision>();
    check::<SimulationMasterCellId>();
    check::<SimulationMasterCoverage>();
    check::<SimulationMasterEdit>();
    check::<SimulationGenArchive>();
    check::<SimulationGenLayout>();
    check::<SimulationGenRevision>();
    check::<SimulationGenParts>();
    check::<SimulationGenCoverage>();
    check::<SimulationGenEdit>();
    check::<SimulationNodeId>();
    check::<SimulationNodeView>();
    check::<SimulationElementView>();
    check::<SimulationStudy>();
    check::<SimulationStudyEdit>();
    check::<SimulationDiagnostic>();
}

#[test]
fn concurrent_edits_are_deterministic_and_source_memory_is_not_retained() {
    let (mut mas, mut raw) = pair();
    let master = SimulationMasterArchive::from_bytes(&mas).unwrap();
    let image = SimulationGenArchive::from_bytes(&raw, &master).unwrap();
    let expected = image.to_bytes();
    mas.fill(0);
    raw.fill(0);
    std::thread::scope(|scope| {
        let threads: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    for _ in 0..32 {
                        let id = image.node_id(0).unwrap();
                        let changed = image
                            .edit_node_coordinates(&id, [0x80000000, 0x7fc12345, 0x3f800000])
                            .unwrap();
                        let reverted = changed
                            .archive
                            .edit_node_coordinates(
                                &changed.archive.node_id(0).unwrap(),
                                image.node(&id).unwrap().coordinate_patterns,
                            )
                            .unwrap();
                        assert_eq!(reverted.archive.to_bytes(), expected);
                    }
                })
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
    });
    assert_eq!(image.to_bytes(), expected);
}
