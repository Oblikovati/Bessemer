use super::*;
use bessemer::Document;
use std::collections::BTreeMap;

const STREAM: &str = "Contents/3DExperienceExchange2";

fn document_with_graph(raw: &[u8]) -> Document {
    let mut document =
        Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, raw.to_vec()).unwrap();
    document
}

fn unrelated_streams(document: &Document) -> BTreeMap<String, Vec<u8>> {
    document
        .streams()
        .filter(|stream| stream.name != STREAM)
        .map(|stream| {
            (
                stream.name.to_string(),
                document.read_stream(stream.name).unwrap(),
            )
        })
        .collect()
}

fn check_containing_document(source: &Document, case: &FileCase) {
    let original = source.to_bytes().unwrap();
    let graph = parse(&source.read_stream(STREAM).unwrap());
    let edit = graph
        .edit_value(case.page, case.offset, &case.replacement)
        .unwrap();
    let mut result = source.clone();
    result
        .replace_stream(STREAM, edit.result().to_bytes())
        .unwrap();
    let mut fresh = reopen_checked(source, &result, case);
    let fresh_graph = parse(&fresh.read_stream(STREAM).unwrap());
    let reverse = fresh_graph
        .edit_value(case.page, case.offset, &case.value)
        .unwrap();
    fresh
        .replace_stream(STREAM, reverse.result().to_bytes())
        .unwrap();
    assert_eq!(fresh.to_bytes().unwrap(), original);
    assert_eq!(source.to_bytes().unwrap(), original);
}

fn reopen_checked(source: &Document, result: &Document, case: &FileCase) -> Document {
    let fresh = Document::from_bytes(&result.to_bytes().unwrap()).unwrap();
    assert_eq!(fresh.read_stream(STREAM).unwrap(), case.expected);
    assert_eq!(unrelated_streams(&fresh), unrelated_streams(source));
    fresh
}

#[test]
fn complete_owned_graph_edits_reopen_in_containing_documents_and_reverse_exactly() {
    let (raw, cases) = cases();
    let source = document_with_graph(&raw);
    for case in cases {
        check_containing_document(&source, &case);
    }
}
