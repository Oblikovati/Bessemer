use super::*;
use bessemer::FailureClass;
use std::io::Write;
fn stored(version: i32, body: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(body).unwrap();
    let mut raw = version.to_le_bytes().to_vec();
    raw.extend(encoder.finish().unwrap());
    raw
}
fn sample() -> ViewerCase {
    cases()
        .into_iter()
        .find(|row| row.name == "opaque-0")
        .unwrap()
}
#[test]
fn exact_full_decoded_and_known_wide_limits_accept_and_one_below_rejects() {
    for row in cases() {
        check_limits(&row);
    }
    let row = sample();
    for maximum in [0, 1, 71] {
        let mut limits = options();
        limits.maximum_decoded_bytes = maximum;
        assert!(open(&row.frame.stored, limits).is_err());
    }
    for maximum in 0..6 {
        let mut limits = options();
        limits.maximum_prefix_wide_values = maximum;
        assert!(open(&row.frame.stored, limits).is_err());
    }
}
fn check_limits(row: &ViewerCase) {
    let count = 3 + row
        .arrays
        .iter()
        .map(|array| array.count as usize)
        .sum::<usize>();
    let mut limits = FlowViewerPrefixOptions {
        maximum_decoded_bytes: row.frame.decoded.len(),
        maximum_prefix_wide_values: count,
        ..options()
    };
    assert!(open(&row.frame.stored, limits).is_ok());
    limits.maximum_decoded_bytes -= 1;
    assert!(open(&row.frame.stored, limits).is_err());
    limits.maximum_decoded_bytes += 1;
    limits.maximum_prefix_wide_values -= 1;
    assert!(open(&row.frame.stored, limits).is_err());
}
#[test]
fn empty_or_extended_opaque_continuations_never_become_complete_body_passes() {
    let row = sample();
    let view = check_case(&row);
    assert!(view.opaque_tail().is_empty());
    assert_eq!(
        view.coverage().body_validation,
        FlowViewerBodyValidation::Incomplete
    );
    for suffix in [
        vec![0],
        vec![255],
        b"unknown body data".to_vec(),
        vec![0; 4096],
    ] {
        check_extended_tail(&row, &suffix);
    }
}
fn check_extended_tail(row: &ViewerCase, suffix: &[u8]) {
    let mut decoded = row.frame.decoded.clone();
    decoded.extend_from_slice(suffix);
    let after = open(&stored(row.frame.version, &decoded), options()).unwrap();
    assert_eq!(
        after.coverage().known_decoded_prefix_bytes,
        row.extent.prefix as usize
    );
    assert_eq!(after.opaque_tail(), suffix);
    assert_eq!(
        after.coverage().body_validation,
        FlowViewerBodyValidation::Incomplete
    );
}
#[test]
fn known_prefix_truncations_and_nonpositive_or_impossible_lengths_are_rejected() {
    let row = sample();
    for end in 0..row.extent.prefix as usize {
        assert!(open(&stored(3, &row.frame.decoded[..end]), options()).is_err());
    }
    for (ordinal, length) in [(0, 0i32), (1, -1), (2, i32::MIN), (0, i32::MAX)] {
        let mut decoded = row.frame.decoded.clone();
        decoded[8 + ordinal * 4..12 + ordinal * 4].copy_from_slice(&length.to_le_bytes());
        let mut limits = options();
        limits.maximum_prefix_wide_values = usize::MAX;
        assert!(open(&stored(3, &decoded), limits).is_err());
    }
}
#[test]
fn unsupported_outer_versions_and_corrupt_truncated_concatenated_or_extended_streams_fail() {
    let row = sample();
    for version in [i32::MIN, -1, 0, 1] {
        assert_eq!(
            open(&version.to_le_bytes(), options()).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
    for end in 0..row.frame.stored.len() {
        assert!(open(&row.frame.stored[..end], options()).is_err());
    }
    for suffix in [vec![0], b"junk".to_vec(), row.frame.stored[4..].to_vec()] {
        let mut encoded = row.frame.stored.clone();
        encoded.extend(suffix);
        assert!(open(&encoded, options()).is_err());
    }
    let mut corrupt = row.frame.stored;
    let end = corrupt.len() - 1;
    corrupt[end] ^= 1;
    assert!(open(&corrupt, options()).is_err());
}
#[test]
fn invalid_indices_foreign_wrong_and_stale_occurrences_leave_sources_unchanged() {
    let row = sample();
    let view = check_case(&row);
    let before = view.source().as_bytes().to_vec();
    for (ordinal, index) in [
        (0, usize::MAX),
        (0, view.array(0).unwrap().word_count()),
        (3, 0),
        (usize::MAX, 0),
    ] {
        assert!(view.replace_array_pattern(ordinal, index, 0).is_err());
    }
    assert!(
        view.replace_discarded_pattern(3, 0).is_err()
            && view.replace_discarded_pattern(usize::MAX, 0).is_err()
    );
    check_occurrences(&view);
    assert_eq!(view.source().as_bytes(), before);
}
fn check_occurrences(view: &FlowViewerPrefix) {
    let stored = view.source().records().nth(1).unwrap().payload;
    let foreign = open(stored, options()).unwrap();
    reject_id(view.source(), foreign.record());
    let wrong = view.source().records().nth(2).unwrap().id;
    reject_id(view.source(), &wrong);
    let edit = view
        .replace_signed(FlowViewerSignedField::FinalWord, -7)
        .unwrap();
    reject_id(edit.result().source(), view.record());
}
fn reject_id(source: &FlowCarrier, id: &FlowRecordId) {
    assert!(source.record_121_viewer_prefix(id, options()).is_err());
}
#[test]
fn all_new_and_reused_public_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<FlowViewerPrefixProfile>();
    check::<FlowViewerPrefixOptions>();
    check::<FlowViewerPrefix>();
    check::<FlowViewerMetadata>();
    check::<FlowViewerSignedField>();
    check::<FlowViewerPrefixCheckMode>();
    check::<FlowViewerBodyValidation>();
    check::<FlowViewerPrefixCoverage>();
    check::<FlowViewerPrefixDiagnostic>();
    check::<FlowViewerPrefixEdit>();
    check::<FlowTableWideArray<'static>>();
    check::<FlowCarrierEdit>();
}
