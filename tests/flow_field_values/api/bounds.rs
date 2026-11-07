use super::*;

#[test]
fn every_small_complete_payload_prefix_and_trailing_body_byte_is_rejected() {
    for case in fixture::cases()
        .into_iter()
        .filter(|case| case.frame.decoded.len() <= 4096)
    {
        for end in 0..case.frame.stored.len() {
            assert_invalid(open(&case.frame.stored[..end], options(&case.frame)));
        }
        let mut raw = case.version.to_le_bytes().to_vec();
        raw.extend_from_slice(&case.frame.decoded);
        raw.push(0);
        let mut context = options(&case.frame);
        context.body_compression = FlowLegacyTableTransport::Raw;
        assert_invalid(open(&raw, context));
    }
}

fn set_limit(options: &mut FlowFieldValueOptions, axis: usize, value: usize) {
    match axis {
        0 => options.maximum_decoded_body_bytes = value,
        1 => options.maximum_items = value,
        2 => options.maximum_table_words = value,
        3 => options.maximum_wide_words = value,
        _ => panic!("axis {axis}; expected four serialized bounds"),
    }
}

#[test]
fn exact_body_and_metadata_limits_include_zero_body_and_zero_width_positions() {
    for case in fixture::cases() {
        let coverage = read_case(&case).coverage();
        let counts = [
            coverage.decoded_body_bytes,
            coverage.objects + coverage.regions,
            coverage.width_table_words,
            coverage.wide_words,
        ];
        for (axis, count) in counts.into_iter().enumerate() {
            check_limit(&case.frame, axis, count);
        }
    }
}

fn check_limit(frame: &FieldFrame, axis: usize, count: usize) {
    let mut context = options(frame);
    set_limit(
        &mut context,
        axis,
        if axis == 0 { count } else { count.max(1) },
    );
    assert!(open(&frame.stored, context.clone()).is_ok());
    if count > 0 {
        set_limit(&mut context, axis, count - 1);
        assert_invalid(open(&frame.stored, context.clone()));
    }
    if axis != 0 {
        set_limit(&mut context, axis, 0);
        assert_invalid(open(&frame.stored, context));
    }
}

#[test]
fn compatibility_gate_precedes_inflate_and_versions_require_their_typed_layouts() {
    let cases = fixture::cases();
    let legacy = cases.iter().find(|case| case.version == 5).unwrap();
    let regions = cases.iter().find(|case| case.version == 7).unwrap();
    let mut restricted = options(&regions.frame);
    restricted.compatibility_restriction = true;
    let mut raw = 8_i32.to_le_bytes().to_vec();
    raw.extend_from_slice(b"invalid-zlib");
    let error = open(&raw, restricted).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Unsupported);
    assert!(error.cause.contains("compatibility"));
    assert_invalid(open(&legacy.frame.stored, options(&regions.frame)));
    assert_invalid(open(&regions.frame.stored, options(&legacy.frame)));
    for version in [i32::MIN, -1, 0, 4] {
        assert_eq!(
            open(&version.to_le_bytes(), options(&legacy.frame))
                .unwrap_err()
                .class,
            bessemer::FailureClass::Unsupported
        );
    }
}

#[test]
fn mismatched_tables_region_counts_widths_and_active_products_fail_before_publication() {
    let cases = fixture::cases();
    let case = cases
        .iter()
        .find(|case| {
            case.version == 7 && case.frame.name.contains("-rich-") && !case.frame.compressed
        })
        .unwrap();
    for offset in [4, 8] {
        let mut raw = case.frame.stored.clone();
        raw[offset..offset + 4].copy_from_slice(&i32::MAX.to_le_bytes());
        assert_invalid(open(&raw, options(&case.frame)));
    }
    let mut options = options(&case.frame);
    options.layout = region_layout(&[i32::MAX as u32; 3], &[2, 0, 1], &[true; 3]);
    assert_invalid(open(&case.frame.stored, options));
    assert!(FlowLegacyFieldLayout::new(count(0), vec![], Some(vec![None])).is_err());
    check_huge_legacy_counts();
}

fn check_huge_legacy_counts() {
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.version == 5)
        .unwrap();
    for (table, widths) in [(i32::MAX as u32, vec![]), (0, vec![count(i32::MAX as u32)])] {
        let mut context = options(&case.frame);
        context.body_compression = FlowLegacyTableTransport::Raw;
        context.layout = FlowFieldLayout::Legacy(
            FlowLegacyFieldLayout::new(count(table), widths, None).unwrap(),
        );
        context.maximum_table_words = usize::MAX;
        context.maximum_wide_words = usize::MAX;
        assert_invalid(open(&5_i32.to_le_bytes(), context));
    }
}

#[test]
fn zlib_is_only_the_post_version_body_and_requires_checksum_and_single_member() {
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.version == 5 && case.frame.compressed)
        .unwrap();
    let mut checksum = case.frame.stored.clone();
    let last = checksum.len() - 1;
    checksum[last] ^= 1;
    assert_invalid(open(&checksum, options(&case.frame)));
    let mut concatenated = case.frame.stored.clone();
    concatenated.extend_from_slice(&case.frame.stored[4..]);
    assert_invalid(open(&concatenated, options(&case.frame)));
    let mut wrapped = 5_i32.to_le_bytes().to_vec();
    wrapped.extend_from_slice(&[0x1f, 0x8b, 8, 0]);
    assert_invalid(open(&wrapped, options(&case.frame)));
    let wrong = carrier(&case.frame.stored, 109, 2_000_000);
    let id = wrong.latest(109).unwrap().unwrap().id;
    assert_invalid(wrong.record_108_values(&id, options(&case.frame)));
    assert_invalid(wrong.record_108_values(read_case(&case).record(), options(&case.frame)));
}
