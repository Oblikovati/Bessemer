use super::*;

#[test]
fn every_stored_state_pair_and_numeric_word_supports_exact_noops_patches_and_reversals() {
    let mut checked = 0;
    for case in fixture::cases() {
        let parsed = list(&case);
        for (index, entry) in case.objects.iter().enumerate() {
            let Some(body) = &entry.body else {
                continue;
            };
            check_state(&parsed, &case, index, entry, body);
            checked += 1;
            checked += check_pairs(&parsed, &case, index, entry, body);
            checked += check_numeric(&parsed, &case, index, entry, body);
        }
    }
    assert_eq!(checked, 3894);
}

struct Patch {
    entry: usize,
    offset: usize,
    bytes: Vec<u8>,
    before: FlowMappedFieldEdit,
    after: FlowMappedFieldEdit,
}

fn check_state(
    parsed: &FlowMappedObjects,
    case: &fixture::MappedCase,
    index: usize,
    entry: &fixture::ExpectedObject,
    body: &fixture::ExpectedBody,
) {
    let patch = Patch {
        entry: index,
        offset: entry.start + 5,
        bytes: vec![body.state ^ 255],
        before: FlowMappedFieldEdit::State(body.state),
        after: FlowMappedFieldEdit::State(body.state ^ 255),
    };
    check_request(parsed, case, patch);
}

fn check_pairs(
    parsed: &FlowMappedObjects,
    case: &fixture::MappedCase,
    index: usize,
    entry: &fixture::ExpectedObject,
    body: &fixture::ExpectedBody,
) -> usize {
    let start = entry.start + 6 + 9 * 8 + 4;
    for (pair, (source, value)) in body.pairs.iter().enumerate() {
        let source_patch = pair_patch(index, pair, start + pair * 8, *source, true);
        check_request(parsed, case, source_patch);
        let value_patch = pair_patch(index, pair, start + pair * 8 + 4, *value, false);
        check_request(parsed, case, value_patch);
    }
    body.pairs.len() * 2
}

fn pair_patch(entry: usize, index: usize, offset: usize, value: i32, source: bool) -> Patch {
    let after = value ^ 0x40000000;
    let before_edit = pair_field(index, value, source);
    let after_edit = pair_field(index, after, source);
    Patch {
        entry,
        offset,
        bytes: after.to_le_bytes().to_vec(),
        before: before_edit,
        after: after_edit,
    }
}

fn check_numeric(
    parsed: &FlowMappedObjects,
    case: &fixture::MappedCase,
    entry_index: usize,
    entry: &fixture::ExpectedObject,
    body: &fixture::ExpectedBody,
) -> usize {
    let mut checked = 0;
    for (field, start, words) in checks::blocks(entry, body) {
        for (index, bits) in words.iter().enumerate() {
            let patch = numeric_patch(entry_index, field, index, start, *bits);
            check_request(parsed, case, patch);
            checked += 1;
        }
    }
    checked
}

fn numeric_patch(
    entry: usize,
    field: FlowMappedNumericField,
    index: usize,
    start: usize,
    bits: u64,
) -> Patch {
    let after = bits ^ 0x8000000000000001;
    Patch {
        entry,
        offset: start + index * 8,
        bytes: after.to_le_bytes().to_vec(),
        before: FlowMappedFieldEdit::Numeric { field, index, bits },
        after: FlowMappedFieldEdit::Numeric {
            field,
            index,
            bits: after,
        },
    }
}

fn check_request(parsed: &FlowMappedObjects, case: &fixture::MappedCase, patch: Patch) {
    check_noop(parsed, patch.entry, patch.before);
    let edited = parsed.replace_field(patch.entry, patch.after).unwrap();
    let mut expected = case.payload.clone();
    expected[patch.offset..patch.offset + patch.bytes.len()].copy_from_slice(&patch.bytes);
    assert_eq!(edited.result().payload(), expected);
    assert_eq!(edited.result().source().as_bytes(), wire(&expected));
    assert_eq!(edited.source().payload(), case.payload);
    check_history(edited.carrier_edit());
    let reverse = edited
        .result()
        .replace_field(patch.entry, patch.before)
        .unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        parsed.source().as_bytes()
    );
    assert_eq!(parsed.payload(), case.payload);
}

fn check_history(edit: &FlowCarrierEdit) {
    assert_eq!(edit.history().len(), 3);
    for ((relation, source), result) in edit
        .history()
        .iter()
        .zip(edit.source().records())
        .zip(edit.result().records())
    {
        assert_eq!(relation.source, source.id);
        assert_eq!(relation.result, result.id);
        let expected = if source.identifier == 144 {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        };
        assert_eq!(relation.relation, expected);
    }
}

#[test]
fn source_buffers_and_context_lists_are_isolated_across_eight_editors() {
    let mut case = fixture::cases().remove(2);
    let mut input = wire(&case.payload);
    let source = FlowCarrier::from_bytes(&input, wire_limits()).unwrap();
    let parsed = source
        .mapped_objects(limits(), context(&case.contexts))
        .unwrap()
        .unwrap();
    input.fill(0);
    case.contexts.as_mut().unwrap().clear();
    check_parallel(&parsed);
    assert_eq!(parsed.entries()[0].body().unwrap().state_byte(), 250);
}

fn check_parallel(parsed: &FlowMappedObjects) {
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let shared = &parsed;
            scope.spawn(move || {
                let edited = shared
                    .replace_field(0, FlowMappedFieldEdit::State(7))
                    .unwrap();
                let reversed = edited
                    .result()
                    .replace_field(0, FlowMappedFieldEdit::State(250))
                    .unwrap();
                assert_eq!(
                    reversed.result().source().as_bytes(),
                    shared.source().as_bytes()
                );
            });
        }
    });
}

fn check_noop(parsed: &FlowMappedObjects, entry: usize, before: FlowMappedFieldEdit) {
    let noop = parsed.replace_field(entry, before).unwrap();
    assert_eq!(
        noop.result().source().as_bytes(),
        parsed.source().as_bytes()
    );
    assert_eq!(noop.result().record(), parsed.record());
}

fn pair_field(index: usize, value: i32, source: bool) -> FlowMappedFieldEdit {
    if source {
        return FlowMappedFieldEdit::PairSource { index, value };
    }
    FlowMappedFieldEdit::PairValue { index, value }
}
