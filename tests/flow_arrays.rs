//! Frozen native/owned signed arrays, strict zlib boundaries and isolated typed cell edits.
#[path = "flow_arrays/census.rs"]
mod census;
#[path = "flow_arrays/fixture.rs"]
mod fixture;
use bessemer::{FailureClass, flow::*};

fn undersized_options(exact: FlowArrayOptions) -> [FlowArrayOptions; 3] {
    [
        FlowArrayOptions {
            maximum_decoded_bytes: exact.maximum_decoded_bytes - 1,
            ..exact
        },
        FlowArrayOptions {
            maximum_elements: exact.maximum_elements - 1,
            ..exact
        },
        FlowArrayOptions {
            expected_count: Some(u32::MAX),
            ..exact
        },
    ]
}

fn assert_group_cells(arrays: &FlowMapArrays, group: FlowArrayGroup) {
    let Some(values) = group_values(arrays, group) else {
        assert!(arrays.replace_value(group, 0, 0).is_err());
        return;
    };
    if values.is_empty() {
        return;
    }
    for index in [0, values.len() - 1] {
        assert_cell_edit(arrays, group, index, values[index]);
        assert_cell_edit(arrays, group, index, values[index].wrapping_add(0x1020304));
    }
}

fn untouched_segments(arrays: &FlowMapArrays) -> (&[u8], &[u8]) {
    let record = arrays
        .source()
        .records()
        .nth(arrays.record().ordinal())
        .unwrap();
    let wire = arrays.source().as_bytes();
    let prefix = &wire[..record.header_offset + 4];
    let suffix = &wire[record.header_offset + 12 + record.payload.len()..];
    (prefix, suffix)
}

#[test]
fn caller_mutations_and_failed_compressed_growth_preserve_original_snapshots() {
    let mut case = fixture::cases().remove(0);
    let arrays = fixture::view(&case);
    let wire = case.raw.clone();
    let decoded = case.decoded.clone();
    assert_failed_growth();
    case.raw.fill(0);
    case.decoded.fill(0);
    case.first.as_mut().unwrap().fill(0);
    case.second.fill(0);
    assert_eq!(arrays.source().as_bytes(), wire);
    assert_eq!(arrays.decoded_bytes(), decoded);
}

fn assert_failed_growth() {
    let carrier = zero_array_carrier();
    let id = carrier.records().next().unwrap().id;
    let arrays = carrier.record_433_arrays(&id, fixture::options()).unwrap();
    let growing = arrays
        .replace_value(FlowArrayGroup::Second, 0, 0x11223344)
        .unwrap();
    assert!(growing.result().source().as_bytes().len() > carrier.as_bytes().len());
    let raw = carrier.as_bytes();
    let limits = FlowLimits {
        maximum_wire_bytes: raw.len(),
        maximum_records: 100,
        maximum_payload_bytes: raw.len(),
    };
    let bounded = FlowCarrier::from_bytes(raw, limits).unwrap();
    assert_bounded_growth(&bounded);
    assert_eq!(carrier.as_bytes(), raw);
}

fn zero_array_carrier() -> FlowCarrier {
    let mut decoded = [1_i32.to_le_bytes(), 1024_i32.to_le_bytes()].concat();
    decoded.resize(8 + 1024 * 4, 0);
    fixture::payload_carrier(433, &fixture::encoded(&decoded))
}

fn assert_bounded_growth(carrier: &FlowCarrier) {
    let original = carrier.as_bytes().to_vec();
    let id = carrier.records().next().unwrap().id;
    let arrays = carrier.record_433_arrays(&id, fixture::options()).unwrap();
    assert!(
        arrays
            .replace_value(FlowArrayGroup::Second, 0, 0x11223344)
            .is_err()
    );
    assert_eq!(arrays.source().as_bytes(), original);
}

#[test]
fn every_owned_and_native_array_matches_independent_complete_expected_values() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 13);
    for case in cases {
        let view = fixture::view(&case);
        assert_eq!(view.selector(), case.selector, "{}", case.name);
        assert_eq!(view.first(), case.first.as_deref());
        assert_eq!(view.second(), case.second);
        assert_eq!(view.decoded_bytes(), case.decoded);
        assert_eq!(view.source().as_bytes(), case.raw);
        assert_eq!(view.record().ordinal(), case.ordinal);
        assert_coverage(&view, &case);
    }
}

