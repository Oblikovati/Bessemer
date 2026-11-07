use super::*;
use bessemer::FailureClass;

fn read_raw(raw: &[u8], options: FlowBlockDataOptions) -> bessemer::Result<FlowLegacyBlockData> {
    let source = carrier(raw, 500_000);
    source.record_406_blocks(&source.latest(406).unwrap().unwrap().id, options)
}

#[test]
fn every_small_owned_prefix_and_extension_rejects_without_partial_bodies() {
    for case in fixture::cases().into_iter().skip(1) {
        for end in 0..case.stored.len() {
            assert!(
                read_raw(&case.stored[..end], options(&case)).is_err(),
                "{} {end}",
                case.name
            );
        }
        let mut extra = case.stored.clone();
        extra.push(0);
        assert!(read_raw(&extra, options(&case)).is_err());
    }
}

#[test]
fn byte_block_entry_and_word_limits_are_independent_and_inclusive() {
    for case in fixture::cases() {
        let mut exact = options(&case);
        exact.maximum_decoded_bytes = case.decoded.len();
        exact.maximum_blocks = (case.leaf_count as usize).max(1);
        exact.maximum_entries = (case.totals[0] as usize).max(1);
        exact.maximum_integer_words = (case.totals[1] as usize + case.totals[2] as usize).max(1);
        assert!(read_raw(&case.stored, exact).is_ok());
        let mut short = exact;
        short.maximum_decoded_bytes -= 1;
        assert!(read_raw(&case.stored, short).is_err());
        check_count_limits(&case, exact);
    }
}

fn check_count_limits(case: &fixture::BlockCase, exact: FlowBlockDataOptions) {
    if case.leaf_count > 0 {
        let mut short = exact;
        short.maximum_blocks = case.leaf_count as usize - 1;
        assert!(read_raw(&case.stored, short).is_err());
    }
    if case.totals[0] > 0 {
        let mut short = exact;
        short.maximum_entries = case.totals[0] as usize - 1;
        assert!(read_raw(&case.stored, short).is_err());
    }
    if case.totals[1] + case.totals[2] > 0 {
        let mut short = exact;
        short.maximum_integer_words = (case.totals[1] + case.totals[2]) as usize - 1;
        assert!(read_raw(&case.stored, short).is_err());
    }
}

#[test]
fn leaf_context_and_signed_domains_are_not_inferred_from_prefix_totals() {
    assert!(FlowLeafCount::try_from(i32::MAX as u32).is_ok());
    assert!(FlowLeafCount::try_from(i32::MAX as u32 + 1).is_err());
    assert!(FlowLeafCount::try_from(u32::MAX).is_err());
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.name == "populated-v14" && !case.compressed)
        .unwrap();
    for count in [0, 1, 3, i32::MAX as u32] {
        let mut configured = options(&case);
        configured.leaf_count = FlowLeafCount::try_from(count).unwrap();
        assert!(read_raw(&case.stored, configured).is_err());
    }
    for value in [-1_i32, i32::MIN] {
        let mut raw = case.decoded.clone();
        raw[4..8].copy_from_slice(&value.to_le_bytes());
        assert!(read_raw(&raw, options(&case)).is_err());
    }
}

#[test]
fn delegated_versions_invalid_options_and_malformed_zlib_are_explicit() {
    let all = fixture::cases();
    let case = all
        .iter()
        .find(|case| case.name == "empty-v14" && !case.compressed)
        .unwrap();
    for version in [i32::MIN, -1, 0, 18, i32::MAX] {
        let mut raw = case.decoded.clone();
        raw[..4].copy_from_slice(&version.to_le_bytes());
        assert_eq!(
            read_raw(&raw, options(case)).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
    check_invalid_options(case);
}

fn check_invalid_options(case: &fixture::BlockCase) {
    for axis in 0..4 {
        let mut invalid = options(case);
        match axis {
            0 => invalid.maximum_decoded_bytes = 11,
            1 => invalid.maximum_blocks = 0,
            2 => invalid.maximum_entries = 0,
            _ => invalid.maximum_integer_words = 0,
        };
        assert!(read_raw(&case.stored, invalid).is_err());
    }
}

#[test]
fn complete_zlib_checksum_and_single_stream_boundaries_are_required() {
    let all = fixture::cases();
    let compressed = all
        .iter()
        .find(|case| case.compressed && case.name == "empty-v14")
        .unwrap();
    let mut bad_checksum = compressed.stored.clone();
    *bad_checksum.last_mut().unwrap() ^= 1;
    let mut concatenated = compressed.stored.clone();
    concatenated.extend_from_slice(&compressed.stored);
    for raw in [bad_checksum, concatenated, vec![], vec![0x1f, 0x8b, 8, 0]] {
        assert!(read_raw(&raw, options(compressed)).is_err());
    }
}

#[test]
fn foreign_stale_wrong_records_and_invalid_field_indices_reject() {
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.name == "populated-v14" && !case.compressed)
        .unwrap();
    let body = parse_case(&case);
    let foreign = carrier(&case.stored, 500_000);
    assert!(
        foreign
            .record_406_blocks(body.record(), options(&case))
            .is_err()
    );
    let wrong = body.source().latest(999).unwrap().unwrap().id;
    assert!(
        body.source()
            .record_406_blocks(&wrong, options(&case))
            .is_err()
    );
    check_invalid_indices(&body);
    check_stale_record(&body, &case);
}

fn check_stale_record(body: &FlowLegacyBlockData, case: &fixture::BlockCase) {
    let changed = body
        .replace_field(FlowBlockFieldEdit::Entry {
            index: 0,
            field: FlowBlockEntryField::State,
            value: FlowBlockFieldValue::Byte(1),
        })
        .unwrap();
    assert!(
        changed
            .result()
            .source()
            .record_406_blocks(body.record(), options(case))
            .is_err()
    );
    assert_eq!(body.decoded_bytes(), case.decoded);
}

fn check_invalid_indices(body: &FlowLegacyBlockData) {
    for field in [
        FlowBlockEntryField::Descriptor { index: 7 },
        FlowBlockEntryField::FirstWord { index: usize::MAX },
        FlowBlockEntryField::SecondWord { index: usize::MAX },
    ] {
        assert!(body.entry(0).unwrap().field(field).is_err());
    }
}
