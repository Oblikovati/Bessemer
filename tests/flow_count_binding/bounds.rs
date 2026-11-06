use super::*;
use bessemer::FailureClass;

fn read_prefix(raw: &[u8], options: FlowCountPrefixOptions) -> bessemer::Result<FlowContextCounts> {
    let source = prefix_carrier(raw);
    source.record_406_context_counts(&source.latest(406).unwrap().unwrap().id, options)
}

#[test]
fn exact_known_prefix_boundaries_and_opaque_body_extents_remain_distinct() {
    for case in fixture::cases() {
        let mut exact = prefix_options(&case);
        exact.maximum_decoded_bytes = case.decoded.len();
        assert!(read_prefix(&case.stored, exact).is_ok());
        let mut short = exact;
        short.maximum_decoded_bytes -= 1;
        assert!(read_prefix(&case.stored, short).is_err());
        if case.compressed {
            check_zlib_prefixes(&case);
        } else {
            check_raw_prefixes(&case);
        }
    }
}

fn check_raw_prefixes(case: &fixture::CountCase) {
    for end in 0..12 {
        assert!(read_prefix(&case.stored[..end], prefix_options(case)).is_err());
    }
    for end in 12..=case.stored.len() {
        let prefix = read_prefix(&case.stored[..end], prefix_options(case)).unwrap();
        assert_eq!(prefix.opaque_body(), &case.stored[12..end]);
        assert!(
            prefix
                .diagnostics()
                .contains(&FlowCountPrefixDiagnostic::OpaqueBodyNotValidated)
        );
    }
    let mut extra = case.stored.clone();
    extra.push(255);
    assert_eq!(
        read_prefix(&extra, prefix_options(case))
            .unwrap()
            .opaque_body(),
        &extra[12..]
    );
}

fn check_zlib_prefixes(case: &fixture::CountCase) {
    if case.name.starts_with("native") {
        return;
    }
    for end in 0..case.stored.len() {
        assert!(read_prefix(&case.stored[..end], prefix_options(case)).is_err());
    }
    let mut extra = case.stored.clone();
    extra.push(0);
    assert!(read_prefix(&extra, prefix_options(case)).is_err());
    let mut concatenated = case.stored.clone();
    concatenated.extend_from_slice(&case.stored);
    assert!(read_prefix(&concatenated, prefix_options(case)).is_err());
    let mut checksum = case.stored.clone();
    *checksum.last_mut().unwrap() ^= 1;
    assert!(read_prefix(&checksum, prefix_options(case)).is_err());
}

#[test]
fn invalid_versions_signed_counts_wrappers_and_options_fail_before_binding() {
    let options = FlowCountPrefixOptions {
        compression: FlowCountTransport::Raw,
        maximum_decoded_bytes: 100,
    };
    check_invalid_versions(options);
    for pair in [[-1, 0], [0, -1], [i32::MIN, i32::MAX]] {
        let raw: Vec<u8> = [14, pair[0], pair[1]]
            .into_iter()
            .flat_map(i32::to_le_bytes)
            .collect();
        assert_eq!(
            read_prefix(&raw, options).unwrap_err().class,
            FailureClass::Invalid
        );
    }
    check_invalid_options_and_wrappers(options);
}

#[test]
fn foreign_stale_and_wrong_identifiers_reject_without_changing_prefix_checkpoint() {
    let case = fixture::cases().remove(0);
    let counts = parse_prefix(&case);
    let foreign = prefix_carrier(&case.stored);
    assert!(
        foreign
            .record_406_context_counts(counts.record(), prefix_options(&case))
            .is_err()
    );
    let wrong = counts.source().latest(999).unwrap().unwrap().id;
    assert!(
        counts
            .source()
            .record_406_context_counts(&wrong, prefix_options(&case))
            .is_err()
    );
    check_stale_prefix(&counts, &case);
    assert_eq!(counts.decoded_bytes(), case.decoded);
}

