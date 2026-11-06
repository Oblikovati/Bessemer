use super::*;
use bessemer::flow::*;
#[path = "bounds.rs"]
mod bounds;
#[path = "edits.rs"]
mod edits;

fn pointer_options(frame: &TableFrame) -> FlowPointerTableOptions {
    FlowPointerTableOptions {
        compression: compression(frame),
        global_pointer_count: frame
            .pointer_count
            .map(|n| FlowGlobalPointerCount::try_from(n).unwrap()),
        maximum_decoded_bytes: 500_000,
        maximum_pairs: 50_000,
        maximum_wide_words: 50_000,
    }
}

fn word_options(frame: &TableFrame) -> FlowLegacyWordOptions {
    FlowLegacyWordOptions {
        compression: compression(frame),
        legacy_counts: frame
            .legacy
            .map(|n| FlowLegacyWordCounts::try_from(n).unwrap()),
        maximum_decoded_bytes: 500_000,
        maximum_words: 50_000,
    }
}

fn compression(frame: &TableFrame) -> FlowLegacyTableTransport {
    if frame.compressed {
        FlowLegacyTableTransport::Zlib
    } else {
        FlowLegacyTableTransport::Raw
    }
}

fn carrier(identifier: u32, payload: &[u8], maximum: usize) -> FlowCarrier {
    let mut raw = 8_u16.to_le_bytes().to_vec();
    for (id, bytes) in [
        (identifier, &b"decoy"[..]),
        (identifier, payload),
        (999, &b"opaque\0"[..]),
    ] {
        raw.extend_from_slice(&id.to_le_bytes());
        raw.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        raw.extend_from_slice(bytes);
    }
    FlowCarrier::from_bytes(
        &raw,
        FlowLimits {
            maximum_wire_bytes: maximum + 100,
            maximum_payload_bytes: maximum,
            maximum_records: 3,
        },
    )
    .unwrap()
}

fn pointer_table(case: &TableCase) -> FlowPointerTables {
    let source = carrier(401, &case.frame.stored, 500_000);
    source
        .record_401_pointer_table(
            &source.latest(401).unwrap().unwrap().id,
            pointer_options(&case.frame),
        )
        .unwrap()
}

fn word_table(case: &TableCase) -> FlowLegacyWords {
    let source = carrier(405, &case.frame.stored, 500_000);
    source
        .record_405_words(
            &source.latest(405).unwrap().unwrap().id,
            word_options(&case.frame),
        )
        .unwrap()
}

pub fn check_case(case: &TableCase) {
    match &case.expected {
        ExpectedTable::Pointers(expected) => {
            check_pointer_table(&pointer_table(case), case, expected)
        }
        ExpectedTable::Words(expected) => check_word_table(&word_table(case), case, expected),
    }
}

fn check_pointer_table(actual: &FlowPointerTables, case: &TableCase, expected: &ExpectedPointers) {
    check_pointer_checkpoint(actual, case);
    assert!(actual.group(2).is_none() && actual.group(usize::MAX).is_none());
    assert_eq!(actual.groups().len(), 2);
    for (index, (group, frozen)) in actual.groups().zip(&expected.groups).enumerate() {
        check_group(group, frozen);
        check_group(actual.group(index).unwrap(), frozen);
    }
    check_pointer_coverage(actual, case, expected);
    assert_eq!(actual.diagnostics(), &expected_diagnostics(true));
}

fn check_group(actual: FlowPointerTableGroup<'_>, expected: &ExpectedGroup) {
    assert_eq!(actual.ordinal(), expected.ordinal as usize);
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(actual.count(), expected.count as usize);
    assert_eq!(actual.discarded_count_word(), expected.discarded_count);
    assert_eq!(actual.body().is_some(), expected.body.is_some());
    if let Some(body) = &expected.body {
        check_body(actual.body().unwrap(), body);
    }
}

fn check_body(actual: FlowPointerGroupBody<'_>, expected: &ExpectedGroupBody) {
    assert_eq!(actual.member_triplet(), expected.members);
    assert_eq!(actual.array_lengths(), expected.lengths.map(|n| n as usize));
    assert_eq!(
        actual.discarded_triplets().collect::<Vec<_>>(),
        expected.discarded
    );
    assert_eq!(actual.discarded_pre_pairs_word(), expected.pre_pairs);
    assert_eq!(actual.pairs().collect::<Vec<_>>(), expected.pairs);
    for (index, pair) in expected.pairs.iter().enumerate() {
        assert_eq!(actual.pair(index), Some(*pair));
    }
    assert!(actual.pair(expected.pairs.len()).is_none() && actual.pair(usize::MAX).is_none());
    for (index, array) in expected.arrays.iter().enumerate() {
        check_array(actual.array(index).unwrap(), array);
    }
    assert!(actual.array(3).is_none() && actual.array(usize::MAX).is_none());
}

fn check_array(actual: FlowTableWideArray<'_>, expected: &ExpectedWideArray) {
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(actual.word_count(), expected.bits.len());
    assert_eq!(
        actual.as_bytes(),
        expected
            .bits
            .iter()
            .flat_map(|bits| bits.to_le_bytes())
            .collect::<Vec<_>>()
    );
    for (index, bits) in expected.bits.iter().enumerate() {
        assert_eq!(actual.raw_bits(index), Some(*bits));
    }
    assert!(
        actual.raw_bits(expected.bits.len()).is_none() && actual.raw_bits(usize::MAX).is_none()
    );
}

