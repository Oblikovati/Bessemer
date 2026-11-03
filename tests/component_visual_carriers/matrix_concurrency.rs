use super::*;
use bessemer::entity_visual::EntityVisualStoredEdit as Edit;
use bessemer::{BessemerError, FailureClass};
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

type MatrixTrace = Option<ZlibEncoder<Vec<u8>>>;

#[derive(Debug, PartialEq, Eq)]
struct MatrixRelationProjection {
    path: String,
    relation: DisplayFieldRelation,
    parents: Vec<DisplayField>,
    result: Option<DisplayField>,
}

#[derive(Debug, PartialEq, Eq)]
struct MatrixOperationProjection {
    bytes: Vec<u8>,
    fields: Vec<DisplayField>,
    completion: DisplayEditCompletion,
    coverage: DisplayEditCoverage,
    fidelity: DisplayEditFidelity,
    diagnostics: Vec<DisplayEditDiagnostic>,
    history: Vec<MatrixRelationProjection>,
}

#[derive(Debug, PartialEq, Eq)]
struct MatrixMixedOutcome {
    changed: MatrixOperationProjection,
    rejected: BessemerError,
    unchanged: MatrixOperationProjection,
}

fn matrix_relation_projection(
    source: &DisplayArchive,
    result: &DisplayArchiveEdit,
    change: &DisplayFieldChange,
) -> MatrixRelationProjection {
    MatrixRelationProjection {
        path: change.path().to_owned(),
        relation: change.relation(),
        parents: change
            .parents()
            .iter()
            .map(|parent| source.resolve_field(parent).unwrap().clone())
            .collect(),
        result: change
            .result()
            .map(|field| result.archive().resolve_field(field).unwrap().clone()),
    }
}

fn matrix_operation_projection(
    source: &DisplayArchive,
    result: &DisplayArchiveEdit,
) -> MatrixOperationProjection {
    MatrixOperationProjection {
        bytes: result.archive().to_bytes().unwrap(),
        fields: result.archive().contents().fields.clone(),
        completion: result.completion(),
        coverage: result.coverage(),
        fidelity: result.fidelity(),
        diagnostics: result.diagnostics().to_vec(),
        history: result
            .changes()
            .iter()
            .map(|change| matrix_relation_projection(source, result, change))
            .collect(),
    }
}

fn matrix_shared_rgb(
    source: &DisplayArchive,
    state: &DisplayEntityVisualStoredState,
) -> DisplayArchiveEdit {
    let color = state.words().color_word ^ 0x00a53cf0;
    let rgb = [color as u8, (color >> 8) as u8, (color >> 16) as u8];
    source
        .edit_entity_visual_state(state, Edit::ColorRgb(rgb), archive_support::LIMITS)
        .unwrap()
}

fn matrix_shared_rejection(
    source: &DisplayArchive,
    state: &DisplayEntityVisualStoredState,
) -> BessemerError {
    let request = Edit::LineWidth {
        standard: 0,
        custom: None,
    };
    let error = source
        .edit_entity_visual_state(state, request, archive_support::LIMITS)
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    error
}

fn matrix_shared_noop(
    source: &DisplayArchive,
    state: &DisplayEntityVisualStoredState,
) -> DisplayArchiveEdit {
    let request = Edit::ColorWord(state.words().color_word);
    let result = source
        .edit_entity_visual_state(state, request, archive_support::LIMITS)
        .unwrap();
    assert_eq!(result.completion(), DisplayEditCompletion::Unchanged);
    assert!(result.changes().is_empty());
    assert!(result.archive().resolve_field(state.source_field()).is_ok());
    result
}

fn matrix_mixed_outcome(
    source: &DisplayArchive,
    state: &DisplayEntityVisualStoredState,
    index: usize,
) -> MatrixMixedOutcome {
    let changed = matrix_shared_rgb(source, state);
    assert_eq!(changed.completion(), DisplayEditCompletion::Changed);
    assert_eq!(changed.changes().len(), 1);
    verify_component_field_history(source, &changed, index + 3);
    assert!(
        changed
            .archive()
            .entity_visual_state(state.source_field())
            .is_err()
    );
    let rejected = matrix_shared_rejection(source, state);
    let unchanged = matrix_shared_noop(source, state);
    MatrixMixedOutcome {
        changed: matrix_operation_projection(source, &changed),
        rejected,
        unchanged: matrix_operation_projection(source, &unchanged),
    }
}