fn assert_coverage(view: &FlowMapArrays, case: &fixture::ArrayCase) {
    let coverage = view.coverage();
    assert_eq!(coverage.mode, FlowArrayCheckMode::ExactZlibAndArrayFrames);
    assert_eq!(coverage.decoded_bytes, case.decoded.len());
    assert_eq!(coverage.elements_per_array, case.second.len());
    assert_eq!(
        coverage.present_arrays,
        if case.first.is_some() { 2 } else { 1 }
    );
    assert_eq!(coverage.count_check, FlowArrayCountCheck::NotSupplied);
    assert_eq!(coverage.achieved_byte_error, 0);
    let record = view.source().records().nth(case.ordinal).unwrap();
    assert_eq!(coverage.compressed_bytes, record.payload.len());
    assert_eq!(view.diagnostics().len(), 2);
}

#[test]
fn supplied_counts_and_exact_budgets_are_checked_without_inventing_owner_state() {
    for case in fixture::cases() {
        let exact = FlowArrayOptions {
            maximum_decoded_bytes: case.decoded.len(),
            maximum_elements: case.second.len().max(1),
            expected_count: Some(case.second.len() as u32),
        };
        let source = fixture::carrier(&case.raw);
        let id = source.records().nth(case.ordinal).unwrap().id;
        let checked = source.record_433_arrays(&id, exact).unwrap();
        assert_eq!(
            checked.coverage().count_check,
            FlowArrayCountCheck::MatchedCallerCount(case.second.len() as u32)
        );
        assert_budget_failures(&source, &id, exact);
        assert_eq!(source.as_bytes(), case.raw);
    }
}

fn assert_budget_failures(source: &FlowCarrier, id: &FlowRecordId, exact: FlowArrayOptions) {
    let wrong = FlowArrayOptions {
        expected_count: exact.expected_count.map(|count| count + 1),
        ..exact
    };
    assert!(source.record_433_arrays(id, wrong).is_err());
    for options in undersized_options(exact) {
        assert!(source.record_433_arrays(id, options).is_err());
    }
}

#[test]
fn every_small_decoded_and_compressed_truncation_and_extra_byte_fails() {
    let case = fixture::cases().remove(3);
    for end in 0..case.decoded.len() {
        let payload = fixture::encoded(&case.decoded[..end]);
        assert!(fixture::decode_payload(&payload, fixture::options()).is_err());
    }
    let payload = fixture::encoded(&case.decoded);
    for end in 0..payload.len() {
        assert!(fixture::decode_payload(&payload[..end], fixture::options()).is_err());
    }
    let mut decoded = case.decoded.clone();
    decoded.push(0);
    assert!(fixture::decode_payload(&fixture::encoded(&decoded), fixture::options()).is_err());
    assert_compression_failures(&payload);
}

fn assert_compression_failures(payload: &[u8]) {
    let mut corrupt = payload.to_vec();
    let end = corrupt.len() - 1;
    corrupt[end] ^= 1;
    let trailing = [payload, &[0]].concat();
    let concatenated = [payload, payload].concat();
    for bytes in [
        &corrupt[..],
        &trailing,
        &concatenated,
        &payload[2..payload.len() - 4],
        b"\x1f\x8b",
        include_bytes!("fixtures/flow-arrays/wrong-wrapper.gz").as_slice(),
    ] {
        assert_eq!(
            fixture::decode_payload(bytes, fixture::options())
                .unwrap_err()
                .class,
            FailureClass::Integrity
        );
    }
}

#[test]
fn negative_and_impossible_shapes_wrong_records_and_foreign_ids_fail() {
    for (selector, count) in [(-1_i32, 0_i32), (0, -1), (0, i32::MAX), (1, 1)] {
        let decoded = [selector.to_le_bytes(), count.to_le_bytes()].concat();
        assert!(fixture::decode_payload(&fixture::encoded(&decoded), fixture::options()).is_err());
    }
    let payload = fixture::encoded(&[0; 8]);
    let wrong = fixture::payload_carrier(434, &payload);
    let id = wrong.records().next().unwrap().id;
    assert_eq!(
        wrong
            .record_433_arrays(&id, fixture::options())
            .unwrap_err()
            .class,
        FailureClass::Invalid
    );
    let source = fixture::payload_carrier(433, &payload);
    assert!(source.record_433_arrays(&id, fixture::options()).is_err());
}

#[test]
fn typed_cells_preserve_all_other_decoded_and_carrier_values_and_retain_checkpoints() {
    for case in fixture::cases() {
        let arrays = fixture::view(&case);
        for group in [FlowArrayGroup::First, FlowArrayGroup::Second] {
            assert_group_cells(&arrays, group);
        }
        assert!(
            arrays
                .replace_value(FlowArrayGroup::Second, arrays.second().len(), 0)
                .is_err()
        );
    }
}

