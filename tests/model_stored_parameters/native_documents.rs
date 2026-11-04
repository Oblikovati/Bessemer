use super::super::stored_sketch_lookup_keys::documents::native_lookup_original_document;
use super::super::unselected_document_streams;
use super::*;
use crate::Document;
use crate::model_archives::{
    ConfigurationId, ModelEditCompletion, ModelFieldChange, ModelParameterSource,
    ModelParameterStoredEdit, ModelParameterValue, StoredDouble,
};

struct NativePrimaryRow {
    context: u32,
    ordinal: usize,
    identity: crate::model_archives::ModelObjectId,
    offset: usize,
    original: u64,
    replacement: u64,
}

fn first_native_rows() -> Vec<NativePrimaryRow> {
    let mut cursor = ArchiveCursor::new(include_bytes!(
        "../fixtures/stored-parameters/native-first/spans.bin"
    ));
    assert_eq!(cursor.take(4).unwrap(), b"BPN1");
    assert_eq!(cursor.u32().unwrap(), 2);
    let rows = (0..2)
        .map(|_| NativePrimaryRow {
            context: cursor.u32().unwrap(),
            ordinal: cursor.u32().unwrap() as usize,
            identity: crate::model_archives::ModelObjectId::new(cursor.u32().unwrap()).unwrap(),
            offset: cursor.u32().unwrap() as usize,
            original: cursor.u64().unwrap(),
            replacement: cursor.u64().unwrap(),
        })
        .collect();
    assert_eq!(cursor.remaining(), 0);
    rows
}

fn read_parameter_document(
    document: &Document,
    case: &ResolvedCorpusCase<'_>,
    row: &NativePrimaryRow,
    kernel: Option<CompressedKernelContext>,
) -> ModelParameterSource {
    let name = case.label.rsplit_once("::").unwrap().1;
    let configuration = ConfigurationId::from_stream_name(name).unwrap();
    match row.context {
        0 => ModelParameterSource::Configuration(
            crate::model_archives::PartConfigurationArchive::from_document(
                document,
                configuration,
                case.version,
                kernel,
                LIMITS,
            )
            .unwrap(),
        ),
        1 => ModelParameterSource::ResolvedFeatures(
            PartResolvedFeaturesArchive::from_document(
                document,
                configuration,
                case.version,
                kernel,
                LIMITS,
            )
            .unwrap(),
        ),
        other => panic!("context {other}; expected configuration0 or features1"),
    }
}

fn parameter_source_bytes(source: &ModelParameterSource) -> Vec<u8> {
    match source {
        ModelParameterSource::Configuration(source) => source.to_bytes().unwrap(),
        ModelParameterSource::ResolvedFeatures(source) => source.to_bytes().unwrap(),
    }
}

fn parameter_source_fields(source: &ModelParameterSource) -> &[ModelField] {
    match source {
        ModelParameterSource::Configuration(source) => source.fields(),
        ModelParameterSource::ResolvedFeatures(source) => source.fields(),
    }
}

fn parameter_at(
    source: &ModelParameterSource,
    row: &NativePrimaryRow,
) -> ModelParameterStoredState {
    let mut parameters = match source {
        ModelParameterSource::Configuration(source) => source.stored_parameters().unwrap(),
        ModelParameterSource::ResolvedFeatures(source) => source.stored_parameters().unwrap(),
    };
    let parameter = parameters.remove(row.ordinal);
    let field = source.resolve_field(parameter.definition()).unwrap();
    assert!(matches!(&field.value, ModelFieldValue::ObjectTag(tag)
        if matches!(&tag.record, ModelObjectRecord::New { index, .. } if *index == row.identity)));
    parameter
}

