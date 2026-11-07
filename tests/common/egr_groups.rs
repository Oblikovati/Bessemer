use super::Cursor;
use bessemer::simulation::*;

pub fn row_view_bytes(view: &SimulationEgrRowView) -> Vec<u8> {
    let (state, name) = match view.name_state {
        SimulationSolverNameState::Known(name) => (1, name.as_bytes()),
        SimulationSolverNameState::MatchedNullSource => (2, b"".as_slice()),
        SimulationSolverNameState::UnmatchedNoWrite => (3, b"".as_slice()),
    };
    let mut words = vec![
        view.group_identifier as u32,
        view.solver_type_code.signed() as u32,
        state,
    ];
    words.extend(view.parameter_words.map(|word| word as u32));
    let mut bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    bytes.extend_from_slice(&(name.len() as u32).to_le_bytes());
    bytes.extend_from_slice(name);
    bytes
}

pub fn assert_file_row(
    image: &SimulationEgrArchive,
    id: &SimulationEgrRowId,
    cursor: &mut Cursor<'_>,
) {
    let applicable =
        image.fields().len() == 32 && image.fields().iter().all(|field| field.width == 4);
    assert_eq!(cursor.word() != 0, applicable);
    if !applicable {
        assert_eq!(
            image.conditional_row(id).unwrap_err().class,
            bessemer::FailureClass::Unsupported
        );
        return;
    }
    let row = image.conditional_row(id).unwrap();
    assert_eq!(row.source_row_id(), Some(id));
    assert_eq!(row.source_table().unwrap().revision(), image.revision());
    assert_eq!(row.source_table().unwrap().to_bytes(), image.to_bytes());
    assert_eq!(row.to_bytes(), image.live_row(id).unwrap());
    assert_eq!(row_view_bytes(&row.view()), cursor.span());
    assert_row_view(&row);
}

pub fn group_case(cursor: &mut Cursor<'_>, totals: &mut [usize; 3]) {
    let raw = cursor.span();
    let logical = SimulationEgrLogicalOrdinal::new(cursor.word()).unwrap();
    let offset = cursor.word() as i32;
    let profile = SimulationEgrRowProfile::from_loaded_widths(&[4; 32]).unwrap();
    let row = SimulationEgrRowImage::from_bytes(raw, profile).unwrap();
    assert_eq!(row.profile(), profile);
    assert!(row.source_table().is_none());
    assert!(row.source_row_id().is_none());
    assert_row_view(&row);
    let projected = row.project_egroup(logical, offset);
    let mut expected = Cursor(cursor.span());
    assert_group(&projected, &mut expected, totals);
    assert_eq!(projected.source.to_bytes(), raw);
    assert_eq!(projected.logical_ordinal, logical);
    assert_eq!(projected.group_offset, offset);
    assert_eq!(row.to_bytes(), raw);
}

fn assert_row_view(row: &SimulationEgrRowImage) {
    let view = row.view();
    let raw = row.to_bytes();
    assert_eq!(view.source.to_bytes(), raw);
    assert_eq!(
        view.check_mode,
        SimulationEgrCheckMode::ExactConditionalFourByteRowValues
    );
    assert!(!view.semantics_complete);
    assert_eq!(view.unassigned_tail, raw[40..]);
    assert_eq!(view.name_state, view.solver_type_code.lookup());
    assert_eq!(view.group_identifier.to_le_bytes(), raw[..4]);
    assert_eq!(view.solver_type_code.signed().to_le_bytes(), raw[4..8]);
    let parameters: Vec<_> = view
        .parameter_words
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    assert_eq!(parameters, raw[8..40]);
}

fn assert_group(
    result: &SimulationEgroupProjection,
    cursor: &mut Cursor<'_>,
    totals: &mut [usize; 3],
) {
    let kind = cursor.word() as usize;
    assert!(!result.native_output_executed);
    assert!(!result.complete_file_schema);
    if kind == 0 {
        assert_skip(result);
    } else {
        assert_selected(result, cursor, kind);
    }
    assert!(cursor.0.is_empty());
    totals[kind] += 1;
}

fn assert_skip(result: &SimulationEgroupProjection) {
    assert_eq!(
        result.completion,
        SimulationEgroupCompletion::SkippedIdentifierMismatch
    );
    assert_eq!(
        result.check_mode,
        SimulationEgrCheckMode::ExactConditionalGroupIdentifierGate
    );
    assert_eq!(result.group_number, None);
    assert_eq!(result.ordered_parameters, None);
    assert_eq!(result.output, None);
    assert_eq!(result.requested_pattern_error_bits, None);
}

fn assert_selected(result: &SimulationEgroupProjection, cursor: &mut Cursor<'_>, kind: usize) {
    assert_eq!(result.group_number, Some(cursor.word() as i32));
    let parameters = std::array::from_fn(|_| cursor.word() as i32);
    assert_eq!(result.ordered_parameters, Some(parameters));
    assert_eq!(result.output.is_some(), cursor.word() != 0);
    assert_eq!(result.output.as_deref().unwrap_or(&[]), cursor.span());
    let (completion, mode, error) = expected_group_state(kind);
    assert_eq!(result.completion, completion);
    assert_eq!(result.check_mode, mode);
    assert_eq!(result.requested_pattern_error_bits, error);
}

fn expected_group_state(
    kind: usize,
) -> (
    SimulationEgroupCompletion,
    SimulationEgrCheckMode,
    Option<u32>,
) {
    if kind == 1 {
        (
            SimulationEgroupCompletion::ProjectedKnownNameLine,
            SimulationEgrCheckMode::ExactConditionalKnownNameLine,
            Some(0),
        )
    } else {
        (
            SimulationEgroupCompletion::IncompleteNameLookupState,
            SimulationEgrCheckMode::ExactConditionalParametersWithIncompleteName,
            None,
        )
    }
}
