//! CString code-unit counts and all length-prefix boundaries, including reserved tags.

use bessemer::history::{ArchiveString, TextEncoding, VersionHistory};

const ANCIENT: &[u8] = include_bytes!("fixtures/history/threshold-0026.bin");
const MIXED: &[u8] = include_bytes!("fixtures/history/mixed.bin");

fn encoded_comment(text: &str, encoding: TextEncoding) -> Vec<u8> {
    let source = VersionHistory::from_bytes(ANCIENT).unwrap();
    let mut contents = source.contents().clone();
    contents.states[0].stamp.comment = ArchiveString::new(text, encoding).unwrap();
    VersionHistory::new(source.root_schema(), contents)
        .unwrap()
        .to_bytes()
        .unwrap()
}

#[test]
fn latin1_reserved_wide_marker_is_escaped_with_a_dword_length() {
    for (count, prefix) in [
        (254, vec![254]),
        (255, vec![255, 255, 0]),
        (65_533, vec![255, 253, 255]),
        (65_534, vec![255, 255, 255, 254, 255, 0, 0]),
        (65_535, vec![255, 255, 255, 255, 255, 0, 0]),
    ] {
        let text = "é".repeat(count);
        let encoded = encoded_comment(&text, TextEncoding::Latin1);
        assert!(encoded[38..].starts_with(&prefix), "count {count}");
        let reopened = VersionHistory::from_bytes(&encoded).unwrap();
        assert_eq!(reopened.contents().states[0].stamp.comment.text(), text);
        assert_eq!(
            reopened.contents().states[0].stamp.comment.encoding(),
            TextEncoding::Latin1
        );
    }
}

#[test]
fn wide_length_is_counted_in_code_units_and_allows_word_fffe() {
    for (count, prefix) in [
        (254, vec![254]),
        (255, vec![255, 255, 0]),
        (65_534, vec![255, 254, 255]),
        (65_535, vec![255, 255, 255, 255, 255, 0, 0]),
    ] {
        let text = "λ".repeat(count);
        let encoded = encoded_comment(&text, TextEncoding::Utf16);
        assert_eq!(&encoded[38..41], &[255, 254, 255]);
        assert!(encoded[41..].starts_with(&prefix), "count {count}");
        let reopened = VersionHistory::from_bytes(&encoded).unwrap();
        assert_eq!(reopened.contents().states[0].stamp.comment.text(), text);
    }
    let emoji = encoded_comment("😀", TextEncoding::Utf16);
    assert_eq!(&emoji[38..46], &[255, 254, 255, 2, 0x3d, 0xd8, 0, 0xde]);
}

#[test]
fn independent_nonminimal_dword_length_is_preserved_by_the_encoder() {
    let mut raw = ANCIENT.to_vec();
    raw.splice(38..39, [255, 255, 255, 1, 0, 0, 0, 0xe9]);
    let source = VersionHistory::from_bytes(&raw).unwrap();
    assert_eq!(source.contents().states[0].stamp.comment.text(), "é");
    let rebuilt = VersionHistory::new(source.root_schema(), source.contents().clone()).unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), raw);
}

#[test]
fn same_text_in_the_same_encoding_does_not_canonicalize_a_pending_edit() {
    let source = VersionHistory::from_bytes(MIXED).unwrap();
    let mut contents = source.contents().clone();
    contents.states[2].stamp.comment = ArchiveString::new("", TextEncoding::Latin1).unwrap();
    assert_eq!(
        source.with_contents(contents).unwrap().to_bytes().unwrap(),
        MIXED
    );
}

#[test]
fn latin1_rejects_unrepresentable_text_and_preserves_embedded_nuls() {
    let error = ArchiveString::new("λ", TextEncoding::Latin1).unwrap_err();
    assert!(error.cause.contains("λ"));
    assert!(error.cause.contains("Latin-1"));
    let text = "a\0ÿ";
    let encoded = encoded_comment(text, TextEncoding::Latin1);
    assert_eq!(&encoded[38..42], &[3, b'a', 0, 255]);
    assert_eq!(
        VersionHistory::from_bytes(&encoded)
            .unwrap()
            .contents()
            .states[0]
            .stamp
            .comment
            .text(),
        text
    );
}
