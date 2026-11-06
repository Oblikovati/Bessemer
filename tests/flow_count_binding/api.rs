use super::*;
#[path = "bounds.rs"]
mod bounds;

fn prefix_options(case: &fixture::CountCase) -> FlowCountPrefixOptions {
    FlowCountPrefixOptions {
        compression: if case.compressed {
            FlowCountTransport::Zlib
        } else {
            FlowCountTransport::Raw
        },
        maximum_decoded_bytes: 500_000,
    }
}

fn prefix_carrier(payload: &[u8]) -> FlowCarrier {
    let mut wire = 7_u16.to_le_bytes().to_vec();
    for (id, raw) in [
        (406_u32, &b"decoy"[..]),
        (406, payload),
        (999, &b"opaque\0"[..]),
    ] {
        wire.extend_from_slice(&id.to_le_bytes());
        wire.extend_from_slice(&(raw.len() as u64).to_le_bytes());
        wire.extend_from_slice(raw);
    }
    FlowCarrier::from_bytes(
        &wire,
        FlowLimits {
            maximum_wire_bytes: 500_100,
            maximum_payload_bytes: 500_000,
            maximum_records: 3,
        },
    )
    .unwrap()
}

fn parse_prefix(case: &fixture::CountCase) -> FlowContextCounts {
    let source = prefix_carrier(&case.stored);
    source
        .record_406_context_counts(
            &source.latest(406).unwrap().unwrap().id,
            prefix_options(case),
        )
        .unwrap()
}

pub fn check_case(case: &fixture::CountCase) {
    let counts = parse_prefix(case);
    check_context_fields(&counts, case);
    assert_eq!(counts.record().ordinal(), 1);
    assert_eq!(counts.options().maximum_decoded_bytes, 500_000);
    check_prefix_coverage(&counts, case);
    let derived = counts.initialize_absent_map(FlowAbsentMapInitialization::InitializeAbsentMap);
    let Some(expected) = &case.binding else {
        assert!(derived.is_err());
        return;
    };
    let binding = derived.unwrap();
    check_derived_binding(&binding, expected, &counts);
    for arrays in &case.arrays {
        check_bound_arrays(&binding, arrays);
    }
}

fn check_prefix_coverage(counts: &FlowContextCounts, case: &fixture::CountCase) {
    let coverage = counts.coverage();
    assert_eq!(
        coverage.mode,
        if case.compressed {
            FlowCountPrefixCheckMode::ExactZlibPrefixOpaqueBody
        } else {
            FlowCountPrefixCheckMode::ExactRawPrefixOpaqueBody
        }
    );
    assert_eq!(coverage.stored_bytes, case.stored.len());
    assert_eq!(coverage.decoded_bytes, case.decoded.len());
    assert_eq!(coverage.known_prefix_bytes, 12);
    assert_eq!(coverage.opaque_body_bytes, case.decoded.len() - 12);
    assert_eq!(coverage.achieved_prefix_error, 0);
    check_prefix_diagnostics(counts);
}

fn check_derived_binding(
    actual: &FlowInitializedCountBinding,
    expected: &fixture::ExpectedBinding,
    counts: &FlowContextCounts,
) {
    assert_eq!(actual.counts().record(), counts.record());
    assert_eq!(
        actual.counts().source().as_bytes(),
        counts.source().as_bytes()
    );
    assert_eq!(actual.expected_count(), expected.expected_count);
    assert_eq!(
        actual.assumption(),
        FlowAbsentMapInitialization::InitializeAbsentMap
    );
    check_pointer_layout(actual.pointer_layout(), expected);
    check_binding_flags(actual, expected);
}

fn array_limits() -> FlowBoundArrayLimits {
    FlowBoundArrayLimits {
        maximum_decoded_bytes: 100_000,
        maximum_elements: 10_000,
    }
}

