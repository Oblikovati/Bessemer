use super::*;
use crate::native_kernel_context::native_test_kernel_context;
use crate::unselected_document_streams::assert_unselected_document_streams;

struct NativeAppearanceCase {
    index: u32,
    version: ArchiveVersion,
    configuration: ConfigurationId,
    path: &'static str,
    stream: &'static str,
    offsets: [usize; 3],
    original: [i32; 3],
}

fn native_appearance_cases() -> Vec<NativeAppearanceCase> {
    include_str!("../fixtures/bitmap-appearance/native-spans.tsv")
        .lines()
        .map(native_appearance_case)
        .collect()
}

fn native_appearance_case(line: &'static str) -> NativeAppearanceCase {
    let words: Vec<_> = line.split('\t').collect();
    assert_eq!(words.len(), 10);
    let configuration = words[3].strip_suffix("-ResolvedFeatures").unwrap();
    NativeAppearanceCase {
        index: words[0].parse().unwrap(),
        version: ArchiveVersion::new(words[1].parse().unwrap()),
        configuration: ConfigurationId::from_stream_name(configuration).unwrap(),
        path: words[2],
        stream: words[3],
        offsets: std::array::from_fn(|role| words[4 + role].parse().unwrap()),
        original: [
            words[7].parse().unwrap(),
            words[8].parse::<u32>().unwrap() as i32,
            words[9].parse().unwrap(),
        ],
    }
}

fn native_appearance_document(case: &NativeAppearanceCase) -> (Document, Vec<u8>) {
    let original = std::fs::read(case.path).unwrap();
    let document = Document::from_bytes(&original).unwrap();
    assert_eq!(document.to_bytes().unwrap(), original);
    (document, original)
}

fn native_appearance_source(
    document: &Document,
    case: &NativeAppearanceCase,
    kernel: &CompressedKernelContext,
) -> PartResolvedFeaturesArchive {
    let source = PartResolvedFeaturesArchive::from_document(
        document,
        case.configuration,
        case.version,
        Some(kernel.clone()),
        LIMITS,
    )
    .unwrap();
    assert_eq!(
        source.to_bytes().unwrap(),
        document.read_stream(case.stream).unwrap()
    );
    source
}

fn native_appearance_state(source: &PartResolvedFeaturesArchive) -> Appearance {
    let mut states = source.bitmap_appearance_states().unwrap();
    assert_eq!(states.len(), 1);
    states.remove(0)
}

fn verify_native_appearance_words(state: &Appearance, case: &NativeAppearanceCase) {
    assert_eq!(
        state.words(),
        ModelBitmapAppearanceWords {
            matching_color_percentage: case.original[0],
            transparent_color: case.original[1] as u32,
            ink_markup_texture: Some(case.original[2]),
        }
    );
    assert_eq!(state.source_fields().len(), 3);
}

fn native_appearance_operation_case(
    case: &NativeAppearanceCase,
    kernel: &CompressedKernelContext,
    output: &mut Option<Vec<u8>>,
) {
    let (document, original) = native_appearance_document(case);
    let source = native_appearance_source(&document, case, kernel);
    let state = native_appearance_state(&source);
    verify_native_appearance_words(&state, case);
    let raw = document.read_stream(case.stream).unwrap();
    for role in 0..3 {
        for bits in REQUESTS {
            let actual =
                verify_appearance_edit(&source, &raw, &state, role, case.offsets[role], bits);
            exports::append_appearance_export(output, case.version.value(), &actual);
        }
    }
    assert_eq!(document.to_bytes().unwrap(), original);
}

#[test]
#[ignore = "Requires four pinned native bitmap files and explicit installed kernel schema"]
fn all_native_appearance_settings_match_independent_spans_and_literal_edits() {
    let path = std::env::var_os("BESSEMER_APPEARANCE_NATIVE_EXPORT");
    let mut output = path
        .as_ref()
        .map(|_| [b"BAN1".as_slice(), &60_u32.to_le_bytes()].concat());
    let kernel = native_test_kernel_context();
    for case in native_appearance_cases() {
        native_appearance_operation_case(&case, &kernel, &mut output);
        println!("APPEARANCE_NATIVE\t{}\t15", case.index);
    }
    exports::finish_appearance_export(path, output);
}

fn native_appearance_reopened(
    original: &Document,
    case: &NativeAppearanceCase,
    edited: &PartResolvedFeaturesArchive,
) -> (Document, Document) {
    let mut output = original.clone();
    output
        .replace_stream(case.stream, edited.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    assert_unselected_document_streams(original, &reopened, case.stream);
    (output, reopened)
}

fn native_appearance_reverse(
    reopened: &PartResolvedFeaturesArchive,
    case: &NativeAppearanceCase,
    role: usize,
) -> PartResolvedFeaturesArchive {
    let state = native_appearance_state(reopened);
    let request = appearance_request(role, case.original[role]).0;
    reopened
        .edit_bitmap_appearance(&state, request)
        .unwrap()
        .archive()
        .clone()
}

fn verify_native_appearance_document_edit(
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    case: &NativeAppearanceCase,
    kernel: &CompressedKernelContext,
    role: usize,
) {
    let state = native_appearance_state(source);
    let request = appearance_request(role, !case.original[role]).0;
    let edit = source.edit_bitmap_appearance(&state, request).unwrap();
    let raw = document.read_stream(case.stream).unwrap();
    verify_appearance_result(
        source,
        &raw,
        &state,
        role,
        case.offsets[role],
        !case.original[role],
        &edit,
    );
    let (output, reopened) = native_appearance_reopened(document, case, edit.archive());
    let actual = native_appearance_source(&reopened, case, kernel);
    assert_eq!(actual.fields(), edit.archive().fields());
    verify_native_appearance_restore(output, document, source, &actual, case, role);
    assert_eq!(source.to_bytes().unwrap(), raw);
}

#[test]
#[ignore = "Requires four pinned native bitmap files and explicit installed kernel schema"]
fn native_appearance_documents_reopen_preserve_streams_and_reverse_original_bytes() {
    let kernel = native_test_kernel_context();
    for case in native_appearance_cases() {
        let (document, original) = native_appearance_document(&case);
        let source = native_appearance_source(&document, &case, &kernel);
        for role in 0..3 {
            verify_native_appearance_document_edit(&document, &source, &case, &kernel, role);
        }
        assert_eq!(document.to_bytes().unwrap(), original);
        println!("APPEARANCE_NATIVE_DOCUMENT\t{}\t3", case.index);
    }
}

fn verify_native_appearance_restore(
    mut output: Document,
    document: &Document,
    source: &PartResolvedFeaturesArchive,
    actual: &PartResolvedFeaturesArchive,
    case: &NativeAppearanceCase,
    role: usize,
) {
    let reverse = native_appearance_reverse(actual, case, role);
    assert_eq!(reverse.fields(), source.fields());
    assert_eq!(reverse.to_bytes().unwrap(), source.to_bytes().unwrap());
    output
        .replace_stream(case.stream, reverse.to_bytes().unwrap())
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
}
