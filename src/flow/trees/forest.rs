use super::*;

struct NodeBuilder {
    start: usize,
    end: usize,
    marker: u8,
    depth: u16,
    word: Option<u32>,
    children: Vec<FlowTreeChild>,
}

struct ForestReader<'cursor, 'bytes> {
    cursor: &'cursor mut ArchiveCursor<'bytes>,
    length: usize,
    version: i32,
    bounds: FlowTreeBounds,
    options: FlowTreeOptions,
    nodes: Vec<NodeBuilder>,
}

enum Pending {
    Node { depth: u16, parent: Option<usize> },
    Finish(usize),
}

pub(super) fn read(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    version: i32,
    bounds: FlowTreeBounds,
    options: FlowTreeOptions,
) -> Result<(Vec<FlowTreeRoot>, Vec<FlowTreeNode>)> {
    let count = root_count(cursor, bounds, options.maximum_roots)?;
    let mut reader = ForestReader {
        cursor,
        length,
        version,
        bounds,
        options,
        nodes: Vec::new(),
    };
    let mut roots = Vec::with_capacity(count);
    for ordinal in 0..count {
        roots.push(read_root(&mut reader, ordinal)?);
    }
    let nodes = publish_nodes(reader.nodes);
    validate_forest(&roots, &nodes)?;
    Ok((roots, nodes))
}

fn root_count(cursor: &ArchiveCursor<'_>, bounds: FlowTreeBounds, maximum: usize) -> Result<usize> {
    let count = bounds.grid_count();
    let available = maximum.min(cursor.remaining());
    if count <= available as u128 {
        return Ok(count as usize);
    }
    Err(tree_failure(
        FailureClass::Invalid,
        format!(
            "grid count {count}; expected at most {available} root positions within wire/root budget"
        ),
    ))
}

fn root_indices(bounds: FlowTreeBounds, ordinal: usize) -> [i32; 3] {
    let widths = bounds.widths();
    let ordinal = ordinal as u64;
    let steps = [
        ordinal % widths[0],
        (ordinal / widths[0]) % widths[1],
        ordinal / widths[0] / widths[1],
    ];
    std::array::from_fn(|index| (i64::from(bounds.pairs()[index][0]) + steps[index] as i64) as i32)
}

fn read_root(reader: &mut ForestReader<'_, '_>, ordinal: usize) -> Result<FlowTreeRoot> {
    let start = reader.length - reader.cursor.remaining();
    let presence = if reader.version >= 2 {
        Some(reader.cursor.u8()?)
    } else {
        None
    };
    let node = if presence != Some(0) {
        Some(read_tree(reader)?)
    } else {
        None
    };
    Ok(FlowTreeRoot {
        start,
        end: reader.length - reader.cursor.remaining(),
        indices: root_indices(reader.bounds, ordinal),
        presence,
        node,
    })
}

fn read_tree(reader: &mut ForestReader<'_, '_>) -> Result<usize> {
    let first = reader.nodes.len();
    let mut pending = vec![Pending::Node {
        depth: 0,
        parent: None,
    }];
    while let Some(task) = pending.pop() {
        match task {
            Pending::Finish(index) => {
                reader.nodes[index].end = reader.length - reader.cursor.remaining()
            }
            Pending::Node { depth, parent } => enter_node(reader, &mut pending, depth, parent)?,
        }
    }
    Ok(first)
}

fn enter_node(
    reader: &mut ForestReader<'_, '_>,
    pending: &mut Vec<Pending>,
    depth: u16,
    parent: Option<usize>,
) -> Result<()> {
    let prefix = if parent.is_some() && matches!(reader.version, 2 | 3) {
        Some(reader.cursor.u8()?)
    } else {
        None
    };
    check_node_budget(reader.nodes.len(), depth, reader.options)?;
    let (index, marker) = read_node_header(reader, depth)?;
    if let Some(parent) = parent {
        reader.nodes[parent].children.push(FlowTreeChild {
            prefix,
            node: index,
        });
    }
    if marker == 255 {
        schedule_children(index, depth, pending);
    }
    Ok(())
}

fn read_node_header(reader: &mut ForestReader<'_, '_>, depth: u16) -> Result<(usize, u8)> {
    let start = reader.length - reader.cursor.remaining();
    let marker = reader.cursor.u8()?;
    let word = if marker != 255 && reader.version == 2 {
        Some(reader.cursor.u32()?)
    } else {
        None
    };
    let index = reader.nodes.len();
    reader.nodes.push(NodeBuilder {
        start,
        end: reader.length - reader.cursor.remaining(),
        marker,
        depth,
        word,
        children: Vec::new(),
    });
    Ok((index, marker))
}

fn check_node_budget(count: usize, depth: u16, options: FlowTreeOptions) -> Result<()> {
    if count < options.maximum_nodes && depth <= options.maximum_depth {
        return Ok(());
    }
    Err(tree_failure(
        FailureClass::Invalid,
        format!(
            "node ordinal {count}, depth {depth}; expected ordinal below {} and depth at most {}",
            options.maximum_nodes, options.maximum_depth
        ),
    ))
}

fn schedule_children(index: usize, depth: u16, pending: &mut Vec<Pending>) {
    pending.push(Pending::Finish(index));
    for _ in 0..8 {
        pending.push(Pending::Node {
            depth: depth + 1,
            parent: Some(index),
        });
    }
}

fn publish_node(node: NodeBuilder, empty: &Arc<[FlowTreeChild]>) -> FlowTreeNode {
    let children = if node.children.is_empty() {
        empty.clone()
    } else {
        Arc::from(node.children)
    };
    FlowTreeNode {
        start: node.start,
        end: node.end,
        marker: node.marker,
        depth: node.depth,
        word: node.word,
        children,
    }
}

fn validate_forest(roots: &[FlowTreeRoot], nodes: &[FlowTreeNode]) -> Result<()> {
    let present = roots.iter().filter(|root| root.node.is_some()).count();
    let splits = nodes.iter().filter(|node| node.marker == 255).count();
    let all_children = nodes
        .iter()
        .all(|node| node.children.len() == if node.marker == 255 { 8 } else { 0 });
    if all_children && nodes.len() == present + 8 * splits {
        return Ok(());
    }
    Err(tree_failure(
        FailureClass::Invalid,
        format!(
            "{} nodes, {present} roots, {splits} splits; expected complete eight-child forest and exact child counts",
            nodes.len()
        ),
    ))
}

fn publish_nodes(nodes: Vec<NodeBuilder>) -> Vec<FlowTreeNode> {
    let empty: Arc<[FlowTreeChild]> = Arc::from([]);
    nodes
        .into_iter()
        .map(|node| publish_node(node, &empty))
        .collect()
}
