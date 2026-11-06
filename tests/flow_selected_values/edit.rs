use super::*;
pub(super) fn indices(row: &SelectionCase) -> BTreeSet<usize> {
    let count = row.expected.selected_count as usize;
    if row.native_count.is_none() {
        return (0..count).collect();
    }
    [0, count / 2, count.saturating_sub(1)]
        .into_iter()
        .filter(|index| *index < count)
        .collect()
}
#[test]
fn generated_values_and_native_endpoints_have_literal_patches_noops_history_and_fresh_reversals() {
    for row in fixture::cases() {
        let values = check_case(&row);
        for index in indices(&row) {
            check_edit(&values, index);
        }
    }
}
pub(super) fn changed(
    values: &FlowSelectedValues,
    index: usize,
) -> FlowSimpleTableEdit<FlowSelectedValues> {
    let id = values.value_id(index).unwrap();
    let bits = !values.value(&id).unwrap();
    let edit = values.replace_value(&id, bits).unwrap();
    let mut expected = values.decoded_bytes().to_vec();
    expected[index * 8..index * 8 + 8].copy_from_slice(&bits.to_le_bytes());
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(
        edit.source().source().as_bytes(),
        values.source().as_bytes()
    );
    assert_eq!(
        edit.result().context().selected_ordinals(),
        values.context().selected_ordinals()
    );
    state_carrier::check_history(edit.carrier_edit());
    edit
}
fn check_edit(values: &FlowSelectedValues, index: usize) {
    let old = values.values().raw_bits(index).unwrap();
    let id = values.value_id(index).unwrap();
    let edit = changed(values, index);
    let fresh = edit
        .result()
        .source()
        .record_112_selected_values(edit.result().record(), values.context(), values.options())
        .unwrap();
    let reverse = fresh
        .replace_value(&fresh.value_id(index).unwrap(), old)
        .unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        values.source().as_bytes()
    );
    check_noop(values, &id, old);
    assert!(edit.result().value(&id).is_err() && edit.result().replace_value(&id, old).is_err());
}

fn check_noop(values: &FlowSelectedValues, id: &FlowSelectedValueId, old: u64) {
    let noop = values.replace_value(id, old).unwrap();
    assert_eq!(
        noop.result().source().as_bytes(),
        values.source().as_bytes()
    );
    state_carrier::check_history(noop.carrier_edit());
}
