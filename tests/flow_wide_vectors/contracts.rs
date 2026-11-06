use super::*;
use bessemer::FailureClass;
fn raw(arrays: &[&[u64]]) -> Vec<u8> {
    let mut raw = 0i32.to_le_bytes().to_vec();
    raw.extend_from_slice(&(arrays.len() as i32).to_le_bytes());
    for array in arrays {
        raw.extend_from_slice(&(array.len() as i32).to_le_bytes());
    }
    for array in arrays {
        for value in *array {
            raw.extend_from_slice(&value.to_le_bytes());
        }
    }
    raw
}
#[test]
fn all_three_exact_resource_bounds_accept_and_one_below_bounds_reject() {
    for row in cases() {
        check_limits(&row);
    }
    for maximum in 0..8 {
        let limits = FlowWideVectorOptions {
            maximum_payload_bytes: maximum,
            maximum_vectors: 0,
            maximum_values: 0,
        };
        assert!(open(&raw(&[]), limits).is_err());
    }
}
fn check_limits(row: &VectorCase) {
    let mut limits = FlowWideVectorOptions {
        maximum_payload_bytes: row.raw.len(),
        maximum_vectors: row.arrays.len(),
        maximum_values: row.arrays.iter().map(|array| array.length as usize).sum(),
    };
    assert!(open(&row.raw, limits).is_ok());
    limits.maximum_payload_bytes -= 1;
    assert!(open(&row.raw, limits).is_err());
    limits.maximum_payload_bytes += 1;
    if limits.maximum_vectors > 0 {
        check_vector_bound(row, limits);
    }
    if limits.maximum_values == 0 {
        return;
    }
    limits.maximum_values -= 1;
    assert!(open(&row.raw, limits).is_err());
}
fn check_vector_bound(row: &VectorCase, mut limits: FlowWideVectorOptions) {
    limits.maximum_vectors -= 1;
    assert!(open(&row.raw, limits).is_err());
}
#[test]
fn zero_value_limits_admit_stored_empty_vectors_and_aggregate_limits_apply_across_vectors() {
    let bytes = raw(&[&[], &[], &[]]);
    let limits = FlowWideVectorOptions {
        maximum_payload_bytes: bytes.len(),
        maximum_vectors: 3,
        maximum_values: 0,
    };
    let view = open(&bytes, limits).unwrap();
    assert_eq!(view.stored_count(), 3);
    assert_eq!(view.coverage().empty_vectors, 3);
    let bytes = raw(&[&[11], &[12]]);
    let mut limits = options();
    limits.maximum_values = 1;
    assert!(open(&bytes, limits).is_err());
    limits.maximum_values = 2;
    assert!(open(&bytes, limits).is_ok());
}
#[test]
fn every_small_prefix_extension_and_unsupported_version_is_rejected() {
    let samples = [
        raw(&[]),
        raw(&[&[]]),
        raw(&[&[0]]),
        raw(&[&[11], &[], &[12, 13]]),
        raw(&[&[], &[], &[]]),
    ];
    for bytes in samples {
        check_malformed(&bytes);
    }
    for version in [i32::MIN, -1, 1, i32::MAX] {
        let mut bytes = raw(&[]);
        bytes[..4].copy_from_slice(&version.to_le_bytes());
        assert_eq!(
            open(&bytes, options()).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
}
fn check_malformed(bytes: &[u8]) {
    for end in 0..bytes.len() {
        assert!(open(&bytes[..end], options()).is_err());
    }
    for suffix in [vec![0], vec![255], vec![0; 8]] {
        let mut extended = bytes.to_vec();
        extended.extend(suffix);
        assert!(open(&extended, options()).is_err());
    }
}
#[test]
fn invalid_counts_and_lengths_and_impossible_totals_do_not_allocate_successful_partial_views() {
    for value in [i32::MIN, -1, 1, i32::MAX] {
        let mut count = raw(&[]);
        count[4..8].copy_from_slice(&value.to_le_bytes());
        assert!(open(&count, options()).is_err());
        let mut length = raw(&[&[]]);
        length[8..12].copy_from_slice(&value.to_le_bytes());
        assert!(open(&length, options()).is_err());
    }
    let mut bytes = raw(&[&[11], &[12]]);
    bytes[8..12].copy_from_slice(&i32::MAX.to_le_bytes());
    assert!(open(&bytes, options()).is_err());
}
#[test]
fn failed_edits_and_wrong_foreign_or_stale_carrier_occurrences_preserve_sources() {
    let view = open(&raw(&[&[], &[7]]), options()).unwrap();
    let original = view.source().as_bytes().to_vec();
    for (vector, index) in [(0, 0), (1, 1), (2, 0), (usize::MAX, 0), (1, usize::MAX)] {
        assert!(view.replace_pattern(vector, index, 0).is_err());
    }
    check_occurrences(&view);
    let edit = view.replace_pattern(1, 0, 8).unwrap();
    assert!(
        edit.result()
            .source()
            .record_148_wide_vectors(view.record(), options())
            .is_err()
    );
    assert_eq!(view.source().as_bytes(), original);
    assert_eq!(view.vector(1).unwrap().raw_bits(0), Some(7));
}
fn check_occurrences(view: &FlowWideVectors) {
    let foreign = open(view.decoded_bytes(), options()).unwrap();
    assert!(
        view.source()
            .record_148_wide_vectors(foreign.record(), options())
            .is_err()
    );
    let wrong = view.source().records().nth(2).unwrap().id;
    assert!(
        view.source()
            .record_148_wide_vectors(&wrong, options())
            .is_err()
    );
}
#[test]
fn all_new_and_reused_public_result_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<FlowWideVectors>();
    check::<FlowWideVectorOptions>();
    check::<FlowWideVectorCoverage>();
    check::<FlowTableWideArray<'static>>();
    check::<FlowSimpleTableDiagnostic>();
    check::<FlowSimpleTableEdit<FlowWideVectors>>();
    check::<FlowCarrierEdit>();
}
