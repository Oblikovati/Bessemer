use super::pair_reference::Cursor;
use bessemer::simulation::*;

pub const INDEX_LIMITS: SimulationElfIndexLimits = SimulationElfIndexLimits {
    maximum_file_bytes: 2_000_000,
    maximum_visited_records: 1000,
};
pub const MAP_LIMITS: SimulationElfMappingLimits = SimulationElfMappingLimits {
    maximum_source_identifiers: 1000,
    maximum_destination_records: 1000,
};

pub fn remaining_owned(cursor: &mut Cursor<'_>) {
    assert_eq!(cursor.word(), 6);
    for _ in 0..6 {
        mapping_case(cursor);
    }
    header_cases(cursor);
    assert_eq!(cursor.word(), 5);
    for _ in 0..5 {
        store_case(cursor);
    }
}

fn mapping_case(cursor: &mut Cursor<'_>) {
    let raw = cursor.span();
    let context = SimulationElfIndexContext::from_bytes(raw, INDEX_LIMITS).unwrap();
    let selector = SimulationElfSelector::new(cursor.word() as usize).unwrap();
    let source_map: Vec<_> = (0..cursor.word()).map(|_| cursor.word()).collect();
    let destination_count = cursor.word() as usize;
    let directory = context.directory(selector).unwrap();
    check_directory(&directory, cursor.span());
    let mapping =
        SimulationElfMappingContext::new(&source_map, destination_count, MAP_LIMITS).unwrap();
    assert_eq!(mapping.destination_count(), destination_count);
    let result = directory.map_records(&mapping).unwrap();
    check_mapping(&directory, &result, cursor.span());
    assert_eq!(context.to_bytes(), raw);
}

pub fn check_directory(directory: &SimulationElfDirectory, expected: &[u8]) {
    let mut cursor = Cursor(expected);
    let header = directory.context().header_words();
    for word in header {
        assert_eq!(word, cursor.word());
    }
    for word in [
        1220,
        directory.selector().one_based() as u32,
        header[0],
        header[2],
        header[5],
        directory.pair_stream_offset() as u32,
        directory.first_record() as u32,
        directory.declared_count(),
    ] {
        assert_eq!(word, cursor.word());
    }
    assert_eq!(
        directory.completion() == SimulationElfDirectoryCompletion::StoppedOnNegative,
        cursor.word() != 0
    );
    assert_eq!(
        directory
            .cursor_stream_offset()
            .map(|offset| offset as u32)
            .unwrap_or(u32::MAX),
        cursor.word()
    );
    assert_eq!(directory.records().len(), cursor.word() as usize);
    for visited in directory.records() {
        check_visited(directory, visited, &mut cursor);
    }
    assert!(cursor.0.is_empty());
    assert!(!directory.coverage().whole_populated_image_checked);
    assert_eq!(
        directory.coverage().visited_bytes,
        directory.records().len() * 244
    );
    assert_eq!(
        directory.coverage().mode,
        SimulationElfDirectoryCheckMode::ExactSelectedTraversal
    );
}

fn check_visited(
    directory: &SimulationElfDirectory,
    visited: &SimulationElfVisitedRecord,
    cursor: &mut Cursor<'_>,
) {
    assert_eq!(visited.id.stream_offset(), cursor.word() as usize);
    assert_eq!(visited.record.identifier() as u32, cursor.word());
    assert_eq!(action(visited.record.getter_action()), cursor.word());
    assert_eq!(visited.record.to_bytes(), cursor.span());
    check_record(&visited.record, cursor.span());
    assert_eq!(
        directory.context().record(&visited.id).unwrap(),
        visited.record
    );
}

pub fn check_record(record: &SimulationElfScalarRecord, expected: &[u8]) {
    let mut cursor = Cursor(expected);
    assert_eq!(record.identifier() as u32, cursor.word());
    assert_eq!(action(record.getter_action()), cursor.word());
    assert_eq!(cursor.word(), 10);
    let mut flat = Vec::new();
    for group in record.groups() {
        for pattern in group {
            assert_eq!(pattern, cursor.word());
            flat.push(pattern);
        }
    }
    assert_eq!(record.patterns().as_slice(), flat);
    assert!(cursor.0.is_empty());
    assert_eq!(
        SimulationElfScalarRecord::from_bytes(&record.to_bytes()).unwrap(),
        *record
    );
}

fn action(action: SimulationElfGetterAction) -> u32 {
    match action {
        SimulationElfGetterAction::Stop => 0,
        SimulationElfGetterAction::Skip => 1,
        SimulationElfGetterAction::RequiresExternalMapping => 2,
    }
}

