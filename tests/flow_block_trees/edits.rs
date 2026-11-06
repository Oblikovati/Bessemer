use super::*;

struct Patch {
    offset: usize,
    before: FlowTreeFieldEdit,
    after: FlowTreeFieldEdit,
    bytes: Vec<u8>,
}

fn requests(case: &fixture::TreeCase) -> Vec<Patch> {
    let mut result = Vec::new();
    for (node, expected) in case.nodes.iter().enumerate() {
        if expected.marker != 255 {
            result.push(leaf_patch(node, expected));
        }
        if let Some(word) = expected.word {
            result.push(word_patch(node, expected.start + 1, word));
        }
        append_child_requests(&mut result, case, node, expected);
    }
    append_root_requests(&mut result, case);
    result
}

fn leaf_patch(node: usize, expected: &fixture::ExpectedNode) -> Patch {
    let byte = if expected.marker == 254 {
        0
    } else {
        expected.marker + 1
    };
    Patch {
        offset: expected.start,
        before: FlowTreeFieldEdit::LeafMarker {
            node,
            byte: expected.marker,
        },
        after: FlowTreeFieldEdit::LeafMarker { node, byte },
        bytes: vec![byte],
    }
}

fn word_patch(node: usize, offset: usize, word: u32) -> Patch {
    let after = word ^ 0x80000001;
    Patch {
        offset,
        before: FlowTreeFieldEdit::DiscardedLeafWord { node, word },
        after: FlowTreeFieldEdit::DiscardedLeafWord { node, word: after },
        bytes: after.to_le_bytes().to_vec(),
    }
}

fn child_patch(node: usize, child: usize, offset: usize, byte: u8) -> Patch {
    let after = byte ^ 255;
    Patch {
        offset,
        before: FlowTreeFieldEdit::ChildPrefix { node, child, byte },
        after: FlowTreeFieldEdit::ChildPrefix {
            node,
            child,
            byte: after,
        },
        bytes: vec![after],
    }
}

fn root_patch(root: usize, offset: usize, byte: u8) -> Patch {
    let after = if byte == 0 {
        0
    } else if byte == 255 {
        1
    } else {
        255
    };
    Patch {
        offset,
        before: FlowTreeFieldEdit::RootPresence { root, byte },
        after: FlowTreeFieldEdit::RootPresence { root, byte: after },
        bytes: vec![after],
    }
}

fn check_request(parsed: &FlowBlockTrees, case: &fixture::TreeCase, patch: Patch) {
    check_noop(parsed, patch.before);
    let mut expected = case.decoded.clone();
    expected[patch.offset..patch.offset + patch.bytes.len()].copy_from_slice(&patch.bytes);
    let edit = parsed.replace_field(patch.after).unwrap();
    assert_eq!(edit.result().decoded_bytes(), expected);
    check_source(&edit, parsed);
    check_history(edit.carrier_edit(), expected != case.decoded);
    let reverse = edit.result().replace_field(patch.before).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), case.decoded);
    if !case.compressed {
        assert_eq!(
            reverse.result().source().as_bytes(),
            parsed.source().as_bytes()
        );
    }
}

fn check_history(edit: &FlowCarrierEdit, changed: bool) {
    assert_eq!(edit.history().len(), 3);
    for ((history, source), result) in edit
        .history()
        .iter()
        .zip(edit.source().records())
        .zip(edit.result().records())
    {
        assert_eq!(history.source, source.id);
        assert_eq!(history.result, result.id);
        let expected = if changed && source.identifier == 400 {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        };
        assert_eq!(history.relation, expected);
        if source.identifier != 400 {
            assert_eq!(source.payload, result.payload);
        }
    }
}

#[test]
fn every_normal_owned_wire_field_has_exact_noop_patch_and_fresh_reversal() {
    for case in fixture::cases()
        .into_iter()
        .skip(1)
        .filter(|case| !matches!(case.name.as_str(), "depth255" | "depth256"))
    {
        let parsed = trees(&case);
        for request in requests(&case) {
            check_request(&parsed, &case, request);
        }
    }
}

#[test]
fn depth_256_edits_and_eight_workers_preserve_the_original_checkpoint() {
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.name == "depth256" && !case.compressed && case.version == 3)
        .unwrap();
    let mut raw = wire(&case.stored);
    let source = FlowCarrier::from_bytes(&raw, limits()).unwrap();
    let parsed = read(&source, options(&case)).unwrap();
    raw.fill(0);
    let deepest = case
        .nodes
        .iter()
        .position(|node| node.depth == 256)
        .unwrap();
    check_request(&parsed, &case, leaf_patch(deepest, &case.nodes[deepest]));
    parallel_edits(&parsed, deepest);
    assert_eq!(parsed.source().as_bytes(), wire(&case.stored));
}

fn parallel_edits(parsed: &FlowBlockTrees, node: usize) {
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(move || {
                let edited = parsed
                    .replace_field(FlowTreeFieldEdit::LeafMarker { node, byte: 7 })
                    .unwrap();
                assert_eq!(edited.result().nodes()[node].marker_byte(), 7);
                assert_eq!(parsed.nodes()[node].marker_byte(), 238);
            });
        }
    });
}

#[test]
#[ignore = "explicit full native record-400 field census"]
fn every_native_stored_wire_field_preserves_all_decoded_bytes_and_reverses_logically() {
    let case = fixture::cases().remove(0);
    let parsed = trees(&case);
    let requests = requests(&case);
    assert_eq!(requests.len(), 8424);
    for request in requests {
        check_request(&parsed, &case, request);
    }
}

fn append_root_requests(result: &mut Vec<Patch>, case: &fixture::TreeCase) {
    for (root, expected) in case.roots.as_deref().unwrap_or(&[]).iter().enumerate() {
        if let Some(byte) = expected.presence {
            result.push(root_patch(root, expected.start, byte));
        }
    }
}

fn check_noop(parsed: &FlowBlockTrees, before: FlowTreeFieldEdit) {
    let noop = parsed.replace_field(before).unwrap();
    assert_eq!(
        noop.result().source().as_bytes(),
        parsed.source().as_bytes()
    );
    assert_eq!(noop.result().record(), parsed.record());
}

fn check_source(edit: &FlowTreeEdit, parsed: &FlowBlockTrees) {
    assert_eq!(
        edit.source().source().as_bytes(),
        parsed.source().as_bytes()
    );
}

fn append_child_requests(
    result: &mut Vec<Patch>,
    case: &fixture::TreeCase,
    node: usize,
    expected: &fixture::ExpectedNode,
) {
    for (child, (prefix, index)) in expected.children.iter().enumerate() {
        if let Some(byte) = prefix {
            result.push(child_patch(
                node,
                child,
                case.nodes[*index].start - 1,
                *byte,
            ));
        }
    }
}
