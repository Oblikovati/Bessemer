use super::*;
use crate::unselected_document_streams::assert_unselected_document_streams;

struct AppearanceDocumentContext {
    configuration: ConfigurationId,
    version: ArchiveVersion,
    kernel: Option<CompressedKernelContext>,
    stream: String,
}

fn appearance_document_context(version: u32) -> (Document, AppearanceDocumentContext) {
    let (document, configuration, kernel) = appearance_document_parent(version);
    let stream = format!("{}-ResolvedFeatures", configuration.stream_name());
    (
        document,
        AppearanceDocumentContext {
            configuration,
            version: ArchiveVersion::new(version),
            kernel,
            stream,
        },
    )
}

fn appearance_document_parent(
    version: u32,
) -> (Document, ConfigurationId, Option<CompressedKernelContext>) {
    if version == 19000 {
        let document =
            Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
        return (document, ConfigurationId::new(0), None);
    }
    let columns = native_appearance_columns(version);
    let document = Document::from_bytes(&std::fs::read(columns[1]).unwrap()).unwrap();
    let configuration = ConfigurationId::from_stream_name(columns[2]).unwrap();
    let kernel = native_kernel_context::native_test_kernel_context();
    (document, configuration, Some(kernel))
}

fn appearance_document_source(
    document: &Document,
    context: &AppearanceDocumentContext,
) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        context.configuration,
        context.version,
        context.kernel.clone(),
        LIMITS,
    )
    .unwrap()
}

fn prepared_appearance_document(
    case: &AppearanceCase<'_>,
) -> (Document, AppearanceDocumentContext) {
    let (mut original, context) = appearance_document_context(case.version);
    original
        .replace_stream(&context.stream, case.raw.to_vec())
        .unwrap();
    let document = Document::from_bytes(&original.to_bytes().unwrap()).unwrap();
    assert_eq!(document.read_stream(&context.stream).unwrap(), case.raw);
    (document, context)
}

fn appearance_document_bits(
    source: &PartResolvedFeaturesArchive,
    state: &Appearance,
    role: usize,
) -> i32 {
    match source
        .resolve_field(&state.source_fields()[role])
        .unwrap()
        .value
    {
        ModelFieldValue::I32(value) => value,
        ModelFieldValue::U32(value) => value as i32,
        ref actual => panic!("appearance scalar {actual:?}; expected I32 or U32"),
    }
}

fn appearance_document_reopen(
    document: &Document,
    edited: &PartResolvedFeaturesArchive,
    context: &AppearanceDocumentContext,
) -> (Document, PartResolvedFeaturesArchive) {
    let mut output = document.clone();
    output
        .replace_stream(&context.stream, edited.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    assert_unselected_document_streams(document, &reopened, &context.stream);
    let actual = appearance_document_source(&reopened, context);
    assert_eq!(actual.fields(), edited.fields());
    assert_eq!(actual.to_bytes().unwrap(), edited.to_bytes().unwrap());
    (output, actual)
}

fn appearance_document_restore(
    actual: &PartResolvedFeaturesArchive,
    source: &PartResolvedFeaturesArchive,
    owner: usize,
    role: usize,
    offset: usize,
    original: i32,
) -> PartResolvedFeaturesEdit {
    let state = &actual.bitmap_appearance_states().unwrap()[owner];
    let request = appearance_request(role, original).0;
    let reverse = actual.edit_bitmap_appearance(state, request).unwrap();
    let raw = actual.to_bytes().unwrap();
    verify_appearance_result(actual, &raw, state, role, offset, original, &reverse);
    assert_eq!(reverse.archive().fields(), source.fields());
    assert_eq!(
        reverse.archive().to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    reverse
}

fn verify_appearance_document_edit(
    document: &Document,
    context: &AppearanceDocumentContext,
    source: &PartResolvedFeaturesArchive,
    owner: usize,
    role: usize,
    offset: usize,
) {
    let state = &source.bitmap_appearance_states().unwrap()[owner];
    let original = appearance_document_bits(source, state, role);
    let edit = source
        .edit_bitmap_appearance(state, appearance_request(role, !original).0)
        .unwrap();
    let raw = source.to_bytes().unwrap();
    verify_appearance_result(source, &raw, state, role, offset, !original, &edit);
    let (mut output, actual) = appearance_document_reopen(document, edit.archive(), context);
    let reverse = appearance_document_restore(&actual, source, owner, role, offset, original);
    output
        .replace_stream(&context.stream, reverse.archive().to_bytes().unwrap())
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
    assert_eq!(source.to_bytes().unwrap(), raw);
}

fn appearance_document_case(index: usize, case: &AppearanceCase<'_>) -> usize {
    let (document, context) = prepared_appearance_document(case);
    let original = document.to_bytes().unwrap();
    let source = appearance_document_source(&document, &context);
    assert_eq!(source.to_bytes().unwrap(), case.raw);
    let mut operations = 0;
    for owner in 0..2 {
        let offsets = appearance_oracle(index, owner).0;
        operations += appearance_document_owner(&document, &context, &source, owner, offsets);
    }
    assert_eq!(document.to_bytes().unwrap(), original);
    operations
}

fn appearance_document_owner(
    document: &Document,
    context: &AppearanceDocumentContext,
    source: &PartResolvedFeaturesArchive,
    owner: usize,
    offsets: [Option<usize>; 3],
) -> usize {
    let selected: Vec<_> = offsets
        .into_iter()
        .enumerate()
        .filter_map(|(role, offset)| offset.map(|offset| (role, offset)))
        .collect();
    for (role, offset) in &selected {
        verify_appearance_document_edit(document, context, source, owner, *role, *offset);
    }
    selected.len()
}

#[test]
fn current_appearance_documents_reopen_preserve_other_streams_and_reverse_all2250_edits() {
    let mut operations = 0;
    for (index, case) in appearance_cases()
        .into_iter()
        .enumerate()
        .filter(|(_, case)| case.version == 19000)
    {
        operations += appearance_document_case(index, &case);
    }
    assert_eq!(operations, 2250);
}

#[test]
#[ignore = "Requires explicit kernel schema and pinned11000/18000 contexts;4800 document edits and reversals"]
fn all_supported_appearance_documents_reopen_preserve_other_streams_and_reverse_all4800_edits() {
    let mut operations = 0;
    for (index, case) in appearance_cases().into_iter().enumerate() {
        operations += appearance_document_case(index, &case);
    }
    assert_eq!(operations, 4800);
}
