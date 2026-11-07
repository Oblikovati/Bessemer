use super::super::visits;
use super::*;
use std::ops::Range;

pub(super) fn scan_type(
    mesh: &SimulationBodyQueryMesh,
    inputs: SimulationBodyTypeInputs,
) -> Result<SimulationBodyTypeResult> {
    let indices = exclusive_indices(inputs, mesh.elements.len() / 140)?;
    let mut result = scan_result(mesh, inputs);
    let mut visited = Vec::new();
    for index in indices {
        record_visit(&mut visited, index)?;
        if let Some(association) = inspect_element(mesh, inputs, index, &mut result.coverage)? {
            complete_association(&mut result, association);
            break;
        }
    }
    result.visited_elements = visited.into();
    Ok(result)
}

fn scan_result(
    mesh: &SimulationBodyQueryMesh,
    inputs: SimulationBodyTypeInputs,
) -> SimulationBodyTypeResult {
    let completion = SimulationBodyTypeCompletion::CompletedNoAssociation;
    let mut result = outcome::early_result(mesh, inputs, 3, Some(-1), -1, completion);
    result.coverage.mode =
        SimulationBodyTypeCheckMode::ExactExclusiveScanUntilFirstActiveAssociation;
    result
}

fn exclusive_indices(inputs: SimulationBodyTypeInputs, count: usize) -> Result<Range<usize>> {
    let (start, end) = (inputs.start, inputs.exclusive_upper);
    require(
        start != i32::MAX && end != i32::MAX,
        "body-type exclusive bounds",
        format!("{start}..{end}; expected nonoverflowing signed endpoints other than i32::MAX"),
    )?;
    require(
        start >= end || (end as usize) <= count,
        "body-type exclusive range",
        format!(
            "{start}..{end} for {count} elements; expected an empty range or an exclusive upper bound at most {count}"
        ),
    )?;
    if start >= end {
        return Ok(0..0);
    }
    Ok(start as usize..end as usize)
}

fn inspect_element(
    mesh: &SimulationBodyQueryMesh,
    inputs: SimulationBodyTypeInputs,
    index: usize,
    coverage: &mut SimulationBodyTypeCoverage,
) -> Result<Option<SimulationBodyTypeAssociation>> {
    let row = &mesh.elements.as_chunks::<140>().0[index];
    let body = u16::from_le_bytes([row[28], row[29]]);
    if row[0] == 0 || u32::from(body) != inputs.body_id.pattern() {
        return Ok(None);
    }
    coverage.selected_elements += 1;
    require(
        row[23] <= 11,
        "body-type selected count",
        format!(
            "element {index}, count {}; expected 0..=11 stored reference slots",
            row[23]
        ),
    )?;
    inspect_references(mesh, index, row, coverage)
}

fn inspect_references(
    mesh: &SimulationBodyQueryMesh,
    index: usize,
    row: &[u8; 140],
    coverage: &mut SimulationBodyTypeCoverage,
) -> Result<Option<SimulationBodyTypeAssociation>> {
    for slot in 0..usize::from(row[23]) {
        coverage.visited_references += 1;
        let id = visits::selected_node_id(row, slot);
        if let Some(node_slot) = active_node(mesh, (index, slot, row[20]), id)? {
            return Ok(Some(SimulationBodyTypeAssociation {
                element_index: index,
                reference_slot: slot,
                node_slot,
            }));
        }
    }
    Ok(None)
}

fn active_node(
    mesh: &SimulationBodyQueryMesh,
    source: (usize, usize, u8),
    id: i32,
) -> Result<Option<usize>> {
    if id == 0 {
        return Ok(None);
    }
    let index = visits::normalize_node_id(mesh, source, id)?;
    Ok((mesh.nodes[index * 52] != 0).then_some(index))
}

fn association_flag(family: u8) -> i32 {
    match family {
        4 => 1,
        8 | 3 => 2,
        2 => 4,
        _ => -1,
    }
}

fn record_visit(visited: &mut Vec<usize>, index: usize) -> Result<()> {
    visited.try_reserve(1).map_err(|error| {
        invalid(
            "body-type visit history",
            format!(
                "{} visits: {error}; expected addressable history within supplied element budget",
                visited.len()
            ),
        )
    })?;
    visited.push(index);
    Ok(())
}

fn complete_association(
    result: &mut SimulationBodyTypeResult,
    association: SimulationBodyTypeAssociation,
) {
    result.type_flag =
        association_flag(result.source.elements[association.element_index * 140 + 20]);
    result.first_association = Some(association);
    result.completion = SimulationBodyTypeCompletion::CompletedFirstAssociation;
}
