use super::*;

pub(super) fn offset(
    table: &FlowPointerTables,
    field: FlowPointerTableField,
) -> Result<(usize, bool)> {
    let group = group_index(field);
    let selected = table
        .groups
        .get(group)
        .ok_or_else(|| table_index_error(401, "group", group, 2))?;
    if let FlowPointerTableField::DiscardedCount { .. } = field {
        return stored_word(selected.discarded_count, field, table.version());
    }
    let body = selected.body.as_ref().ok_or_else(|| {
        table_failure(
            401,
            FailureClass::Missing,
            format!(
                "group {group} count 0 for {field:?}; expected physically populated group fields"
            ),
        )
    })?;
    body_offset(body, field, table.version())
}

fn group_index(field: FlowPointerTableField) -> usize {
    match field {
        FlowPointerTableField::DiscardedCount { group }
        | FlowPointerTableField::Member { group, .. }
        | FlowPointerTableField::DiscardedTriplet { group, .. }
        | FlowPointerTableField::WideWord { group, .. }
        | FlowPointerTableField::DiscardedPrePairs { group }
        | FlowPointerTableField::PairIndex { group, .. } => group,
    }
}

fn body_offset(
    body: &GroupBodySpan,
    field: FlowPointerTableField,
    version: i32,
) -> Result<(usize, bool)> {
    match field {
        FlowPointerTableField::Member { component, .. } => {
            component_offset(body.members, component, 3)
        }
        FlowPointerTableField::DiscardedTriplet {
            triplet, component, ..
        } => triplet_offset(body, triplet, component),
        FlowPointerTableField::WideWord { array, index, .. } => wide_offset(body, array, index),
        FlowPointerTableField::DiscardedPrePairs { .. } => {
            stored_word(body.pre_pairs, field, version)
        }
        FlowPointerTableField::PairIndex {
            pair, component, ..
        } => pair_offset(body, pair, component),
        _ => Err(table_failure(
            401,
            FailureClass::Invalid,
            format!("field {field:?}; expected a populated-group selector"),
        )),
    }
}

fn stored_word(
    offset: Option<usize>,
    field: FlowPointerTableField,
    version: i32,
) -> Result<(usize, bool)> {
    offset.map(|start| (start, false)).ok_or_else(|| table_failure(401, FailureClass::Missing, format!("field {field:?}, version {version}; expected a physically stored version-one discarded word")))
}

fn component_offset(start: usize, component: usize, count: usize) -> Result<(usize, bool)> {
    if component >= count {
        return Err(table_index_error(401, "component", component, count));
    }
    Ok((start + component * 4, false))
}

fn triplet_offset(body: &GroupBodySpan, index: usize, component: usize) -> Result<(usize, bool)> {
    let count = (body.discarded.1 - body.discarded.0) / 12;
    if index >= count {
        return Err(table_index_error(401, "discarded triplet", index, count));
    }
    component_offset(body.discarded.0 + index * 12, component, 3)
}

fn wide_offset(body: &GroupBodySpan, array: usize, index: usize) -> Result<(usize, bool)> {
    let &(start, end) = body
        .arrays
        .get(array)
        .ok_or_else(|| table_index_error(401, "wide array", array, 3))?;
    let count = (end - start) / 8;
    if index >= count {
        return Err(table_index_error(401, "wide word", index, count));
    }
    Ok((start + index * 8, true))
}

fn pair_offset(body: &GroupBodySpan, pair: usize, component: usize) -> Result<(usize, bool)> {
    let count = (body.pairs.1 - body.pairs.0) / 8;
    if pair >= count {
        return Err(table_index_error(401, "pair", pair, count));
    }
    component_offset(body.pairs.0 + pair * 8, component, 2)
}
