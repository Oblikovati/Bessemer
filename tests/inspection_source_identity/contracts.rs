use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ComparisonSnapshot {
    values: InspectionSourceIdentityValues,
    completion: InspectionSourceCompletion,
    coverage: InspectionSourceCoverage,
    diagnostics: [InspectionSourceDiagnostic; 3],
    history: [InspectionRecordHistory; 1],
    document: InspectionRecordId,
    xml: Vec<u8>,
}

fn snapshot(result: InspectionSourceComparison) -> ComparisonSnapshot {
    ComparisonSnapshot {
        xml: result.source.envelope().xml_bytes().to_vec(),
        document: result.document,
        values: result.values,
        completion: result.completion,
        coverage: result.coverage,
        diagnostics: result.diagnostics,
        history: result.history,
    }
}

#[test]
fn caller_byte_mutation_and_failed_comparisons_preserve_immutable_results_and_source() {
    let mut bytes = b"abc".to_vec();
    let source = canonical_project(3, "900150983cd24fb0d6963f7d28e17f72");
    let id = &document(&source).id;
    let original = source.envelope().xml_bytes().to_vec();
    let checked = source.compare_document_source(id, &bytes).unwrap();
    bytes[2] ^= 1;
    let unchanged_digest = checked.values.actual_md5.as_ref();
    assert_eq!(unchanged_digest, "900150983cd24fb0d6963f7d28e17f72");
    assert!(checked.values.md5_matches);
    let changed = source.compare_document_source(id, &bytes).unwrap();
    assert!(changed.values.size_matches && !changed.values.md5_matches);
    assert_eq!(changed.completion, InspectionSourceCompletion::Completed);
    assert!(
        source
            .compare_document_source(&source.records()[0].id, &bytes)
            .is_err()
    );
    assert_eq!(source.envelope().xml_bytes(), original);
    assert!(source.record(id).is_ok());
}

#[test]
fn duplicate_uid_text_never_merges_physical_document_comparison_owners() {
    let source = project(include_bytes!("../fixtures/inspection-records/ordered.xml"));
    let documents: Vec<_> = source
        .records()
        .iter()
        .filter(|r| r.kind == InspectionRecordKind::Document)
        .collect();
    assert_eq!(documents.len(), 2);
    assert_eq!(documents[0].attribute("Uid"), documents[1].attribute("Uid"));
    let first = source
        .compare_document_source(&documents[0].id, b"abc")
        .unwrap();
    let second = source
        .compare_document_source(&documents[1].id, b"abc")
        .unwrap();
    assert!(first.values.size_matches && first.values.md5_matches);
    assert!(!second.values.size_matches && !second.values.md5_matches);
    assert_ne!(first.document, second.document);
    assert_eq!(second.values.stored_size, None);
    assert_eq!(second.values.stored_md5, None);
}

fn comparison_matrix(source: &InspectionRecords) -> Vec<ComparisonSnapshot> {
    fixtures::checksum_cases()
        .iter()
        .map(|case| {
            snapshot(
                source
                    .compare_document_source(&document(source).id, case.source)
                    .unwrap(),
            )
        })
        .collect()
}

#[test]
fn eight_workers_repeat_all_vectors_in_total_order_against_serial_complete_results() {
    let source = canonical_project(3, "900150983cd24fb0d6963f7d28e17f72");
    let expected = comparison_matrix(&source);
    std::thread::scope(|scope| {
        let jobs: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    for _ in 0..8 {
                        assert_eq!(comparison_matrix(&source), expected);
                    }
                })
            })
            .collect();
        for job in jobs {
            job.join().unwrap();
        }
    });
    assert_eq!(comparison_matrix(&source), expected);
    assert_eq!(source.envelope().xml_bytes(), expected[0].xml);
}
