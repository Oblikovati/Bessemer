use super::*;

pub(super) fn check_case(case: &SimpleCase) {
    match (read_case(case), &case.expected) {
        (SimpleView::Parallel(actual), ExpectedSimpleTable::Parallel(expected)) => {
            check_parallel(&actual, expected, case)
        }
        (SimpleView::LeafPairs(actual), ExpectedSimpleTable::LeafPairs(expected)) => {
            check_leaf_pairs(&actual, expected, case)
        }
        (SimpleView::Five(actual), ExpectedSimpleTable::FiveWords(expected)) => {
            check_five(&actual, expected, case)
        }
        (SimpleView::Bytes(actual), ExpectedSimpleTable::Bytes(expected)) => {
            check_bytes(&actual, expected, case)
        }
        _ => panic!("mismatched table view for {}", case.frame.name),
    }
}

fn checkpoint(
    source: &FlowCarrier,
    id: &FlowRecordId,
    decoded: &[u8],
    version: i32,
    case: &SimpleCase,
) {
    assert_eq!(decoded, case.frame.decoded);
    assert_eq!(version, case.version);
    assert_eq!(id.ordinal(), 1);
    assert_eq!(
        source.latest(case.frame.identifier).unwrap().unwrap().id,
        *id
    );
    assert_eq!(
        source
            .latest(case.frame.identifier)
            .unwrap()
            .unwrap()
            .payload,
        case.frame.stored
    );
}

fn check_parallel(actual: &FlowParallelTable, expected: &[ParallelGroup; 2], case: &SimpleCase) {
    checkpoint(
        actual.source(),
        actual.record(),
        actual.decoded_bytes(),
        actual.version(),
        case,
    );
    assert_eq!(actual.options().compression, compression(&case.frame));
    assert_eq!(actual.groups().len(), 2);
    assert!(actual.group(2).is_none() && actual.group(usize::MAX).is_none());
    for (group, frozen) in actual.groups().zip(expected) {
        check_group(group, frozen);
        check_group(actual.group(frozen.ordinal as usize).unwrap(), frozen);
    }
    let wide = expected
        .iter()
        .flat_map(|g| &g.arrays)
        .map(|array| array.bits.len())
        .sum();
    check_coverage(actual.coverage(), &case.frame, [0, wide, 0, 0]);
    check_diagnostics(actual.diagnostics(), false);
}

fn check_group(actual: FlowParallelGroup<'_>, expected: &ParallelGroup) {
    assert_eq!(actual.ordinal(), expected.ordinal as usize);
    assert_eq!(actual.declared_count().value(), expected.count);
    assert_eq!(actual.presence_word(), expected.presence);
    assert_eq!(actual.arrays_present(), !expected.arrays.is_empty());
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    check_group_arrays(actual, expected);
}

fn check_wide(actual: FlowTableWideArray<'_>, values: &[u64], start: usize) {
    assert_eq!(actual.payload_interval(), (start, start + values.len() * 8));
    assert_eq!(actual.word_count(), values.len());
    assert_eq!(
        actual.as_bytes(),
        values
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<_>>()
    );
    for (index, &value) in values.iter().enumerate() {
        assert_eq!(actual.raw_bits(index), Some(value));
    }
    assert!(actual.raw_bits(values.len()).is_none() && actual.raw_bits(usize::MAX).is_none());
}

fn check_leaf_pairs(actual: &FlowLeafPairs, expected: &[LeafPair], case: &SimpleCase) {
    checkpoint(
        actual.source(),
        actual.record(),
        actual.decoded_bytes(),
        actual.version(),
        case,
    );
    check_leaf_context(actual, case);
    assert_eq!(actual.rows().len(), expected.len());
    assert!(actual.row(expected.len()).is_none() && actual.row(usize::MAX).is_none());
    assert_eq!(actual.raw_bytes(), &case.frame.decoded[4..]);
    check_leaf_rows(actual, expected);
    check_coverage(
        actual.coverage(),
        &case.frame,
        [expected.len(), expected.len() * 2, 0, 0],
    );
    check_diagnostics(actual.diagnostics(), true);
}

fn check_five(actual: &FlowFiveWordTable, expected: &FiveWords, case: &SimpleCase) {
    checkpoint(
        actual.source(),
        actual.record(),
        actual.decoded_bytes(),
        actual.version(),
        case,
    );
    check_five_metadata(actual, expected, case);
    check_five_rows(actual, expected);
    check_coverage(
        actual.coverage(),
        &case.frame,
        [expected.rows.len(), 0, expected.rows.len() * 5, 0],
    );
    check_diagnostics(actual.diagnostics(), false);
}

fn check_five_row(actual: FlowFiveWordRow<'_>, expected: &[u32; 5], ordinal: usize, start: usize) {
    assert_eq!(actual.ordinal(), ordinal);
    assert_eq!(
        actual.payload_interval(),
        (start + ordinal * 20, start + (ordinal + 1) * 20)
    );
    assert_eq!(
        actual.as_bytes(),
        expected
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<_>>()
    );
    assert_eq!(actual.words(), *expected);
    for (component, &value) in expected.iter().enumerate() {
        assert_eq!(actual.word(component), Some(value));
    }
    assert!(actual.word(5).is_none() && actual.word(usize::MAX).is_none());
}

fn count_origin(count: Option<u32>, stored: bool) -> FlowSimpleCountOrigin {
    if stored {
        return FlowSimpleCountOrigin::Stored;
    }
    if count.is_some() {
        return FlowSimpleCountOrigin::External;
    }
    FlowSimpleCountOrigin::Unspecified
}