fn check_bound_arrays(binding: &FlowInitializedCountBinding, expected: &fixture::ExpectedArrays) {
    let source = array_carrier(&expected.stored);
    let id = source.latest(433).unwrap().unwrap().id;
    let result = binding
        .read_record_433(&source, &id, array_limits())
        .unwrap();
    check_parsed_arrays(result.arrays(), expected, binding.expected_count());
    assert_eq!(
        result.binding().counts().record(),
        binding.counts().record()
    );
    assert_eq!(result.arrays().record(), &id);
    assert_ne!(result.arrays().record(), result.binding().counts().record());
    check_rebound_edit(&result);
}

fn check_rebound_edit(bound: &FlowBoundMapArrays) {
    let Some(value) = bound.arrays().second().first() else {
        return;
    };
    let edit = bound
        .arrays()
        .replace_value(FlowArrayGroup::Second, 0, value ^ 1)
        .unwrap();
    let result = reopen_bound(bound.binding(), edit.result());
    assert_eq!(result.arrays().second()[0], value ^ 1);
    assert_eq!(
        result.binding().counts().record(),
        bound.binding().counts().record()
    );
    assert_eq!(bound.arrays().second()[0], *value);
}

#[test]
fn all_public_count_binding_types_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<FlowCountTransport>();
    assert_traits::<FlowCountPrefixOptions>();
    assert_traits::<FlowCountBodyBranch>();
    assert_traits::<FlowCountPrefixCheckMode>();
    assert_traits::<FlowCountPrefixCoverage>();
    assert_traits::<FlowCountPrefixDiagnostic>();
    assert_traits::<FlowContextCounts>();
    assert_traits::<FlowAbsentMapInitialization>();
    assert_traits::<FlowInitializedPointerLayout>();
    assert_traits::<FlowInitializedCountBinding>();
    assert_traits::<FlowBoundArrayLimits>();
    assert_traits::<FlowBoundMapArrays>();
}

fn check_context_fields(counts: &FlowContextCounts, case: &fixture::CountCase) {
    assert_eq!(counts.decoded_bytes(), case.decoded);
    assert_eq!(counts.opaque_body(), case.prefix.opaque);
    assert_eq!(
        [counts.version(), counts.g_fluid(), counts.g_solid()],
        [
            case.prefix.version,
            case.prefix.counts[0],
            case.prefix.counts[1]
        ]
    );
    assert_eq!(counts.mathematical_total(), case.prefix.mathematical);
    assert_eq!(counts.native_total(), case.prefix.native);
    assert_eq!(
        counts.body_branch(),
        if case.prefix.restored {
            FlowCountBodyBranch::Restored18Plus
        } else {
            FlowCountBodyBranch::Legacy1To17
        }
    );
}

fn check_prefix_diagnostics(counts: &FlowContextCounts) {
    assert_eq!(
        counts.diagnostics(),
        &[
            FlowCountPrefixDiagnostic::OpaqueBodyNotValidated,
            FlowCountPrefixDiagnostic::Record433ContextNotProven,
            FlowCountPrefixDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowCountPrefixDiagnostic::NativeApplicationNotChecked
        ]
    );
}

fn check_pointer_layout(layout: FlowInitializedPointerLayout, expected: &fixture::ExpectedBinding) {
    assert_eq!(
        u32::from(layout.pointer_slot_bytes),
        expected.pointer_slot_bytes
    );
    assert_eq!(layout.pointer_array_bytes, expected.pointer_array_bytes);
    assert_eq!(
        [layout.fluid_start_index, layout.solid_start_index],
        expected.starts
    );
    assert_eq!(
        [
            layout.fluid_pointer_byte_offset,
            layout.solid_pointer_byte_offset
        ],
        expected.offsets
    );
}

fn check_binding_flags(actual: &FlowInitializedCountBinding, expected: &fixture::ExpectedBinding) {
    assert_eq!(
        [
            true,
            actual.count_assignment_proven_under_assumption(),
            actual.native_context_executed(),
            actual.pointer_population_reproduced(),
            actual.existing_map_context_supported()
        ],
        expected.flags
    );
}

fn reopen_bound(
    binding: &FlowInitializedCountBinding,
    arrays: &FlowMapArrays,
) -> FlowBoundMapArrays {
    binding
        .read_record_433(arrays.source(), arrays.record(), array_limits())
        .unwrap()
}
