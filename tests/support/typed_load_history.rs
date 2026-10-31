use bessemer::model_archives::SimulationEmbeddedArchive;
use bessemer::simulation::*;
use std::collections::BTreeMap;
use std::sync::Arc;

pub(super) fn assert_outer(
    source_len: usize,
    source_id: impl Fn(usize) -> SimulationThermalFieldId,
    result_id: impl Fn(usize) -> SimulationThermalFieldId,
    history: &[SimulationThermalHistory],
    changed: usize,
) {
    assert_eq!(history.len(), source_len);
    for (index, row) in history.iter().enumerate() {
        assert_outer_row(row, source_id(index), result_id(index), index == changed);
    }
}

fn assert_outer_row(
    row: &SimulationThermalHistory,
    parent: SimulationThermalFieldId,
    result: SimulationThermalFieldId,
    changed: bool,
) {
    let original = SimulationThermalHistoryParent::Source(parent);
    let expected = if changed {
        vec![original, SimulationThermalHistoryParent::Replacement]
    } else {
        vec![original]
    };
    assert_eq!(row.parents.as_ref(), expected);
    let relation = if changed {
        SimulationThermalRelation::Derived
    } else {
        SimulationThermalRelation::Copied
    };
    assert_eq!(row.relation, relation);
    assert_eq!(row.result, result);
}

pub(super) fn assert_nested(
    source: &[SimulationThermalField],
    result: &[SimulationThermalField],
    source_id: impl Fn(usize) -> SimulationThermalFieldId,
    result_id: impl Fn(usize) -> SimulationThermalFieldId,
    replacement: &SimulationThermalValue,
    history: &[SimulationThermalNestedHistory],
    changed: usize,
) {
    assert_eq!(source.len(), result.len());
    let mut checked = 0;
    for (outer, (before, after)) in source.iter().zip(result).enumerate() {
        let rows = archived_rows(history, source_id(outer), result_id(outer));
        checked += assert_nested_occurrence(
            &before.value,
            &after.value,
            (outer == changed).then_some(replacement),
            &rows,
        );
    }
    assert_eq!(checked, history.len());
}

pub(super) fn assert_nested_occurrence(
    before: &SimulationThermalValue,
    after: &SimulationThermalValue,
    supplied: Option<&SimulationThermalValue>,
    rows: &[&SimulationThermalNestedHistory],
) -> usize {
    let SimulationThermalValue::Embedded(before) = before else {
        assert!(rows.is_empty());
        return 0;
    };
    let SimulationThermalValue::Embedded(after) = after else {
        panic!("expected retained archive kind");
    };
    let supplied = supplied.map(|value| match value {
        SimulationThermalValue::Embedded(archive) => archive.as_ref(),
        _ => panic!("expected supplied archive kind"),
    });
    assert_nested_parents(before, after, supplied, rows);
    rows.len()
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

pub(super) fn assert_nested_parents(
    source: &SimulationEmbeddedArchive,
    after: &SimulationEmbeddedArchive,
    supplied: Option<&SimulationEmbeddedArchive>,
    rows: &[&SimulationThermalNestedHistory],
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
    row: &SimulationThermalNestedHistory,
    indices: (Option<usize>, Option<usize>, Option<usize>),
) {
    let (before, result, replacement) = indices;
    let mut parents = Vec::new();
    if let Some(index) = before {
        parents.push(SimulationThermalNestedParent::Source(
            source.field_handle(index).unwrap(),
        ));
    }
    if let (Some(archive), Some(index)) = (supplied, replacement) {
        parents.push(SimulationThermalNestedParent::Replacement(
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
) -> SimulationThermalRelation {
    match (before, result) {
        (Some(_), Some(_)) if supplied => SimulationThermalRelation::Derived,
        (Some(_), Some(_)) => SimulationThermalRelation::Copied,
        (Some(_), None) => SimulationThermalRelation::Deleted,
        (None, Some(_)) => SimulationThermalRelation::Created,
        _ => panic!("expected at least one structural occurrence"),
    }
}

pub(super) fn archived_rows(
    history: &[SimulationThermalNestedHistory],
    parent: SimulationThermalFieldId,
    result: SimulationThermalFieldId,
) -> Vec<&SimulationThermalNestedHistory> {
    let rows: Vec<&SimulationThermalNestedHistory> = history
        .iter()
        .filter(|row| row.outer_parent == parent)
        .collect();
    for row in &rows {
        assert_eq!(row.outer_result, result);
        assert_eq!(row.outer_parent, parent);
    }
    rows
}
