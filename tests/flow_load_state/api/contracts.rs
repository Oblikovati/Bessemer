use super::*;

pub(super) fn compact_cases() -> Vec<LoadCase> {
    fixture::cases()
        .into_iter()
        .filter(|case| case.name == "shape-m0-n1-w1" || case.name == "shape-m1-n1-w1")
        .collect()
}
#[test]
fn complete_load_state_rejects_all_compact_prefixes_extensions_negative_counts_and_wrong_versions()
{
    let cases = compact_cases();
    assert_eq!(cases.len(), 2);
    for case in cases {
        check_malformed(&case);
    }
}
fn check_malformed(case: &LoadCase) {
    for end in 0..case.raw.len() {
        assert!(
            open(&case.raw[..end], options(case.load_mode)).is_err(),
            "{}:{end}",
            case.name
        );
    }
    let mut extra = case.raw.clone();
    extra.push(0);
    assert!(open(&extra, options(case.load_mode)).is_err());
    check_bad_counts(case);
    for version in [i32::MIN, -1, 0, 17, 19, i32::MAX] {
        let mut raw = case.raw.clone();
        raw[..4].copy_from_slice(&version.to_le_bytes());
        assert_eq!(
            open(&raw, options(case.load_mode)).unwrap_err().class,
            bessemer::FailureClass::Unsupported
        );
    }
}

fn check_bad_counts(case: &LoadCase) {
    for &offset in &case.counts {
        for count in [-1i32, i32::MAX] {
            let mut raw = case.raw.clone();
            raw[offset as usize..offset as usize + 4].copy_from_slice(&count.to_le_bytes());
            assert!(
                open(&raw, options(case.load_mode)).is_err(),
                "{}:{offset}",
                case.name
            );
        }
    }
}
#[test]
fn all_load_state_resource_axes_accept_exact_counts_and_reject_one_below() {
    for case in compact_cases() {
        let state = open(&case.raw, options(case.load_mode)).unwrap();
        let exact = exact_options(state.coverage());
        assert!(open(&case.raw, exact).is_ok());
        for axis in 0..7 {
            if limit(exact, axis) > 0 {
                assert!(
                    open(&case.raw, reduced(exact, axis)).is_err(),
                    "{}:{axis}",
                    case.name
                );
            }
        }
    }
}
fn exact_options(coverage: FlowLoadCoverage) -> FlowLoadStateOptions {
    FlowLoadStateOptions {
        mode: coverage.load_mode,
        maximum_payload_bytes: coverage.payload_bytes,
        maximum_items: coverage.items,
        maximum_text_units: coverage.text_units,
        maximum_array_values: coverage.array_values,
        maximum_values: coverage.values,
        maximum_fields: coverage.fields,
        maximum_scopes: coverage.scopes,
    }
}
fn limit(options: FlowLoadStateOptions, axis: usize) -> usize {
    [
        options.maximum_payload_bytes,
        options.maximum_items,
        options.maximum_text_units,
        options.maximum_array_values,
        options.maximum_values,
        options.maximum_fields,
        options.maximum_scopes,
    ][axis]
}
fn reduced(mut options: FlowLoadStateOptions, axis: usize) -> FlowLoadStateOptions {
    match axis {
        0 => options.maximum_payload_bytes -= 1,
        1 => options.maximum_items -= 1,
        2 => options.maximum_text_units -= 1,
        3 => options.maximum_array_values -= 1,
        4 => options.maximum_values -= 1,
        5 => options.maximum_fields -= 1,
        6 => options.maximum_scopes -= 1,
        _ => panic!("Expected resource axis0..7"),
    }
    options
}
#[test]
fn mode_context_ids_are_distinct_even_when_empty_bytes_match_and_native_wrong_mode_is_incomplete() {
    let cases = fixture::cases();
    let case = cases.iter().find(|case| case.count == 0).unwrap();
    let source = carrier(&case.raw);
    let id = source.latest(11).unwrap().unwrap().id;
    let first = source.record_11_load_state(&id, options(false)).unwrap();
    let second = source.record_11_load_state(&id, options(true)).unwrap();
    assert!(second.field(&first.field_at(2).unwrap().id()).is_err());
    assert_eq!(first.source().as_bytes(), second.source().as_bytes());
    assert!(open(&cases[0].raw, options(true)).is_err());
    assert!(open(&cases[1].raw, options(false)).is_err());
    for offset in [0, 1, 4, 9, 32, u32::MAX] {
        assert!(FlowLoadMemberOffset::try_from(offset).is_err());
    }
}
#[test]
fn load_state_edits_reject_structural_fields_mismatched_values_and_stale_foreign_ids() {
    for case in compact_cases() {
        let state = open(&case.raw, options(case.load_mode)).unwrap();
        for field in state.fields().filter(|field| !field.is_editable()) {
            assert!(
                state
                    .replace_field(&field.id(), 0, field.value(0).unwrap())
                    .is_err()
            );
        }
        check_invalid_edits(&state, &case);
    }
}
fn check_invalid_edits(state: &FlowLoadState, case: &LoadCase) {
    let field = member_field(state);
    check_bad_scalars(state, field);
    check_bad_text(state);
    let next = FlowLoadValue::Scalar(FlowPrimaryValue::Signed32(42));
    let edit = state.replace_field(&field.id(), 0, next).unwrap();
    assert!(edit.result().field(&field.id()).is_err());
    let foreign = open(&case.raw, options(case.load_mode)).unwrap();
    assert!(foreign.field(&field.id()).is_err());
    let wrong = state.source().latest(999).unwrap().unwrap().id;
    assert!(
        state
            .source()
            .record_11_load_state(&wrong, options(case.load_mode))
            .is_err()
    );
    assert_eq!(state.decoded_bytes(), case.raw);
}

