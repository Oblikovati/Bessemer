use super::packet::Cursor;
use bessemer::simulation::*;

pub const LIMITS: SimulationBodyQueryLimits = SimulationBodyQueryLimits {
    maximum_nodes: 100000,
    maximum_elements: 100000,
    maximum_write_visits: 1000000,
};

#[derive(Clone, Copy)]
pub struct TypeCall {
    pub inputs: SimulationBodyTypeInputs,
    pub mode: i32,
    pub callback: Option<i16>,
}

pub fn type_call(cursor: &mut Cursor<'_>) -> TypeCall {
    let body_id = SimulationBodySelectionId::from_pattern(cursor.word());
    let start = cursor.word() as i32;
    let exclusive_upper = cursor.word() as i32;
    let mode = cursor.word() as i32;
    let callback = (cursor.word() != 0).then_some(cursor.word() as i16);
    TypeCall {
        inputs: SimulationBodyTypeInputs {
            body_id,
            start,
            exclusive_upper,
        },
        mode,
        callback,
    }
}

pub fn type_query(mesh: &SimulationBodyQueryMesh, call: TypeCall, expected: &[u8]) -> Vec<u8> {
    let result = mesh
        .project_body_type(call.inputs, call.mode, call.callback)
        .unwrap();
    let actual = type_bytes(&result);
    assert_eq!(actual, expected);
    assert_eq!(result.source.revision(), mesh.revision());
    assert_eq!(result.source.gen_revision(), mesh.gen_revision());
    assert_eq!(result.inputs, call.inputs);
    assert_eq!(result.query_mode, call.mode);
    assert!(!result.file_descriptor_binding);
    assert_eq!(result.flag_pattern_error_bits, 0);
    assert!(
        result
            .diagnostics
            .contains(&SimulationBodyQueryDiagnostic::NativeCallbackNotExecuted)
    );
    actual
}

pub fn type_bytes(result: &SimulationBodyTypeResult) -> Vec<u8> {
    let mut values = vec![
        result.type_flag as u32,
        result.visited_elements.len() as u32,
    ];
    values.extend(result.visited_elements.iter().map(|i| *i as u32));
    values.extend(callback_words(result));
    values.extend(type_tags(result));
    values.extend([
        result.coverage.selected_elements as u32,
        result.coverage.visited_references as u32,
    ]);
    values.extend(first_words(result.first_association));
    values.into_iter().flat_map(u32::to_le_bytes).collect()
}

fn type_tags(result: &SimulationBodyTypeResult) -> [u32; 2] {
    let completion = match result.completion {
        SimulationBodyTypeCompletion::GuardedNegativeBounds => 0,
        SimulationBodyTypeCompletion::CompletedCallbackOverride => 1,
        SimulationBodyTypeCompletion::CompletedFixedMode => 2,
        SimulationBodyTypeCompletion::CompletedNoAssociation => 3,
        SimulationBodyTypeCompletion::CompletedFirstAssociation => 4,
    };
    let check = match result.coverage.mode {
        SimulationBodyTypeCheckMode::NegativeBoundsOnly => 0,
        SimulationBodyTypeCheckMode::SuppliedCallbackAndModeOnly => 1,
        SimulationBodyTypeCheckMode::ExactExclusiveScanUntilFirstActiveAssociation => 2,
    };
    [completion, check]
}

pub fn fields(row: &SimulationAuxiliaryBodyRow, expected: &[u8]) -> Vec<u8> {
    let actual = field_bytes(row.fields());
    assert_eq!(actual, expected);
    assert_eq!(row.profile().stride(), row.to_bytes().len());
    actual
}

pub fn field_bytes(fields: SimulationAuxiliaryBodyFields) -> Vec<u8> {
    let mut bytes = fields.body_id.pattern().to_le_bytes().to_vec();
    bytes.extend_from_slice(&fields.stored_low.to_le_bytes());
    bytes.extend_from_slice(&fields.stored_high.to_le_bytes());
    bytes.extend_from_slice(&fields.runtime_start.to_le_bytes());
    bytes.extend_from_slice(&fields.runtime_upper.to_le_bytes());
    bytes.extend_from_slice(&u32::from(fields.descriptor_appended).to_le_bytes());
    bytes
}

