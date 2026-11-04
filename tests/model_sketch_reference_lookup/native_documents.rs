use super::*;
use crate::Document;
use crate::model_archives::ConfigurationId;

use super::super::unselected_document_streams;

fn native_lookup_document_source(
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

fn native_lookup_document_reverse(
    reopened: &PartResolvedFeaturesArchive,
    original: &LookupState,
) -> PartResolvedFeaturesArchive {
    let leading = reopened
        .field_handle(original.source_fields()[0].index())
        .unwrap();
    let current = reopened.sketch_reference_lookup_state(&leading).unwrap();
    let keys = original.keys();
    reopened
        .edit_sketch_reference_lookup_keys(
            &current,
            LookupEdit::BothKeys {
                configuration_object_key: keys.configuration_object_key,
                configuration_record_key: keys.configuration_record_key,
            },
        )
        .unwrap()
        .archive()
        .clone()
}

fn native_lookup_document_output(
    document: &Document,
    case: &ResolvedCorpusCase<'_>,
    edited: &PartResolvedFeaturesArchive,
) -> (Document, Document, String) {
    let stream = format!(
        "{}-ResolvedFeatures",
        case.label.rsplit_once("::").unwrap().1
    );
    let mut output = document.clone();
    output
        .replace_stream(&stream, edited.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    unselected_document_streams::assert_unselected_document_streams(document, &reopened, &stream);
    (output, reopened, stream)
}

fn verify_native_lookup_document_restore(
    mut output: Document,
    stream: &str,
    original: &Document,
    source: &PartResolvedFeaturesArchive,
    reopened: &PartResolvedFeaturesArchive,
    state: &LookupState,
) {
    let reverse = native_lookup_document_reverse(reopened, state);
    assert_eq!(reverse.fields(), source.fields());
    assert_eq!(reverse.to_bytes().unwrap(), source.to_bytes().unwrap());
    output
        .replace_stream(stream, reverse.to_bytes().unwrap())
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), original.to_bytes().unwrap());
}

fn verify_native_lookup_document_edit(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    state: &LookupState,
    row: &NativeLookupRow,
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
    kind: usize,
) {
    let edit = verify_native_lookup_edit(source, state, row, case.features, kind);
    let (output, reopened, stream) = native_lookup_document_output(document, case, edit.archive());
    let actual = native_lookup_document_source(&reopened, case, kernel);
    assert_eq!(actual.fields(), edit.archive().fields());
    assert_eq!(
        actual.to_bytes().unwrap(),
        edit.archive().to_bytes().unwrap()
    );
    verify_native_lookup_document_restore(output, &stream, document, source, &actual, state);
    assert_eq!(source.to_bytes().unwrap(), case.features);
}

fn native_lookup_original_document(case: &ResolvedCorpusCase<'_>) -> (Document, Vec<u8>) {
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
    assert_eq!(document.to_bytes().unwrap(), bytes);
    (document, bytes)
}

fn verify_native_lookup_document_case(
    case: &ResolvedCorpusCase<'_>,
    rows: &[&NativeLookupRow],
    kernel: Option<CompressedKernelContext>,
    _edit: bool,
) {
    if rows.is_empty() {
        return;
    }
    let (document, bytes) = native_lookup_original_document(case);
    let source = native_lookup_document_source(&document, case, kernel.clone());
    assert_eq!(source.to_bytes().unwrap(), case.features);
    verify_native_lookup_document_states(&document, &source, rows, case, kernel);
    assert_eq!(document.to_bytes().unwrap(), bytes);
}

#[test]
#[ignore = "Requires retained native corpus, installed original files and kernel schema;20589 document edits"]
fn all_native_lookup_documents_reopen_preserve_other_streams_and_reverse() {
    run_native_lookup_corpus(verify_native_lookup_document_case, true);
}

fn verify_native_lookup_document_states(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    rows: &[&NativeLookupRow],
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
) {
    let states = source.sketch_reference_lookup_states().unwrap();
    assert_eq!(states.len(), rows.len());
    for (state, row) in states.iter().zip(rows) {
        for kind in 0..3 {
            verify_native_lookup_document_edit(
                document,
                source,
                state,
                row,
                case,
                kernel.clone(),
                kind,
            );
        }
    }
}
