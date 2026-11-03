use super::{DisplayArchive, DisplayArchiveContext, DisplayDocumentKind, archive_support};
use bessemer::Document;
use bessemer::tessellation::{
    ArchiveVersion, FacetArchiveMode, TessellationContext, TessellationLimits,
};

const DISPLAY_STREAM: &str = "Contents/DisplayLists";

fn component_document(kind: DisplayDocumentKind) -> Document {
    let bytes: &[u8] = match kind {
        DisplayDocumentKind::Part => include_bytes!("../fixtures/blank-part.SLDPRT"),
        DisplayDocumentKind::Assembly => include_bytes!("../fixtures/blank-assembly.SLDASM"),
    };
    Document::from_bytes(bytes).unwrap()
}

fn component_document_context(
    document: &Document,
    archive: &DisplayArchive,
) -> DisplayArchiveContext {
    let tessellation = TessellationContext {
        archive_version: ArchiveVersion::new(19000),
        mode: FacetArchiveMode::Full,
    };
    let context = DisplayArchiveContext::from_document(
        document,
        archive.context().kind,
        archive.context().file_version,
        tessellation,
        TessellationLimits {
            maximum_components: 100,
            maximum_faces: 100,
            maximum_array_elements: 1000,
        },
        archive_support::LIMITS.maximum_bytes,
    )
    .unwrap();
    assert_eq!(context, archive.context());
    context
}

fn verify_component_document_streams(before: &Document, after: &Document) {
    crate::unselected_document_streams::assert_unselected_document_streams(
        before,
        after,
        DISPLAY_STREAM,
    );
}

fn reopen_component_document(document: &Document, archive: &DisplayArchive) -> Document {
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    let context = component_document_context(&reopened, archive);
    let bytes = reopened.read_stream(DISPLAY_STREAM).unwrap();
    let actual = DisplayArchive::from_bytes(&bytes, context, archive_support::LIMITS).unwrap();
    assert_eq!(actual.contents(), archive.contents());
    reopened
}

pub(super) fn verify_component_document(source: &DisplayArchive, result: &DisplayArchive) {
    let mut original = component_document(source.context().kind);
    let original_bytes = source.to_bytes().unwrap();
    original
        .replace_stream(DISPLAY_STREAM, original_bytes.clone())
        .unwrap();
    let before = reopen_component_document(&original, source);
    let mut output = before.clone();
    output
        .replace_stream(DISPLAY_STREAM, result.to_bytes().unwrap())
        .unwrap();
    let mut reopened = reopen_component_document(&output, result);
    verify_component_document_streams(&before, &reopened);
    reopened
        .replace_stream(DISPLAY_STREAM, original_bytes)
        .unwrap();
    assert_eq!(reopened.to_bytes().unwrap(), before.to_bytes().unwrap());
    assert_eq!(original.to_bytes().unwrap(), before.to_bytes().unwrap());
}