fn verify_matrix_serial_outcome(
    source: &DisplayArchive,
    raw: &[u8],
    index: usize,
    offset: usize,
    serial: &MatrixMixedOutcome,
) {
    let color = u32::from_le_bytes(raw[offset + 5..offset + 9].try_into().unwrap()) ^ 0x00a53cf0;
    let mut expected = raw.to_vec();
    expected[offset + 5..offset + 9].copy_from_slice(&color.to_le_bytes());
    assert_eq!(serial.changed.bytes, expected);
    let mut fields = source.contents().fields.clone();
    fields[index + 3].value = DisplayFieldValue::U32(color);
    assert_eq!(serial.changed.fields, fields);
    assert_eq!(serial.unchanged.bytes, raw);
    assert_eq!(serial.unchanged.fields, source.contents().fields);
    verify_matrix_operation_coverage(source, raw, &serial.changed);
}

fn verify_matrix_shared_workers(
    source: &DisplayArchive,
    state: &DisplayEntityVisualStoredState,
    index: usize,
    serial: &MatrixMixedOutcome,
) {
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| matrix_mixed_outcome(source, state, index)))
            .collect();
        for worker in workers {
            assert_eq!(&worker.join().unwrap(), serial);
        }
    });
}

fn concurrent_matrix_case(
    template: &ComponentMatrixTemplate,
    columns: &[&str],
) -> (Vec<u8>, MatrixMixedOutcome) {
    let words = component_matrix_word_bytes(columns);
    let raw = component_matrix_raw(template, columns, &words);
    let source = matrix_archive(&raw, template.kind);
    let index = matrix_visual_position(&source);
    let states = source.entity_visual_states().unwrap();
    assert_eq!(states.len(), 1);
    assert_eq!(
        source.contents().fields[index..index + 7]
            .iter()
            .map(|field| field.value.clone())
            .collect::<Vec<_>>(),
        bit_edits::component_literal_words(&words)
    );
    let serial = matrix_mixed_outcome(&source, &states[0], index);
    verify_matrix_serial_outcome(&source, &raw, index, template.offset, &serial);
    verify_matrix_shared_workers(&source, &states[0], index, &serial);
    assert_eq!(source.to_bytes().unwrap(), raw);
    let rebuilt = component_matrix_forced_bytes(&source);
    assert_eq!(rebuilt, raw);
    (rebuilt, serial)
}

fn append_concurrent_matrix_trace(
    trace: &mut MatrixTrace,
    index: usize,
    serial: &MatrixMixedOutcome,
) {
    let Some(trace) = trace else {
        return;
    };
    writeln!(trace, "{index}\t{serial:?}").unwrap();
}

fn run_concurrent_component_matrix(
    output: &mut Option<Vec<u8>>,
    trace: &mut MatrixTrace,
) -> [usize; 2] {
    let templates = component_matrix_templates();
    let mut counts = [0, 0];
    for (index, row) in include_str!("../fixtures/entity-visual-state/component-words.tsv")
        .lines()
        .enumerate()
    {
        let columns: Vec<_> = row.split('\t').collect();
        let template = matrix_template_for(&templates, columns[0]);
        let (raw, serial) = concurrent_matrix_case(template, &columns);
        append_matrix_outputs(output, &raw, &serial.changed.bytes);
        append_concurrent_matrix_trace(trace, index, &serial);
        counts[usize::from(template.kind == DisplayDocumentKind::Assembly)] += 1;
        println!("VISUAL_COMPONENT_CONCURRENT\t{index}\t8");
    }
    counts
}

#[test]
fn every_complete_component_matrix_snapshot_isolated_across_eight_mixed_workers() {
    let output_path = std::env::var_os("BESSEMER_COMPONENT_VISUAL_CONCURRENT_OUTPUT");
    let trace_path = std::env::var_os("BESSEMER_COMPONENT_VISUAL_CONCURRENT_TRACE");
    let mut output = output_path
        .as_ref()
        .map(|_| 7662_u32.to_le_bytes().to_vec());
    let mut trace = trace_path
        .as_ref()
        .map(|_| ZlibEncoder::new(Vec::new(), Compression::fast()));
    assert_eq!(
        run_concurrent_component_matrix(&mut output, &mut trace),
        [3831, 3831]
    );
    if let (Some(path), Some(output)) = (output_path, output) {
        std::fs::write(path, output).unwrap();
    }
    if let (Some(path), Some(trace)) = (trace_path, trace) {
        std::fs::write(path, trace.finish().unwrap()).unwrap();
    }
}

fn verify_matrix_operation_coverage(
    source: &DisplayArchive,
    raw: &[u8],
    projection: &MatrixOperationProjection,
) {
    assert_eq!(
        projection.coverage.mode,
        DisplayEditCheckMode::ExactStoredArchive
    );
    assert_eq!(projection.coverage.wire_bytes, raw.len());
    assert_eq!(
        projection.coverage.typed_fields,
        source.contents().fields.len()
    );
    assert_eq!(projection.fidelity, DisplayEditFidelity::ExactStoredFields);
}
