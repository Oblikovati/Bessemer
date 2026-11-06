use super::*;
#[derive(Clone, Copy)]
enum Cell {
    Base,
    Key,
    State,
    Wide(FlowMediumKindTwoWideField, usize),
    Flag(FlowMediumCurveScope, usize),
    Name(usize),
    ModelState(usize),
    Word(usize),
    Member,
}
#[test]
fn every_small_field_and_large_endpoints_support_exact_noop_patch_history_and_fresh_reverse() {
    for case in kind_fixture::cases() {
        let body = open(&case.raw, options()).unwrap();
        for cell in [Cell::Base, Cell::Key, Cell::State] {
            check_edit(&body, &case, cell);
        }
        for scope in [FlowMediumCurveScope::Body, FlowMediumCurveScope::Model] {
            for curve in body.curves(scope) {
                check_edit(&body, &case, Cell::Flag(scope, curve.ordinal()));
            }
        }
        for (field, _, words) in expected::fields(&case) {
            for index in selected(words.len()) {
                check_edit(&body, &case, Cell::Wide(field, index));
            }
        }
        check_other_fields(&body, &case);
    }
}
fn check_other_fields(body: &FlowMediumKindTwo, case: &KindTwoCase) {
    for index in selected(body.name().stored_units()) {
        check_edit(body, case, Cell::Name(index));
    }
    for index in 0..body.model_state_bytes().len() {
        check_edit(body, case, Cell::ModelState(index));
    }
    if let Some(words) = body.model_word_patterns() {
        for index in selected(words.word_count()) {
            check_edit(body, case, Cell::Word(index));
        }
        check_edit(body, case, Cell::Member);
    }
}
fn selected(count: usize) -> Vec<usize> {
    if count <= 128 {
        return (0..count).collect();
    }
    vec![0, count - 1]
}
impl Cell {
    fn original(self, body: &FlowMediumKindTwo, case: &KindTwoCase) -> (usize, Vec<u8>) {
        let start = body.payload_interval().0;
        let (offset, len) = match self {
            Self::Base => (1, 1),
            Self::Key => (2, 16),
            Self::State => (body.body_bytes().len() - 1, 1),
            Self::Wide(field, index) => (
                body.wide_words(field).unwrap().payload_interval().0 - start + index * 8,
                8,
            ),
            Self::Flag(scope, index) => (flag_position(body, scope, index), 1),
            Self::Name(index) => (case.prefix.text.start as usize + 4 + index * 2, 2),
            Self::ModelState(index) => (expected::states(&case.model).0 + index, 1),
            Self::Word(index) => (
                body.model_word_patterns().unwrap().payload_interval().0 - start + index * 4,
                4,
            ),
            Self::Member => (case.model.end as usize - 22, 4),
        };
        (offset, body.body_bytes()[offset..offset + len].to_vec())
    }
    fn apply(self, body: &FlowMediumKindTwo, raw: &[u8]) -> FlowMediumKindTwoEdit {
        match self {
            Self::Base => body.replace_base_byte(raw[0]),
            Self::Key => body.replace_base_key(raw.try_into().unwrap()),
            Self::State => body.replace_state_byte(raw[0]),
            Self::Wide(field, index) => {
                body.replace_wide_word(field, index, u64::from_le_bytes(raw.try_into().unwrap()))
            }
            Self::Flag(scope, index) => body.replace_curve_flag(scope, index, raw[0]),
            Self::Name(index) => {
                body.replace_name_unit(index, u16::from_le_bytes(raw.try_into().unwrap()))
            }
            Self::ModelState(index) => body.replace_model_state(index, raw[0]),
            Self::Word(index) => {
                body.replace_model_word(index, u32::from_le_bytes(raw.try_into().unwrap()))
            }
            Self::Member => {
                body.replace_model_member_8c(i32::from_le_bytes(raw.try_into().unwrap()))
            }
        }
        .unwrap()
    }
    fn changed(self, body: &FlowMediumKindTwo, old: &[u8]) -> Vec<u8> {
        if let Self::Flag(..) = self {
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
fn check_edit(body: &FlowMediumKindTwo, case: &KindTwoCase, cell: Cell) {
    let (offset, old) = cell.original(body, case);
    let noop = cell.apply(body, &old);
    check_noop(body, &noop);
    let changed = cell.changed(body, &old);
    let edit = cell.apply(body, &changed);
    let mut expected = body.body_bytes().to_vec();
    expected[offset..offset + changed.len()].copy_from_slice(&changed);
    assert_eq!(edit.result().body_bytes(), expected);
    assert_eq!(edit.result().opaque_tail(), body.opaque_tail());
    assert_eq!(source(edit.source()).as_bytes(), source(body).as_bytes());
    check_history(&edit);
    let reverse = cell.apply(edit.result(), &old);
    assert_eq!(source(reverse.result()).as_bytes(), source(body).as_bytes());
}
fn source(body: &FlowMediumKindTwo) -> &FlowCarrier {
    body.backend_prefix().metadata_prefix().source()
}
fn check_noop(body: &FlowMediumKindTwo, edit: &FlowMediumKindTwoEdit) {
    assert_eq!(
        edit.result().backend_prefix().metadata_prefix().record(),
        body.backend_prefix().metadata_prefix().record()
    );
    assert_eq!(source(edit.result()).as_bytes(), source(body).as_bytes());
    check_history(edit);
}
fn check_history(edit: &FlowMediumKindTwoEdit) {
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
        assert_eq!(entry.relation, relation(before.id.ordinal(), changed));
        if before.id.ordinal() != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}
fn relation(ordinal: usize, changed: bool) -> FlowPayloadRelation {
    if ordinal == 1 && changed {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    }
}

fn flag_position(body: &FlowMediumKindTwo, scope: FlowMediumCurveScope, index: usize) -> usize {
    let curve = body.curve(scope, index).unwrap();
    curve.body_interval().0 + 13 + curve.value_count() * 16
}
