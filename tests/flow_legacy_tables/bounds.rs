use super::*;
use bessemer::FailureClass;
use std::io::Write;

fn read_pointers(
    raw: &[u8],
    options: FlowPointerTableOptions,
) -> bessemer::Result<FlowPointerTables> {
    let source = carrier(401, raw, 500_000);
    source.record_401_pointer_table(&source.latest(401).unwrap().unwrap().id, options)
}

fn read_words(raw: &[u8], options: FlowLegacyWordOptions) -> bessemer::Result<FlowLegacyWords> {
    let source = carrier(405, raw, 500_000);
    source.record_405_words(&source.latest(405).unwrap().unwrap().id, options)
}

fn selected_case(identifier: u32, profile: &str) -> TableCase {
    cases()
        .into_iter()
        .find(|case| {
            case.frame.identifier == identifier
                && !case.frame.compressed
                && case.frame.name.contains(profile)
        })
        .unwrap()
}

#[test]
fn every_small_owned_prefix_and_extension_rejects_without_partial_tables() {
    for case in cases().into_iter().skip(3) {
        for end in 0..case.frame.stored.len() {
            assert!(!accepted(&case, &case.frame.stored[..end]));
        }
        let mut extra = case.frame.stored.clone();
        extra.push(0);
        assert!(!accepted(&case, &extra));
    }
}

fn accepted(case: &TableCase, raw: &[u8]) -> bool {
    if case.frame.identifier == 401 {
        read_pointers(raw, pointer_options(&case.frame)).is_ok()
    } else {
        read_words(raw, word_options(&case.frame)).is_ok()
    }
}

#[test]
fn exact_and_one_below_decoded_pair_wide_and_word_budgets_are_independent() {
    for case in cases() {
        match &case.expected {
            ExpectedTable::Pointers(_) => check_pointer_limits(&case),
            ExpectedTable::Words(_) => check_word_limits(&case),
        }
    }
}

fn check_pointer_limits(case: &TableCase) {
    let coverage = pointer_table(case).coverage();
    let mut exact = pointer_options(&case.frame);
    exact.maximum_decoded_bytes = case.frame.decoded.len();
    exact.maximum_pairs = coverage.pointer_pairs.max(1);
    exact.maximum_wide_words = coverage.wide_words.max(1);
    assert!(read_pointers(&case.frame.stored, exact).is_ok());
    for axis in 0..3 {
        let mut short = exact;
        match axis {
            0 => short.maximum_decoded_bytes -= 1,
            1 => short.maximum_pairs = coverage.pointer_pairs.saturating_sub(1),
            _ => short.maximum_wide_words = coverage.wide_words.saturating_sub(1),
        }
        assert!(read_pointers(&case.frame.stored, short).is_err());
    }
}

fn check_word_limits(case: &TableCase) {
    let count = word_table(case).word_count();
    let mut exact = word_options(&case.frame);
    exact.maximum_decoded_bytes = case.frame.decoded.len();
    exact.maximum_words = count.max(1);
    assert!(read_words(&case.frame.stored, exact).is_ok());
    let mut short = exact;
    short.maximum_decoded_bytes -= 1;
    assert!(read_words(&case.frame.stored, short).is_err());
    short = exact;
    short.maximum_words = count.saturating_sub(1);
    assert!(read_words(&case.frame.stored, short).is_err());
}

#[test]
fn native_context_domains_and_versioned_context_presence_are_explicit() {
    check_context_domains();
    let case = selected_case(405, "words-v0");
    let mut options = word_options(&case.frame);
    options.legacy_counts = None;
    assert_eq!(
        read_words(&case.frame.stored, options).unwrap_err().class,
        FailureClass::Missing
    );
    let case = selected_case(405, "words-v1");
    options = word_options(&case.frame);
    options.legacy_counts = Some(FlowLegacyWordCounts::try_from([0, 0]).unwrap());
    assert!(read_words(&case.frame.stored, options).is_err());
}

#[test]
fn delegated_versions_negative_counts_and_impossible_lengths_fail_before_allocating_arrays() {
    let pointers = selected_case(401, "pairs-v2-full");
    check_bad_pointer_headers(&pointers);
    check_bad_word_headers();
}