fn check_mapping(
    directory: &SimulationElfDirectory,
    result: &SimulationElfMappingResult,
    expected: &[u8],
) {
    let mut cursor = Cursor(expected);
    assert_eq!(result.parent_revision, *directory.context().revision());
    assert_eq!(result.selector, directory.selector());
    assert_eq!(result.slots.len(), cursor.word() as usize);
    for slot in result.slots.iter() {
        assert_eq!(slot.is_some(), cursor.word() != 0);
        if let Some(slot) = slot {
            assert_eq!(slot.record.to_bytes(), cursor.span());
            assert_eq!(
                directory.context().record(&slot.source).unwrap(),
                slot.record
            );
        }
    }
    check_writes(directory, result, &mut cursor);
    assert_eq!(result.unassigned_indices.len(), cursor.word() as usize);
    for index in result.unassigned_indices.iter() {
        assert_eq!(*index, cursor.word() as usize);
    }
    assert!(cursor.0.is_empty());
    assert_eq!(result.copied_pattern_error_bits, 0);
    assert_eq!(
        result.coverage.mode,
        SimulationElfProjectionCheckMode::ExactSelectedRecordCopies
    );
    assert_eq!(result.coverage.selected_source, directory.coverage());
    assert_eq!(result.coverage.copied_records, result.writes.len());
    assert_eq!(result.coverage.copied_bytes, result.writes.len() * 244);
    assert_eq!(result.coverage.destination_slots, result.slots.len());
    assert_eq!(
        result.coverage.assigned_slots,
        result.slots.iter().filter(|slot| slot.is_some()).count()
    );
}

fn check_writes(
    directory: &SimulationElfDirectory,
    result: &SimulationElfMappingResult,
    cursor: &mut Cursor<'_>,
) {
    assert_eq!(result.writes.len(), cursor.word() as usize);
    let mut previous = vec![None; result.slots.len()];
    for write in result.writes.iter() {
        assert_eq!(write.identifier as u32, cursor.word());
        assert_eq!(write.destination_index, cursor.word() as usize);
        assert_eq!(write.overwritten_source, previous[write.destination_index]);
        previous[write.destination_index] = Some(write.source.clone());
        assert_eq!(
            directory
                .context()
                .record(&write.source)
                .unwrap()
                .identifier(),
            write.identifier
        );
    }
    for (slot, parent) in result.slots.iter().zip(previous) {
        assert_eq!(slot.as_ref().map(|record| record.source.clone()), parent);
    }
    assert_eq!(
        result.completion == SimulationElfMappingCompletion::CompletedEmpty,
        result.writes.is_empty()
    );
}

fn header_cases(cursor: &mut Cursor<'_>) {
    let raw = cursor.span();
    let context = SimulationElfIndexContext::from_bytes(raw, INDEX_LIMITS).unwrap();
    assert_eq!(cursor.word(), 100);
    for _ in 0..100 {
        let selector = SimulationElfSelector::new(cursor.word() as usize).unwrap();
        let proposal = context.project_index_update(selector).unwrap();
        assert_eq!(proposal.pair_stream_offset, cursor.word() as usize);
        assert_eq!(proposal.first_record, cursor.word());
        assert_eq!(proposal.count, cursor.word());
        assert_eq!(proposal.next_record_marker, cursor.word());
        assert_eq!(proposal.header_body_bytes(), cursor.span());
        assert_eq!(proposal.parent_revision, *context.revision());
        assert_eq!(
            proposal.coverage.mode,
            SimulationElfProjectionCheckMode::ExactHeaderWriteProposal
        );
        assert!(!proposal.coverage.complete_populated_file_written);
        assert_eq!(
            (
                proposal.coverage.header_body_bytes,
                proposal.coverage.cleared_header_padding_bytes,
                proposal.coverage.cleared_other_directory_pairs,
                proposal.coverage.cleared_index_padding_bytes
            ),
            (1220, 216, 99, 176)
        );
        assert_eq!(proposal.selector, selector);
        assert_eq!(
            proposal.completion,
            SimulationElfIndexProjectionCompletion::HeaderOnlyNoDataRecords
        );
        assert_eq!(
            (
                proposal.write_stream_offset,
                proposal.data_records_written,
                proposal.header_pattern_error_bits
            ),
            (4, 0, 0)
        );
    }
    assert_eq!(context.to_bytes(), raw);
}

fn store_case(cursor: &mut Cursor<'_>) {
    let identifier = cursor.word() as i32;
    let groups: Vec<Vec<u32>> = (0..cursor.word())
        .map(|_| (0..cursor.word()).map(|_| cursor.word()).collect())
        .collect();
    let slices: Vec<&[u32]> = groups.iter().map(Vec::as_slice).collect();
    let record = SimulationElfScalarRecord::from_store_groups(identifier, &slices).unwrap();
    assert_eq!(record.to_bytes(), cursor.span());
    check_record(&record, cursor.span());
}
