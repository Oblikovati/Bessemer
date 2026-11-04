use super::*;
use crate::Document;
use crate::model_archives::ConfigurationId;

#[path = "../support/unselected_document_streams.rs"]
pub(super) mod unselected_document_streams;

pub(super) fn native_reference_document_source(
    document: &Document,
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
) -> PartResolvedFeaturesArchive {
    let name = case.label.rsplit_once("::").unwrap().1;
    let configuration = ConfigurationId::from_stream_name(name).unwrap();
    PartResolvedFeaturesArchive::from_document(
        document,
        configuration,
        case.version,
        kernel,
        LIMITS,
    )
    .unwrap()
}

fn reverse_native_reference_document(
    reopened: &PartResolvedFeaturesArchive,
    original: &State,
) -> PartResolvedFeaturesArchive {
    let handle = reopened
        .field_handle(original.source_fields()[0].index())
        .unwrap();
    let current = reopened.intersection_reference_state(&handle).unwrap();
    let words = original.words();
    reopened
        .edit_intersection_reference_state(
            &current,
            Edit::SpanAndBackwardStates {
                span_count_state: words.span_count_state,
                backward_offset_state: words.backward_offset_state,
            },
        )
        .unwrap()
        .archive()
        .clone()
}

fn verify_native_document_roundtrip(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    state: &State,
    span: &NativeReferenceSpan,
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
) {
    let edit = verify_native_reference_edit(source, case.features, state, span);
    let stream = format!(
        "{}-ResolvedFeatures",
        case.label.rsplit_once("::").unwrap().1
    );
    let mut output = document.clone();
    output
        .replace_stream(&stream, edit.archive().to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let actual = native_reference_document_source(&reopened, case, kernel);
    assert_eq!(actual.fields(), edit.archive().fields());
    unselected_document_streams::assert_unselected_document_streams(document, &reopened, &stream);
    let reverse = reverse_native_reference_document(&actual, state);
    assert_eq!(reverse.fields(), source.fields());
    assert_eq!(reverse.to_bytes().unwrap(), case.features);
    output
        .replace_stream(&stream, reverse.to_bytes().unwrap())
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}

fn verify_native_reference_document_case(
    case: &ResolvedCorpusCase<'_>,
    oracle: &NativeReferenceOracle,
    kernel: Option<CompressedKernelContext>,
) {
    assert_eq!(case.label, oracle.label);
    assert_eq!(case.version.value(), oracle.version);
    let path = case
        .label
        .split_once(": ")
        .unwrap()
        .1
        .rsplit_once("::")
        .unwrap()
        .0;
    let bytes = std::fs::read(path).unwrap();
    let document = Document::from_bytes(&bytes).unwrap();
    let source = native_reference_document_source(&document, case, kernel.clone());
    assert_eq!(source.to_bytes().unwrap(), case.features);
    let states = source.intersection_reference_states().unwrap();
    assert_eq!(states.len(), oracle.spans.len());
    for (state, span) in states.iter().zip(&oracle.spans) {
        verify_native_document_roundtrip(&document, &source, state, span, case, kernel.clone());
    }
    assert_eq!(document.to_bytes().unwrap(), bytes);
}

#[test]
#[ignore = "Requires retained native corpus packets, installed native files and kernel schema"]
fn every_native_intersection_reference_document_reopens_preserves_streams_and_reverses() {
    let probe = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let spans = std::fs::read("target/intersection-reference-native-source-spans.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    run_native_reference_cohort(
        &probe,
        &spans,
        kernel,
        verify_native_reference_document_case,
    );
}
