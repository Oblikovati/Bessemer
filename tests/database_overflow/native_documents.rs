use super::*;
use crate::documents::{payload_from_stream, stream_with_pages};
use bessemer::Document;
use std::{
    collections::BTreeMap,
    path::{Component, Path, PathBuf},
};
#[path = "../support/unselected_document_streams.rs"]
mod unselected_document_streams;
use unselected_document_streams::assert_unselected_document_streams;

struct DocumentBinding {
    relative: String,
    stream: String,
    length: usize,
}

struct NativeDocumentInput {
    original: Vec<u8>,
    document: Document,
    stream: String,
    raw: Vec<u8>,
}

fn manifest_text(cursor: &mut &[u8]) -> String {
    let length = u32::from_le_bytes(word(cursor)) as usize;
    std::str::from_utf8(take(cursor, length))
        .unwrap()
        .to_owned()
}

fn document_binding(cursor: &mut &[u8]) -> (u32, DocumentBinding) {
    let ordinal = u32::from_le_bytes(word(cursor));
    let relative = manifest_text(cursor);
    let stream = manifest_text(cursor);
    let length = u64::from_le_bytes(word(cursor)) as usize;
    // #229: the explicit Python source-root preflight authenticates these complete file digests.
    take(cursor, 32);
    (
        ordinal,
        DocumentBinding {
            relative,
            stream,
            length,
        },
    )
}

fn document_sources() -> BTreeMap<u32, DocumentBinding> {
    let compressed = include_bytes!("../fixtures/database-overflow/native-documents.bin.gz");
    let reader = flate2::read::GzDecoder::new(compressed.as_slice());
    let mut raw = Vec::new();
    reader.take(53_287).read_to_end(&mut raw).unwrap();
    assert_eq!(raw.len(), 53_286);
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"BDD1");
    assert_eq!(u32::from_le_bytes(word(&mut cursor)), 440);
    let mut sources = BTreeMap::new();
    for _ in 0..440 {
        let (ordinal, binding) = document_binding(&mut cursor);
        assert!(sources.insert(ordinal, binding).is_none());
    }
    assert!(cursor.is_empty());
    sources
}

fn source_path(root: &Path, relative: &str) -> PathBuf {
    let suffix = Path::new(relative);
    assert!(
        !relative.is_empty()
            && suffix
                .components()
                .all(|item| matches!(item, Component::Normal(_))),
        "source suffix {relative:?}; expected nonempty relative normal components"
    );
    root.join(suffix)
}

fn load_document(root: &Path, binding: &DocumentBinding) -> NativeDocumentInput {
    let path = source_path(root, &binding.relative);
    let original = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("source {path:?}; expected retained native file: {error}"));
    assert_eq!(original.len(), binding.length, "native source {path:?}");
    let document = Document::from_bytes(&original).unwrap();
    assert_eq!(document.to_bytes().unwrap(), original);
    let raw = document.read_stream(&binding.stream).unwrap();
    NativeDocumentInput {
        original,
        document,
        stream: binding.stream.clone(),
        raw,
    }
}

fn native_document_edit(
    input: &NativeDocumentInput,
    case: &NativeChainCase,
) -> (Document, Document) {
    assert_eq!(case.first, case.payload.pages[0].identifier);
    let payload = payload_from_stream(&input.raw, &case.payload);
    assert_eq!(
        payload.pages(),
        expected_pages(&case.payload, &case.payload.value)
    );
    let replacement: Vec<_> = case.payload.value.iter().map(|byte| byte ^ 0x5a).collect();
    let edit = payload.edit_value(&replacement).unwrap();
    assert_eq!(
        edit.result().pages(),
        expected_pages(&case.payload, &replacement)
    );
    reopen_changed_stream(input, stream_with_pages(&input.raw, edit.result()))
}

fn reopen_changed_stream(input: &NativeDocumentInput, changed: Vec<u8>) -> (Document, Document) {
    let mut output = input.document.clone();
    output
        .replace_stream(&input.stream, changed.clone())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    assert_eq!(reopened.read_stream(&input.stream).unwrap(), changed);
    assert_unselected_document_streams(&input.document, &reopened, &input.stream);
    (output, reopened)
}

fn reverse_native_document(
    input: &NativeDocumentInput,
    case: &NativeChainCase,
    mut output: Document,
    reopened: &Document,
) {
    let raw = reopened.read_stream(&input.stream).unwrap();
    let fresh = payload_from_stream(&raw, &case.payload);
    let replacement: Vec<_> = case.payload.value.iter().map(|byte| byte ^ 0x5a).collect();
    assert_eq!(fresh.value(), replacement);
    let reverse = fresh.edit_value(&case.payload.value).unwrap();
    assert_eq!(
        reverse.result().pages(),
        expected_pages(&case.payload, &case.payload.value)
    );
    let restored = stream_with_pages(&raw, reverse.result());
    assert_eq!(restored, input.raw);
    output.replace_stream(&input.stream, restored).unwrap();
    assert_eq!(output.to_bytes().unwrap(), input.original);
    assert_eq!(input.document.to_bytes().unwrap(), input.original);
}

fn check_native_document(input: &NativeDocumentInput, case: &NativeChainCase) {
    let (output, reopened) = native_document_edit(input, case);
    reverse_native_document(input, case, output, &reopened);
    println!(
        "{{\"stream_ordinal\":{},\"first_page\":{},\"document_bytes\":{}}}",
        case.ordinal,
        case.first,
        input.original.len()
    );
}

fn cached_document<'a>(
    current: &'a mut Option<(u32, NativeDocumentInput)>,
    root: &Path,
    sources: &BTreeMap<u32, DocumentBinding>,
    ordinal: u32,
) -> &'a NativeDocumentInput {
    if current.as_ref().map(|(id, _)| *id) != Some(ordinal) {
        *current = Some((ordinal, load_document(root, sources.get(&ordinal).unwrap())));
    }
    &current.as_ref().unwrap().1
}

#[test]
#[ignore = "requires explicit BESSEMER_OVERFLOW_SAMPLE_ROOT and verified original installed samples"]
fn every_native_overflow_edit_reopens_original_document_and_reverses_with_retained_checkpoint() {
    let root = std::env::var_os("BESSEMER_OVERFLOW_SAMPLE_ROOT")
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
        .expect("expected explicit existing BESSEMER_OVERFLOW_SAMPLE_ROOT");
    let sources = document_sources();
    let raw = unpack_census();
    let mut cursor = &raw[8..];
    let mut current = None;
    for _ in 0..5140 {
        let length = u32::from_le_bytes(word(&mut cursor)) as usize;
        let case = native_record(take(&mut cursor, length));
        let input = cached_document(&mut current, &root, &sources, case.ordinal);
        check_native_document(input, &case);
    }
    assert!(cursor.is_empty());
    println!("OVERFLOW_NATIVE_DOCUMENT_TOTAL\t5140\t440\t357");
}
