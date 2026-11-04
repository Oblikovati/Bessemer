use super::*;
use crate::unselected_document_streams::assert_unselected_document_streams;

struct NativeResourceCase<'a> {
    index: usize,
    path: &'a str,
    stream: &'a str,
    raw: &'a [u8],
}

fn native_resource_cases(packet: &[u8]) -> Vec<NativeResourceCase<'_>> {
    assert_eq!(&packet[..4], b"BRB1");
    assert_eq!(u32::from_le_bytes(packet[4..8].try_into().unwrap()), 4);
    let mut cursor = &packet[8..];
    let mut cases = Vec::new();
    while !cursor.is_empty() {
        cases.push(next_native_resource_case(&mut cursor));
    }
    assert_eq!(cases.len(), 4);
    cases
}

fn next_native_resource_case<'a>(cursor: &mut &'a [u8]) -> NativeResourceCase<'a> {
    let index = u32::from_le_bytes(cursor[..4].try_into().unwrap()) as usize;
    let label_length = u32::from_le_bytes(cursor[4..8].try_into().unwrap()) as usize;
    let size = u32::from_le_bytes(cursor[8..12].try_into().unwrap()) as usize;
    let label = std::str::from_utf8(&cursor[12..12 + label_length]).unwrap();
    let (path, stream) = label.rsplit_once("::").unwrap();
    let start = 12 + label_length;
    let raw = &cursor[start..start + size];
    *cursor = &cursor[start + size..];
    NativeResourceCase {
        index,
        path,
        stream,
        raw,
    }
}

fn native_resource_replacement(source: &BitmapResourceArray, kind: usize) -> Vec<u8> {
    let mut payload = source.payload().to_vec();
    match kind {
        0 => {}
        1 => payload.clear(),
        2 => {
            payload[0] ^= 0xa5;
            let last = payload.len() - 1;
            payload[last] ^= 0x5a;
        }
        3 => payload.extend_from_slice(&[0, 1, 254, 255]),
        4 => payload.truncate(127),
        _ => panic!("native resource mutation {kind}; expected 0..5"),
    }
    payload
}

fn verify_native_resource_document_edit(
    document: &Document,
    case: &NativeResourceCase<'_>,
    kind: usize,
) {
    let source = BitmapResourceArray::from_document(document, case.stream, MAXIMUM).unwrap();
    assert_eq!(source.to_bytes().unwrap(), case.raw);
    let replacement = native_resource_replacement(&source, kind);
    let edit = source.edit_payload(&replacement).unwrap();
    let expected = literal_array(&replacement, 6);
    verify_resource_completion(&edit, expected != case.raw);
    verify_resource_coverage(&edit, &expected, 6);
    verify_resource_history(&edit, case.raw, &expected, [6; 2]);
    let (mut output, reopened) = native_resource_reopen(document, case, edit.resource());
    let actual = BitmapResourceArray::from_document(&reopened, case.stream, MAXIMUM).unwrap();
    assert_eq!(actual.payload(), replacement);
    assert_eq!(actual.to_bytes().unwrap(), expected);
    let reverse = actual.edit_payload(source.payload()).unwrap();
    assert_eq!(reverse.resource().to_bytes().unwrap(), case.raw);
    output
        .replace_stream(case.stream, reverse.resource().to_bytes().unwrap())
        .unwrap();
    assert_eq!(output.to_bytes().unwrap(), document.to_bytes().unwrap());
    assert_eq!(source.to_bytes().unwrap(), case.raw);
}

fn native_resource_reopen(
    document: &Document,
    case: &NativeResourceCase<'_>,
    resource: &BitmapResourceArray,
) -> (Document, Document) {
    let mut output = document.clone();
    output
        .replace_stream(case.stream, resource.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&output.to_bytes().unwrap()).unwrap();
    assert_unselected_document_streams(document, &reopened, case.stream);
    (output, reopened)
}

#[test]
#[ignore = "Requires retained BRB1 resource packet and four original installed native files"]
fn all_native_resources_read_edit_reopen_preserve_other_streams_and_reverse_exact_containers() {
    let packet =
        std::fs::read("target/bitmap-resource-native-source/native-resources.bin").unwrap();
    let cases = native_resource_cases(&packet);
    let mut bytes = 0;
    for case in cases {
        let original = std::fs::read(case.path).unwrap();
        let document = Document::from_bytes(&original).unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
        for kind in 0..5 {
            verify_native_resource_document_edit(&document, &case, kind);
        }
        assert_eq!(document.to_bytes().unwrap(), original);
        bytes += case.raw.len();
        println!("RESOURCE_NATIVE_DOCUMENT\t{}\t5", case.index);
    }
    assert_eq!(bytes, 924273);
}
