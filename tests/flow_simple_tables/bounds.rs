use super::*;
use bessemer::FailureClass;

fn owned(identifier: u32, name: &str) -> SimpleCase {
    cases()
        .into_iter()
        .find(|case| {
            case.frame.identifier == identifier && case.frame.name == name && !case.frame.compressed
        })
        .unwrap()
}

fn limited(case: &SimpleCase, decoded: usize, maximum: usize) -> bessemer::Result<SimpleView> {
    let source = carrier(&case.frame, 2_000_000);
    let id = source.latest(case.frame.identifier).unwrap().unwrap().id;
    match case.frame.identifier {
        407 => limited_parallel(&source, &id, &case.frame, decoded, maximum),
        437 => limited_leaf(&source, &id, &case.frame, decoded, maximum),
        411 => limited_five(&source, &id, &case.frame, decoded, maximum),
        413 => limited_bytes(&source, &id, &case.frame, decoded, maximum),
        value => panic!("identifier {value}; expected simple table"),
    }
}

fn actual_units(expected: &ExpectedSimpleTable) -> usize {
    match expected {
        ExpectedSimpleTable::Parallel(groups) => groups
            .iter()
            .flat_map(|g| &g.arrays)
            .map(|a| a.bits.len())
            .sum(),
        ExpectedSimpleTable::LeafPairs(rows) => rows.len(),
        ExpectedSimpleTable::FiveWords(table) => table.rows.len(),
        ExpectedSimpleTable::Bytes(table) => table.bytes.len(),
    }
}

#[test]
fn exact_and_one_below_decoded_and_actual_wire_budgets_do_not_use_inactive_counts() {
    for case in cases() {
        let units = actual_units(&case.expected);
        assert!(limited(&case, case.frame.decoded.len(), units.max(1)).is_ok());
        assert!(limited(&case, case.frame.decoded.len() - 1, units.max(1)).is_err());
        assert!(limited(&case, case.frame.decoded.len(), units.saturating_sub(1)).is_err());
    }
}

#[test]
fn every_owned_proper_prefix_and_extension_rejects_without_partial_tables() {
    for mut case in cases().into_iter().skip(4) {
        let complete = case.frame.stored.clone();
        for end in 0..complete.len() {
            case.frame.stored = complete[..end].to_vec();
            assert!(
                read_fallible(&case.frame).is_err(),
                "{} {end}",
                case.frame.name
            );
        }
        case.frame.stored = complete;
        case.frame.stored.push(0);
        assert!(read_fallible(&case.frame).is_err());
    }
}

fn read_fallible(frame: &SimpleFrame) -> bessemer::Result<SimpleView> {
    let source = carrier(frame, 2_000_000);
    read_at(
        &source,
        &source.latest(frame.identifier).unwrap().unwrap().id,
        frame,
    )
}