fn group_values(arrays: &FlowMapArrays, group: FlowArrayGroup) -> Option<&[i32]> {
    match group {
        FlowArrayGroup::First => arrays.first(),
        FlowArrayGroup::Second => Some(arrays.second()),
    }
}

#[test]
fn duplicate_record_ids_keep_exact_occurrence_reads_and_edits() {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    for value in [19_i32, 23] {
        let decoded = [
            1_i32.to_le_bytes(),
            1_i32.to_le_bytes(),
            value.to_le_bytes(),
        ]
        .concat();
        let single = fixture::payload_carrier(433, &fixture::encoded(&decoded));
        wire.extend_from_slice(&single.as_bytes()[2..]);
    }
    let source = fixture::carrier(&wire);
    for (record, expected) in source.records().zip([19, 23]) {
        let arrays = source
            .record_433_arrays(&record.id, fixture::options())
            .unwrap();
        assert_eq!(arrays.second(), &[expected]);
        assert_cell_edit(&arrays, FlowArrayGroup::Second, 0, -19);
    }
}

fn assert_cell_edit(arrays: &FlowMapArrays, group: FlowArrayGroup, index: usize, value: i32) {
    let before = group_values(arrays, group).unwrap()[index];
    let edit = arrays.replace_value(group, index, value).unwrap();
    let result = edit.result();
    assert_eq!(group_values(result, group).unwrap()[index], value);
    assert_cell_history(&edit, arrays, group, index, before, value);
    let mut expected = arrays.decoded_bytes().to_vec();
    let skip = if group == FlowArrayGroup::Second {
        arrays.first().map_or(0, <[i32]>::len)
    } else {
        0
    };
    let offset = 8 + 4 * (skip + index);
    expected[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    assert_eq!(result.decoded_bytes(), expected);
    assert_untouched_carrier(arrays, result);
    let reversed = result.replace_value(group, index, before).unwrap();
    assert_eq!(reversed.result().decoded_bytes(), arrays.decoded_bytes());
    if before == value {
        assert_eq!(result.source().as_bytes(), arrays.source().as_bytes());
    }
}

fn assert_cell_history(
    edit: &FlowArrayEdit,
    arrays: &FlowMapArrays,
    group: FlowArrayGroup,
    index: usize,
    before: i32,
    value: i32,
) {
    let history = edit.history();
    assert_eq!(edit.diagnostics().len(), 1);
    assert_eq!(&history.source_record, arrays.record());
    assert_eq!(&history.result_record, edit.result().record());
    assert_eq!(history.group, group);
    assert_eq!(history.index, index);
    assert_eq!(history.before, before);
    assert_eq!(history.after, value);
    assert_eq!(
        history.relation,
        if before == value {
            FlowPayloadRelation::Copied
        } else {
            FlowPayloadRelation::Derived
        }
    );
    assert_source_checkpoint(edit, arrays);
}

fn assert_source_checkpoint(edit: &FlowArrayEdit, arrays: &FlowMapArrays) {
    assert_eq!(edit.source().decoded_bytes(), arrays.decoded_bytes());
    assert_eq!(
        edit.carrier_edit().source().as_bytes(),
        arrays.source().as_bytes()
    );
}

fn assert_untouched_carrier(source: &FlowMapArrays, result: &FlowMapArrays) {
    assert_eq!(untouched_segments(source), untouched_segments(result));
}

#[test]
fn all_public_array_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FlowMapArrays>();
    assert_send_sync::<FlowArrayEdit>();
    assert_send_sync::<FlowArrayCoverage>();
    assert_send_sync::<FlowArrayDiagnostic>();
    assert_send_sync::<FlowArrayEditDiagnostic>();
}

#[test]
fn immutable_arrays_and_mixed_worker_edits_are_isolated() {
    let case = fixture::cases().remove(0);
    let arrays = fixture::view(&case);
    let expected = arrays.replace_value(FlowArrayGroup::Second, 0, 19).unwrap();
    std::thread::scope(|scope| {
        let joins: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| worker_edit(&arrays)))
            .collect();
        for join in joins {
            assert_eq!(
                join.join().unwrap().result().source().as_bytes(),
                expected.result().source().as_bytes()
            );
        }
    });
    assert_eq!(arrays.source().as_bytes(), case.raw);
}

fn worker_edit(arrays: &FlowMapArrays) -> FlowArrayEdit {
    assert!(
        arrays
            .replace_value(FlowArrayGroup::Second, arrays.second().len(), 19)
            .is_err()
    );
    arrays.replace_value(FlowArrayGroup::Second, 0, 19).unwrap()
}