fn check_bad_scalars(state: &FlowLoadState, field: FlowLoadField<'_>) {
    let signed = FlowLoadValue::Scalar(FlowPrimaryValue::Signed32(4));
    assert!(
        state
            .replace_field(&field.id(), usize::MAX, signed)
            .is_err()
    );
    assert!(
        state
            .replace_field(&field.id(), 0, FlowLoadValue::Utf16Unit(5))
            .is_err()
    );
    let byte = FlowLoadValue::Scalar(FlowPrimaryValue::Byte(5));
    assert!(state.replace_field(&field.id(), 0, byte).is_err());
}

fn check_bad_text(state: &FlowLoadState) {
    let text = state
        .texts()
        .find(|text| text.units().layout().element_count > 0)
        .unwrap();
    let word = FlowLoadValue::Scalar(FlowPrimaryValue::WordBits(7));
    assert!(state.replace_field(&text.units().id(), 0, word).is_err());
}
#[test]
fn minimal_empty_text_items_and_unused_maximum_widths_do_not_spend_absent_array_budgets() {
    for mode in [false, true] {
        let mut raw = vec![0; 112 + if mode { 0 } else { 192 }];
        raw[..4].copy_from_slice(&18i32.to_le_bytes());
        raw[4..8].copy_from_slice(&1i32.to_le_bytes());
        let state = open(&raw, options(mode)).unwrap();
        assert_eq!(state.texts().len(), 3);
        assert!(state.texts().all(|text| text.raw_units_bytes().is_empty()));
    }
    let cases = fixture::cases();
    let case = cases
        .iter()
        .find(|case| case.count == 0 && case.width == i32::MAX as u32)
        .unwrap();
    let mut bounded = options(false);
    bounded.maximum_items = 0;
    bounded.maximum_array_values = 0;
    bounded.maximum_text_units = 0;
    assert!(open(&case.raw, bounded).is_ok());
    bounded.maximum_payload_bytes = 15;
    assert!(open(&case.raw, bounded).is_err());
}
