use super::*;
#[test]
fn all_five_public_readers_match_every_owned_raw_value_interval_and_nested_source_projection() {
    let mut totals = FlowCheckpointCounts::default();
    let mut edits = 0;
    for row in cases() {
        let view = component(&row);
        assert_eq!(view.options(), options());
        let selected = fields(&row, &view);
        edits += selected.len();
        accumulate(&mut totals, view.coverage().counts);
    }
    assert_eq!(totals.wide_patterns, 1267591);
    assert_eq!(totals.indices, 840930);
    assert_eq!(totals.row_positions, 422594);
    assert_eq!(totals.slots, 210681);
    assert_eq!(totals.containers, 837);
    assert_eq!(totals.chunks, 70274);
    assert_eq!(totals.encoded_bytes, 104556);
    assert!(edits > 10_000);
}
pub(super) fn fields(row: &ComponentCase, view: &FlowCheckpointComponent) -> Vec<EditableField> {
    let mut edits = Vec::new();
    let mut counts = FlowCheckpointCounts::default();
    projection_fields(row, view, &mut edits, &mut counts);
    check_coverage(row, view, counts);
    edits
}
fn projection_fields(
    row: &ComponentCase,
    view: &FlowCheckpointComponent,
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) {
    match &row.projection {
        ComponentProjection::Graph1(values) => {
            arrays::wides(view.graph1_patterns().unwrap(), 0, values, edits);
            counts.wide_patterns += values.len();
        }
        ComponentProjection::Grid(expected) => {
            grids::grid(view.grid().unwrap(), expected, edits, counts)
        }
        ComponentProjection::Trailing(expected) => trailing(view, expected, edits, counts),
        ComponentProjection::Chunks(expected) => {
            chunks::check(view.chunk_buffer().unwrap(), expected, edits, counts)
        }
    }
}

fn trailing(
    view: &FlowCheckpointComponent,
    expected: &[ContainerProjection],
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) {
    for (index, expected) in expected.iter().enumerate() {
        arrays::container(
            view.trailing_container(index).unwrap(),
            expected,
            edits,
            counts,
        );
    }
}
fn check_coverage(
    row: &ComponentCase,
    view: &FlowCheckpointComponent,
    counts: FlowCheckpointCounts,
) {
    let coverage = view.coverage();
    assert_eq!(view.as_bytes(), row.raw);
    assert_eq!(view.context(), FlowCheckpointContext::LogicalVersionSix);
    assert_eq!(view.kind(), kind(row.kind));
    assert_eq!(
        coverage.mode,
        FlowCheckpointCheckMode::ExactLogicalComponent
    );
    assert_eq!(coverage.context, view.context());
    assert_eq!(coverage.kind, view.kind());
    assert_eq!(coverage.component_bytes, row.end);
    assert_eq!(coverage.counts, counts);
    assert!(coverage.complete_component_framing);
    assert!(!coverage.complete_parent_record && !coverage.engineering_fields_assigned);
    assert!(!coverage.chunk_codec_decoded && !coverage.decoded_size_verified);
    assert_eq!(coverage.achieved_byte_error, 0);
    check_diagnostics(view, counts);
}
fn check_diagnostics(view: &FlowCheckpointComponent, counts: FlowCheckpointCounts) {
    let diagnostics = view.diagnostics();
    assert!(diagnostics.contains(&FlowCheckpointDiagnostic::LogicalContextAssumed));
    assert!(diagnostics.contains(&FlowCheckpointDiagnostic::InstalledNativeComponentNotVerified));
    assert!(diagnostics.contains(&FlowCheckpointDiagnostic::ParentRecordNotDecoded));
    assert!(diagnostics.contains(&FlowCheckpointDiagnostic::EngineeringMeaningsNotAssigned));
    assert!(diagnostics.contains(&FlowCheckpointDiagnostic::NativeRuntimeNotReproduced));
    assert!(diagnostics.contains(&FlowCheckpointDiagnostic::NativeApplicationNotChecked));
    assert_eq!(
        diagnostics.contains(&FlowCheckpointDiagnostic::NoncanonicalWriterTagsRetained),
        counts.noncanonical_tags != 0
    );
    assert_eq!(
        diagnostics.contains(&FlowCheckpointDiagnostic::ChunkCodecNotDecoded),
        view.kind() == FlowCheckpointKind::ChunkBuffer
    );
    assert_eq!(
        diagnostics.contains(&FlowCheckpointDiagnostic::DeclaredRawSizeNotVerified),
        view.kind() == FlowCheckpointKind::ChunkBuffer
    );
}
fn accumulate(total: &mut FlowCheckpointCounts, next: FlowCheckpointCounts) {
    total.row_positions += next.row_positions;
    total.present_rows += next.present_rows;
    total.indices += next.indices;
    total.slots += next.slots;
    total.containers += next.containers;
    total.wide_patterns += next.wide_patterns;
    total.chunks += next.chunks;
    total.encoded_bytes += next.encoded_bytes;
    total.noncanonical_tags += next.noncanonical_tags;
}