fn check_bad_word_headers() {
    let words = selected_case(405, "words-v1");
    for version in [-1_i32, 2, i32::MAX] {
        let mut raw = words.frame.decoded.clone();
        raw[..4].copy_from_slice(&version.to_le_bytes());
        assert_eq!(
            read_words(&raw, word_options(&words.frame))
                .unwrap_err()
                .class,
            FailureClass::Unsupported
        );
    }
    let raw: Vec<u8> = [1_i32, -1].into_iter().flat_map(i32::to_le_bytes).collect();
    assert!(read_words(&raw, word_options(&words.frame)).is_err());
    let mut options = word_options(&words.frame);
    options.legacy_counts = Some(FlowLegacyWordCounts::try_from([i32::MAX as u32, 0]).unwrap());
    options.maximum_words = usize::MAX;
    assert!(read_words(&0_i32.to_le_bytes(), options).is_err());
}

#[test]
fn pair_range_failures_and_bad_field_selectors_keep_the_original_input_isolated() {
    let case = selected_case(401, "pairs-v2-first");
    let mut options = pointer_options(&case.frame);
    options.global_pointer_count = Some(FlowGlobalPointerCount::try_from(1).unwrap());
    let table = read_pointers(&case.frame.stored, options).unwrap();
    for index in [-1, 1] {
        assert!(
            table
                .replace_field(pair_field(0, 0, 0), FlowPointerTableValue::Signed32(index))
                .is_err()
        );
    }
    check_bad_selectors(&table);
    assert_eq!(table.decoded_bytes(), case.frame.decoded);
    check_unbounded_pair(&case, options);
}

fn check_bad_selectors(table: &FlowPointerTables) {
    for field in rejected_selectors() {
        assert!(
            table
                .replace_field(field, FlowPointerTableValue::Signed32(0))
                .is_err()
        );
    }
    check_missing_fields(table);
    assert!(
        table
            .replace_field(member_field(0, 0), FlowPointerTableValue::WideBits(0))
            .is_err()
    );
}

#[test]
fn foreign_stale_and_wrong_record_ids_reject() {
    let case = selected_case(401, "pairs-v2-full");
    let table = pointer_table(&case);
    let foreign = carrier(401, &case.frame.stored, 500_000);
    assert!(
        foreign
            .record_401_pointer_table(table.record(), table.options())
            .is_err()
    );
    let wrong = table.source().latest(999).unwrap().unwrap().id;
    assert!(
        table
            .source()
            .record_401_pointer_table(&wrong, table.options())
            .is_err()
    );
    check_stale_pointer(&table);
    check_word_ids();
}

#[test]
fn zlib_checksums_wrappers_and_concatenated_streams_are_rejected() {
    let case = cases()
        .into_iter()
        .find(|case| {
            case.frame.identifier == 401
                && case.frame.compressed
                && !case.frame.name.starts_with("native")
        })
        .unwrap();
    let mut checksum = case.frame.stored.clone();
    *checksum.last_mut().unwrap() ^= 1;
    let mut extra = case.frame.stored.clone();
    extra.extend_from_slice(&case.frame.stored);
    for raw in [checksum, extra, vec![], vec![0x1f, 0x8b, 8, 0]] {
        assert!(read_pointers(&raw, pointer_options(&case.frame)).is_err());
    }
}

#[test]
fn measured_compressed_growth_obeys_the_original_physical_payload_limit() {
    let mut raw: Vec<u8> = [1_i32, 1024]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    raw.resize(4104, 0);
    let stored = compressed(&raw);
    let options = FlowLegacyWordOptions {
        compression: FlowLegacyTableTransport::Zlib,
        legacy_counts: None,
        maximum_decoded_bytes: 500_000,
        maximum_words: 50_000,
    };
    check_growth(&stored, &raw, options);
}

fn check_context_domains() {
    assert!(FlowGlobalPointerCount::try_from(i32::MAX as u32).is_ok());
    for count in [i32::MAX as u32 + 1, u32::MAX] {
        assert!(FlowGlobalPointerCount::try_from(count).is_err());
    }
    assert_eq!(
        FlowLegacyWordCounts::try_from([i32::MAX as u32, 0])
            .unwrap()
            .total(),
        i32::MAX as u32
    );
    for pair in [[u32::MAX, 0], [i32::MAX as u32, 1], [i32::MAX as u32; 2]] {
        assert!(FlowLegacyWordCounts::try_from(pair).is_err());
    }
}

