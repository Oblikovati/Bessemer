use super::*;

impl FlowBlockTrees {
    /// Edits one existing fixed-width wire field; e.g. `let edit = trees.replace_field(FlowTreeFieldEdit::LeafMarker { node: 0, byte: 7 })?`.
    pub fn replace_field(&self, request: FlowTreeFieldEdit) -> Result<FlowTreeEdit> {
        let (offset, bytes) = select_field(self, request)?;
        let mut decoded = self.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = edited_carrier(self, &decoded)?;
        let result = checked_result(self, &carrier_edit, &decoded)?;
        Ok(FlowTreeEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn select_field(trees: &FlowBlockTrees, request: FlowTreeFieldEdit) -> Result<(usize, Vec<u8>)> {
    match request {
        FlowTreeFieldEdit::LeafMarker { node, byte } => leaf_marker(trees, node, byte),
        FlowTreeFieldEdit::ChildPrefix { node, child, byte } => {
            child_prefix(trees, node, child, byte)
        }
        FlowTreeFieldEdit::DiscardedLeafWord { node, word } => leaf_word(trees, node, word),
        FlowTreeFieldEdit::RootPresence { root, byte } => root_presence(trees, root, byte),
    }
}

fn selected_node(trees: &FlowBlockTrees, index: usize) -> Result<&FlowTreeNode> {
    trees
        .nodes
        .get(index)
        .ok_or_else(|| index_error("node", index, trees.nodes.len()))
}

fn leaf_marker(trees: &FlowBlockTrees, index: usize, byte: u8) -> Result<(usize, Vec<u8>)> {
    let node = selected_node(trees, index)?;
    if node.marker == 255 || byte == 255 {
        return Err(tree_failure(
            FailureClass::Unsupported,
            format!(
                "node {index} marker {} to {byte}; expected an existing leaf and replacement byte 0 through 254",
                node.marker
            ),
        ));
    }
    Ok((node.start, vec![byte]))
}

fn child_prefix(
    trees: &FlowBlockTrees,
    index: usize,
    child: usize,
    byte: u8,
) -> Result<(usize, Vec<u8>)> {
    let node = selected_node(trees, index)?;
    let selected = node
        .children
        .get(child)
        .ok_or_else(|| index_error("child", child, node.children.len()))?;
    if selected.prefix.is_none() {
        return Err(tree_failure(
            FailureClass::Missing,
            format!(
                "node {index} child {child}, version {}; expected a physically stored child prefix",
                trees.version
            ),
        ));
    }
    Ok((trees.nodes[selected.node].start - 1, vec![byte]))
}

fn leaf_word(trees: &FlowBlockTrees, index: usize, word: u32) -> Result<(usize, Vec<u8>)> {
    let node = selected_node(trees, index)?;
    if node.word.is_none() {
        return Err(tree_failure(
            FailureClass::Missing,
            format!(
                "node {index}, version {}; expected a physically stored version-2 leaf word",
                trees.version
            ),
        ));
    }
    Ok((node.start + 1, word.to_le_bytes().to_vec()))
}

fn root_presence(trees: &FlowBlockTrees, index: usize, byte: u8) -> Result<(usize, Vec<u8>)> {
    let root = selected_root(trees, index)?;
    let before = root.presence.ok_or_else(|| {
        tree_failure(
            FailureClass::Missing,
            format!(
                "root {index} in legacy version {}; expected a stored root presence byte",
                trees.version
            ),
        )
    })?;
    if (before == 0) != (byte == 0) {
        return Err(tree_failure(
            FailureClass::Unsupported,
            format!(
                "root {index} presence {before} to {byte}; expected unchanged zero/nonzero branch"
            ),
        ));
    }
    Ok((root.start, vec![byte]))
}

fn edited_carrier(trees: &FlowBlockTrees, decoded: &[u8]) -> Result<FlowCarrierEdit> {
    if decoded == trees.decoded.as_ref() {
        let payload = super::super::payload_codec::record_payload(
            &trees.source,
            &trees.record,
            400,
            "Flow block trees",
        )?;
        return trees.source.replace_payload(&trees.record, payload);
    }
    let payload = match trees.options.compression {
        FlowTreeCompression::Raw => decoded.to_vec(),
        FlowTreeCompression::Zlib => {
            super::super::payload_codec::encode_zlib(decoded, "Flow block trees")?
        }
    };
    trees.source.replace_payload(&trees.record, &payload)
}

fn checked_result(
    trees: &FlowBlockTrees,
    carrier: &FlowCarrierEdit,
    decoded: &[u8],
) -> Result<FlowBlockTrees> {
    let id = result_id(carrier.result(), trees.record.ordinal())?;
    let result = carrier.result().record_400_trees(&id, trees.options)?;
    if result.decoded_bytes() == decoded {
        return Ok(result);
    }
    Err(tree_failure(
        FailureClass::Invalid,
        format!(
            "reopened {} decoded bytes; expected complete patched {} bytes",
            result.decoded.len(),
            decoded.len()
        ),
    ))
}

fn index_error(label: &str, index: usize, count: usize) -> BessemerError {
    tree_failure(
        FailureClass::Invalid,
        format!("{label} index {index}; expected below {count} in this immutable selected forest"),
    )
}

fn selected_root(trees: &FlowBlockTrees, index: usize) -> Result<&FlowTreeRoot> {
    let roots = trees.roots.as_ref().ok_or_else(|| {
        tree_failure(
            FailureClass::Missing,
            format!(
                "root {index} in inactive version {}; expected an active legacy forest",
                trees.version
            ),
        )
    })?;
    roots
        .get(index)
        .ok_or_else(|| index_error("root", index, roots.len()))
}

fn result_id(carrier: &FlowCarrier, ordinal: usize) -> Result<FlowRecordId> {
    carrier
        .records()
        .nth(ordinal)
        .map(|record| record.id)
        .ok_or_else(|| index_error("result occurrence", ordinal, carrier.records().len()))
}
