//! Whole inline documents resolve their declared versions without any face directory.
use super::*;
use bessemer::Document;

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 2] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/resumed-inline-document-part.bin"),
        include_bytes!("../fixtures/display-lists/resumed-inline-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/resumed-inline-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/resumed-inline-edited-document-assembly.bin"),
    ),
];

fn resolved_archive(document: &Document, kind: DisplayDocumentKind) -> DisplayArchive {
    let directory = TessellationContext {
        archive_version: ArchiveVersion::new(19000),
        mode: FacetArchiveMode::Full,
    };
    let limits = directory_limits();
    let context =
        DisplayArchiveContext::from_document(document, kind, 19000, directory, limits, 1024 * 1024)
            .unwrap();
    assert!(matches!(
        context.version.value(),
        1286 | 1332
            | 1645
            | 1857
            | 1860
            | 1875
            | 1900
            | 2011
            | 2078
            | 2130
            | 2277
            | 2359
            | 2388
            | 2563
            | 2596
    ));
    assert!(context.face_tessellation_components.is_empty());
    let raw = document.read_stream("Contents/DisplayLists").unwrap();
    DisplayArchive::from_bytes(&raw, context, LIMITS).unwrap()
}

fn directory_limits() -> TessellationLimits {
    TessellationLimits {
        maximum_components: 100,
        maximum_faces: 100,
        maximum_array_elements: 1000,
    }
}

fn replace_reopen(original: &Document, archive: &DisplayArchive) -> Document {
    let mut result = original.clone();
    result
        .replace_stream("Contents/DisplayLists", archive.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&result.to_bytes().unwrap()).unwrap();
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

pub(super) fn check_document_edit(kind: DisplayDocumentKind, raw: &[u8], expected_raw: &[u8]) {
    let original = Document::from_bytes(raw).unwrap();
    let source = resolved_archive(&original, kind);
    let expected = resolved_archive(&Document::from_bytes(expected_raw).unwrap(), kind);
    let edited = source
        .with_contents(resumed_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected.to_bytes().unwrap());
    let reopened = replace_reopen(&original, &edited);
    let result = resolved_archive(&reopened, kind);
    assert_eq!(inline_facets(&result).len(), 2);
    check_document_revert(&original, &source, &reopened, &result, kind);
}

fn check_document_revert(
    original: &Document,
    source: &DisplayArchive,
    reopened: &Document,
    result: &DisplayArchive,
    kind: DisplayDocumentKind,
) {
    let reverted = result
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    let restored = replace_reopen(reopened, &reverted);
    assert_eq!(
        resolved_archive(&restored, kind).to_bytes().unwrap(),
        source.to_bytes().unwrap()
    );
    assert_eq!(
        source.to_bytes().unwrap(),
        original.read_stream("Contents/DisplayLists").unwrap()
    );
}

#[test]
fn complete_inline_part_and_assembly_documents_edit_reopen_and_authentically_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        check_document_edit(kind, raw, expected);
    }
}
