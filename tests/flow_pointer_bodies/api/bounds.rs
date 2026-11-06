use super::*;

#[test]
fn every_small_owned_prefix_extension_and_missing_runtime_context_is_rejected() {
    for case in cases()
        .into_iter()
        .filter(|case| case.frame.decoded.len() <= 4096)
    {
        let mut context = options(&case.frame);
        context.compression = FlowLegacyTableTransport::Raw;
        for end in 0..case.frame.decoded.len() {
            assert_invalid(open(&case.frame.decoded[..end], context.clone()));
        }
        let mut extended = case.frame.decoded.clone();
        extended.push(0);
        assert_invalid(open(&extended, context.clone()));
        if case.version < 3 && case.frame.counts[2] > 0 {
            context.entry_codes = None;
            assert_invalid(open(&case.frame.decoded, context));
        }
    }
}

fn set_limit(options: &mut FlowPointerBodyOptions, axis: usize, limit: usize) {
    match axis {
        0 => options.maximum_decoded_bytes = limit,
        1 => options.maximum_pools = limit,
        2 => options.maximum_entries = limit,
        3 => options.maximum_vectors = limit,
        4 => options.maximum_wide_words = limit,
        5 => options.maximum_raw_bytes = limit,
        _ => panic!("axis {axis}; expected one of six bounds"),
    }
}

#[test]
fn each_allocation_axis_accepts_exact_and_rejects_one_below_or_zero() {
    for case in cases() {
        let coverage = read_case(&case).coverage();
        let counts = [
            coverage.decoded_bytes,
            coverage.pools,
            coverage.entries,
            coverage.vectors,
            coverage.wide_words,
            coverage.raw_bytes,
        ];
        for (axis, count) in counts.into_iter().enumerate() {
            check_limit(&case.frame, axis, count);
        }
    }
}

fn check_limit(frame: &PointerFrame, axis: usize, count: usize) {
    let mut context = options(frame);
    set_limit(&mut context, axis, count.max(1));
    assert!(open(&frame.stored, context.clone()).is_ok());
    if count > 0 {
        set_limit(&mut context, axis, count - 1);
        assert_invalid(open(&frame.stored, context.clone()));
    }
    set_limit(&mut context, axis, 0);
    assert_invalid(open(&frame.stored, context));
}

#[test]
fn count_domains_and_mathematical_totals_are_admitted_without_wrapping() {
    assert!(FlowPointerBodyCounts::try_from([u32::MAX, 0, 0]).is_err());
    let counts = FlowPointerBodyCounts::try_from([i32::MAX as u32; 3]).unwrap();
    assert_eq!(counts.total(), 3 * i32::MAX as u64);
    let case = cases().remove(0);
    let mut context = options(&case.frame);
    context.group_counts = counts;
    context.entry_codes = None;
    context.maximum_entries = usize::MAX;
    assert_invalid(open(&3_i32.to_le_bytes(), context));
}

#[test]
fn delegated_versions_negative_counts_short_payloads_and_unused_tagged_context_fail() {
    let case = cases()
        .into_iter()
        .find(|case| case.version == 3 && case.frame.counts == [0; 3])
        .unwrap();
    for version in [-1_i32, 6, i32::MAX] {
        let error = open(&version.to_le_bytes(), options(&case.frame)).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
    }
    let mut context = options(&case.frame);
    context.entry_codes = Some(Arc::from([]));
    assert_invalid(open(&3_i32.to_le_bytes(), context));
    context = options(&case.frame);
    context.group_counts = FlowPointerBodyCounts::try_from([1, 0, 0]).unwrap();
    for count in [-1_i32, i32::MAX] {
        let mut raw = 3_i32.to_le_bytes().to_vec();
        raw.push(2);
        raw.extend_from_slice(&count.to_le_bytes());
        assert_invalid(open(&raw, context.clone()));
    }
    assert_invalid(open(&[3, 0, 0, 0, 2, 0, 0, 0, 0], context));
}

#[test]
fn legacy_code_lengths_and_opaque_tagged_payloads_remain_explicit() {
    let case = cases()
        .into_iter()
        .find(|case| case.version == 0 && case.frame.counts[2] == 3)
        .unwrap();
    for codes in [
        None,
        Some(Arc::from([])),
        Some(Arc::from([3, 0])),
        Some(Arc::from([3, 0, 0, 0])),
    ] {
        let mut context = options(&case.frame);
        context.entry_codes = codes;
        assert_invalid(open(&case.frame.stored, context));
    }
    check_opaque_tagged_context();
}

fn check_opaque_tagged_context() {
    let tagged = cases()
        .into_iter()
        .find(|case| case.version == 5 && case.frame.counts[0] > 0)
        .unwrap();
    assert_eq!(
        read_case(&tagged)
            .entry(0, 0)
            .unwrap()
            .packed_payload_decoded(),
        Some(false)
    );
}

#[test]
fn record_identity_transport_checksum_and_stream_boundaries_are_checked() {
    let case = cases()
        .into_iter()
        .find(|case| case.frame.compressed && case.version == 3)
        .unwrap();
    let mut checksum = case.frame.stored.clone();
    let end = checksum.len() - 1;
    checksum[end] ^= 1;
    assert_invalid(open(&checksum, options(&case.frame)));
    let mut concatenated = case.frame.stored.clone();
    concatenated.extend_from_slice(&case.frame.stored);
    assert_invalid(open(&concatenated, options(&case.frame)));
    assert_invalid(open(&case.frame.stored[..end], options(&case.frame)));
    assert_invalid(open(&[0x1f, 0x8b, 8, 0], options(&case.frame)));
    let source = carrier(&case.frame.stored, 420, 2_000_000);
    let foreign = read_case(&case);
    assert_invalid(source.record_421_bodies(foreign.record(), options(&case.frame)));
    let id = source.latest(420).unwrap().unwrap().id;
    assert_invalid(source.record_421_bodies(&id, options(&case.frame)));
}
