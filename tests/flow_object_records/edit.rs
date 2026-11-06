use super::*;
#[derive(Clone, Copy)]
pub(super) enum Location {
    Row(usize, FlowObjectField),
    Trailer(FlowObjectTrailerField),
}
pub(super) fn identity(view: &FlowObjectRecords, location: Location) -> FlowObjectFieldId {
    match location {
        Location::Row(index, field) => view.row(index).unwrap().field_id(field).unwrap(),
        Location::Trailer(field) => view.legacy_trailer().unwrap().field_id(field).unwrap(),
    }
}
pub(super) fn selected_rows(count: usize) -> Vec<usize> {
    if count <= 32 {
        return (0..count).collect();
    }
    vec![0, count / 2, count - 1]
}
pub(super) fn locations(view: &FlowObjectRecords) -> Vec<Location> {
    let mut result = Vec::new();
    for index in selected_rows(view.stored_count()) {
        result.extend(
            row_fields(view.version())
                .into_iter()
                .map(|field| Location::Row(index, field)),
        );
    }
    if view.legacy_trailer().is_some() {
        result.push(Location::Trailer(FlowObjectTrailerField::Member10));
        result.extend(
            (0..16).map(|index| Location::Trailer(FlowObjectTrailerField::BlockByte(index))),
        );
    }
    result
}
fn patch(
    view: &FlowObjectRecords,
    id: &FlowObjectFieldId,
    change: bool,
) -> FlowSimpleTableEdit<FlowObjectRecords> {
    let (start, end) = id.interval();
    if end - start == 1 {
        return view
            .replace_byte(
                id,
                view.decoded_bytes()[start] ^ if change { 0x5a } else { 0 },
            )
            .unwrap();
    }
    view.replace_signed(
        id,
        view.signed(id).unwrap() ^ if change { 0x5aa5f00f } else { 0 },
    )
    .unwrap()
}
pub(super) fn check_edit(view: &FlowObjectRecords, location: Location) -> Vec<u8> {
    let id = identity(view, location);
    let original = view.source().as_bytes().to_vec();
    let unchanged = patch(view, &id, false);
    assert_eq!(unchanged.result().source().as_bytes(), original);
    state_carrier::check_history(unchanged.carrier_edit());
    let edit = patch(view, &id, true);
    check_expected(view, &id, &edit);
    let fresh = open(edit.result().decoded_bytes(), view.options()).unwrap();
    let restored = restore(&fresh, &identity(&fresh, location), view, &id);
    assert_eq!(restored.result().source().as_bytes(), original);
    assert_eq!(view.source().as_bytes(), original);
    assert!(edit.result().byte(&id).is_err() && edit.result().signed(&id).is_err());
    state_carrier::check_history(restored.carrier_edit());
    edit.result().source().as_bytes().to_vec()
}
fn check_expected(
    view: &FlowObjectRecords,
    id: &FlowObjectFieldId,
    edit: &FlowSimpleTableEdit<FlowObjectRecords>,
) {
    let mut expected = view.decoded_bytes().to_vec();
    let (start, end) = id.interval();
    let mask = if end - start == 1 {
        vec![0x5a]
    } else {
        0x5aa5f00fu32.to_le_bytes().to_vec()
    };
    for (byte, xor) in expected[start..end].iter_mut().zip(mask) {
        *byte ^= xor;
    }
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().decoded_bytes(), view.decoded_bytes());
    assert_eq!(edit.result().coverage(), view.coverage());
    state_carrier::check_history(edit.carrier_edit());
}
fn restore(
    view: &FlowObjectRecords,
    id: &FlowObjectFieldId,
    source: &FlowObjectRecords,
    old: &FlowObjectFieldId,
) -> FlowSimpleTableEdit<FlowObjectRecords> {
    if id.interval().1 - id.interval().0 == 1 {
        return view.replace_byte(id, source.byte(old).unwrap()).unwrap();
    }
    view.replace_signed(id, source.signed(old).unwrap())
        .unwrap()
}
#[test]
fn selected_fields_have_independent_byte_patches_noops_history_and_fresh_reversals() {
    for row in cases() {
        let view = open(&row.raw, options()).unwrap();
        for location in locations(&view) {
            check_edit(&view, location);
        }
    }
}
