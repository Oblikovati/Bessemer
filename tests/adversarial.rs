//! Independently constructed ZIP-layout cases, without native filler or a key table.

use bessemer::{Document, FailureClass};

const STORED: &[u8] = &[0xcb, 0x48, 0xcd, 0xc9, 0xc9, 0x07, 0x00]; // RFC 1951 encoding of hello.

fn little_word(bytes: &mut [u8], offset: usize, value: u16) {
    bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn little_dword(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn fixture() -> Vec<u8> {
    let mut bytes = vec![0; 8 + 30 + 1 + STORED.len() + 46 + 1 + 22];
    bytes[..8].copy_from_slice(&[0x12, 0x34, 0x56, 0x78, 0, 0, 0, 4]);
    bytes[8..12].copy_from_slice(b"loc!");
    bytes[38] = b'x'.rotate_left(4);
    bytes[39..46].copy_from_slice(STORED);
    let central = 46;
    bytes[central..central + 4].copy_from_slice(b"dir!");
    for (local, directory, value) in [(12, 52, 20), (14, 54, 6), (16, 56, 8), (34, 74, 1)] {
        little_word(&mut bytes, local, value);
        little_word(&mut bytes, directory, value);
    }
    for (local, directory, value) in [(22, 62, 0x3610_a686), (26, 66, 7), (30, 70, 5)] {
        little_dword(&mut bytes, local, value);
        little_dword(&mut bytes, directory, value);
    }
    finish_directory(&mut bytes);
    bytes
}

fn finish_directory(bytes: &mut [u8]) {
    bytes[92] = b'x'.rotate_left(4);
    bytes[93..97].copy_from_slice(b"end!");
    little_word(bytes, 101, 1);
    little_word(bytes, 103, 1);
    little_dword(bytes, 105, 47);
    little_dword(bytes, 109, 38);
}

#[test]
fn filler_free_container_and_empty_replacement() {
    let mut document = Document::from_bytes(&fixture()).unwrap();
    assert_eq!(document.read_stream("x").unwrap(), b"hello");
    document.replace_stream("x", vec![]).unwrap();
    let output = document.to_bytes().unwrap();
    assert!(
        Document::from_bytes(&output)
            .unwrap()
            .read_stream("x")
            .unwrap()
            .is_empty()
    );
}

#[test]
fn bad_crc_and_expanded_sizes_fail_extraction() {
    for (local, central, value) in [(22, 62, 0), (30, 70, 4), (30, 70, 6)] {
        let mut bytes = fixture();
        little_dword(&mut bytes, local, value);
        little_dword(&mut bytes, central, value);
        let document = Document::from_bytes(&bytes).unwrap();
        let error = document.read_stream("x").unwrap_err();
        assert_eq!(error.class, FailureClass::Integrity);
        assert_eq!(error.entity, "x");
        assert!(error.to_string().contains("expected"));
    }
}

#[test]
fn mismatched_names_sizes_and_offsets_fail_parsing() {
    for (offset, value) in [(92, 0), (66, 8), (88, 255), (101, 2), (109, 255)] {
        let mut bytes = fixture();
        bytes[offset] = value;
        assert!(Document::from_bytes(&bytes).is_err(), "offset {offset}");
    }
}

#[test]
fn unsupported_methods_and_flags_are_typed() {
    for (local, central, value) in [(16, 56, 99), (14, 54, 1), (14, 54, 8)] {
        let mut bytes = fixture();
        little_word(&mut bytes, local, value);
        little_word(&mut bytes, central, value);
        assert_eq!(
            Document::from_bytes(&bytes).unwrap_err().class,
            FailureClass::Unsupported
        );
    }
}

#[test]
fn stored_method_checks_sizes_and_crc() {
    let mut bytes = fixture();
    for offset in [16, 56] {
        little_word(&mut bytes, offset, 0);
    }
    bytes[39..46].copy_from_slice(b"hello!!");
    let document = Document::from_bytes(&bytes).unwrap();
    assert_eq!(
        document.read_stream_limited("x", 5).unwrap_err().class,
        FailureClass::Integrity
    );
}

#[test]
fn every_truncation_of_filler_free_file_is_rejected() {
    let bytes = fixture();
    for end in 0..bytes.len() {
        assert!(Document::from_bytes(&bytes[..end]).is_err());
    }
}

fn two_stream_fixture() -> Vec<u8> {
    let original = fixture();
    let mut bytes = original[..46].to_vec();
    bytes.extend_from_slice(&original[8..39]);
    bytes[76] = b'y'.rotate_left(4);
    bytes.extend_from_slice(&[1, 5, 0, 250, 255, b'h', b'e', b'l', b'l', b'o']);
    little_dword(&mut bytes, 64, 10);
    bytes.extend_from_slice(&original[46..93]);
    let mut second = original[46..93].to_vec();
    second[46] = b'y'.rotate_left(4);
    little_dword(&mut second, 20, 10);
    little_dword(&mut second, 42, 38);
    bytes.extend_from_slice(&second);
    bytes.extend_from_slice(&original[93..]);
    let end = bytes.len() - 22;
    little_word(&mut bytes, end + 8, 2);
    little_word(&mut bytes, end + 10, 2);
    little_dword(&mut bytes, end + 12, 94);
    little_dword(&mut bytes, end + 16, 79);
    bytes
}

#[test]
fn untouched_deflate_representation_is_preserved_exactly() {
    let bytes = two_stream_fixture();
    let mut document = Document::from_bytes(&bytes).unwrap();
    assert_eq!(document.read_stream("y").unwrap(), b"hello");
    document.replace_stream("x", vec![]).unwrap();
    let output = document.to_bytes().unwrap();
    // The independent y encoding uses an uncompressed deflate block. Recompression
    // would replace it with a different representation even though the CRC agrees.
    assert!(output.windows(41).any(|window| window == &bytes[46..87]));
    let reopened = Document::from_bytes(&output).unwrap();
    assert_eq!(reopened.read_stream("y").unwrap(), b"hello");
}

#[test]
fn truncated_deflate_with_correct_crc_is_rejected() {
    let mut bytes = fixture();
    bytes.remove(45);
    little_dword(&mut bytes, 26, 6);
    little_dword(&mut bytes, 65, 6);
    little_dword(&mut bytes, 108, 37);
    let document = Document::from_bytes(&bytes).unwrap();
    assert_eq!(
        document.read_stream("x").unwrap_err().class,
        FailureClass::Integrity
    );
}
