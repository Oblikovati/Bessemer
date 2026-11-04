use super::*;

#[test]
#[ignore = "requires explicit BESSEMER_INSPECTION_PDF_SOURCE installed tutorial PDF"]
fn original_tutorial_source_and_modified_or_truncated_bytes_match_independent_native_oracles() {
    let path = std::env::var_os("BESSEMER_INSPECTION_PDF_SOURCE")
        .expect("expected explicit tutorial PDF source path");
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(bytes.len(), 164616);
    let envelope = InspectionEnvelope::from_xml(
        InspectionProfile::Historical2015,
        include_bytes!("../fixtures/standalone-inspection/historical.xml"),
        LIMITS,
    )
    .unwrap();
    let source = InspectionRecords::from_envelope(&envelope).unwrap();
    check_native_variants(&source, &bytes);
}

fn check_native_variants(source: &InspectionRecords, bytes: &[u8]) {
    let mut modified = bytes.to_vec();
    let last = modified.len() - 1;
    modified[last] ^= 1;
    for (input, digest, size_matches, md5_matches) in [
        (bytes, "32c2e42f4813cfedb8edc1d7a5b39b8e", true, true),
        (
            modified.as_slice(),
            "2224c75b93c5d26dba7e8a753b1ab158",
            true,
            false,
        ),
        (
            &bytes[..last],
            "dcfa7a144d3be62c7e01d2a421c90fc8",
            false,
            false,
        ),
    ] {
        check_native_comparison(source, input, digest, (size_matches, md5_matches));
    }
}

fn check_native_comparison(
    source: &InspectionRecords,
    bytes: &[u8],
    digest: &str,
    matches: (bool, bool),
) {
    let result = source
        .compare_document_source(&document(source).id, bytes)
        .unwrap();
    assert_eq!(result.values.actual_md5.as_ref(), digest);
    assert_eq!(
        (result.values.size_matches, result.values.md5_matches),
        matches
    );
    check_comparison_contract(source, &result, bytes.len());
}
