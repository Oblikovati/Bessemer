use super::*;
use ModelIntersectionReferenceStoredEdit as Edit;
use ModelIntersectionReferenceStoredState as State;
use ModelIntersectionReferenceStoredWords as Words;

pub(super) fn reference_oracle(columns: &[&str]) -> Words {
    assert_eq!(columns.len(), 7);
    Words {
        raw_state: columns[3].parse().unwrap(),
        raw_word_20: columns[4].parse().unwrap(),
        span_count_state: columns[5].parse().unwrap(),
        backward_offset_state: columns[6].parse().unwrap(),
    }
}

fn paired_reference_request(span_count_state: i32, backward_offset_state: i32) -> Edit {
    Edit::SpanAndBackwardStates {
        span_count_state,
        backward_offset_state,
    }
}

pub(super) fn reference_requests(words: Words) -> [(Edit, [i32; 2]); 4] {
    let span = words.span_count_state ^ 0x1357_9bdf;
    let backward = words.backward_offset_state ^ 0x2468_ace0;
    [
        (
            Edit::SpanCountState(span),
            [span, words.backward_offset_state],
        ),
        (
            Edit::BackwardOffsetState(backward),
            [words.span_count_state, backward],
        ),
        (paired_reference_request(span, backward), [span, backward]),
        (
            paired_reference_request(words.span_count_state, backward),
            [words.span_count_state, backward],
        ),
    ]
}

pub(super) fn verify_reference_case(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    rows: &[&str],
    output: &mut Option<Vec<u8>>,
    matrix: bool,
) {
    let states = source.intersection_reference_states().unwrap();
    assert_eq!(states.len(), 8);
    for (ordinal, (state, row)) in states.iter().zip(rows).enumerate() {
        let columns: Vec<_> = row.split('\t').collect();
        assert_eq!(columns[1].parse::<usize>().unwrap(), ordinal);
        if matrix {
            assert_eq!(state.words(), reference_oracle(&columns));
        }
        let offset = columns[2].parse::<usize>().unwrap();
        verify_reference_projection(source, state, raw, offset);
        verify_reference_noops(source, state);
        for (request, expected) in reference_requests(state.words()) {
            let result = source
                .edit_intersection_reference_state(state, request)
                .unwrap();
            verify_reference_edit(source, raw, state, &result, offset, expected);
            append_intersection_result(output, &result);
        }
    }
    assert_eq!(source.to_bytes().unwrap(), raw);
}

fn append_intersection_result(output: &mut Option<Vec<u8>>, result: &PartResolvedFeaturesEdit) {
    if output.is_some() {
        append_intersection_export(output, &result.archive().to_bytes().unwrap());
    }
}

fn verify_reference_projection(
    source: &PartResolvedFeaturesArchive,
    state: &State,
    raw: &[u8],
    offset: usize,
) {
    let literal = &raw[offset..offset + 14];
    let expected = Words {
        raw_state: u16::from_le_bytes(literal[..2].try_into().unwrap()),
        raw_word_20: i32::from_le_bytes(literal[2..6].try_into().unwrap()),
        span_count_state: i32::from_le_bytes(literal[6..10].try_into().unwrap()),
        backward_offset_state: i32::from_le_bytes(literal[10..14].try_into().unwrap()),
    };
    assert_eq!(state.words(), expected);
    for (handle, suffix) in state
        .source_fields()
        .iter()
        .zip([".raw_00", ".raw_20", ".raw_24", ".raw_28"])
    {
        assert!(source.resolve_field(handle).unwrap().path.ends_with(suffix));
    }
}

pub(super) fn verify_reference_edit(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    state: &State,
    result: &PartResolvedFeaturesEdit,
    offset: usize,
    expected: [i32; 2],
) {
    let mut literal = raw.to_vec();
    literal[offset + 6..offset + 10].copy_from_slice(&expected[0].to_le_bytes());
    literal[offset + 10..offset + 14].copy_from_slice(&expected[1].to_le_bytes());
    assert_eq!(result.archive().to_bytes().unwrap(), literal);
    let mut fields = source.fields().to_vec();
    for (handle, value) in state.source_fields()[2..].iter().zip(expected) {
        fields[handle.index()].value = ModelFieldValue::I32(value);
    }
    assert_eq!(result.archive().fields(), fields);
    verify_reference_history(source, state, result, expected);
    verify_reference_coverage(source, result, raw.len());
    verify_reference_revision_rejection(result, state);
}

fn verify_reference_revision_rejection(result: &PartResolvedFeaturesEdit, state: &State) {
    assert!(
        result
            .archive()
            .intersection_reference_state(&state.source_fields()[0])
            .is_err()
    );
    assert!(
        result
            .archive()
            .edit_intersection_reference_state(state, Edit::SpanCountState(0))
            .is_err()
    );
}

fn verify_reference_history(
    source: &PartResolvedFeaturesArchive,
    state: &State,
    result: &PartResolvedFeaturesEdit,
    expected: [i32; 2],
) {
    let changed = changed_reference_indices(state, expected);
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(result.changes().len(), changed.len());
    for (change, relative) in result.changes().iter().zip(changed) {
        verify_reference_change(source, state, result, change, relative, expected[relative]);
    }
    assert_eq!(
        result.unchanged_fields() + result.changes().len(),
        source.fields().len()
    );
}

fn changed_reference_indices(state: &State, expected: [i32; 2]) -> Vec<usize> {
    let before = [
        state.words().span_count_state,
        state.words().backward_offset_state,
    ];
    before
        .into_iter()
        .zip(expected)
        .enumerate()
        .filter(|(_, (left, right))| left != right)
        .map(|(index, _)| index)
        .collect()
}

fn verify_reference_change(
    source: &PartResolvedFeaturesArchive,
    state: &State,
    result: &PartResolvedFeaturesEdit,
    change: &ModelFieldChange,
    relative: usize,
    expected: i32,
) {
    let parent = &state.source_fields()[relative + 2];
    let original = source.resolve_field(parent).unwrap();
    let replacement = result
        .archive()
        .resolve_field(change.result().unwrap())
        .unwrap();
    assert_eq!(change.parents(), std::slice::from_ref(parent));
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(change.path(), original.path.as_ref());
    assert_eq!(replacement.path, original.path);
    assert_eq!(replacement.value, ModelFieldValue::I32(expected));
}

fn verify_reference_coverage(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    wire_bytes: usize,
) {
    assert_eq!(
        result.coverage().mode,
        ModelEditCheckMode::ExactStoredArchive
    );
    assert_eq!(result.coverage().wire_bytes, wire_bytes);
    assert_eq!(result.coverage().typed_fields, source.fields().len());
    assert_eq!(result.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(result.diagnostics(), source.diagnostics());
}

fn verify_reference_noops(source: &PartResolvedFeaturesArchive, state: &State) {
    for edit in reference_noop_requests(state.words()) {
        let result = source
            .edit_intersection_reference_state(state, edit)
            .unwrap();
        assert_eq!(result.completion(), ModelEditCompletion::Unchanged);
        assert!(result.changes().is_empty());
        assert_eq!(result.archive().fields(), source.fields());
        assert_eq!(
            &result
                .archive()
                .intersection_reference_state(&state.source_fields()[0])
                .unwrap(),
            state
        );
    }
}

pub(super) fn reference_noop_requests(words: Words) -> [Edit; 3] {
    [
        Edit::SpanCountState(words.span_count_state),
        Edit::BackwardOffsetState(words.backward_offset_state),
        paired_reference_request(words.span_count_state, words.backward_offset_state),
    ]
}