fn edit_native_primary(
    source: &ModelParameterSource,
    parameter: &ModelParameterStoredState,
    bits: u64,
) -> ModelParameterSource {
    let request = ModelParameterStoredEdit::Primary(ModelParameterValue::Double(
        StoredDouble::from_bits(bits),
    ));
    match source {
        ModelParameterSource::Configuration(archive) => {
            let edit = archive.edit_stored_parameter(parameter, request).unwrap();
            assert_primary_history(source, edit.completion(), edit.changes());
            ModelParameterSource::Configuration(edit.archive().clone())
        }
        ModelParameterSource::ResolvedFeatures(archive) => {
            let edit = archive.edit_stored_parameter(parameter, request).unwrap();
            assert_primary_history(source, edit.completion(), edit.changes());
            ModelParameterSource::ResolvedFeatures(edit.archive().clone())
        }
    }
}

fn assert_primary_history(
    source: &ModelParameterSource,
    completion: ModelEditCompletion,
    changes: &[ModelFieldChange],
) {
    assert_eq!(completion, ModelEditCompletion::Changed);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].parents().len(), 1);
    let parent = source.resolve_field(&changes[0].parents()[0]).unwrap();
    assert!(parent.path.ends_with(".value"));
    assert!(matches!(parent.value, ModelFieldValue::F64(_)));
}

fn native_parameter_stream(case: &ResolvedCorpusCase<'_>, row: &NativePrimaryRow) -> String {
    let configuration = case.label.rsplit_once("::").unwrap().1;
    if row.context == 0 {
        return configuration.to_owned();
    }
    format!("{configuration}-ResolvedFeatures")
}

fn assert_native_primary_operation(
    document: &Document,
    case: &ResolvedCorpusCase<'_>,
    row: &NativePrimaryRow,
    kernel: Option<CompressedKernelContext>,
) {
    let source = read_parameter_document(document, case, row, kernel.clone());
    let original = if row.context == 0 {
        case.configuration
    } else {
        case.features
    };
    assert_eq!(parameter_source_bytes(&source), original);
    let parameter = parameter_at(&source, row);
    assert_eq!(
        parameter.primary(),
        &ModelParameterValue::Double(StoredDouble::from_bits(row.original))
    );
    let changed = edit_native_primary(&source, &parameter, row.replacement);
    let mut expected = original.to_vec();
    assert_eq!(
        &expected[row.offset..row.offset + 8],
        &row.original.to_le_bytes()
    );
    expected[row.offset..row.offset + 8].copy_from_slice(&row.replacement.to_le_bytes());
    assert_eq!(parameter_source_bytes(&changed), expected);
    assert_document_primary_reversal(document, case, row, &source, &changed, kernel);
    assert_eq!(parameter_source_bytes(&source), original);
}

fn assert_document_primary_reversal(
    document: &Document,
    case: &ResolvedCorpusCase<'_>,
    row: &NativePrimaryRow,
    source: &ModelParameterSource,
    changed: &ModelParameterSource,
    kernel: Option<CompressedKernelContext>,
) {
    let stream = native_parameter_stream(case, row);
    let mut output = document.clone();
    output
        .replace_stream(&stream, parameter_source_bytes(changed))
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    unselected_document_streams::assert_unselected_document_streams(document, &reopened, &stream);
    let current = read_parameter_document(&reopened, case, row, kernel);
    assert_eq!(
        parameter_source_fields(&current),
        parameter_source_fields(changed)
    );
    let reverse = edit_native_primary(&current, &parameter_at(&current, row), row.original);
    assert_eq!(
        parameter_source_fields(&reverse),
        parameter_source_fields(source)
    );
    assert_eq!(
        parameter_source_bytes(&reverse),
        parameter_source_bytes(source)
    );
    output
        .replace_stream(&stream, parameter_source_bytes(&reverse))
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}

#[test]
#[ignore = "requires retained native packet, original native file and explicit kernel schema; first two complete parameter edits"]
fn first_native_parameter_edits_match_original_spans_reopen_and_reverse_complete_documents() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(cursor.u32().unwrap(), 906);
    let case = next_case(&mut cursor);
    let (document, original) = native_lookup_original_document(&case);
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    for row in first_native_rows() {
        assert_native_primary_operation(&document, &case, &row, kernel.clone());
    }
    assert_eq!(document.to_bytes().unwrap(), original);
}
