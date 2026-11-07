//! Pin conditional initialization arithmetic separately from opaque record-406 bodies.
#[path = "flow_count_binding/api.rs"]
mod api;
#[path = "flow_count_binding/fixture.rs"]
mod fixture;
use bessemer::flow::*;
use std::io::Read;

#[test]
fn all_193_prefixes_rebuild_exact_bytes_and_preserve_wrapped_totals_and_opaque_bodies() {
    for case in fixture::cases() {
        let prefix = &case.prefix;
        assert!(prefix.version >= 1 && prefix.counts.into_iter().all(|n| n >= 0));
        let mathematical: u64 = prefix.counts.into_iter().map(|n| n as u64).sum();
        assert_eq!(prefix.mathematical, mathematical);
        assert_eq!(prefix.native, mathematical as u32 as i32);
        assert_eq!(prefix.restored, prefix.version >= 18);
        let mut rebuilt = prefix.version.to_le_bytes().to_vec();
        rebuilt.extend(prefix.counts.into_iter().flat_map(i32::to_le_bytes));
        rebuilt.extend_from_slice(&prefix.opaque);
        assert_eq!(rebuilt, case.decoded, "{}", case.name);
        check_transport(case.compressed, &case.stored, &case.decoded);
        check_binding(&case);
        api::check_case(&case);
    }
}

fn check_binding(case: &fixture::CountCase) {
    assert_eq!(
        case.binding.is_some(),
        case.prefix.mathematical <= i32::MAX as u64
    );
    let Some(binding) = &case.binding else {
        assert!(case.arrays.is_empty());
        return;
    };
    assert_eq!(u64::from(binding.expected_count), case.prefix.mathematical);
    assert_eq!(binding.pointer_slot_bytes, 8);
    assert_eq!(binding.pointer_array_bytes, case.prefix.mathematical * 8);
    assert_eq!(binding.starts, [0, case.prefix.counts[0] as u32]);
    assert_eq!(binding.offsets, [0, case.prefix.counts[0] as u64 * 8]);
    assert_eq!(binding.flags, [true, true, false, false, false]);
    for arrays in &case.arrays {
        check_arrays(arrays, binding.expected_count);
    }
}

fn check_arrays(expected: &fixture::ExpectedArrays, bound_count: u32) {
    check_transport(true, &expected.stored, &expected.decoded);
    let source = array_carrier(&expected.stored);
    let id = source.latest(433).unwrap().unwrap().id;
    let options = array_options(bound_count);
    let actual = source.record_433_arrays(&id, options).unwrap();
    check_parsed_arrays(&actual, expected, bound_count);
    check_array_wire(expected);
    assert!(
        source
            .record_433_arrays(
                &id,
                FlowArrayOptions {
                    expected_count: Some(bound_count + 1),
                    ..options
                }
            )
            .is_err()
    );
}

fn array_options(count: u32) -> FlowArrayOptions {
    FlowArrayOptions {
        maximum_decoded_bytes: 100_000,
        maximum_elements: 10_000,
        expected_count: Some(count),
    }
}

fn check_array_wire(expected: &fixture::ExpectedArrays) {
    let mut raw = expected.selector.to_le_bytes().to_vec();
    raw.extend_from_slice(&expected.count.to_le_bytes());
    if let Some(first) = &expected.first {
        raw.extend(first.iter().flat_map(|n| n.to_le_bytes()));
    }
    raw.extend(expected.second.iter().flat_map(|n| n.to_le_bytes()));
    assert_eq!(raw, expected.decoded);
    assert_eq!(expected.first.is_some(), expected.selector == 0);
}

fn array_carrier(payload: &[u8]) -> FlowCarrier {
    let mut raw = 7_u16.to_le_bytes().to_vec();
    raw.extend_from_slice(&433_u32.to_le_bytes());
    raw.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    raw.extend_from_slice(payload);
    FlowCarrier::from_bytes(
        &raw,
        FlowLimits {
            maximum_wire_bytes: 100_000,
            maximum_records: 1,
            maximum_payload_bytes: 100_000,
        },
    )
    .unwrap()
}

fn check_transport(compressed: bool, stored: &[u8], decoded: &[u8]) {
    if !compressed {
        assert_eq!(stored, decoded);
        return;
    }
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(stored)
        .take(500_001)
        .read_to_end(&mut raw)
        .unwrap();
    assert_eq!(raw, decoded);
}

#[test]
fn native_cross_carrier_binding_and_huge_pointer_layouts_keep_runtime_exclusions() {
    let cases = fixture::cases();
    let native = &cases[0];
    assert_eq!(native.name, "native-cpt-cfld");
    assert_eq!(native.prefix.counts, [3368, 2584]);
    let bound = native.binding.as_ref().unwrap();
    assert_eq!((bound.expected_count, bound.offsets[1]), (5952, 26944));
    assert_eq!(bound.pointer_array_bytes, 47616);
    assert_eq!(native.arrays.len(), 1);
    assert_eq!(native.arrays[0].count, 5952);
    assert_eq!(native.prefix.opaque.len(), 352713);
    let huge = cases
        .iter()
        .find(|case| case.prefix.counts == [i32::MAX, 0])
        .unwrap();
    assert_eq!(
        huge.binding.as_ref().unwrap().pointer_array_bytes,
        17179869176
    );
    assert!(huge.arrays.is_empty());
}

fn check_parsed_arrays(
    actual: &FlowMapArrays,
    expected: &fixture::ExpectedArrays,
    bound_count: u32,
) {
    assert_eq!(actual.selector(), expected.selector);
    assert_eq!(actual.second().len(), expected.count as usize);
    assert_eq!(actual.first(), expected.first.as_deref());
    assert_eq!(actual.second(), expected.second);
    assert_eq!(actual.decoded_bytes(), expected.decoded);
    assert_eq!(
        actual.coverage().count_check,
        FlowArrayCountCheck::MatchedCallerCount(bound_count)
    );
    assert!(expected.conditional_count_matched);
}
