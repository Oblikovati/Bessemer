use super::{
    owned,
    pair_reference::Cursor,
    reference::{INDEX_LIMITS, MAP_LIMITS},
};
use bessemer::simulation::*;

fn populated() -> Vec<u8> {
    let raw = owned();
    let mut cursor = Cursor(&raw);
    cursor.take(4);
    for _ in 0..cursor.word() {
        cursor.span();
        cursor.span();
    }
    assert_eq!(cursor.word(), 6);
    cursor.span().to_vec()
}

fn context(raw: &[u8]) -> SimulationElfIndexContext {
    SimulationElfIndexContext::from_bytes(raw, INDEX_LIMITS).unwrap()
}

fn selector() -> SimulationElfSelector {
    SimulationElfSelector::new(1).unwrap()
}

fn patched(raw: &[u8], offset: usize, word: u32) -> Vec<u8> {
    let mut result = raw.to_vec();
    result[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
    result
}

#[test]
fn indexed_context_guards_transfer_header_extents_selectors_and_visit_budgets() {
    let raw = populated();
    for length in [0, 4, 31, 1223] {
        assert!(SimulationElfIndexContext::from_bytes(&raw[..length], INDEX_LIMITS).is_err());
    }
    for (offset, word) in [(0, 2), (8, 240), (12, 99)] {
        assert!(
            SimulationElfIndexContext::from_bytes(&patched(&raw, offset, word), INDEX_LIMITS)
                .is_err()
        );
    }
    for value in [0, 101, usize::MAX] {
        assert!(SimulationElfSelector::new(value).is_err());
    }
    let limits = SimulationElfIndexLimits {
        maximum_file_bytes: raw.len() - 1,
        ..INDEX_LIMITS
    };
    assert!(SimulationElfIndexContext::from_bytes(&raw, limits).is_err());
    let limits = SimulationElfIndexLimits {
        maximum_visited_records: 2,
        ..INDEX_LIMITS
    };
    let bounded = SimulationElfIndexContext::from_bytes(&raw, limits).unwrap();
    assert!(bounded.directory(selector()).is_err());
}

#[test]
fn selected_addresses_counts_and_short_records_fail_without_claiming_whole_file_acceptance() {
    let raw = populated();
    for (offset, word) in [(252, u32::MAX), (248, 0), (248, 1), (248, i32::MAX as u32)] {
        let source = context(&patched(&raw, offset, word));
        assert!(source.directory(selector()).is_err());
    }
    let short = patched(&raw[..1224 + 243], 252, 1);
    assert!(context(&short).directory(selector()).is_err());
    let huge = patched(&raw, 252, i32::MAX as u32);
    let directory = context(&huge).directory(selector()).unwrap();
    assert_eq!(directory.records().len(), 3);
    assert_eq!(
        directory.completion(),
        SimulationElfDirectoryCompletion::StoppedOnNegative
    );
    assert!(!directory.coverage().whole_populated_image_checked);
}

#[test]
fn empty_pairs_ignore_the_first_word_and_opaque_unvisited_tails_remain_exact() {
    let raw = populated();
    let empty = patched(&patched(&raw, 248, u32::MAX), 252, 0);
    let directory = context(&empty).directory(selector()).unwrap();
    assert_eq!(directory.first_record(), -1);
    assert_eq!(
        directory.completion(),
        SimulationElfDirectoryCompletion::Empty
    );
    assert_eq!(directory.cursor_stream_offset(), None);
    let mut extended = raw;
    extended.extend_from_slice(b"opaque unvisited tail");
    let source = context(&extended);
    assert_eq!(source.directory(selector()).unwrap().records().len(), 3);
    assert_eq!(source.to_bytes(), extended);
}

#[test]
fn mapping_requires_explicit_valid_context_and_zero_skips_complete_empty() {
    let source = context(&populated());
    let directory = source.directory(selector()).unwrap();
    let missing = SimulationElfMappingContext::new(&[], 3, MAP_LIMITS).unwrap();
    assert!(directory.map_records(&missing).is_err());
    assert!(SimulationElfMappingContext::new(&[4], 3, MAP_LIMITS).is_err());
    let limits = SimulationElfMappingLimits {
        maximum_source_identifiers: 1,
        maximum_destination_records: 1,
    };
    assert!(SimulationElfMappingContext::new(&[0, 0], 1, limits).is_err());
    assert!(SimulationElfMappingContext::new(&[], 2, limits).is_err());
    let skipped = SimulationElfMappingContext::new(&[0; 7], 0, MAP_LIMITS).unwrap();
    let mapped = directory.map_records(&skipped).unwrap();
    assert_eq!(
        mapped.completion,
        SimulationElfMappingCompletion::CompletedEmpty
    );
    assert!(
        mapped.slots.is_empty() && mapped.writes.is_empty() && mapped.unassigned_indices.is_empty()
    );
}

#[test]
fn impossible_destination_storage_returns_a_typed_failure_before_any_copy() {
    let directory = context(&populated()).directory(selector()).unwrap();
    let limits = SimulationElfMappingLimits {
        maximum_source_identifiers: 0,
        maximum_destination_records: usize::MAX,
    };
    let mapping = SimulationElfMappingContext::new(&[], usize::MAX, limits).unwrap();
    let error = directory.map_records(&mapping).unwrap_err();
    assert!(error.entity.contains("allocation"));
    assert!(error.cause.contains("addressable"));
}

#[test]
fn header_proposals_guard_marker_math_and_preserve_signed_watermark_rules() {
    let raw = populated();
    for (marker, count) in [(5, 1), (i32::MAX as u32, 1), (6, u32::MAX)] {
        let changed = patched(&patched(&raw, 4, marker), 24, count);
        assert!(context(&changed).project_index_update(selector()).is_err());
    }
    for (watermark, expected) in [(u32::MAX, 1_u32), (200, 200), (1, 1)] {
        let changed = patched(&raw, 20, watermark);
        let proposal = context(&changed).project_index_update(selector()).unwrap();
        assert_eq!(
            &proposal.header_body_bytes()[16..20],
            &expected.to_le_bytes()
        );
        assert_eq!(proposal.data_records_written, 0);
        assert_eq!(
            proposal.completion,
            SimulationElfIndexProjectionCompletion::HeaderOnlyNoDataRecords
        );
        assert!(SimulationElfInitialArchive::from_bytes(proposal.header_body_bytes()).is_err());
    }
}

#[test]
fn scalar_records_and_store_groups_reject_wrong_shapes_and_preserve_omitted_positive_zero() {
    for length in [0, 243, 245] {
        assert!(SimulationElfScalarRecord::from_bytes(&vec![0; length]).is_err());
    }
    let empty_groups: [&[u32]; 11] = [&[]; 11];
    assert!(SimulationElfScalarRecord::from_store_groups(7, &empty_groups).is_err());
    assert!(SimulationElfScalarRecord::from_store_groups(7, &[&[0; 7]]).is_err());
    let record = SimulationElfScalarRecord::from_store_groups(7, &[&[], &[0x80000000]]).unwrap();
    assert_eq!(record.patterns()[6], 0x80000000);
    for (index, pattern) in record.patterns().iter().enumerate() {
        if index != 6 {
            assert_eq!(*pattern, 0);
        }
    }
}

#[test]
fn foreign_record_ids_source_and_mapping_mutation_cannot_change_published_copies() {
    let mut raw = populated();
    let source = context(&raw);
    let foreign = context(&raw);
    let directory = source.directory(selector()).unwrap();
    assert!(foreign.record(&directory.records()[0].id).is_err());
    let mut targets = vec![0, 0, 0, 0, 0, 0, 2];
    let map = SimulationElfMappingContext::new(&targets, 3, MAP_LIMITS).unwrap();
    let result = directory.map_records(&map).unwrap();
    let expected = result.slots[1].as_ref().unwrap().record.to_bytes();
    raw.fill(0);
    targets.fill(0);
    drop(directory);
    drop(source);
    drop(foreign);
    drop(map);
    assert_eq!(
        result.slots[1].as_ref().unwrap().record.to_bytes(),
        expected
    );
    assert_eq!(result.slots[1].as_ref().unwrap().record.identifier(), 7);
}

#[test]
fn indexed_projection_types_are_send_sync_and_parallel_copy_outputs_are_deterministic() {
    fn check<T: Send + Sync>() {}
    check::<SimulationElfGetterAction>();
    check::<SimulationElfScalarRecord>();
    check::<SimulationElfIndexLimits>();
    check::<SimulationElfSelector>();
    check::<SimulationElfIndexRevision>();
    check::<SimulationElfRecordId>();
    check::<SimulationElfIndexContext>();
    check::<SimulationElfDirectoryCompletion>();
    check::<SimulationElfDirectoryCheckMode>();
    check::<SimulationElfDirectoryCoverage>();
    check::<SimulationElfVisitedRecord>();
    check::<SimulationElfDirectory>();
    check::<SimulationElfIndexProjectionCompletion>();
    check::<SimulationElfIndexProjection>();
    check::<SimulationElfMappingLimits>();
    check::<SimulationElfMappingContext>();
    check::<SimulationElfMappedRecord>();
    check::<SimulationElfMappingWrite>();
    check::<SimulationElfMappingCompletion>();
    check::<SimulationElfMappingResult>();
    check::<SimulationElfProjectionCheckMode>();
    check::<SimulationElfHeaderProjectionCoverage>();
    check::<SimulationElfMappingCoverage>();
    let directory = context(&populated()).directory(selector()).unwrap();
    let mapping = SimulationElfMappingContext::new(&[0, 0, 0, 0, 0, 0, 2], 3, MAP_LIMITS).unwrap();
    let expected = directory.map_records(&mapping).unwrap();
    std::thread::scope(|scope| {
        let tasks: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    for _ in 0..32 {
                        let actual = directory.map_records(&mapping).unwrap();
                        assert_eq!(actual.slots, expected.slots);
                        assert_eq!(actual.writes, expected.writes);
                    }
                })
            })
            .collect();
        for task in tasks {
            task.join().unwrap();
        }
    });
}
