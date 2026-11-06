use super::*;
#[derive(Clone, Copy)]
enum Cell {
    Base,
    Key,
    State,
    Flag(usize),
    Name(usize),
    Wide(FlowMediumWideField, usize),
}
#[test]
fn every_small_field_and_large_array_endpoints_support_exact_noops_patches_history_and_fresh_reversals()
 {
    for case in fixture::cases() {
        let body = open(&case.raw, options()).unwrap();
        for cell in [Cell::Base, Cell::Key, Cell::State] {
            check_edit(&body, cell);
        }
        for curve in body.curves() {
            check_edit(&body, Cell::Flag(curve.ordinal()));
        }
        for index in selected(body.name().stored_units()) {
            check_edit(&body, Cell::Name(index));
        }
        for (field, _, words) in fields(&case) {
            for index in selected(words.len()) {
                check_edit(&body, Cell::Wide(field, index));
            }
        }
    }
}
fn selected(count: usize) -> Vec<usize> {
    if count <= 256 {
        return (0..count).collect();
    }
    vec![0, count - 1]
}
impl Cell {
    fn original(self, body: &FlowMediumKindOne) -> (usize, Vec<u8>) {
        let (offset, length) = match self {
            Self::Base => (1, 1),
            Self::Key => (2, 16),
            Self::State => (body.body_bytes().len() - 1, 1),
            Self::Flag(curve) => (flag_offset(body, curve), 1),
            Self::Name(index) => (
                body.name().payload_interval().0 - body.payload_interval().0 + 4 + index * 2,
                2,
            ),
            Self::Wide(field, index) => (
                body.wide_words(field).unwrap().payload_interval().0 - body.payload_interval().0
                    + index * 8,
                8,
            ),
        };
        (offset, body.body_bytes()[offset..offset + length].to_vec())
    }
    fn apply(self, body: &FlowMediumKindOne, raw: &[u8]) -> bessemer::Result<FlowMediumBodyEdit> {
        match self {
            Self::Base => body.replace_base_byte(raw[0]),
            Self::Key => body.replace_base_key(raw.try_into().unwrap()),
            Self::State => body.replace_state_byte(raw[0]),
            Self::Flag(curve) => body.replace_curve_flag(curve, raw[0]),
            Self::Name(index) => {
                body.replace_name_unit(index, u16::from_le_bytes(raw.try_into().unwrap()))
            }
            Self::Wide(field, index) => {
                body.replace_wide_word(field, index, u64::from_le_bytes(raw.try_into().unwrap()))
            }
        }
    }
    fn changed(self, body: &FlowMediumKindOne, old: &[u8]) -> Vec<u8> {
        if let Self::Flag(_) = self {
            return vec![match old[0] {
                0 => 0,
                1 => 255,
                _ => 1,
            }];
        }
        if let Self::Name(index) = self
            && index + 1 == body.name().stored_units()
        {
            return old.to_vec();
        }
        old.iter().map(|byte| byte ^ 255).collect()
    }
}
fn flag_offset(body: &FlowMediumKindOne, ordinal: usize) -> usize {
    let curve = body.curve(ordinal).unwrap();
    curve.body_interval().0 + 13 + curve.value_count() * 16
}
fn check_edit(body: &FlowMediumKindOne, cell: Cell) {
    let (offset, old) = cell.original(body);
    check_noop(body, cell, &old);
    let changed = cell.changed(body, &old);
    let edit = cell.apply(body, &changed).unwrap();
    let mut expected = body.body_bytes().to_vec();
    expected[offset..offset + changed.len()].copy_from_slice(&changed);
    assert_eq!(edit.result().body_bytes(), expected);
    assert_eq!(edit.result().opaque_tail(), body.opaque_tail());
    assert_eq!(carrier(edit.source()).as_bytes(), carrier(body).as_bytes());
    check_history(&edit);
    let reverse = cell.apply(edit.result(), &old).unwrap();
    assert_eq!(
        carrier(reverse.result()).as_bytes(),
        carrier(body).as_bytes()
    );
}
fn check_noop(body: &FlowMediumKindOne, cell: Cell, old: &[u8]) {
    let edit = cell.apply(body, old).unwrap();
    assert_eq!(
        edit.result().backend_prefix().metadata_prefix().record(),
        body.backend_prefix().metadata_prefix().record()
    );
    assert_eq!(carrier(edit.result()).as_bytes(), carrier(body).as_bytes());
    check_history(&edit);
}
fn carrier(body: &FlowMediumKindOne) -> &FlowCarrier {
    body.backend_prefix().metadata_prefix().source()
}
fn check_history(edit: &FlowMediumBodyEdit) {
    let history = edit.prefix_edit().carrier_edit();
    assert_eq!(history.history().len(), 3);
    let changed = history.source().as_bytes() != history.result().as_bytes();
    for ((before, after), entry) in history
        .source()
        .records()
        .zip(history.result().records())
        .zip(history.history())
    {
        assert_eq!(entry.source, before.id);
        assert_eq!(entry.result, after.id);
        let expected = relation_kind(before.id.ordinal(), changed);
        assert_eq!(entry.relation, expected);
        if before.id.ordinal() != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}

fn relation_kind(ordinal: usize, changed: bool) -> FlowPayloadRelation {
    if ordinal == 1 && changed {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    }
}