#[test]
fn bound_array_limits_and_mismatched_counts_cannot_override_conditional_provenance() {
    let all = fixture::cases();
    let case = case_with_counts(&all, [3, 5]);
    let counts = parse_prefix(case);
    let binding = counts
        .initialize_absent_map(FlowAbsentMapInitialization::InitializeAbsentMap)
        .unwrap();
    let raw = &case.arrays[0];
    let source = array_carrier(&raw.stored);
    let id = source.latest(433).unwrap().unwrap().id;
    check_rejected_limits(&binding, &source, &id, raw);
    let other = case_with_counts(&all, [1, 0]);
    let other_binding = parse_prefix(other)
        .initialize_absent_map(FlowAbsentMapInitialization::InitializeAbsentMap)
        .unwrap();
    assert!(
        other_binding
            .read_record_433(&source, &id, array_limits())
            .is_err()
    );
}

#[test]
fn input_mutation_and_eight_workers_preserve_conditional_counts_and_opaque_body() {
    let mut case = fixture::cases().remove(0);
    let counts = parse_prefix(&case);
    let binding = counts
        .initialize_absent_map(FlowAbsentMapInitialization::InitializeAbsentMap)
        .unwrap();
    let source = array_carrier(&case.arrays[0].stored);
    case.stored.fill(255);
    case.decoded.fill(255);
    check_native_workers(&binding, &source);
    assert_eq!(binding.counts().opaque_body(), case.prefix.opaque);
}

fn check_invalid_versions(options: FlowCountPrefixOptions) {
    for version in [i32::MIN, -1, 0] {
        let raw: Vec<u8> = [version, 0, 0]
            .into_iter()
            .flat_map(i32::to_le_bytes)
            .collect();
        assert_eq!(
            read_prefix(&raw, options).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
}

fn check_invalid_options_and_wrappers(options: FlowCountPrefixOptions) {
    let invalid = FlowCountPrefixOptions {
        maximum_decoded_bytes: 11,
        ..options
    };
    assert!(read_prefix(&[0; 12], invalid).is_err());
    let zlib = FlowCountPrefixOptions {
        compression: FlowCountTransport::Zlib,
        ..options
    };
    for raw in [vec![], vec![0x1f, 0x8b, 8, 0]] {
        assert!(read_prefix(&raw, zlib).is_err());
    }
}

fn check_stale_prefix(counts: &FlowContextCounts, case: &fixture::CountCase) {
    let edit = counts
        .source()
        .replace_payload(counts.record(), b"changed")
        .unwrap();
    assert!(
        edit.result()
            .record_406_context_counts(counts.record(), prefix_options(case))
            .is_err()
    );
}

fn rejected_array_limits(raw: &fixture::ExpectedArrays) -> [FlowBoundArrayLimits; 4] {
    [
        FlowBoundArrayLimits {
            maximum_decoded_bytes: 7,
            maximum_elements: 100,
        },
        FlowBoundArrayLimits {
            maximum_decoded_bytes: 100,
            maximum_elements: 0,
        },
        FlowBoundArrayLimits {
            maximum_decoded_bytes: raw.decoded.len() - 1,
            maximum_elements: 8,
        },
        FlowBoundArrayLimits {
            maximum_decoded_bytes: 100,
            maximum_elements: 7,
        },
    ]
}

fn check_native_workers(binding: &FlowInitializedCountBinding, source: &FlowCarrier) {
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    read_native_worker(binding, source);
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

fn check_rejected_limits(
    binding: &FlowInitializedCountBinding,
    source: &FlowCarrier,
    id: &FlowRecordId,
    raw: &fixture::ExpectedArrays,
) {
    let exact = FlowBoundArrayLimits {
        maximum_decoded_bytes: raw.decoded.len(),
        maximum_elements: 8,
    };
    assert!(binding.read_record_433(source, id, exact).is_ok());
    for limits in rejected_array_limits(raw) {
        assert!(binding.read_record_433(source, id, limits).is_err());
    }
}

fn read_native_worker(binding: &FlowInitializedCountBinding, source: &FlowCarrier) {
    let id = source.latest(433).unwrap().unwrap().id;
    let result = binding
        .read_record_433(source, &id, array_limits())
        .unwrap();
    assert_eq!(result.arrays().second().len(), 5952);
    assert_eq!(result.binding().counts().opaque_body().len(), 352713);
    assert_eq!(
        result.binding().pointer_layout().solid_pointer_byte_offset,
        26944
    );
}

fn case_with_counts(cases: &[fixture::CountCase], counts: [i32; 2]) -> &fixture::CountCase {
    cases
        .iter()
        .find(|case| case.prefix.counts == counts && !case.compressed)
        .unwrap()
}