fn five_storage(version: i32) -> FlowFiveWordStorage {
    match version {
        0..=2 => FlowFiveWordStorage::NoSerializedRows,
        3..=4 => FlowFiveWordStorage::RawRows,
        _ => FlowFiveWordStorage::InactiveRestored,
    }
}

fn check_bytes(actual: &FlowRawByteTable, expected: &ByteTable, case: &SimpleCase) {
    checkpoint(
        actual.source(),
        actual.record(),
        actual.decoded_bytes(),
        actual.version(),
        case,
    );
    check_byte_metadata(actual, expected, case);
    assert_eq!(actual.raw_bytes(), expected.bytes);
    for (index, &byte) in expected.bytes.iter().enumerate() {
        assert_eq!(actual.byte(index), Some(byte));
    }
    assert!(actual.byte(expected.bytes.len()).is_none() && actual.byte(usize::MAX).is_none());
    check_coverage(
        actual.coverage(),
        &case.frame,
        [0, 0, 0, expected.bytes.len()],
    );
    check_diagnostics(actual.diagnostics(), false);
}

fn check_coverage(coverage: FlowSimpleTableCoverage, frame: &SimpleFrame, expected: [usize; 4]) {
    assert_eq!(
        (coverage.stored_bytes, coverage.decoded_bytes),
        (frame.stored.len(), frame.decoded.len())
    );
    assert_eq!(
        [
            coverage.wire_rows,
            coverage.wide_words,
            coverage.raw_words,
            coverage.raw_bytes
        ],
        expected
    );
    assert_eq!(coverage.achieved_error, 0);
    assert_eq!(
        coverage.mode,
        if frame.compressed {
            FlowLegacyTableCheckMode::ExactZlibTable
        } else {
            FlowLegacyTableCheckMode::ExactRawTable
        }
    );
}

fn check_diagnostics(actual: &[FlowSimpleTableDiagnostic], leaf: bool) {
    let mut expected = vec![
        FlowSimpleTableDiagnostic::EngineeringRolesNotResolved,
        FlowSimpleTableDiagnostic::NativeRuntimeNotReproduced,
        FlowSimpleTableDiagnostic::OwnerCompressionContextNotReconstructed,
    ];
    if leaf {
        expected.push(FlowSimpleTableDiagnostic::TreeOrderNotAssigned);
    }
    expected.push(FlowSimpleTableDiagnostic::NativeApplicationNotChecked);
    assert_eq!(actual, expected);
}

fn check_group_arrays(actual: FlowParallelGroup<'_>, expected: &ParallelGroup) {
    for (ordinal, array) in expected.arrays.iter().enumerate() {
        check_wide(
            actual.array(ordinal).unwrap(),
            &array.bits,
            array.start as usize,
        );
        assert_eq!(
            actual.array(ordinal).unwrap().payload_interval().1,
            array.end as usize
        );
    }
    for index in expected.arrays.len()..=4 {
        assert!(actual.array(index).is_none());
    }
    assert!(actual.array(usize::MAX).is_none());
}

fn check_leaf_rows(actual: &FlowLeafPairs, expected: &[LeafPair]) {
    for (row, frozen) in actual.rows().zip(expected) {
        assert_eq!(row.ordinal(), frozen.ordinal as usize);
        assert_eq!(
            row.payload_interval(),
            (frozen.start as usize, frozen.end as usize)
        );
        check_wide(row.wide_words(), &frozen.bits, frozen.start as usize);
        assert_eq!(
            actual.row(row.ordinal()).unwrap().payload_interval(),
            row.payload_interval()
        );
    }
}

fn check_five_metadata(actual: &FlowFiveWordTable, expected: &FiveWords, case: &SimpleCase) {
    assert_eq!(actual.options().legacy_count, legacy_count(&case.frame));
    assert_eq!(actual.declared_count().map(|v| v.value()), expected.count);
    assert_eq!(
        actual.count_origin(),
        count_origin(expected.count, expected.stored)
    );
    assert_eq!(actual.storage(), five_storage(case.version));
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, case.frame.decoded.len())
    );
    assert_eq!(
        actual.raw_bytes(),
        &case.frame.decoded[expected.start as usize..]
    );
    assert_eq!(actual.rows().len(), expected.rows.len());
    assert_eq!(!expected.rows.is_empty(), expected.wire_present);
    assert!(actual.row(expected.rows.len()).is_none() && actual.row(usize::MAX).is_none());
}

fn check_five_rows(actual: &FlowFiveWordTable, expected: &FiveWords) {
    for (ordinal, (row, frozen)) in actual.rows().zip(&expected.rows).enumerate() {
        check_five_row(row, frozen, ordinal, expected.start as usize);
        check_five_row(
            actual.row(ordinal).unwrap(),
            frozen,
            ordinal,
            expected.start as usize,
        );
    }
}

fn check_byte_metadata(actual: &FlowRawByteTable, expected: &ByteTable, case: &SimpleCase) {
    assert_eq!(actual.options().legacy_count, legacy_count(&case.frame));
    assert_eq!(actual.declared_count().value(), expected.count);
    assert_eq!(
        actual.count_origin(),
        count_origin(Some(expected.count), expected.stored)
    );
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, case.frame.decoded.len())
    );
}

fn check_leaf_context(actual: &FlowLeafPairs, case: &SimpleCase) {
    assert_eq!(
        actual.options().leaf_count.value(),
        case.frame.leaf.unwrap()
    );
}
