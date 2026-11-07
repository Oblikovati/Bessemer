use super::classification::verify_classification_edit as selector_change;
use super::edge_index::verify_index_edit as index_change;
use super::*;
use crate::model_operation_snapshot::{ModelVisualOperation, model_visual_operation};
use bessemer::{BessemerError, FailureClass};
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

type RoleTrace = Option<ZlibEncoder<Vec<u8>>>;

#[derive(Debug, PartialEq, Eq)]
struct StoredRoleOutcome {
    changed: ModelVisualOperation,
    unchanged: ModelVisualOperation,
    unchanged_counts: [usize; 2],
    foreign: BessemerError,
    stale: BessemerError,
}

fn role_outcome(
    source: &PartResolvedFeaturesArchive,
    parent: &ModelFieldHandle,
    changed: PartResolvedFeaturesEdit,
    unchanged: PartResolvedFeaturesEdit,
    failures: (BessemerError, BessemerError),
) -> StoredRoleOutcome {
    let (foreign, stale) = failures;
    assert_eq!(unchanged.completion(), ModelEditCompletion::Unchanged);
    assert!(unchanged.changes().is_empty());
    assert!(unchanged.archive().resolve_field(parent).is_ok());
    assert_eq!(unchanged.archive().fields(), source.fields());
    assert_eq!(
        unchanged.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    assert_eq!(foreign.class, FailureClass::Invalid);
    assert_eq!(stale.class, FailureClass::Invalid);
    StoredRoleOutcome {
        changed: model_visual_operation(source, &changed),
        unchanged: model_visual_operation(source, &unchanged),
        unchanged_counts: [changed.unchanged_fields(), unchanged.unchanged_fields()],
        foreign,
        stale,
    }
}

fn selector_outcome(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionReferenceStoredState,
    offset: usize,
) -> StoredRoleOutcome {
    let value = state.classification_selector() ^ 0xa55a;
    let raw = source.to_bytes().unwrap();
    let changed = selector_change(source, &raw, state, offset, value);
    let unchanged = source
        .edit_intersection_reference_classification(state, state.classification_selector())
        .unwrap();
    let rejected = foreign
        .edit_intersection_reference_classification(state, value)
        .unwrap_err();
    let stale = changed
        .archive()
        .edit_intersection_reference_classification(state, value)
        .unwrap_err();
    role_outcome(
        source,
        &state.source_fields()[0],
        changed,
        unchanged,
        (rejected, stale),
    )
}

fn index_outcome(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionEdgeStoredIndex,
    offset: usize,
) -> StoredRoleOutcome {
    let value = !state.value();
    let raw = source.to_bytes().unwrap();
    let changed = index_change(source, &raw, state, offset, value);
    let unchanged = source
        .edit_intersection_edge_index(state, state.value())
        .unwrap();
    let rejected = foreign
        .edit_intersection_edge_index(state, value)
        .unwrap_err();
    let stale = changed
        .archive()
        .edit_intersection_edge_index(state, value)
        .unwrap_err();
    role_outcome(
        source,
        state.source_field(),
        changed,
        unchanged,
        (rejected, stale),
    )
}

fn stored_role_outcomes(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    selectors: &[ModelIntersectionReferenceStoredState],
    indices: &[ModelIntersectionEdgeStoredIndex],
) -> Vec<StoredRoleOutcome> {
    let mut outcomes: Vec<_> = selectors
        .iter()
        .zip([440, 460, 480, 500, 1187, 1207, 1227, 1247])
        .map(|(state, offset)| selector_outcome(source, foreign, state, offset))
        .collect();
    outcomes.extend(
        indices
            .iter()
            .zip([436, 1183])
            .map(|(state, offset)| index_outcome(source, foreign, state, offset)),
    );
    outcomes
}

fn stored_role_concurrent_case(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    raw: &[u8],
) -> Vec<StoredRoleOutcome> {
    let original = source.fields().to_vec();
    let selectors = source.intersection_reference_states().unwrap();
    let indices = source.intersection_edge_indices().unwrap();
    assert_eq!((selectors.len(), indices.len()), (8, 2));
    let serial = stored_role_outcomes(source, foreign, &selectors, &indices);
    crate::visual_workers::assert_eight_visual_workers(&serial, || {
        stored_role_outcomes(source, foreign, &selectors, &indices)
    });
    assert_eq!(source.fields(), original);
    assert_eq!(foreign.fields(), original);
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(foreign.to_bytes().unwrap(), raw);
    serial
}

fn append_stored_role_case(
    output: &mut Option<Vec<u8>>,
    trace: &mut RoleTrace,
    index: usize,
    raw: &[u8],
    serial: &[StoredRoleOutcome],
) {
    append_intersection_export(output, raw);
    for outcome in serial {
        append_intersection_export(output, &outcome.changed.bytes);
    }
    if let Some(trace) = trace {
        writeln!(trace, "{index}\t{serial:?}").unwrap();
    }
}

fn stored_role_concurrent_matrix(output: &mut Option<Vec<u8>>, trace: &mut RoleTrace) {
    let (context, manager) = intersection_context();
    let original = context.to_bytes().unwrap();
    let mut cursor = ROLE_PACKET;
    for index in 0..264 {
        let raw = next_intersection_image(&mut cursor);
        let source = intersection_archive(raw, &context, &manager);
        let foreign = intersection_archive(raw, &context, &manager);
        let serial = stored_role_concurrent_case(&source, &foreign, raw);
        append_stored_role_case(output, trace, index, raw, &serial);
        println!("STORED_ROLE_CONCURRENT\t{index}\t8\t8\t2");
    }
    assert_eq!(next_intersection_image(&mut cursor).len(), 1672584);
    assert!(cursor.is_empty());
    assert_eq!(context.to_bytes().unwrap(), original);
}

#[test]
fn every_stored_role_matrix_case_isolates_mixed_requests_across_eight_workers() {
    let output_path = std::env::var_os("BESSEMER_STORED_ROLE_CONCURRENT_OUTPUT");
    let trace_path = std::env::var_os("BESSEMER_STORED_ROLE_CONCURRENT_TRACE");
    let mut output = output_path.as_ref().map(|_| 264_u32.to_le_bytes().to_vec());
    let mut trace = trace_path
        .as_ref()
        .map(|_| ZlibEncoder::new(Vec::new(), Compression::fast()));
    stored_role_concurrent_matrix(&mut output, &mut trace);
    write_intersection_export(output_path, output);
    if let (Some(path), Some(trace)) = (trace_path, trace) {
        std::fs::write(path, trace.finish().unwrap()).unwrap();
    }
}
