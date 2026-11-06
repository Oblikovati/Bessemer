use super::*;

#[test]
fn all_load_state_fields_scopes_raw_units_prefixes_and_coverage_match_the_complete_source_corpus() {
    for case in fixture::cases() {
        check_case(&case);
    }
}

pub(super) fn check_case(case: &LoadCase) -> FlowLoadState {
    let state = open(&case.raw, options(case.load_mode)).unwrap();
    assert_eq!(state.version(), case.version);
    assert_eq!(state.mode().value(), case.load_mode);
    assert_eq!(state.decoded_bytes(), case.raw);
    assert_eq!(state.options(), options(case.load_mode));
    check_fields(&state, case);
    check_scopes(&state, case);
    check_texts(&state, case);
    check_coverage(&state, case);
    state
}
fn check_fields(state: &FlowLoadState, case: &LoadCase) {
    assert_eq!(state.fields().len(), case.fields.len(), "{}", case.name);
    for (field, expected) in state.fields().zip(&case.fields) {
        assert_eq!(field_path(field), expected.path, "{}", case.name);
        assert_eq!(field.layout().first_byte, expected.offset as usize);
        assert_eq!(field.layout().element_count, expected.count as usize);
        assert_eq!(field.layout().stride_bytes, expected.width());
        assert_eq!(field.encoding().width(), expected.width());
        assert_eq!(state.field(&field.id()).unwrap().layout(), field.layout());
        assert_eq!(field.id().record(), state.record());
        assert_eq!(field.id().mode(), state.mode());
        assert_eq!(
            state.field_at(field.id().ordinal()).unwrap().kind(),
            field.kind()
        );
        check_values(field, expected);
        check_members(field, expected);
    }
    assert!(state.field_at(case.fields.len()).is_none());
}
fn check_values(field: FlowLoadField<'_>, expected: &LoadField) {
    for (index, raw) in expected.raw.chunks_exact(expected.width()).enumerate() {
        assert_eq!(field.element_bytes(index).unwrap(), raw);
        assert_eq!(field.value(index), Some(typed(expected.encoding, raw)));
    }
    assert!(field.value(expected.count as usize).is_none());
    assert!(field.element_bytes(usize::MAX).is_none());
}
pub(super) fn typed(encoding: u8, raw: &[u8]) -> FlowLoadValue {
    match encoding {
        1 => FlowLoadValue::Scalar(FlowPrimaryValue::Signed32(i32::from_le_bytes(
            raw.try_into().unwrap(),
        ))),
        2 => FlowLoadValue::Scalar(FlowPrimaryValue::WordBits(u32::from_le_bytes(
            raw.try_into().unwrap(),
        ))),
        3 => FlowLoadValue::Scalar(FlowPrimaryValue::WideBits(u64::from_le_bytes(
            raw.try_into().unwrap(),
        ))),
        4 => FlowLoadValue::Utf16Unit(u16::from_le_bytes(raw.try_into().unwrap())),
        other => panic!("Expected field encoding1..4, got {other}"),
    }
}
fn check_members(field: FlowLoadField<'_>, expected: &LoadField) {
    for (slot, offset) in [8, 24, 12, 16, 20, 28, 40, 36, 44].into_iter().enumerate() {
        let member = FlowLoadMemberOffset::try_from(offset).unwrap();
        assert_eq!(member.value(), offset);
        assert_eq!(member.slot(), slot);
        let expected = if field.kind() == FlowLoadFieldKind::MemberWords {
            Some(i32::from_le_bytes(
                expected.raw[slot * 4..slot * 4 + 4].try_into().unwrap(),
            ))
        } else {
            None
        };
        assert_eq!(field.native_member(member), expected);
    }
}
fn check_scopes(state: &FlowLoadState, case: &LoadCase) {
    assert_eq!(state.scopes().len(), case.scopes.len());
    for (scope, expected) in state.scopes().zip(&case.scopes) {
        assert_eq!(scope_path(scope.scope), expected.path);
        assert_eq!(
            scope.payload_interval,
            (expected.start as usize, expected.end as usize)
        );
    }
}
fn check_texts(state: &FlowLoadState, case: &LoadCase) {
    assert_eq!(state.texts().len(), case.texts.len());
    for (index, (text, expected)) in state.texts().zip(&case.texts).enumerate() {
        assert_eq!(scope_path(text.scope()), expected.path);
        assert_eq!(
            text.payload_interval(),
            (expected.start as usize, expected.end as usize)
        );
        assert_eq!(text.units().layout().element_count, expected.units as usize);
        assert_eq!(
            text.raw_units_bytes(),
            &case.raw[expected.start as usize + 4..expected.end as usize]
        );
        assert_eq!(text.native_prefix_units(), expected.prefix_units as usize);
        assert_eq!(text.native_prefix_bytes(), expected.prefix_raw);
        assert_eq!(
            state.text_at(index).unwrap().units().id(),
            text.units().id()
        );
    }
    assert!(state.text_at(case.texts.len()).is_none());
}
fn check_coverage(state: &FlowLoadState, case: &LoadCase) {
    let covered = state.coverage();
    assert_eq!(covered.load_mode, state.mode());
    assert_eq!(covered.payload_bytes, case.raw.len());
    assert_eq!(covered.items, case.count as usize);
    assert_eq!(covered.array_width, case.width as usize);
    assert_eq!(
        covered.text_units,
        case.texts.iter().map(|text| text.units as usize).sum()
    );
    assert_eq!(
        covered.values,
        case.fields.iter().map(|field| field.count as usize).sum()
    );
    assert_eq!(covered.fields, case.fields.len());
    assert_eq!(covered.scopes, case.scopes.len());
    assert_eq!(covered.texts, case.texts.len());
    assert_eq!(covered.mode_tail_present, case.mode_tail.is_some());
    assert_eq!(covered.achieved_error, 0);
    check_checkpoint(state);
    check_arrays_and_counts(state, case);
}
fn check_arrays_and_counts(state: &FlowLoadState, case: &LoadCase) {
    let counts: Vec<_> = state
        .fields()
        .filter(|field| field.kind().is_structural() && field.kind() != FlowLoadFieldKind::Version)
        .map(|field| field.layout().first_byte as u32)
        .collect();
    assert_eq!(counts, case.counts);
    let arrays: usize = case
        .fields
        .iter()
        .filter(|field| {
            field.path.contains("\"wide_arrays\"")
                || field.path.starts_with("[\"shared_arrays\",")
                || field.path.starts_with("[\"mode_tail\",")
                || field.path.ends_with("\"extra_patterns\"]")
        })
        .map(|field| field.count as usize)
        .sum();
    assert_eq!(state.coverage().array_values, arrays);
}

fn check_checkpoint(state: &FlowLoadState) {
    let covered = state.coverage();
    assert_eq!(
        covered.check_mode,
        FlowLoadCheckMode::ExactRawDeclaredLoadMode
    );

    assert_eq!(state.record().ordinal(), 1);
    assert_eq!(state.source().records().len(), 3);
    assert_eq!(state.diagnostics().len(), 4);
}