#[test]
fn later_populated_restore_branches_reject_while_empty_branches_stay_complete() {
    for (identifier, version) in restore_cases() {
        let mut case = raw_sample(identifier);
        case.frame.stored = version.to_le_bytes().to_vec();
        if identifier != 437 {
            case.frame.stored.extend_from_slice(&1_i32.to_le_bytes());
        }
        if identifier == 407 {
            case.frame.stored.extend_from_slice(&0_i32.to_le_bytes());
        }
        if identifier == 437 {
            case.frame.leaf = Some(0);
        }
        assert_eq!(
            read_fallible(&case.frame).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
}

fn raw_sample(identifier: u32) -> SimpleCase {
    match identifier {
        407 => owned(407, "parallel-v0-2"),
        437 => owned(437, "leaf-pairs-2"),
        411 => owned(411, "five-v4-2"),
        413 => owned(413, "bytes-v1-3"),
        value => panic!("identifier {value}; expected simple table"),
    }
}

#[test]
fn signed_versions_counts_and_impossible_extents_reject_before_allocating_rows() {
    for identifier in [407, 437, 411, 413] {
        let mut case = raw_sample(identifier);
        case.frame.stored[..4].copy_from_slice(&(-1_i32).to_le_bytes());
        assert_eq!(
            read_fallible(&case.frame).unwrap_err().class,
            FailureClass::Invalid
        );
    }
    for identifier in [407, 411, 413] {
        for count in [-1_i32, i32::MAX] {
            let mut case = raw_sample(identifier);
            case.frame.stored[4..8].copy_from_slice(&count.to_le_bytes());
            assert!(limited(&case, 2_000_000, usize::MAX).is_err());
        }
    }
    let mut case = raw_sample(437);
    case.frame.leaf = Some(i32::MAX as u32);
    assert!(limited(&case, 2_000_000, usize::MAX).is_err());
}

#[test]
fn legacy_context_presence_and_native_domains_are_enforced_without_guessing_rows() {
    for value in [i32::MAX as u32 + 1, u32::MAX] {
        assert!(FlowSimpleLegacyCount::try_from(value).is_err());
    }
    assert_eq!(
        FlowSimpleLegacyCount::try_from(i32::MAX as u32)
            .unwrap()
            .value(),
        i32::MAX as u32
    );
    check_legacy_context(411, "five-v3-1", None, FailureClass::Missing);
    check_legacy_context(413, "bytes-v0-3", None, FailureClass::Missing);
    check_legacy_context(411, "five-v4-1", Some(0), FailureClass::Invalid);
    check_legacy_context(413, "bytes-v1-3", Some(0), FailureClass::Invalid);
    let case = owned(411, "five-v1-context-2147483647");
    let SimpleView::Five(table) = read_case(&case) else {
        panic!("expected inactive five-word table")
    };
    assert_eq!(table.declared_count().unwrap().value(), i32::MAX as u32);
    assert_eq!(table.rows().len(), 0);
    assert!(table.replace_word(0, 0, 0).is_err());
}

fn check_legacy_context(identifier: u32, name: &str, context: Option<u32>, class: FailureClass) {
    let mut case = owned(identifier, name);
    case.frame.legacy = context;
    assert_eq!(read_fallible(&case.frame).unwrap_err().class, class);
}

#[test]
fn absent_and_empty_arrays_and_inactive_fields_keep_typed_edit_failures() {
    let SimpleView::Parallel(absent) = read_case(&owned(407, "parallel-v0-3")) else {
        panic!("expected parallel")
    };
    assert_eq!(
        absent.replace_word(0, 0, 0, 0).unwrap_err().class,
        FailureClass::Missing
    );
    let SimpleView::Parallel(empty) = read_case(&owned(407, "parallel-v0-1")) else {
        panic!("expected parallel")
    };
    assert_eq!(
        empty.replace_word(0, 0, 0, 0).unwrap_err().class,
        FailureClass::Invalid
    );
    check_parallel_indices(&empty);
    check_inactive_parallel();
    check_bad_row_indices();
}

fn check_bad_row_indices() {
    let SimpleView::LeafPairs(leaf) = read_case(&raw_sample(437)) else {
        panic!("expected leaf pairs")
    };
    assert!(leaf.replace_word(usize::MAX, 0, 0).is_err());
    assert!(leaf.replace_word(0, 2, 0).is_err());
    let SimpleView::Five(five) = read_case(&raw_sample(411)) else {
        panic!("expected five-word rows")
    };
    assert!(five.replace_word(usize::MAX, 0, 0).is_err());
    assert!(five.replace_word(0, 5, 0).is_err());
    let SimpleView::Bytes(bytes) = read_case(&raw_sample(413)) else {
        panic!("expected bytes")
    };
    assert!(bytes.replace_byte(usize::MAX, 0).is_err());
}

#[test]
fn valid_empty_presence_reinterpretations_are_rejected_by_the_fixed_structure_editor() {
    for (name, after) in [("parallel-v0-0", 1_i32), ("parallel-v0-1", 0)] {
        let case = owned(407, name);
        let SimpleView::Parallel(source) = read_case(&case) else {
            panic!("expected parallel")
        };
        let mut alternate = owned(407, name);
        alternate.frame.stored[12..16].copy_from_slice(&after.to_le_bytes());
        assert!(read_fallible(&alternate.frame).is_ok());
        let error = source.replace_presence(0, after).unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains("array presence"));
        assert_eq!(source.decoded_bytes(), case.frame.decoded);
    }
}

#[test]
fn foreign_stale_wrong_record_ids_and_zlib_failures_do_not_publish_partial_views() {
    for identifier in [407, 437, 411, 413] {
        check_ids(identifier);
        check_bad_streams(identifier);
    }
}

fn check_ids(identifier: u32) {
    let case = raw_sample(identifier);
    let source = read_case(&case);
    let foreign = carrier(&case.frame, 2_000_000);
    assert!(read_at(&foreign, source.record(), &case.frame).is_err());
    let wrong = source.source().records().nth(2).unwrap().id;
    assert!(read_at(source.source(), &wrong, &case.frame).is_err());
    let changed = source
        .source()
        .replace_payload(source.record(), b"changed")
        .unwrap();
    assert!(read_at(changed.result(), source.record(), &case.frame).is_err());
    assert_eq!(source.decoded(), case.frame.decoded);
}

fn check_bad_streams(identifier: u32) {
    let mut case = raw_sample(identifier);
    let source_name = case.frame.name.clone();
    case = cases()
        .into_iter()
        .find(|case| {
            case.frame.identifier == identifier
                && case.frame.name == source_name
                && case.frame.compressed
        })
        .unwrap();
    let mut checksum = case.frame.stored.clone();
    *checksum.last_mut().unwrap() ^= 1;
    let mut concatenated = case.frame.stored.clone();
    concatenated.extend_from_slice(&case.frame.stored);
    for raw in [Vec::new(), vec![0x1f, 0x8b, 0], checksum, concatenated] {
        case.frame.stored = raw;
        assert!(read_fallible(&case.frame).is_err());
    }
}

#[test]
fn all_public_simple_table_types_and_completed_edits_are_send_and_sync() {
    assert_shared::<FlowSimpleLegacyCount>();
    assert_shared::<FlowParallelTableOptions>();
    assert_shared::<FlowLeafPairOptions>();
    assert_shared::<FlowFiveWordTableOptions>();
    assert_shared::<FlowRawByteTableOptions>();
    assert_shared::<FlowParallelTable>();
    assert_shared::<FlowParallelGroup<'static>>();
    assert_shared::<FlowLeafPairs>();
    assert_shared::<FlowLeafPairRow<'static>>();
    assert_shared::<FlowFiveWordTable>();
    assert_shared::<FlowFiveWordRow<'static>>();
    assert_shared::<FlowRawByteTable>();
    assert_shared::<FlowSimpleCountOrigin>();
    assert_shared::<FlowFiveWordStorage>();
    assert_shared::<FlowSimpleTableCoverage>();
    assert_shared::<FlowSimpleTableDiagnostic>();
    assert_edit_types();
}

fn limited_parallel(
    source: &FlowCarrier,
    id: &FlowRecordId,
    frame: &SimpleFrame,
    decoded: usize,
    maximum: usize,
) -> bessemer::Result<SimpleView> {
    let mut options = parallel_options(frame);
    options.maximum_decoded_bytes = decoded;
    options.maximum_wide_words = maximum;
    source
        .record_407_arrays(id, options)
        .map(SimpleView::Parallel)
}

fn limited_leaf(
    source: &FlowCarrier,
    id: &FlowRecordId,
    frame: &SimpleFrame,
    decoded: usize,
    maximum: usize,
) -> bessemer::Result<SimpleView> {
    let mut options = leaf_options(frame);
    options.maximum_decoded_bytes = decoded;
    options.maximum_wire_rows = maximum;
    source
        .record_437_leaf_pairs(id, options)
        .map(SimpleView::LeafPairs)
}

fn limited_five(
    source: &FlowCarrier,
    id: &FlowRecordId,
    frame: &SimpleFrame,
    decoded: usize,
    maximum: usize,
) -> bessemer::Result<SimpleView> {
    let mut options = five_options(frame);
    options.maximum_decoded_bytes = decoded;
    options.maximum_wire_rows = maximum;
    source.record_411_rows(id, options).map(SimpleView::Five)
}

fn limited_bytes(
    source: &FlowCarrier,
    id: &FlowRecordId,
    frame: &SimpleFrame,
    decoded: usize,
    maximum: usize,
) -> bessemer::Result<SimpleView> {
    let mut options = byte_options(frame);
    options.maximum_decoded_bytes = decoded;
    options.maximum_raw_bytes = maximum;
    source.record_413_bytes(id, options).map(SimpleView::Bytes)
}

fn restore_cases() -> [(u32, i32); 7] {
    [
        (407, 1),
        (407, i32::MAX),
        (437, 1),
        (411, 5),
        (411, i32::MAX),
        (413, 2),
        (413, i32::MAX),
    ]
}

fn assert_shared<T: Send + Sync>() {}

fn assert_edit_types() {
    assert_shared::<FlowSimpleTableEdit<FlowParallelTable>>();
    assert_shared::<FlowSimpleTableEdit<FlowLeafPairs>>();
    assert_shared::<FlowSimpleTableEdit<FlowFiveWordTable>>();
    assert_shared::<FlowSimpleTableEdit<FlowRawByteTable>>();
}

fn check_parallel_indices(empty: &FlowParallelTable) {
    assert!(empty.replace_word(usize::MAX, 0, 0, 0).is_err());
    assert!(empty.replace_word(0, usize::MAX, 0, 0).is_err());
    assert!(empty.replace_presence(usize::MAX, 0).is_err());
}

fn check_inactive_parallel() {
    let SimpleView::Parallel(later) = read_case(&owned(407, "parallel-v1-empty")) else {
        panic!("expected parallel")
    };
    assert_eq!(
        later.replace_presence(0, 1).unwrap_err().class,
        FailureClass::Missing
    );
}
