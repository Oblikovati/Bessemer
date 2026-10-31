use super::*;
use bessemer::model_archives::SimulationEmbeddedArchive;
use std::collections::BTreeMap;

pub(super) fn assert_outer(
    source: &SimulationShellProperty,
    edit: &SimulationShellPropertyEdit,
    changed: usize,
) {
    assert_eq!(edit.history().len(), source.fields().len());
    for (index, row) in edit.history().iter().enumerate() {
        let original =
            SimulationShellPropertyHistoryParent::Source(source.field_id(index).unwrap());
        let expected = if index == changed {
            vec![original, SimulationShellPropertyHistoryParent::Replacement]
        } else {
            vec![original]
        };
        assert_eq!(row.parents.as_ref(), expected);
        let relation = if index == changed {
            SimulationShellPropertyRelation::Derived
        } else {
            SimulationShellPropertyRelation::Copied
        };
        assert_eq!(row.relation, relation);
        assert_eq!(row.result, edit.archive().field_id(index).unwrap());
    }
}

fn occurrences(archive: &SimulationEmbeddedArchive) -> Vec<(Arc<str>, usize)> {
    let mut counts = BTreeMap::<Arc<str>, usize>::new();
    archive
        .fields()
        .iter()
        .map(|field| {
            let ordinal = counts.entry(field.path.clone()).or_default();
            let key = (field.path.clone(), *ordinal);
            *ordinal += 1;
            key
        })
        .collect()
}

fn supplied_archive(
    edit: &SimulationShellPropertyEdit,
    changed: usize,
    outer: usize,
) -> Option<&SimulationEmbeddedArchive> {
    if outer != changed {
        return None;
    }
    match edit.replacement() {
        SimulationShellPropertyValue::Embedded(archive) => Some(archive.as_ref()),
        _ => None,
    }
}

fn assert_nested_parents(
    source: &SimulationEmbeddedArchive,
    after: &SimulationEmbeddedArchive,
    supplied: Option<&SimulationEmbeddedArchive>,
    rows: &[&SimulationShellPropertyNestedHistory],
) {
    let before_keys = occurrences(source);
    let after_keys = occurrences(after);
    let supplied_keys = supplied.map(occurrences).unwrap_or_default();
    let mut expected_keys = before_keys.clone();
    expected_keys.extend(
        after_keys
            .iter()
            .filter(|key| !before_keys.contains(key))
            .cloned(),
    );
    assert_eq!(rows.len(), expected_keys.len());
    for (row, key) in rows.iter().zip(expected_keys) {
        let before = before_keys.iter().position(|candidate| *candidate == key);
        let result = after_keys.iter().position(|candidate| *candidate == key);
        let replacement = supplied_keys.iter().position(|candidate| *candidate == key);
        assert_nested_row(source, after, supplied, row, (before, result, replacement));
    }
}

fn assert_nested_row(
    source: &SimulationEmbeddedArchive,
    after: &SimulationEmbeddedArchive,
    supplied: Option<&SimulationEmbeddedArchive>,
    row: &SimulationShellPropertyNestedHistory,
    indices: (Option<usize>, Option<usize>, Option<usize>),
) {
    let (before, result, replacement) = indices;
    let mut parents = Vec::new();
    if let Some(index) = before {
        parents.push(SimulationShellPropertyNestedParent::Source(
            source.field_handle(index).unwrap(),
        ));
    }
    if let (Some(archive), Some(index)) = (supplied, replacement) {
        parents.push(SimulationShellPropertyNestedParent::Replacement(
            archive.field_handle(index).unwrap(),
        ));
    }
    assert_eq!(row.parents.as_ref(), parents);
    assert_eq!(row.result, result_handle(after, result));
    assert_eq!(
        row.relation,
        nested_relation(before, result, supplied.is_some())
    );
}

fn result_handle(
    archive: &SimulationEmbeddedArchive,
    index: Option<usize>,
) -> Option<bessemer::model_archives::ModelFieldHandle> {
    index.map(|index| archive.field_handle(index).unwrap())
}

fn nested_relation(
    before: Option<usize>,
    result: Option<usize>,
    supplied: bool,
) -> SimulationShellPropertyRelation {
    match (before, result) {
        (Some(_), Some(_)) if supplied => SimulationShellPropertyRelation::Derived,
        (Some(_), Some(_)) => SimulationShellPropertyRelation::Copied,
        (Some(_), None) => SimulationShellPropertyRelation::Deleted,
        (None, Some(_)) => SimulationShellPropertyRelation::Created,
        _ => panic!("expected at least one structural occurrence"),
    }
}

pub(super) fn assert_nested(
    source: &SimulationShellProperty,
    edit: &SimulationShellPropertyEdit,
    changed: usize,
) {
    let mut checked = 0;
    for (outer, field) in source.fields().iter().enumerate() {
        let SimulationShellPropertyValue::Embedded(before) = &field.value else {
            continue;
        };
        let SimulationShellPropertyValue::Embedded(after) = &edit.archive().fields()[outer].value
        else {
            panic!("expected retained archive kind")
        };
        let rows = archived_rows(source, edit, outer);
        assert_nested_parents(before, after, supplied_archive(edit, changed, outer), &rows);
        checked += rows.len();
    }
    assert_eq!(checked, edit.nested_history().len());
}

fn archived_rows<'a>(
    source: &SimulationShellProperty,
    edit: &'a SimulationShellPropertyEdit,
    outer: usize,
) -> Vec<&'a SimulationShellPropertyNestedHistory> {
    let parent = source.field_id(outer).unwrap();
    let result = edit.archive().field_id(outer).unwrap();
    let rows: Vec<&SimulationShellPropertyNestedHistory> = edit
        .nested_history()
        .iter()
        .filter(|row| row.outer_parent == parent)
        .collect();
    for row in &rows {
        assert_eq!(row.outer_result, result);
        assert_eq!(row.outer_parent, parent);
    }
    rows
}
