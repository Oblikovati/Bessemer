use super::*;
use crate::model_operation_snapshot::{ModelVisualOperation, model_visual_operation};
use bessemer::{BessemerError, FailureClass};
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

type ReferenceTrace = Option<ZlibEncoder<Vec<u8>>>;
type ChangedReferenceSnapshot = (ModelVisualOperation, usize, BessemerError);

#[derive(Debug, PartialEq, Eq)]
struct ReferenceConcurrentOutcome {
    changed: Vec<ChangedReferenceSnapshot>,
    unchanged: Vec<(ModelVisualOperation, usize)>,
    foreign: BessemerError,
}

fn concurrent_reference_change(
    source: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
    offset: usize,
    request: (ModelIntersectionReferenceStoredEdit, [i32; 2]),
) -> ChangedReferenceSnapshot {
    let result = source
        .edit_intersection_reference_state(state, request.0)
        .unwrap();
    contracts::verify_reference_edit(
        source,
        &source.to_bytes().unwrap(),
        state,
        &result,
        offset,
        request.1,
    );
    let stale = result
        .archive()
        .edit_intersection_reference_state(state, request.0)
        .unwrap_err();
    assert_eq!(stale.class, FailureClass::Invalid);
    let snapshot = concurrent_reference_snapshot(source, &result);
    (snapshot.0, snapshot.1, stale)
}

fn concurrent_reference_noop(
    source: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
    request: ModelIntersectionReferenceStoredEdit,
) -> (ModelVisualOperation, usize) {
    let result = source
        .edit_intersection_reference_state(state, request)
        .unwrap();
    assert_eq!(result.completion(), ModelEditCompletion::Unchanged);
    assert!(result.changes().is_empty());
    assert_eq!(result.archive().fields(), source.fields());
    assert_eq!(
        result.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    assert_eq!(
        &result
            .archive()
            .intersection_reference_state(&state.source_fields()[0])
            .unwrap(),
        state
    );
    concurrent_reference_snapshot(source, &result)
}

fn concurrent_reference_snapshot(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
) -> (ModelVisualOperation, usize) {
    (
        model_visual_operation(source, result),
        result.unchanged_fields(),
    )
}

fn concurrent_reference_outcome(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
    offset: usize,
) -> ReferenceConcurrentOutcome {
    let changed = contracts::reference_requests(state.words())
        .into_iter()
        .map(|request| concurrent_reference_change(source, state, offset, request))
        .collect();
    let unchanged = contracts::reference_noop_requests(state.words())
        .into_iter()
        .map(|request| concurrent_reference_noop(source, state, request))
        .collect();
    let foreign = foreign
        .edit_intersection_reference_state(
            state,
            ModelIntersectionReferenceStoredEdit::SpanCountState(0),
        )
        .unwrap_err();
    assert_eq!(foreign.class, FailureClass::Invalid);
    ReferenceConcurrentOutcome {
        changed,
        unchanged,
        foreign,
    }
}

fn concurrent_reference_outcomes(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    states: &[ModelIntersectionReferenceStoredState],
) -> Vec<ReferenceConcurrentOutcome> {
    states
        .iter()
        .zip([440, 460, 480, 500, 1187, 1207, 1227, 1247])
        .map(|(state, offset)| concurrent_reference_outcome(source, foreign, state, offset))
        .collect()
}

fn concurrent_reference_case(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    raw: &[u8],
    rows: &[&str; 8],
) -> Vec<ReferenceConcurrentOutcome> {
    let original = source.fields().to_vec();
    let states = source.intersection_reference_states().unwrap();
    assert_eq!(states.len(), 8);
    for (state, row) in states.iter().zip(rows) {
        assert_eq!(
            state.words(),
            contracts::reference_oracle(&row.split('\t').collect::<Vec<_>>())
        );
    }
    let serial = concurrent_reference_outcomes(source, foreign, &states);
    crate::visual_workers::assert_eight_visual_workers(&serial, || {
        concurrent_reference_outcomes(source, foreign, &states)
    });
    assert_eq!(source.fields(), original);
    assert_eq!(foreign.fields(), original);
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(foreign.to_bytes().unwrap(), raw);
    serial
}

fn append_concurrent_reference_case(
    output: &mut Option<Vec<u8>>,
    trace: &mut ReferenceTrace,
    index: usize,
    raw: &[u8],
    serial: &[ReferenceConcurrentOutcome],
) {
    append_intersection_export(output, raw);
    for outcome in serial {
        for (changed, _, _) in &outcome.changed {
            append_intersection_export(output, &changed.bytes);
        }
    }
    if let Some(trace) = trace {
        writeln!(trace, "{index}\t{serial:?}").unwrap();
    }
}

fn concurrent_reference_matrix(output: &mut Option<Vec<u8>>, trace: &mut ReferenceTrace) {
    let (context, manager) = intersection_context();
    let original = context.to_bytes().unwrap();
    let mut cursor = MATRIX;
    let rows: Vec<_> = WORDS.lines().collect();
    assert_eq!(rows.len(), 10752);
    for (index, words) in rows.as_chunks::<8>().0.iter().enumerate() {
        let raw = next_intersection_image(&mut cursor);
        let source = intersection_archive(raw, &context, &manager);
        let foreign = intersection_archive(raw, &context, &manager);
        let serial = concurrent_reference_case(&source, &foreign, raw, words);
        append_concurrent_reference_case(output, trace, index, raw, &serial);
        println!("INTERSECTION_CONCURRENT\t{index}\t8\t8");
    }
    assert!(cursor.is_empty());
    assert_eq!(context.to_bytes().unwrap(), original);
}

#[test]
fn every_intersection_reference_matrix_isolates_mixed_requests_across_eight_workers() {
    let output_path = std::env::var_os("BESSEMER_INTERSECTION_CONCURRENT_OUTPUT");
    let trace_path = std::env::var_os("BESSEMER_INTERSECTION_CONCURRENT_TRACE");
    let mut output = output_path
        .as_ref()
        .map(|_| 1344_u32.to_le_bytes().to_vec());
    let mut trace = trace_path
        .as_ref()
        .map(|_| ZlibEncoder::new(Vec::new(), Compression::fast()));
    concurrent_reference_matrix(&mut output, &mut trace);
    write_intersection_export(output_path, output);
    if let (Some(path), Some(trace)) = (trace_path, trace) {
        std::fs::write(path, trace.finish().unwrap()).unwrap();
    }
}
