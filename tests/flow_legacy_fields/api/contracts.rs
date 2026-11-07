use super::*;
use FlowConditionalFieldKind as Kind;

pub(super) fn open_group(
    raw: &[u8],
    options: FlowConditionalItemOptions,
) -> bessemer::Result<FlowLegacyGroup155> {
    let source = carrier::carrier(155, raw);
    let id = source.latest(155).unwrap().unwrap().id;
    source.record_155_fields(&id, options)
}
fn exact_options(coverage: FlowConditionalCoverage) -> FlowConditionalItemOptions {
    FlowConditionalItemOptions {
        maximum_stored_bytes: coverage.stored_bytes,
        maximum_decoded_bytes: coverage.decoded_bytes,
        maximum_entries: coverage.entries,
        maximum_items: coverage.items,
        maximum_groups: coverage.groups,
        maximum_subrecords: coverage.subrecords,
        maximum_vector_values: coverage.vector_values,
        maximum_values: coverage.values,
        maximum_scopes: coverage.scopes,
        maximum_fields: coverage.fields,
    }
}
fn limits_mut(options: &mut FlowConditionalItemOptions) -> [&mut usize; 10] {
    [
        &mut options.maximum_stored_bytes,
        &mut options.maximum_decoded_bytes,
        &mut options.maximum_entries,
        &mut options.maximum_items,
        &mut options.maximum_groups,
        &mut options.maximum_subrecords,
        &mut options.maximum_vector_values,
        &mut options.maximum_values,
        &mut options.maximum_scopes,
        &mut options.maximum_fields,
    ]
}
#[test]
fn every_raw155_fixture_accepts_exact_resource_limits_and_rejects_each_reduced_axis() {
    for row in fixture::groups() {
        check_group_limits(&row);
    }
}
fn check_group_limits(row: &ConditionalCase) {
    let fields = open_group(&row.stored, group_options()).unwrap();
    let exact = exact_options(fields.coverage());
    assert!(open_group(&row.stored, exact).is_ok());
    for axis in 0..10 {
        let mut reduced = exact;
        let limit = &mut limits_mut(&mut reduced)[axis];
        if **limit == 0 {
            continue;
        }
        **limit -= 1;
        assert!(
            open_group(&row.stored, reduced).is_err(),
            "{}:{axis}",
            row.name
        );
    }
}
#[test]
fn raw155_rejects_compact_prefixes_all_extensions_and_negative_reached_counts() {
    for row in fixture::groups() {
        check_compact_prefixes(&row);
        let mut extended = row.stored.clone();
        extended.push(0);
        assert!(open_group(&extended, group_options()).is_err());
        check_negative_counts(&row);
    }
}
fn check_compact_prefixes(row: &ConditionalCase) {
    if row.stored.len() > 128 {
        return;
    }
    for end in 0..row.stored.len() {
        assert!(open_group(&row.stored[..end], group_options()).is_err());
    }
}
fn check_negative_counts(row: &ConditionalCase) {
    for field in row
        .fields
        .iter()
        .filter(|field| field.path.ends_with("\"count\"]"))
    {
        let mut raw = row.stored.clone();
        let offset = field.offset as usize;
        raw[offset..offset + 4].copy_from_slice(&(-1i32).to_le_bytes());
        assert!(
            open_group(&raw, group_options()).is_err(),
            "{}:{}",
            row.name,
            field.path
        );
    }
}
fn indices(count: usize) -> Vec<usize> {
    if count <= 64 {
        return (0..count).collect();
    }
    vec![0, count / 2, count - 1]
}
fn same_class(value: u32) -> u32 {
    match value {
        0 => 0,
        1 => u32::MAX,
        _ => 1,
    }
}
fn signed_replacement(kind: Kind, value: i32) -> i32 {
    match (kind, value) {
        (Kind::Dimension | Kind::Protocol, _) | (Kind::Mode, 0) | (Kind::SubKind, 2) => value,
        (Kind::Mode, _) => 3 - value,
        (Kind::SubKind, 0) => -1,
        (Kind::SubKind, _) => 0,
        _ => !value,
    }
}
fn replacement(
    field: FlowConditionalField<'_>,
    value: FlowConditionalValue,
) -> FlowConditionalValue {
    use FlowConditionalValue::*;
    match value {
        Byte(value) if matches!(field.kind(), Kind::Presence | Kind::Active | Kind::ItemMode) => {
            Byte(same_class(u32::from(value)) as u8)
        }
        Byte(value) => Byte(!value),
        Signed32(value) => Signed32(signed_replacement(field.kind(), value)),
        WordBits(value) if field.kind() == Kind::GatePattern => WordBits(same_class(value)),
        WordBits(value) => WordBits(!value),
        WideBits(value) => WideBits(!value),
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
#[test]
fn raw155_selected_edits_match_frozen_offsets_full_history_noops_and_inverse_wires() {
    for row in fixture::groups() {
        let fields = open_group(&row.stored, group_options()).unwrap();
        check_group_edits(&fields, &row);
        assert_eq!(fields.payload_bytes(), row.stored);
    }
}
pub(super) fn check_group_edits(fields: &FlowLegacyGroup155, row: &ConditionalCase) {
    for (field, expected) in fields
        .fields()
        .zip(&row.fields)
        .filter(|(field, _)| field.is_editable())
    {
        for index in indices(expected.count as usize) {
            check_edit(fields, row, field, expected, index);
        }
    }
}
fn check_edit(
    fields: &FlowLegacyGroup155,
    row: &ConditionalCase,
    field: FlowConditionalField<'_>,
    expected: &ConditionalField,
    index: usize,
) {
    let original = field.value(index).unwrap();
    let value = replacement(field, original);
    let edit = fields.replace_field(&field.id(), index, value).unwrap();
    let mut raw = row.stored.clone();
    let offset = expected.offset as usize + index * expected.width();
    raw[offset..offset + expected.width()].copy_from_slice(&value_bytes(value));
    assert_eq!(
        edit.result().payload_bytes(),
        raw,
        "{}:{}:{index}",
        row.name,
        expected.path
    );
    carrier::check_history(edit.carrier_edit());
    check_inverse_and_noop(fields, &edit, field.id().ordinal(), index, original, value);
}
fn check_inverse_and_noop(
    fields: &FlowLegacyGroup155,
    edit: &FlowSimpleTableEdit<FlowLegacyGroup155>,
    ordinal: usize,
    index: usize,
    original: FlowConditionalValue,
    value: FlowConditionalValue,
) {
    let changed = edit.result().field_at(ordinal).unwrap();
    assert_eq!(changed.value(index), Some(value));
    let inverse = edit
        .result()
        .replace_field(&changed.id(), index, original)
        .unwrap();
    assert_eq!(
        inverse.result().source().as_bytes(),
        fields.source().as_bytes()
    );
    let noop = fields
        .replace_field(&fields.field_at(ordinal).unwrap().id(), index, original)
        .unwrap();
    assert_eq!(noop.result().record(), fields.record());
    assert_eq!(
        noop.result().source().as_bytes(),
        fields.source().as_bytes()
    );
}
#[test]
fn raw155_invalid_types_structural_edits_and_foreign_or_stale_ids_preserve_sources() {
    for row in fixture::groups()
        .into_iter()
        .filter(|row| row.stored.len() <= 128)
    {
        let fields = open_group(&row.stored, group_options()).unwrap();
        let foreign = open_group(&row.stored, group_options()).unwrap();
        for field in fields.fields() {
            check_invalid_field(&fields, &foreign, field);
        }
        assert!(fields.scope(&foreign.scope_at(0).unwrap().id()).is_err());
        assert_eq!(fields.payload_bytes(), row.stored);
    }
}
fn check_invalid_field(
    fields: &FlowLegacyGroup155,
    foreign: &FlowLegacyGroup155,
    field: FlowConditionalField<'_>,
) {
    assert!(foreign.field(&field.id()).is_err());
    assert!(
        fields
            .replace_field(&field.id(), usize::MAX, FlowConditionalValue::Byte(0))
            .is_err()
    );
    let Some(value) = field.value(0) else {
        return;
    };
    if !field.is_editable() {
        assert!(fields.replace_field(&field.id(), 0, value).is_err());
        return;
    }
    check_wrong_type_and_stale(fields, field, value);
}
fn check_wrong_type_and_stale(
    fields: &FlowLegacyGroup155,
    field: FlowConditionalField<'_>,
    value: FlowConditionalValue,
) {
    let wrong = if field.encoding() == FlowConditionalEncoding::Byte {
        FlowConditionalValue::WideBits(0)
    } else {
        FlowConditionalValue::Byte(0)
    };
    assert!(fields.replace_field(&field.id(), 0, wrong).is_err());
    let changed = replacement(field, value);
    let edit = fields.replace_field(&field.id(), 0, changed).unwrap();
    if changed != value {
        assert!(edit.result().field(&field.id()).is_err());
        assert!(
            edit.result()
                .scope(&fields.scope_at(0).unwrap().id())
                .is_err()
        );
    }
}

fn open_words(identifier: u32, raw: &[u8]) -> bessemer::Result<FlowLegacyWordFields> {
    let source = carrier::carrier(identifier, raw);
    let id = source.latest(identifier).unwrap().unwrap().id;
    if identifier == 105 {
        return source.record_105_fields(&id, word_options());
    }
    source.record_113_fields(&id, word_options())
}
#[test]
fn unversioned_word_profiles_reject_every_fixed_prefix_extensions_and_hostile_counts() {
    for end in 0..2560 {
        assert!(open_words(105, &vec![0; end]).is_err());
    }
    for row in fixture::words() {
        let mut extended = row.raw.clone();
        extended.push(0);
        assert!(open_words(row.identifier, &extended).is_err());
        check_word_record_ids(&row);
    }
    for count in [-1i32, i32::MIN, i32::MAX] {
        assert!(open_words(113, &count.to_le_bytes()).is_err());
    }
}
fn check_word_record_ids(row: &fixture::WordCase) {
    let fields = read_words(row);
    let foreign = read_words(row);
    check_foreign_words(&fields, &foreign);
    let decoy = fields.source().records().next().unwrap().id;
    assert!(
        fields
            .source()
            .record_105_fields(&decoy, word_options())
            .is_err()
    );
    assert!(
        fields
            .source()
            .record_113_fields(&decoy, word_options())
            .is_err()
    );
}

fn check_foreign_words(fields: &FlowLegacyWordFields, foreign: &FlowLegacyWordFields) {
    assert!(
        fields
            .source()
            .record_105_fields(foreign.record(), word_options())
            .is_err()
    );
    assert!(
        fields
            .source()
            .record_113_fields(foreign.record(), word_options())
            .is_err()
    );
}
