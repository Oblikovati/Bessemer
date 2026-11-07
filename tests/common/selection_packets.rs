use super::packet::Cursor;
use bessemer::simulation::*;
use std::collections::BTreeSet;

pub const LIMITS: SimulationBodyQueryLimits = SimulationBodyQueryLimits {
    maximum_nodes: 100_000,
    maximum_elements: 100_000,
    maximum_write_visits: 1_000_000,
};

pub struct QueryInputs {
    pub context: SimulationBodyQueryContext,
    pub initial: Option<Vec<Option<u32>>>,
}

pub fn query_inputs(cursor: &mut Cursor<'_>) -> QueryInputs {
    let body = SimulationBodySelectionId::from_pattern(cursor.word());
    let mode = cursor.word() as i32;
    let callback = (cursor.word() != 0).then_some(cursor.word() as i16);
    let range_present = cursor.word() != 0;
    let endpoints = (cursor.word() as i32, cursor.word() as i32);
    let range = range_present.then_some(endpoints);
    let initial = if cursor.word() != 0 {
        Some(optional_patterns(cursor))
    } else {
        None
    };
    QueryInputs {
        context: SimulationBodyQueryContext::new(body, mode, callback, range).unwrap(),
        initial,
    }
}

fn optional_patterns(cursor: &mut Cursor<'_>) -> Vec<Option<u32>> {
    let count = cursor.word();
    (0..count)
        .map(|_| (cursor.word() != 0).then_some(cursor.word()))
        .collect()
}

pub fn predicate(
    record: &SimulationElementRecord,
    mask: u32,
    key: i32,
    expected: &[u8],
) -> Vec<u8> {
    let view = record.selection();
    let pattern = mask;
    let mask = SimulationElementFamilyMask::from_pattern(mask);
    assert_eq!(mask.pattern(), pattern);
    let mut values = vec![
        u32::from(view.body_selection_id),
        u32::from(view.family),
        u32::from(record.valid_for_mask(mask)),
        u32::from(record.solid_averaging_scalar_gate(mask, key)),
    ];
    values.extend(exclusion_flags(record, mask));
    let actual: Vec<_> = values.into_iter().flat_map(u32::to_le_bytes).collect();
    assert_eq!(actual, expected);
    actual
}

fn exclusion_flags(
    record: &SimulationElementRecord,
    mask: SimulationElementFamilyMask,
) -> Vec<u32> {
    let stored = i32::from(record.selection().body_selection_id);
    let mut values = Vec::new();
    for keys in [
        vec![],
        vec![stored],
        vec![stored + 1],
        vec![-1, 65535],
        vec![0, 1, 1],
    ] {
        let excluded: BTreeSet<_> = keys.into_iter().collect();
        values.push(u32::from(
            record.solid_averaging_exclusion_gate(mask, &excluded),
        ));
    }
    values
}

pub fn query(mesh: &SimulationBodyQueryMesh, inputs: &QueryInputs, expected: &[u8]) -> Vec<u8> {
    let original = (mesh.node_bytes().to_vec(), mesh.element_bytes().to_vec());
    let result = mesh
        .project_body_nodes(inputs.context, inputs.initial.as_deref())
        .unwrap();
    let actual = query_bytes(&result);
    assert_eq!(actual, expected);
    assert_query_contract(mesh, inputs, &result);
    assert_eq!(mesh.node_bytes(), original.0);
    assert_eq!(mesh.element_bytes(), original.1);
    actual
}

pub fn query_bytes(result: &SimulationBodyQueryResult) -> Vec<u8> {
    let mut values = vec![result.marks.len() as u32];
    for mark in result.marks.iter() {
        values.extend([u32::from(mark.is_some()), mark.unwrap_or(0)]);
    }
    values.push(result.writes.len() as u32);
    values.extend(result.writes.iter().map(|write| write.node_slot as u32));
    values.extend([result.type_flag as u32, result.native_return_value as u32]);
    values.extend(callback_words(result));
    values.push(result.writes.len() as u32);
    for write in result.writes.iter() {
        values.extend(history_words(write));
    }
    values.into_iter().flat_map(u32::to_le_bytes).collect()
}

fn callback_words(result: &SimulationBodyQueryResult) -> [u32; 8] {
    let callback = result.context.callback_result();
    [
        u32::from(result.callback_initial_i16.is_some()),
        result.callback_initial_i16.unwrap_or(0) as i32 as u32,
        u32::from(callback.is_some()),
        callback.unwrap_or(0) as i32 as u32,
        u32::from(result.type_flag_resolved),
        result.coverage.visited_elements as u32,
        result.coverage.selected_elements as u32,
        result.coverage.visited_references as u32,
    ]
}

fn history_words(write: &SimulationBodyNodeWrite) -> [u32; 7] {
    let (kind, element, slot) = match write.source {
        SimulationBodyWriteSource::NodeActivity => (0, u32::MAX, u32::MAX),
        SimulationBodyWriteSource::ElementReference {
            element_index,
            reference_slot,
        } => (1, element_index as u32, reference_slot as u32),
    };
    [
        kind,
        write.node_slot as u32,
        element,
        slot,
        u32::from(write.previous_pattern.is_some()),
        write.previous_pattern.unwrap_or(0),
        write.pattern,
    ]
}

fn assert_query_contract(
    mesh: &SimulationBodyQueryMesh,
    inputs: &QueryInputs,
    result: &SimulationBodyQueryResult,
) {
    assert_eq!(result.source.revision(), mesh.revision());
    assert_eq!(result.source.gen_revision(), mesh.gen_revision());
    assert_eq!(result.context, inputs.context);
    assert_eq!(
        result.context.body_id().pattern(),
        inputs.context.body_id().pattern()
    );
    assert_eq!(result.mark_pattern_error_bits, 0);
    assert_eq!(result.native_return_value, 0);
    assert_eq!(result.coverage.write_visits, result.writes.len());
    assert_eq!(
        result.coverage.unknown_slots,
        result.marks.iter().filter(|v| v.is_none()).count()
    );
    assert_initial_marks(inputs, result);
    assert_query_completion(result);
}

fn assert_query_completion(result: &SimulationBodyQueryResult) {
    let completion = if result.writes.is_empty() {
        SimulationBodyQueryCompletion::CompletedEmpty
    } else {
        SimulationBodyQueryCompletion::CompletedWrites
    };
    assert_eq!(result.completion, completion);
    let mode = if result.context.mode() == 1 {
        SimulationBodyQueryCheckMode::ExactAllNodeActivity
    } else {
        SimulationBodyQueryCheckMode::ExactSelectedReferencesAndNodeActivity
    };
    assert_eq!(result.coverage.mode, mode);
    assert!(
        result
            .diagnostics
            .contains(&SimulationBodyQueryDiagnostic::GeometryNotValidated)
    );
}

fn assert_initial_marks(inputs: &QueryInputs, result: &SimulationBodyQueryResult) {
    if let Some(initial) = &inputs.initial {
        assert_eq!(result.initial_marks.as_ref(), initial.as_slice());
    } else {
        assert!(result.initial_marks.iter().all(Option::is_none));
    }
    assert_eq!(result.source.node_bytes().len() / 52, result.marks.len());
    assert_eq!(result.initial_marks.len(), result.marks.len());
}
