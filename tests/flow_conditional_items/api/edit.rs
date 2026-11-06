use super::*;
use FlowConditionalFieldKind as Kind;

#[test]
fn complete_conditional_edits_match_frozen_field_offsets_preserve_noops_history_and_fresh_decoded_reversals()
 {
    for case in fixture::cases().into_iter().filter(representative) {
        let items = open(case.identifier, &case.stored, options()).unwrap();
        for (field, expected) in items
            .fields()
            .zip(&case.fields)
            .filter(|(field, _)| field.is_editable())
        {
            for index in selected(expected.count as usize) {
                check_edit(&items, &case, field, expected, index);
            }
        }
    }
}

fn representative(case: &ConditionalCase) -> bool {
    [
        "native-",
        "legacy-v1-",
        "branches-",
        "large-",
        "subkind-",
        "mode255-",
    ]
    .iter()
    .any(|prefix| case.name.starts_with(prefix))
}

fn selected(count: usize) -> Vec<usize> {
    if count <= 4 {
        return (0..count).collect();
    }
    vec![0, count / 2, count - 1]
}

fn check_edit(
    items: &FlowConditionalItems,
    case: &ConditionalCase,
    field: FlowConditionalField<'_>,
    expected: &ConditionalField,
    index: usize,
) {
    let width = expected.width();
    let offset = expected.offset as usize + index * width;
    let old = read::typed(
        expected.encoding,
        &expected.raw[index * width..(index + 1) * width],
    );
    check_noop(items, field, index, old);
    let next = replacement(items, field, old);
    let edit = items.replace_field(&field.id(), index, next).unwrap();
    let mut changed = case.decoded.clone();
    changed[offset..offset + width].copy_from_slice(&value_bytes(next));
    assert_eq!(
        edit.result().decoded_bytes(),
        changed,
        "{}:{}",
        case.name,
        expected.path
    );
    check_reversal(items, &edit, case, field.id().ordinal(), index, old);
}

fn check_noop(
    items: &FlowConditionalItems,
    field: FlowConditionalField<'_>,
    index: usize,
    old: FlowConditionalValue,
) {
    let noop = items.replace_field(&field.id(), index, old).unwrap();
    assert_eq!(noop.result().record(), items.record());
    assert_eq!(noop.result().source().as_bytes(), items.source().as_bytes());
}

fn check_reversal(
    items: &FlowConditionalItems,
    edit: &FlowConditionalItemEdit,
    case: &ConditionalCase,
    ordinal: usize,
    index: usize,
    old: FlowConditionalValue,
) {
    assert_eq!(edit.source().source().as_bytes(), items.source().as_bytes());
    check_history(edit.carrier_edit());
    let current = edit.result().field_at(ordinal).unwrap();
    let reversed = edit
        .result()
        .replace_field(&current.id(), index, old)
        .unwrap();
    assert_eq!(reversed.result().decoded_bytes(), case.decoded);
    check_fresh_reverse(edit, case, ordinal, index, old);
    assert_eq!(items.decoded_bytes(), case.decoded);
}

fn replacement(
    items: &FlowConditionalItems,
    field: FlowConditionalField<'_>,
    old: FlowConditionalValue,
) -> FlowConditionalValue {
    use FlowConditionalValue::{Byte, Signed32, WideBits, WordBits};
    match old {
        Byte(value) => Byte(byte_replacement(items, field, value)),
        Signed32(value) => Signed32(signed_replacement(field.kind(), value)),
        WordBits(value) if field.kind() == Kind::GatePattern => WordBits(nonzero(value)),
        WordBits(value) => WordBits(!value),
        WideBits(value) => WideBits(!value),
    }
}

fn signed_replacement(kind: Kind, value: i32) -> i32 {
    match (kind, value) {
        (Kind::Dimension | Kind::Protocol, _) => value,
        (Kind::Mode, 0) => 0,
        (Kind::Mode, _) => 3 - value,
        (Kind::InnerWord, 0) => -1,
        (Kind::InnerWord, _) => 0,
        (Kind::SubKind, 2) => 2,
        (Kind::SubKind, 0) => -1,
        (Kind::SubKind, _) => 0,
        (Kind::GateSigned, 0) => 0,
        (Kind::GateSigned, 1) => -1,
        (Kind::GateSigned, _) => 1,
        _ => !value,
    }
}

fn byte_replacement(
    items: &FlowConditionalItems,
    field: FlowConditionalField<'_>,
    value: u8,
) -> u8 {
    if matches!(field.kind(), Kind::Presence | Kind::Active) {
        return nonzero(u32::from(value)) as u8;
    }
    if field.kind() == Kind::ItemMode && item_version(items, field) == 1 {
        return 0;
    }
    !value
}

fn nonzero(value: u32) -> u32 {
    match value {
        0 => 0,
        1 => u32::MAX,
        _ => 1,
    }
}

fn value_bytes(value: FlowConditionalValue) -> Vec<u8> {
    match value {
        FlowConditionalValue::Byte(value) => vec![value],
        FlowConditionalValue::Signed32(value) => value.to_le_bytes().to_vec(),
        FlowConditionalValue::WordBits(value) => value.to_le_bytes().to_vec(),
        FlowConditionalValue::WideBits(value) => value.to_le_bytes().to_vec(),
    }
}

fn check_fresh_reverse(
    edit: &FlowConditionalItemEdit,
    case: &ConditionalCase,
    ordinal: usize,
    index: usize,
    old: FlowConditionalValue,
) {
    let payload = edit
        .result()
        .source()
        .latest(case.identifier)
        .unwrap()
        .unwrap()
        .payload;
    let fresh = open(case.identifier, payload, options()).unwrap();
    let field = fresh.field_at(ordinal).unwrap();
    let reverse = fresh.replace_field(&field.id(), index, old).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), case.decoded);
    if case.identifier == 126 {
        assert_eq!(
            reverse.result().source().as_bytes(),
            edit.source().source().as_bytes()
        );
    }
}

fn check_history(edit: &FlowCarrierEdit) {
    assert_eq!(edit.history().len(), 3);
    for ((before, after), history) in edit
        .source()
        .records()
        .zip(edit.result().records())
        .zip(edit.history())
    {
        assert_eq!(history.source, before.id);
        assert_eq!(history.result, after.id);
        let relation = if before.payload == after.payload {
            FlowPayloadRelation::Copied
        } else {
            FlowPayloadRelation::Derived
        };
        assert_eq!(history.relation, relation);
        if before.id.ordinal() != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}
