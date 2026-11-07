use super::*;

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowTreeOptions,
) -> Result<FlowBlockTrees> {
    validate_options(options)?;
    let stored = super::super::payload_codec::record_payload(source, id, 400, "Flow block trees")?;
    let decoded = expand(stored, options)?;
    let parsed = parse_decoded(&decoded, options)?;
    Ok(published(
        source,
        id,
        options,
        stored.len(),
        decoded,
        parsed,
    ))
}

fn validate_options(options: FlowTreeOptions) -> Result<()> {
    if options.maximum_decoded_bytes >= 4
        && options.maximum_roots > 0
        && options.maximum_nodes > 0
        && options.maximum_depth <= 256
    {
        return Ok(());
    }
    Err(tree_failure(
        FailureClass::Invalid,
        format!(
            "options {options:?}; expected decoded bytes >=4, positive root/node bounds and maximum depth 0 through 256"
        ),
    ))
}

fn expand(stored: &[u8], options: FlowTreeOptions) -> Result<Vec<u8>> {
    if options.compression == FlowTreeCompression::Zlib {
        return super::super::payload_codec::inflate_zlib(
            stored,
            options.maximum_decoded_bytes,
            "Flow block trees",
        );
    }
    if stored.len() <= options.maximum_decoded_bytes {
        return Ok(stored.to_vec());
    }
    Err(tree_failure(
        FailureClass::Invalid,
        format!(
            "raw payload {} bytes; expected at most {} decoded bytes",
            stored.len(),
            options.maximum_decoded_bytes
        ),
    ))
}

fn header(
    cursor: &mut ArchiveCursor<'_>,
    options: FlowTreeOptions,
) -> Result<(i32, FlowTreeBounds)> {
    let version = cursor.i32()?;
    if version < 0 {
        return Err(tree_failure(
            FailureClass::Unsupported,
            format!("version {version}; expected nonnegative signed-i32 profile"),
        ));
    }
    Ok((
        version,
        read_bounds(cursor, version, options.legacy_bounds)?,
    ))
}

type ParsedForest = (Option<Vec<FlowTreeRoot>>, Vec<FlowTreeNode>);

fn read_body(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    version: i32,
    bounds: FlowTreeBounds,
    options: FlowTreeOptions,
) -> Result<ParsedForest> {
    if version < 4 {
        return forest::read(cursor, length, version, bounds, options)
            .map(|(roots, nodes)| (Some(roots), nodes));
    }
    let presence = cursor.u8()?;
    if presence == 0 {
        return Ok((None, Vec::new()));
    }
    Err(tree_failure(
        FailureClass::Unsupported,
        format!(
            "version {version} presence {presence}; expected inactive zero byte; active delegated tree layout is outside this profile"
        ),
    ))
}

fn coverage(
    stored: usize,
    decoded: usize,
    roots: &Option<Vec<FlowTreeRoot>>,
    nodes: &[FlowTreeNode],
    compression: FlowTreeCompression,
) -> FlowTreeCoverage {
    let mode = check_mode(compression);
    let splits = nodes.iter().filter(|node| node.marker == 255).count();
    FlowTreeCoverage {
        mode,
        stored_bytes: stored,
        decoded_bytes: decoded,
        checked_roots: roots.as_ref().map_or(0, Vec::len),
        checked_nodes: nodes.len(),
        checked_splits: splits,
        checked_leaves: nodes.len() - splits,
        discarded_child_bytes: nodes
            .iter()
            .flat_map(|node| node.children.iter())
            .filter(|child| child.prefix.is_some())
            .count(),
        discarded_leaf_word_bytes: nodes.iter().filter(|node| node.word.is_some()).count() * 4,
        maximum_wire_depth: nodes.iter().map(|node| node.depth).max().unwrap_or(0),
        achieved_byte_error: 0,
    }
}

struct ParsedTree {
    version: i32,
    bounds: FlowTreeBounds,
    roots: Option<Vec<FlowTreeRoot>>,
    nodes: Vec<FlowTreeNode>,
}

fn parse_decoded(decoded: &[u8], options: FlowTreeOptions) -> Result<ParsedTree> {
    let mut cursor = ArchiveCursor::new(decoded);
    let (version, bounds) = header(&mut cursor, options)?;
    let (roots, nodes) = read_body(&mut cursor, decoded.len(), version, bounds, options)?;
    cursor.finish()?;
    Ok(ParsedTree {
        version,
        bounds,
        roots,
        nodes,
    })
}

fn read_bounds(
    cursor: &mut ArchiveCursor<'_>,
    version: i32,
    legacy: Option<FlowTreeBounds>,
) -> Result<FlowTreeBounds> {
    if version < 2 {
        return legacy.ok_or_else(|| tree_failure(FailureClass::Missing,
            format!("version {version} without legacy bounds; expected explicit three signed inclusive range pairs")));
    }
    if legacy.is_some() {
        return Err(tree_failure(
            FailureClass::Invalid,
            format!(
                "version {version} with legacy bounds {legacy:?}; expected stored bounds without unused caller context"
            ),
        ));
    }
    let pairs = [
        [cursor.i32()?, cursor.i32()?],
        [cursor.i32()?, cursor.i32()?],
        [cursor.i32()?, cursor.i32()?],
    ];
    Ok(FlowTreeBounds::from_pairs(pairs))
}

fn check_mode(compression: FlowTreeCompression) -> FlowTreeCheckMode {
    match compression {
        FlowTreeCompression::Raw => FlowTreeCheckMode::ExactRawForestFrames,
        FlowTreeCompression::Zlib => FlowTreeCheckMode::ExactZlibForestFrames,
    }
}

fn published(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowTreeOptions,
    stored: usize,
    decoded: Vec<u8>,
    parsed: ParsedTree,
) -> FlowBlockTrees {
    let coverage = coverage(
        stored,
        decoded.len(),
        &parsed.roots,
        &parsed.nodes,
        options.compression,
    );
    FlowBlockTrees {
        source: source.clone(),
        record: id.clone(),
        decoded: Arc::from(decoded),
        version: parsed.version,
        bounds: parsed.bounds,
        roots: parsed.roots.map(Arc::from),
        nodes: Arc::from(parsed.nodes),
        options,
        coverage,
    }
}