pub fn auxiliary_edit(
    row: &SimulationAuxiliaryBodyRow,
    golden: &[u8],
    expected_fields: &[u8],
) -> SimulationAuxiliaryBodyEdit {
    let body = SimulationBodySelectionId::from_pattern(0x89abcdef);
    let edit = row.edit_fields(body, -7, i32::MAX).unwrap();
    assert_eq!(edit.row.to_bytes(), golden);
    fields(&edit.row, expected_fields);
    assert_eq!(edit.parent, *row.revision());
    assert_eq!(edit.result, *edit.row.revision());
    assert_eq!(edit.row.gen_parent(), row.gen_parent());
    assert_ne!(edit.parent, edit.result);
    assert_edit_metadata(&edit);
    assert_coverage(edit.coverage, golden.len());
    edit
}

pub fn proposal(row: &SimulationAuxiliaryBodyRow) -> Option<SimulationBodyTypeInputs> {
    let fields = row.fields();
    if fields.stored_low == i32::MIN || fields.stored_high == i32::MIN {
        assert!(row.descriptor_proposal().is_err());
        return None;
    }
    let result = row.descriptor_proposal().unwrap();
    assert_eq!(result.parent_revision, *row.revision());
    assert_eq!(result.source.to_bytes(), row.to_bytes());
    assert_eq!(result.field_pattern_error_bits, 0);
    assert_coverage(result.coverage, row.profile().stride());
    assert_eq!(result.inputs.is_some(), fields.descriptor_appended);
    assert_proposal_inputs(&result, fields);
    result.inputs
}

fn assert_proposal_inputs(
    result: &SimulationAuxiliaryDescriptorProposal,
    fields: SimulationAuxiliaryBodyFields,
) {
    let expected = if fields.descriptor_appended {
        SimulationAuxiliaryDescriptorCompletion::ProposedThreeRuntimeFields
    } else {
        SimulationAuxiliaryDescriptorCompletion::SkippedBothZero
    };
    assert_eq!(result.completion, expected);
    if let Some(inputs) = result.inputs {
        assert_eq!(inputs.body_id, fields.body_id);
        assert_eq!(i64::from(inputs.start), fields.runtime_start);
        assert_eq!(i64::from(inputs.exclusive_upper), fields.runtime_upper);
    }
    assert!(
        result
            .diagnostics
            .contains(&SimulationAuxiliaryBodyDiagnostic::ConditionalBuilderNotExecuted)
    );
}

fn assert_coverage(coverage: SimulationAuxiliaryBodyCoverage, length: usize) {
    assert_eq!(
        coverage.mode,
        SimulationAuxiliaryBodyCheckMode::ExactCompleteRowAndThreeFieldBinding
    );
    assert_eq!(coverage.stored_bytes, length);
    assert_eq!(coverage.interpreted_field_bytes, 12);
    assert!(!coverage.complete_descriptor_serialization);
}

fn callback_words(result: &SimulationBodyTypeResult) -> [u32; 4] {
    [
        u32::from(result.callback_initial_i16.is_some()),
        result.callback_initial_i16.unwrap_or(0) as i32 as u32,
        u32::from(result.supplied_callback_result.is_some()),
        result.supplied_callback_result.unwrap_or(0) as i32 as u32,
    ]
}

fn first_words(first: Option<SimulationBodyTypeAssociation>) -> [u32; 4] {
    [
        u32::from(first.is_some()),
        first.map_or(0, |a| a.element_index as u32),
        first.map_or(0, |a| a.reference_slot as u32),
        first.map_or(0, |a| a.node_slot as u32),
    ]
}

fn assert_edit_metadata(edit: &SimulationAuxiliaryBodyEdit) {
    let completion = bessemer::display_lists::DisplayEditCompletion::Changed;
    assert_eq!(edit.completion, completion);
    assert_eq!(
        edit.fidelity,
        SimulationAuxiliaryBodyFidelity::ExactOutsideIdentifierAndBounds
    );
    assert_eq!(edit.requested_pattern_error_bits, 0);
    assert!(
        edit.diagnostics
            .contains(&SimulationAuxiliaryBodyDiagnostic::CompleteDescriptorUnassigned)
    );
}
