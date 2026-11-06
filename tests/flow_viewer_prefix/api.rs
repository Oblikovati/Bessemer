use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "edit.rs"]
mod edit;
#[path = "replay.rs"]
mod replay;
fn options() -> FlowViewerPrefixOptions {
    FlowViewerPrefixOptions {
        profile: FlowViewerPrefixProfile::InflateMetadataQuery,
        maximum_decoded_bytes: 2_000_000,
        maximum_prefix_wide_values: 1_000_000,
    }
}
fn open(raw: &[u8], options: FlowViewerPrefixOptions) -> bessemer::Result<FlowViewerPrefix> {
    let source = state_carrier::carrier(121, raw);
    let id = source.records().nth(1).unwrap().id;
    source.record_121_viewer_prefix(&id, options)
}
fn check_case(row: &ViewerCase) -> FlowViewerPrefix {
    let view = open(&row.frame.stored, options()).unwrap();
    assert_eq!(view.options(), options());
    assert_eq!(view.outer_version(), row.frame.version);
    assert_eq!(view.record(), &view.source().records().nth(1).unwrap().id);
    assert_eq!(view.decoded_body(), row.frame.decoded);
    assert_eq!(view.opaque_tail(), row.tail);
    check_scalars(&view, row);
    for (ordinal, array) in row.arrays.iter().enumerate() {
        check_array(view.array(ordinal).unwrap(), array);
    }
    check_triplet(&view, row);
    check_coverage(&view, row);
    assert!(view.array(3).is_err() && view.array(usize::MAX).is_err());
    view
}
fn check_scalars(view: &FlowViewerPrefix, row: &ViewerCase) {
    assert_eq!(view.ignored_initial(), row.frame.initial);
    assert_eq!(view.stored_second(), row.frame.second);
    assert_eq!(view.final_word(), row.final_word);
    assert_eq!(
        view.native_metadata(),
        FlowViewerMetadata {
            first_i32: row.metadata[0],
            second_i32: row.metadata[1]
        }
    );
}

fn check_array(array: FlowTableWideArray<'_>, expected: &ViewerArray) {
    assert_eq!(array.as_bytes(), expected.raw);
    assert_eq!(array.word_count(), expected.count as usize);
    assert_eq!(
        array.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    check_patterns(array, &expected.raw);
}
fn check_patterns(array: FlowTableWideArray<'_>, raw: &[u8]) {
    let (values, remainder) = raw.as_chunks::<8>();
    assert!(remainder.is_empty());
    for (index, value) in values.iter().enumerate() {
        assert_eq!(array.raw_bits(index), Some(u64::from_le_bytes(*value)));
    }
    assert!(array.raw_bits(array.word_count()).is_none() && array.raw_bits(usize::MAX).is_none());
}
fn check_triplet(view: &FlowViewerPrefix, row: &ViewerCase) {
    let array = view.discarded_triplet();
    assert_eq!(array.word_count(), 3);
    assert_eq!(array.as_bytes(), row.triplet);
    let start = row.extent.prefix as usize - 28;
    assert_eq!(array.payload_interval(), (start, start + 24));
    check_patterns(array, &row.triplet);
}
fn check_coverage(view: &FlowViewerPrefix, row: &ViewerCase) {
    let proof = view.coverage();
    check_scope(proof);
    assert_eq!(proof.stored_payload_bytes, row.frame.stored.len());
    assert_eq!(proof.decoded_body_bytes, row.frame.decoded.len());
    assert_eq!(proof.known_decoded_prefix_bytes, row.extent.prefix as usize);
    assert_eq!(proof.opaque_decoded_bytes, row.tail.len());
    assert_eq!(
        proof.known_wide_values,
        3 + row
            .arrays
            .iter()
            .map(|array| array.count as usize)
            .sum::<usize>()
    );
    check_diagnostics(view);
}
fn check_scope(proof: FlowViewerPrefixCoverage) {
    assert_eq!(
        proof.mode,
        FlowViewerPrefixCheckMode::ExactVersionZlibAndKnownPrefix
    );
    assert_eq!(proof.profile, FlowViewerPrefixProfile::InflateMetadataQuery);
    assert_eq!(proof.raw_outer_prefix_bytes, 4);
    assert_eq!(proof.body_validation, FlowViewerBodyValidation::Incomplete);
    assert_eq!(proof.achieved_prefix_error, 0);
}

fn check_diagnostics(view: &FlowViewerPrefix) {
    use FlowViewerPrefixDiagnostic::*;
    assert_eq!(
        view.diagnostics(),
        &[
            ViewerQueryProfileAssumed,
            OpaqueBodyNotValidated,
            EngineeringRolesNotResolved,
            NativeRuntimeNotReproduced,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn every_owned_viewer_field_array_metadata_output_and_opaque_byte_matches_the_source() {
    for row in cases() {
        check_case(&row);
    }
}
