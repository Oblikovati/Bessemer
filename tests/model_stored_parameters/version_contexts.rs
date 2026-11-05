use super::*;
#[path = "version_context_fixtures.rs"]
mod fixtures;

#[test]
#[ignore = "requires original native documents, retained packet and explicit kernel schema; owned older-version parameter graphs"]
fn all_parameter_classes_in_older_native_contexts_reopen_and_reverse_owned_documents() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let mut cursor = version_case_packet(&bytes);
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut checked = 0;
    for ordinal in 0..=210 {
        let case = next_case(&mut cursor);
        if let Some(fixture) = fixtures::CONTEXTS
            .iter()
            .find(|fixture| fixture.case == ordinal)
        {
            assert_eq!(
                case.version,
                crate::header::ArchiveVersion::new(fixture.version)
            );
            check_version_parameter_context(&case, fixture, kernel.clone());
            checked += 1;
        }
    }
    assert_eq!(checked, 2);
}

fn version_case_packet(bytes: &[u8]) -> ArchiveCursor<'_> {
    let mut cursor = ArchiveCursor::new(bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(cursor.u32().unwrap(), 906);
    cursor
}

fn check_version_parameter_context(
    case: &ResolvedCorpusCase<'_>,
    fixture: &fixtures::VersionParameterFixture,
    kernel: Option<CompressedKernelContext>,
) {
    let (document, original) = native_lookup_original_document(case);
    let stream = version_feature_stream(case);
    for graphs in fixture.graphs {
        for raw in &graphs[..4] {
            let (_, reopened) = primary_output_document(&document, &stream, raw.to_vec());
            let source = read_parameter_context(&reopened, case, 1, kernel.clone());
            assert_eq!(parameter_source_bytes(&source), *raw);
            let parameter = version_parameter(&source);
            assert_eq!(parameter.achieved_stored_value_error(), 0);
        }
        check_version_parameter_edit(&document, case, graphs, kernel.clone());
    }
    assert_eq!(document.to_bytes().unwrap(), original);
}

fn version_feature_stream(case: &ResolvedCorpusCase<'_>) -> String {
    let configuration = case.label.rsplit_once("::").unwrap().1;
    format!("{configuration}-ResolvedFeatures")
}

fn version_parameter(source: &ModelParameterSource) -> ModelParameterStoredState {
    let ModelParameterSource::ResolvedFeatures(archive) = source else {
        panic!("expected a complete resolved-feature archive");
    };
    let mut parameters = archive.stored_parameters().unwrap();
    assert_eq!(parameters.len(), 1);
    parameters.remove(0)
}

fn check_version_parameter_edit(
    document: &Document,
    case: &ResolvedCorpusCase<'_>,
    graphs: [&[u8]; 5],
    kernel: Option<CompressedKernelContext>,
) {
    let stream = version_feature_stream(case);
    let (_, owned) = primary_output_document(document, &stream, graphs[0].to_vec());
    let source = read_parameter_context(&owned, case, 1, kernel.clone());
    let parameter = version_parameter(&source);
    let (_, expected_document) = primary_output_document(&owned, &stream, graphs[4].to_vec());
    let expected = read_parameter_context(&expected_document, case, 1, kernel.clone());
    let changed = edit_native_value(
        &source,
        &parameter,
        version_parameter(&expected).primary().clone(),
    );
    assert_eq!(parameter_source_bytes(&changed), graphs[4]);
    reverse_version_parameter_document(&owned, case, &source, &changed, kernel);
    assert_eq!(parameter_source_bytes(&source), graphs[0]);
}

fn reverse_version_parameter_document(
    document: &Document,
    case: &ResolvedCorpusCase<'_>,
    source: &ModelParameterSource,
    changed: &ModelParameterSource,
    kernel: Option<CompressedKernelContext>,
) {
    let stream = version_feature_stream(case);
    let (mut output, reopened) =
        primary_output_document(document, &stream, parameter_source_bytes(changed));
    let current = read_parameter_context(&reopened, case, 1, kernel);
    assert_parameter_archives_equal(&current, changed);
    let reverse = edit_native_value(
        &current,
        &version_parameter(&current),
        version_parameter(source).primary().clone(),
    );
    assert_parameter_archives_equal(&reverse, source);
    output
        .replace_stream(&stream, parameter_source_bytes(&reverse))
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}
