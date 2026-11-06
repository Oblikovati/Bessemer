use super::*;
#[derive(Clone, Copy)]
pub(super) enum Cell {
    Count(usize),
    Kind(usize),
    Key(usize),
}
#[test]
fn all_existing_backend_fields_have_independent_whole_carrier_patches_noops_history_and_fresh_reverses()
 {
    for case in cases() {
        let view = open(&case.raw).unwrap();
        for table in view.tables() {
            check_edit(&view, Cell::Count(table.ordinal()));
            if table.first_kind().is_some() {
                check_edit(&view, Cell::Kind(table.ordinal()));
            }
            if table.first_key_bytes().is_some() {
                check_edit(&view, Cell::Key(table.ordinal()));
            }
        }
    }
}
impl Cell {
    fn original(self, view: &FlowMediumBackendPrefix) -> (usize, Vec<u8>) {
        let ordinal = match self {
            Self::Count(n) | Self::Kind(n) | Self::Key(n) => n,
        };
        let table = view.table(ordinal).unwrap();
        let start = table.backend_interval().0;
        match self {
            Self::Count(_) => (start + 1, table.declared_count().to_le_bytes().to_vec()),
            Self::Kind(_) => (
                start + 5,
                table.first_kind().unwrap().to_le_bytes().to_vec(),
            ),
            Self::Key(_) => (start + 9, table.first_key_bytes().unwrap().to_vec()),
        }
    }
    fn apply(self, view: &FlowMediumBackendPrefix, raw: &[u8]) -> FlowMediumBackendEdit {
        match self {
            Self::Count(n) => {
                view.replace_declared_count(n, i32::from_le_bytes(raw.try_into().unwrap()))
            }
            Self::Kind(n) => {
                view.replace_first_kind(n, i32::from_le_bytes(raw.try_into().unwrap()))
            }
            Self::Key(n) => view.replace_first_key(n, raw.try_into().unwrap()),
        }
        .unwrap()
    }
    fn changed(self, view: &FlowMediumBackendPrefix, old: &[u8]) -> Vec<u8> {
        match self {
            Self::Count(n) => match view.table(n).unwrap().declared_count() {
                0 => 0i32,
                1 => i32::MAX,
                _ => 1,
            }
            .to_le_bytes()
            .to_vec(),
            Self::Kind(n) => {
                let table = view.table(n).unwrap();
                if table.first_key_bytes().is_some() {
                    return old.to_vec();
                }
                let kind: i32 = if table.first_kind() == Some(0) { -1 } else { 0 };
                kind.to_le_bytes().to_vec()
            }
            Self::Key(_) => old.iter().map(|byte| byte ^ 255).collect(),
        }
    }
}
pub(super) fn check_edit(view: &FlowMediumBackendPrefix, cell: Cell) {
    let (offset, old) = cell.original(view);
    let source = view.metadata_prefix();
    check_noop(view, cell, &old);
    let changed = cell.changed(view, &old);
    let edit = cell.apply(view, &changed);
    let mut expected = view.backend_bytes().to_vec();
    expected[offset..offset + changed.len()].copy_from_slice(&changed);
    assert_eq!(edit.result().backend_bytes(), expected);
    assert_eq!(edit.result().opaque_tail(), view.opaque_tail());
    assert_eq!(
        edit.source().metadata_prefix().source().as_bytes(),
        source.source().as_bytes()
    );
    check_history(&edit);
    let reverse = cell.apply(edit.result(), &old);
    assert_eq!(
        reverse.result().metadata_prefix().source().as_bytes(),
        source.source().as_bytes()
    );
}
fn check_history(edit: &FlowMediumBackendEdit) {
    let carrier = edit.prefix_edit().carrier_edit();
    assert_eq!(carrier.history().len(), 3);
    let changed = carrier.source().as_bytes() != carrier.result().as_bytes();
    for ((before, after), history) in carrier
        .source()
        .records()
        .zip(carrier.result().records())
        .zip(carrier.history())
    {
        assert_eq!(history.source, before.id);
        assert_eq!(history.result, after.id);
        let expected = relation_kind(before.id.ordinal(), changed);
        assert_eq!(history.relation, expected);
        if before.id.ordinal() != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}

fn check_noop(view: &FlowMediumBackendPrefix, cell: Cell, old: &[u8]) {
    let noop = cell.apply(view, old);
    let source = view.metadata_prefix();
    assert_eq!(noop.result().metadata_prefix().record(), source.record());
    assert_eq!(
        noop.result().metadata_prefix().source().as_bytes(),
        source.source().as_bytes()
    );
    check_history(&noop);
}

fn relation_kind(ordinal: usize, changed: bool) -> FlowPayloadRelation {
    if ordinal == 1 && changed {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    }
}
