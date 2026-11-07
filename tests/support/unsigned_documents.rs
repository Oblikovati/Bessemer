//! Negative and positive aliases resolve to the same unsigned document context.
use super::*;
use bessemer::Document;
use bessemer::tessellation::{
    ArchiveVersion, FacetArchiveMode, TessellationContext, TessellationLimits,
};

fn resolved_context(document: &Document, kind: DisplayDocumentKind) -> DisplayArchiveContext {
    let tessellation = TessellationContext {
        archive_version: ArchiveVersion::new(19000),
        mode: FacetArchiveMode::Full,
    };
    let limits = TessellationLimits {
        maximum_components: 100,
        maximum_faces: 100,
        maximum_array_elements: 1000,
    };
    DisplayArchiveContext::from_document(document, kind, 19000, tessellation, limits, 1024 * 1024)
        .unwrap()
}

fn replace_reopen(document: &mut Document, archive: &DisplayArchive) -> Document {
    document
        .replace_stream("Contents/DisplayLists", archive.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    assert_eq!(
        reopened.read_stream("Unrelated/Payload").unwrap(),
        b"retained\0payload"
    );
    reopened
}

fn check_document(bytes: &[u8], kind: DisplayDocumentKind, expected: &[u8]) {
    let mut document = Document::from_bytes(bytes).unwrap();
    let context = resolved_context(&document, kind);
    assert_eq!(context.version.value(), u32::MAX);
    assert_eq!(context.file_version, 19000);
    let raw = document.read_stream("Contents/DisplayLists").unwrap();
    let source = DisplayArchive::from_bytes(&raw, context, LIMITS).unwrap();
    let mut contents = source.contents().clone();
    archive_support::set_integer(&mut contents, "root.views[0].view_index", 23);
    let edited = source.with_contents(contents, LIMITS).unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected);
    let mut reopened = replace_reopen(&mut document, &edited);
    check_revert(&mut reopened, &source, kind, &raw);
    assert_eq!(source.to_bytes().unwrap(), raw);
}

fn check_revert(
    document: &mut Document,
    source: &DisplayArchive,
    kind: DisplayDocumentKind,
    raw: &[u8],
) {
    let context = resolved_context(document, kind);
    assert_eq!(context.version.value(), u32::MAX);
    let edited = DisplayArchive::from_bytes(
        &document.read_stream("Contents/DisplayLists").unwrap(),
        context,
        LIMITS,
    )
    .unwrap();
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    let reopened = replace_reopen(document, &reverted);
    assert_eq!(reopened.read_stream("Contents/DisplayLists").unwrap(), raw);
}

#[test]
fn both_document_kinds_keep_unsigned_markers_through_independent_edits_and_reverts() {
    check_document(
        include_bytes!("../fixtures/display-lists/unsigned-document-part.bin"),
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/unsigned-edited-part.bin"),
    );
    check_document(
        include_bytes!("../fixtures/display-lists/unsigned-document-assembly.bin"),
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/unsigned-edited-assembly.bin"),
    );
}