fn check_pointer_coverage(
    actual: &FlowPointerTables,
    case: &TableCase,
    expected: &ExpectedPointers,
) {
    let coverage = actual.coverage();
    check_common_coverage(coverage, &case.frame);
    assert_eq!(
        coverage.pointer_range.map(FlowGlobalPointerCount::value),
        case.frame.pointer_count
    );
    assert!(coverage.external_word_counts.is_none());
    assert_eq!(coverage.raw_words, 0);
    assert_eq!(
        coverage.pointer_pairs,
        expected.groups.iter().map(|g| g.count as usize).sum()
    );
    let wide: usize = expected
        .groups
        .iter()
        .filter_map(|g| g.body.as_ref())
        .flat_map(|body| &body.arrays)
        .map(|array| array.bits.len())
        .sum();
    assert_eq!(coverage.wide_words, wide);
}

fn check_word_table(actual: &FlowLegacyWords, case: &TableCase, expected: &ExpectedWords) {
    check_word_fields(actual, case, expected);
    check_word_checkpoint(actual, case);
    for (index, word) in expected.words.iter().enumerate() {
        assert_eq!(actual.raw_word(index), Some(*word));
    }
    assert!(
        actual.raw_word(expected.words.len()).is_none() && actual.raw_word(usize::MAX).is_none()
    );
    check_word_coverage(actual, case);
    assert_eq!(actual.diagnostics(), &expected_diagnostics(false));
}

fn check_word_coverage(actual: &FlowLegacyWords, case: &TableCase) {
    let coverage = actual.coverage();
    check_common_coverage(coverage, &case.frame);
    assert_eq!((coverage.pointer_pairs, coverage.wide_words), (0, 0));
    assert_eq!(coverage.raw_words, actual.word_count());
    assert!(coverage.pointer_range.is_none());
    assert_eq!(
        coverage
            .external_word_counts
            .map(FlowLegacyWordCounts::pair),
        case.frame.legacy
    );
    if let Some(counts) = coverage.external_word_counts {
        assert_eq!(counts.total() as usize, actual.word_count());
    }
}

fn check_common_coverage(coverage: FlowLegacyTableCoverage, frame: &TableFrame) {
    assert_eq!(
        coverage.mode,
        if frame.compressed {
            FlowLegacyTableCheckMode::ExactZlibTable
        } else {
            FlowLegacyTableCheckMode::ExactRawTable
        }
    );
    assert_eq!(coverage.stored_bytes, frame.stored.len());
    assert_eq!(coverage.decoded_bytes, frame.decoded.len());
    assert_eq!(coverage.achieved_error, 0);
}

#[test]
fn all_public_legacy_table_types_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<FlowLegacyTableTransport>();
    assert_traits::<FlowGlobalPointerCount>();
    assert_traits::<FlowLegacyWordCounts>();
    assert_traits::<FlowPointerTableOptions>();
    assert_traits::<FlowLegacyWordOptions>();
    assert_traits::<FlowPointerTables>();
    assert_traits::<FlowLegacyWords>();
    assert_traits::<FlowPointerTableGroup<'static>>();
    assert_traits::<FlowPointerGroupBody<'static>>();
    assert_traits::<FlowTableWideArray<'static>>();
    assert_traits::<FlowLegacyTableCheckMode>();
    assert_traits::<FlowLegacyTableCoverage>();
    assert_traits::<FlowLegacyTableDiagnostic>();
    assert_traits::<FlowPointerTableEdit>();
    assert_traits::<FlowLegacyWordEdit>();
    assert_traits::<FlowPointerTableField>();
    assert_traits::<FlowPointerTableValue>();
}

fn check_pointer_checkpoint(actual: &FlowPointerTables, case: &TableCase) {
    assert_eq!(actual.version(), case.version);
    assert_eq!(actual.decoded_bytes(), case.frame.decoded);
    assert_eq!(actual.record().ordinal(), 1);
    assert_eq!(
        actual.source().as_bytes(),
        carrier(401, &case.frame.stored, 500_000).as_bytes()
    );
    assert_eq!(
        actual
            .options()
            .global_pointer_count
            .map(FlowGlobalPointerCount::value),
        case.frame.pointer_count
    );
}

fn check_word_fields(actual: &FlowLegacyWords, case: &TableCase, expected: &ExpectedWords) {
    assert_eq!(actual.version(), case.version);
    assert_eq!(actual.count_stored(), expected.count_stored);
    assert_eq!(actual.word_count(), expected.count as usize);
    assert_eq!(
        actual.payload_interval(),
        (expected.words_start as usize, case.frame.decoded.len())
    );
    assert_eq!(actual.decoded_bytes(), case.frame.decoded);
    assert_eq!(
        actual.raw_bytes(),
        &case.frame.decoded[expected.words_start as usize..]
    );
}

fn check_word_checkpoint(actual: &FlowLegacyWords, case: &TableCase) {
    assert_eq!(actual.record().ordinal(), 1);
    assert_eq!(
        actual.source().as_bytes(),
        carrier(405, &case.frame.stored, 500_000).as_bytes()
    );
    assert_eq!(
        actual
            .options()
            .legacy_counts
            .map(FlowLegacyWordCounts::pair),
        case.frame.legacy
    );
}

fn expected_diagnostics(pointers: bool) -> [FlowLegacyTableDiagnostic; 3] {
    let semantic = if pointers {
        FlowLegacyTableDiagnostic::NativePointerPopulationNotReproduced
    } else {
        FlowLegacyTableDiagnostic::WordRolesNotResolved
    };
    [
        semantic,
        FlowLegacyTableDiagnostic::OwnerCompressionContextNotReconstructed,
        FlowLegacyTableDiagnostic::NativeApplicationNotChecked,
    ]
}
