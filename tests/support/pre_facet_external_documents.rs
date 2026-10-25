//! Complete older containers keep cache links intact through edits and reopenings.
use super::*;
use bessemer::Document;

type DocumentCase = (u32, &'static [u8], &'static [u8]);

fn documents() -> [DocumentCase; 2] {
    [
        (
            3065,
            include_bytes!("../fixtures/display-lists/pre-facet-external-3065-0.SLDASM"),
            include_bytes!("../fixtures/display-lists/pre-facet-external-3065-1.SLDASM"),
        ),
        (
            3195,
            include_bytes!("../fixtures/display-lists/pre-facet-external-3195-0.SLDASM"),
            include_bytes!("../fixtures/display-lists/pre-facet-external-3195-1.SLDASM"),
        ),
    ]
}

fn resolved_archive(document: &Document) -> DisplayArchive {
    let context = DisplayArchiveContext::from_document(
        document,
        DisplayDocumentKind::Assembly,
        19000,
        tess_support::modern_context(),
        tess_support::LIMITS,
        1024 * 1024,
    )
    .unwrap();
    let raw = document.read_stream("Contents/DisplayLists").unwrap();
    DisplayArchive::from_bytes(&raw, context, LIMITS).unwrap()
}

fn check_document_links(document: &Document, archive: &DisplayArchive) {
    let context = tess_support::modern_context();
    let directory = document.read_stream("FaceTessellations/Directory").unwrap();
    let directory =
        TessellationDirectory::from_bytes(&directory, context, tess_support::LIMITS).unwrap();
    let raw = document
        .read_stream("FaceTessellations/000-000-007")
        .unwrap();
    let block = FaceTessellationBlock::from_bytes(&raw, context, tess_support::LIMITS).unwrap();
    let blocks = [("000-000-007".to_owned(), block)].into();
    let links = archive
        .check_external_face_links(&directory, &blocks)
        .unwrap();
    assert_eq!(links.coverage().checked_face_references, 2);
    assert_eq!(links.coverage().unique_referenced_faces, 2);
    assert_eq!(links.coverage().unreferenced_block_faces, 0);
}

fn replace_reopen(original: &Document, archive: &DisplayArchive) -> Document {
    let mut document = original.clone();
    document
        .replace_stream("Contents/DisplayLists", archive.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            original.read_stream(stream.name).unwrap()
        );
    }
    reopened
}

pub(crate) fn check_edit(
    version: u32,
    raw: &[u8],
    expected: &[u8],
    build_contents: fn(&DisplayArchive, &DisplayArchive) -> DisplayArchiveContents,
) {
    let original = Document::from_bytes(raw).unwrap();
    let source = resolved_archive(&original);
    assert_eq!(source.context().version.value(), version);
    assert_eq!(source.context().file_version, 19000);
    check_document_links(&original, &source);
    let expected = resolved_archive(&Document::from_bytes(expected).unwrap());
    let edited = source
        .with_contents(build_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected.to_bytes().unwrap());
    let reopened = replace_reopen(&original, &edited);
    let result = resolved_archive(&reopened);
    check_document_links(&reopened, &result);
    check_revert(&original, &reopened, &source, &result);
}

fn check_revert(
    original: &Document,
    edited: &Document,
    source: &DisplayArchive,
    result: &DisplayArchive,
) {
    let reverted = result
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    let reopened = replace_reopen(edited, &reverted);
    let result = resolved_archive(&reopened);
    assert_eq!(result.to_bytes().unwrap(), source.to_bytes().unwrap());
    assert_eq!(
        result.to_bytes().unwrap(),
        original.read_stream("Contents/DisplayLists").unwrap()
    );
    check_document_links(&reopened, &result);
}

#[test]
fn complete_older_documents_keep_newer_cache_versions_through_independent_edits_and_reverts() {
    for (version, source, expected) in documents() {
        check_edit(version, source, expected, edited_contents);
    }
}