fn check_bad_pointer_headers(pointers: &TableCase) {
    for version in [-1_i32, 4, i32::MAX] {
        let mut raw = pointers.frame.decoded.clone();
        raw[..4].copy_from_slice(&version.to_le_bytes());
        assert_eq!(
            read_pointers(&raw, pointer_options(&pointers.frame))
                .unwrap_err()
                .class,
            FailureClass::Unsupported
        );
    }
    for offset in [4, 20] {
        let mut raw = pointers.frame.decoded.clone();
        raw[offset..offset + 4].copy_from_slice(&(-1_i32).to_le_bytes());
        assert!(read_pointers(&raw, pointer_options(&pointers.frame)).is_err());
    }
    let mut raw = pointers.frame.decoded.clone();
    raw[4..8].copy_from_slice(&i32::MAX.to_le_bytes());
    let mut options = pointer_options(&pointers.frame);
    options.maximum_pairs = usize::MAX;
    assert!(read_pointers(&raw, options).is_err());
}

fn member_field(group: usize, component: usize) -> FlowPointerTableField {
    FlowPointerTableField::Member { group, component }
}
fn pair_field(group: usize, pair: usize, component: usize) -> FlowPointerTableField {
    FlowPointerTableField::PairIndex {
        group,
        pair,
        component,
    }
}
fn wide_field(group: usize, array: usize, index: usize) -> FlowPointerTableField {
    FlowPointerTableField::WideWord {
        group,
        array,
        index,
    }
}
fn rejected_selectors() -> [FlowPointerTableField; 7] {
    [
        member_field(2, 0),
        member_field(0, 3),
        FlowPointerTableField::DiscardedTriplet {
            group: 0,
            triplet: usize::MAX,
            component: 0,
        },
        wide_field(0, 3, 0),
        wide_field(0, 0, usize::MAX),
        pair_field(0, usize::MAX, 0),
        pair_field(0, 0, 2),
    ]
}
fn check_missing_fields(table: &FlowPointerTables) {
    for field in [
        FlowPointerTableField::DiscardedCount { group: 0 },
        FlowPointerTableField::DiscardedPrePairs { group: 0 },
        member_field(1, 0),
    ] {
        assert_eq!(
            table
                .replace_field(field, FlowPointerTableValue::Signed32(0))
                .unwrap_err()
                .class,
            FailureClass::Missing
        );
    }
}
fn check_unbounded_pair(case: &TableCase, mut options: FlowPointerTableOptions) {
    options.global_pointer_count = None;
    let table = read_pointers(&case.frame.stored, options).unwrap();
    let edited = table
        .replace_field(pair_field(0, 0, 0), FlowPointerTableValue::Signed32(-1))
        .unwrap();
    assert_eq!(
        edited
            .result()
            .group(0)
            .unwrap()
            .body()
            .unwrap()
            .pair(0)
            .unwrap()[0],
        -1
    );
}
fn check_stale_pointer(table: &FlowPointerTables) {
    let edit = table
        .replace_field(member_field(0, 0), FlowPointerTableValue::Signed32(1))
        .unwrap();
    assert!(
        edit.result()
            .source()
            .record_401_pointer_table(table.record(), table.options())
            .is_err()
    );
}
fn check_word_ids() {
    let case = selected_case(405, "words-v1");
    let words = word_table(&case);
    let wrong = words.source().latest(999).unwrap().unwrap().id;
    assert!(words.replace_word(usize::MAX, 0).is_err());
    assert!(
        words
            .source()
            .record_405_words(&wrong, words.options())
            .is_err()
    );
}
fn compressed(raw: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}
fn check_growth(stored: &[u8], raw: &[u8], options: FlowLegacyWordOptions) {
    let source = carrier(405, stored, 500_000);
    let words = source
        .record_405_words(&source.latest(405).unwrap().unwrap().id, options)
        .unwrap();
    assert!(
        words
            .replace_word(0, 0x12345678)
            .unwrap()
            .result()
            .coverage()
            .stored_bytes
            > stored.len()
    );
    let source = carrier(405, stored, stored.len());
    let words = source
        .record_405_words(&source.latest(405).unwrap().unwrap().id, options)
        .unwrap();
    assert!(words.replace_word(0, 0x12345678).is_err());
    assert_eq!(words.decoded_bytes(), raw);
}
