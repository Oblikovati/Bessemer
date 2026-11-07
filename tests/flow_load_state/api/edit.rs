use super::*;

#[test]
fn load_state_fixed_edits_match_frozen_offsets_preserve_noops_history_and_fresh_raw_reversals() {
    for case in fixture::cases() {
        let state = open(&case.raw, options(case.load_mode)).unwrap();
        for (field, expected) in state
            .fields()
            .zip(&case.fields)
            .filter(|(field, _)| field.is_editable())
        {
            for index in selected(expected.count as usize) {
                check_edit(&state, &case, field, expected, index);
            }
        }
    }
}
fn selected(count: usize) -> Vec<usize> {
    if count <= 4 {
        return (0..count).collect();
    }
    vec![0, count / 2, count - 1]
}
fn check_edit(
    state: &FlowLoadState,
    case: &LoadCase,
    field: FlowLoadField<'_>,
    expected: &LoadField,
    index: usize,
) {
    let width = expected.width();
    let old = read::typed(
        expected.encoding,
        &expected.raw[index * width..(index + 1) * width],
    );
    check_noop(state, field, index, old);
    let next = changed(old);
    let edit = state.replace_field(&field.id(), index, next).unwrap();
    check_changed_payload(edit.result(), case, expected, index, next);
    check_text_prefixes(edit.result());
    check_history(edit.carrier_edit());
    check_reverse(&edit, field.id().ordinal(), index, old, case.load_mode);
    assert_eq!(edit.source().record(), state.record());
    assert_eq!(state.decoded_bytes(), case.raw);
}
fn changed(value: FlowLoadValue) -> FlowLoadValue {
    match value {
        FlowLoadValue::Utf16Unit(value) => FlowLoadValue::Utf16Unit(!value),
        FlowLoadValue::Scalar(FlowPrimaryValue::Byte(value)) => {
            FlowLoadValue::Scalar(FlowPrimaryValue::Byte(!value))
        }
        FlowLoadValue::Scalar(FlowPrimaryValue::Signed32(value)) => {
            FlowLoadValue::Scalar(FlowPrimaryValue::Signed32(!value))
        }
        FlowLoadValue::Scalar(FlowPrimaryValue::WordBits(value)) => {
            FlowLoadValue::Scalar(FlowPrimaryValue::WordBits(!value))
        }
        FlowLoadValue::Scalar(FlowPrimaryValue::WideBits(value)) => {
            FlowLoadValue::Scalar(FlowPrimaryValue::WideBits(!value))
        }
    }
}
pub(super) fn encoded(value: FlowLoadValue) -> Vec<u8> {
    match value {
        FlowLoadValue::Utf16Unit(value) => value.to_le_bytes().to_vec(),
        FlowLoadValue::Scalar(FlowPrimaryValue::Byte(value)) => vec![value],
        FlowLoadValue::Scalar(FlowPrimaryValue::Signed32(value)) => value.to_le_bytes().to_vec(),
        FlowLoadValue::Scalar(FlowPrimaryValue::WordBits(value)) => value.to_le_bytes().to_vec(),
        FlowLoadValue::Scalar(FlowPrimaryValue::WideBits(value)) => value.to_le_bytes().to_vec(),
    }
}
fn check_text_prefixes(state: &FlowLoadState) {
    for text in state.texts() {
        let (units, remainder) = text.raw_units_bytes().as_chunks::<2>();
        assert!(remainder.is_empty());
        let prefix = units
            .iter()
            .position(|unit| *unit == [0, 0])
            .unwrap_or(units.len());
        assert_eq!(text.native_prefix_units(), prefix);
        assert_eq!(
            text.native_prefix_bytes(),
            &text.raw_units_bytes()[..prefix * 2]
        );
    }
}
fn check_reverse(
    edit: &FlowLoadStateEdit,
    ordinal: usize,
    index: usize,
    old: FlowLoadValue,
    mode: bool,
) {
    let fresh = open(edit.result().decoded_bytes(), options(mode)).unwrap();
    let id = fresh.field_at(ordinal).unwrap().id();
    let reverse = fresh.replace_field(&id, index, old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        edit.source().source().as_bytes()
    );
}
fn check_history(edit: &FlowCarrierEdit) {
    assert_eq!(edit.history().len(), 3);
    for ((before, after), entry) in edit
        .source()
        .records()
        .zip(edit.result().records())
        .zip(edit.history())
    {
        assert_eq!(entry.source, before.id);
        assert_eq!(entry.result, after.id);
        let relation = if before.payload == after.payload {
            FlowPayloadRelation::Copied
        } else {
            FlowPayloadRelation::Derived
        };
        assert_eq!(entry.relation, relation);
        if before.id.ordinal() != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}

fn check_noop(state: &FlowLoadState, field: FlowLoadField<'_>, index: usize, old: FlowLoadValue) {
    let noop = state.replace_field(&field.id(), index, old).unwrap();
    assert_eq!(noop.result().record(), state.record());
    assert_eq!(noop.result().source().as_bytes(), state.source().as_bytes());
}

fn check_changed_payload(
    result: &FlowLoadState,
    case: &LoadCase,
    expected: &LoadField,
    index: usize,
    next: FlowLoadValue,
) {
    let width = expected.width();
    let offset = expected.offset as usize + index * width;
    let mut raw = case.raw.clone();
    raw[offset..offset + width].copy_from_slice(&encoded(next));
    assert_eq!(
        result.decoded_bytes(),
        raw,
        "{}:{}",
        case.name,
        expected.path
    );
    assert_eq!(result.source().as_bytes(), carrier(&raw).as_bytes());
}
