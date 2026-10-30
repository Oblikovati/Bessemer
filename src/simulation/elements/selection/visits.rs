use super::projection::MarkBuilder;
use super::*;

pub(super) fn visit_active_nodes(
    mesh: &SimulationBodyQueryMesh,
    context: SimulationBodyQueryContext,
    builder: &mut MarkBuilder,
) -> Result<()> {
    for (node_slot, node) in mesh.nodes.as_chunks::<52>().0.iter().enumerate() {
        if node[0] == 0 {
            continue;
        }
        builder.write(
            node_slot,
            SimulationBodyWriteSource::NodeActivity,
            context.body.pattern(),
            mesh.limits.maximum_write_visits,
        )?;
    }
    Ok(())
}

pub(super) fn visit_elements(
    mesh: &SimulationBodyQueryMesh,
    context: SimulationBodyQueryContext,
    builder: &mut MarkBuilder,
) -> Result<()> {
    let rows = mesh.elements.as_chunks::<140>().0;
    for index in context.indices(rows.len())? {
        builder.visited_elements += 1;
        let record = &rows[index];
        if !selected_element(record, context) {
            continue;
        }
        builder.selected_elements += 1;
        visit_references(mesh, context, builder, index, record)?;
    }
    Ok(())
}

fn selected_element(record: &[u8; 140], context: SimulationBodyQueryContext) -> bool {
    let stored = u16::from_le_bytes([record[28], record[29]]);
    record[0] != 0
        && u32::from(stored) == context.body.pattern()
        && ((2..=5).contains(&context.mode) || record[20] == 4)
}

fn visit_references(
    mesh: &SimulationBodyQueryMesh,
    context: SimulationBodyQueryContext,
    builder: &mut MarkBuilder,
    element_index: usize,
    record: &[u8; 140],
) -> Result<()> {
    let count = usize::from(record[23]);
    require(
        count <= 11,
        "body query selected node count",
        format!("element {element_index}, count {count}; expected 0..=11 stored reference slots"),
    )?;
    for slot in 0..count {
        builder.visited_references += 1;
        let id = selected_node_id(record, slot);
        let origin = (element_index, slot, record[20]);
        visit_node(mesh, context, builder, origin, id)?;
    }
    Ok(())
}

fn visit_node(
    mesh: &SimulationBodyQueryMesh,
    context: SimulationBodyQueryContext,
    builder: &mut MarkBuilder,
    source: (usize, usize, u8),
    id: i32,
) -> Result<()> {
    if id == 0 {
        return Ok(());
    }
    let node_slot = normalize_node_id(mesh, source, id)?;
    if mesh.nodes[node_slot * 52] == 0 {
        return Ok(());
    }
    let signed = context.body.pattern() as i32;
    let pattern = if signed > 1 { signed as u32 } else { 1 };
    let origin = SimulationBodyWriteSource::ElementReference {
        element_index: source.0,
        reference_slot: source.1,
    };
    builder.write(node_slot, origin, pattern, mesh.limits.maximum_write_visits)?;
    builder.resolve_family(context.mode, source.2);
    Ok(())
}

fn selected_node_id(record: &[u8; 140], slot: usize) -> i32 {
    let start = 96 + slot * 4;
    i32::from_le_bytes([
        record[start],
        record[start + 1],
        record[start + 2],
        record[start + 3],
    ])
}

fn normalize_node_id(
    mesh: &SimulationBodyQueryMesh,
    source: (usize, usize, u8),
    id: i32,
) -> Result<usize> {
    let count = mesh.nodes.len() / 52;
    require(
        id > 0 && (id as usize) <= count,
        "body query selected node ID",
        format!(
            "element {}, reference slot {}, ID {id}; expected zero or 1..={count}",
            source.0, source.1
        ),
    )?;
    Ok(id as usize - 1)
}
