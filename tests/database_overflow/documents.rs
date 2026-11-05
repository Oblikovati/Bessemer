use super::*;
use bessemer::Document;
const STREAM: &str = "Contents/3DExperienceExchange2";

pub(super) fn stream_with_pages(raw: &[u8], payload: &OverflowPayload) -> Vec<u8> {
    let mut output = raw.to_vec();
    for page in payload.pages() {
        let start = page.identifier.get() as usize * 4096;
        output[start..start + 4096].copy_from_slice(&page.bytes);
    }
    output
}

pub(super) fn payload_from_stream(raw: &[u8], case: &OwnedOverflowCase) -> OverflowPayload {
    let pages: Vec<_> = case
        .pages
        .iter()
        .map(|page| {
            let start = page.identifier.get() as usize * 4096;
            OverflowPageImage {
                identifier: page.identifier,
                bytes: raw[start..start + 4096].to_vec(),
            }
        })
        .collect();
    OverflowPayload::from_pages(
        &pages,
        case.pages[0].identifier,
        case.key.len() as u32,
        case.value.len() as u64,
        context(case),
    )
    .unwrap()
}

pub(super) fn assert_unselected_streams(source: &Document, result: &Document) {
    for stream in source.streams().filter(|stream| stream.name != STREAM) {
        assert_eq!(
            source.read_stream(stream.name).unwrap(),
            result.read_stream(stream.name).unwrap()
        );
    }
}

pub(super) fn containing_document() -> Document {
    let mut source = Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    source
        .replace_stream(STREAM, NATIVE_STREAMS[2].to_vec())
        .unwrap();
    source
}

fn reverse_document(source: &Document, mut reopened: Document, case: &OwnedOverflowCase) {
    let raw = reopened.read_stream(STREAM).unwrap();
    let fresh = payload_from_stream(&raw, case);
    let reverse = fresh.edit_value(&case.value).unwrap();
    reopened
        .replace_stream(STREAM, stream_with_pages(&raw, reverse.result()))
        .unwrap();
    assert_eq!(reopened.to_bytes().unwrap(), source.to_bytes().unwrap());
}

#[test]
fn two_page_native_storage_value_reopens_in_complete_owned_document_and_reverses_exactly() {
    let case = native_case(2);
    let source = containing_document();
    let original = source.to_bytes().unwrap();
    let payload = parse_case(&case).unwrap();
    let replacement: Vec<_> = case.value.iter().map(|byte| byte ^ 0x5a).collect();
    let edit = payload.edit_value(&replacement).unwrap();
    let mut output = source.clone();
    output
        .replace_stream(STREAM, stream_with_pages(NATIVE_STREAMS[2], edit.result()))
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    let selected = payload_from_stream(&reopened.read_stream(STREAM).unwrap(), &case);
    assert_eq!(selected.value(), replacement);
    assert_unselected_streams(&source, &reopened);
    reverse_document(&source, reopened, &case);
    assert_eq!(source.to_bytes().unwrap(), original);
}
