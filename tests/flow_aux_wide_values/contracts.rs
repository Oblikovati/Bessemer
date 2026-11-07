use super::*;
use bessemer::FailureClass;
use std::io::Write;
fn compress(raw: &[u8]) -> Vec<u8> {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(raw).unwrap();
    encoder.finish().unwrap()
}
fn raw(version: i32, count: i32) -> Vec<u8> {
    let mut result = version.to_le_bytes().to_vec();
    if version == 0 {
        result.extend_from_slice(&(-7i32).to_le_bytes());
    }
    result.extend_from_slice(&count.to_le_bytes());
    result
}
#[test]
fn all_exact_resource_limits_accept_and_one_below_limits_reject() {
    for row in cases() {
        check_limits(&row);
    }
    for maximum in 0..8 {
        let limits = FlowAuxWideOptions {
            maximum_decoded_bytes: maximum,
            maximum_values: 0,
        };
        assert!(open(&compress(&raw(1, 0)), limits).is_err());
    }
}
fn check_limits(row: &AuxWideCase) {
    let mut limits = FlowAuxWideOptions {
        maximum_decoded_bytes: row.decoded.len(),
        maximum_values: row.count as usize,
    };
    assert!(open(&row.stored, limits).is_ok());
    limits.maximum_decoded_bytes -= 1;
    assert!(open(&row.stored, limits).is_err());
    limits.maximum_decoded_bytes += 1;
    if row.count == 0 {
        return;
    }
    limits.maximum_values -= 1;
    assert!(open(&row.stored, limits).is_err());
}
#[test]
fn every_small_decoded_prefix_extension_and_invalid_count_or_version_is_rejected() {
    for version in 0..=1 {
        for count in [0, 1, 2, 7] {
            check_decoded_framing(version, count);
        }
        for count in [i32::MIN, -1, 1, i32::MAX] {
            assert!(open(&compress(&raw(version, count)), options()).is_err());
        }
    }
    for version in [i32::MIN, -1, 2, i32::MAX] {
        let error = open(&compress(&raw(version, 0)), options()).unwrap_err();
        assert_eq!(error.class, FailureClass::Unsupported);
    }
}
fn check_decoded_framing(version: i32, count: i32) {
    let mut bytes = raw(version, count);
    bytes.resize(bytes.len() + count as usize * 8, 255);
    for end in 0..bytes.len() {
        assert!(open(&compress(&bytes[..end]), options()).is_err());
    }
    for suffix in [vec![0], vec![255], vec![0; 8]] {
        let mut extended = bytes.clone();
        extended.extend(suffix);
        assert!(open(&compress(&extended), options()).is_err());
    }
}
#[test]
fn truncated_corrupted_concatenated_or_extended_compressed_streams_are_rejected() {
    let stored = compress(&raw(0, 0));
    for end in 0..stored.len() {
        assert!(open(&stored[..end], options()).is_err());
    }
    for suffix in [vec![0], b"junk".to_vec(), stored.clone()] {
        let mut extended = stored.clone();
        extended.extend(suffix);
        assert!(open(&extended, options()).is_err());
    }
    let mut corrupt = stored;
    let index = corrupt.len() - 1;
    corrupt[index] ^= 1;
    assert!(open(&corrupt, options()).is_err());
}
#[test]
fn missing_legacy_out_of_range_edits_and_foreign_occurrences_leave_sources_unchanged() {
    let source = open(&compress(&raw(1, 0)), options()).unwrap();
    let original = source.source().as_bytes().to_vec();
    assert_eq!(
        source.replace_discarded_legacy(0).unwrap_err().class,
        FailureClass::Missing
    );
    assert!(
        source.replace_pattern(0, 0).is_err() && source.replace_pattern(usize::MAX, 0).is_err()
    );
    check_foreign_occurrences(&source);
    assert_eq!(source.source().as_bytes(), original);
}
fn check_foreign_occurrences(source: &FlowAuxWideValues) {
    let foreign = open(&compress(&raw(1, 0)), options()).unwrap();
    assert!(
        source
            .source()
            .record_444_aux_wides(foreign.record(), options())
            .is_err()
    );
    let wrong = source.source().records().nth(2).unwrap().id;
    assert!(
        source
            .source()
            .record_444_aux_wides(&wrong, options())
            .is_err()
    );
}
#[test]
fn all_public_new_and_reused_result_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<FlowAuxWideOptions>();
    check::<FlowAuxWideValues>();
    check::<FlowTableWideArray<'static>>();
    check::<FlowSimpleTableCoverage>();
    check::<FlowSimpleTableDiagnostic>();
    check::<FlowSimpleTableEdit<FlowAuxWideValues>>();
    check::<FlowCarrierEdit>();
}
