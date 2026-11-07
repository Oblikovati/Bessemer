use super::documents::{native_reference_document_source, unselected_document_streams};
use super::*;
use crate::Document;
use crate::model_archives::{ModelIntersectionEdgeStoredIndex, PartResolvedFeaturesEdit};

fn native_role_document_roundtrip(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
    edit: &PartResolvedFeaturesEdit,
    reverse: impl FnOnce(&PartResolvedFeaturesArchive) -> PartResolvedFeaturesEdit,
) {
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
    let reversed = reverse(&actual);
    assert_eq!(reversed.archive().fields(), source.fields());
    assert_eq!(reversed.archive().to_bytes().unwrap(), case.features);
    output
        .replace_stream(&stream, reversed.archive().to_bytes().unwrap())
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}

fn native_selector_reversal(
    actual: &PartResolvedFeaturesArchive,
    state: &State,
) -> PartResolvedFeaturesEdit {
    let field = actual
        .field_handle(state.source_fields()[0].index())
        .unwrap();
    let current = actual.intersection_reference_state(&field).unwrap();
    actual
        .edit_intersection_reference_classification(&current, state.classification_selector())
        .unwrap()
}

fn native_index_reversal(
    actual: &PartResolvedFeaturesArchive,
    state: &ModelIntersectionEdgeStoredIndex,
) -> PartResolvedFeaturesEdit {
    let field = actual.field_handle(state.source_field().index()).unwrap();
    let current = actual.intersection_edge_index(&field).unwrap();
    actual
        .edit_intersection_edge_index(&current, state.value())
        .unwrap()
}

fn native_selector_documents(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    case: &ResolvedCorpusCase<'_>,
    oracle: &NativeReferenceOracle,
    kernel: Option<CompressedKernelContext>,
) {
    let states = source.intersection_reference_states().unwrap();
    assert_eq!(states.len(), oracle.spans.len());
    for (state, span) in states.iter().zip(&oracle.spans) {
        assert_eq!(state.words(), span.words);
        let value = state.classification_selector() ^ 0xa55a;
        let edit = source
            .edit_intersection_reference_classification(state, value)
            .unwrap();
        super::roles::verify_native_role_payload(
            source,
            case.features,
            &state.source_fields()[0],
            ModelFieldValue::U16(value),
            (span.offset, &value.to_le_bytes()),
            &edit,
        );
        native_role_document_roundtrip(document, source, case, kernel.clone(), &edit, |actual| {
            native_selector_reversal(actual, state)
        });
    }
}

fn native_index_documents(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    case: &ResolvedCorpusCase<'_>,
    oracle: &NativeReferenceOracle,
    kernel: Option<CompressedKernelContext>,
) {
    let states = source.intersection_edge_indices().unwrap();
    let expected = super::roles::native_index_oracles(oracle.index);
    assert_eq!(states.len(), expected.len());
    for (state, (offset, original)) in states.iter().zip(expected) {
        assert_eq!(state.value(), original);
        let value = !original;
        let edit = source.edit_intersection_edge_index(state, value).unwrap();
        super::roles::verify_native_role_payload(
            source,
            case.features,
            state.source_field(),
            ModelFieldValue::I32(value),
            (offset, &value.to_le_bytes()),
            &edit,
        );
        native_role_document_roundtrip(document, source, case, kernel.clone(), &edit, |actual| {
            native_index_reversal(actual, state)
        });
    }
}

fn native_role_document_case(
    case: &ResolvedCorpusCase<'_>,
    oracle: &NativeReferenceOracle,
    kernel: Option<CompressedKernelContext>,
) {
    assert_eq!(case.label, oracle.label);
    assert_eq!(case.version.value(), oracle.version);
    let labelled = case.label.split_once(": ").unwrap().1;
    let path = labelled.rsplit_once("::").unwrap().0;
    let original = std::fs::read(path).unwrap();
    let document = Document::from_bytes(&original).unwrap();
    let source = native_reference_document_source(&document, case, kernel.clone());
    assert_eq!(source.to_bytes().unwrap(), case.features);
    native_selector_documents(&document, &source, case, oracle, kernel.clone());
    native_index_documents(&document, &source, case, oracle, kernel);
    assert_eq!(source.to_bytes().unwrap(), case.features);
    assert_eq!(document.to_bytes().unwrap(), original);
    println!(
        "INTERSECTION_NATIVE_ROLE_DOCUMENT\t{}\t{}\t{}",
        oracle.index,
        oracle.spans.len(),
        oracle.spans.len() / 4
    );
}

#[test]
#[ignore = "Requires original native files, retained role packets and explicit kernel schema"]
fn every_native_role_document_reopens_preserves_streams_and_reverses_original_bytes() {
    let probe = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let spans = std::fs::read("target/intersection-reference-native-source-spans.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    run_native_reference_cohort(&probe, &spans, kernel, native_role_document_case);
}
