//! Owned public-boundary contracts pinned before sharing the bounded RFC 1950 decoder.
#[path = "support/png.rs"]
mod support;
use bessemer::{BessemerError, FailureClass, Result, preview::*};
use support::*;

fn text_preview(stored: &[u8], maximum: usize) -> Result<PngPreview> {
    let mut contents = parse(BASE).contents().clone();
    contents
        .chunks
        .retain(|chunk| matches!(&chunk.kind.bytes(), b"IHDR" | b"IDAT" | b"IEND"));
    let payload = [b"Name\0\0".as_slice(), stored].concat();
    before_image(&mut contents, chunk(*b"zTXt", &payload));
    PngPreview::new(
        contents,
        PngLimits {
            maximum_inflated_bytes: maximum,
            ..LIMITS
        },
    )
}

fn assert_failure(error: &BessemerError, class: FailureClass, cause: &str) {
    assert_eq!(error.operation, "PNG preview");
    assert_eq!(error.entity, "PNG datastream");
    assert_eq!(error.class, class);
    assert_eq!(error.cause, cause);
}

#[test]
fn exact_remaining_budget_and_one_beyond_have_owned_expected_results() {
    assert!(text_preview(&compressed(b"hi"), 335).is_ok());
    let error = text_preview(&compressed(b"hii"), 335).unwrap_err();
    assert_failure(
        &error,
        FailureClass::Invalid,
        "ancillary expansion exceeds 2; expected within remaining budget",
    );
    assert!(text_preview(&compressed(b""), 333).is_ok());
}

#[test]
fn trailing_and_concatenated_streams_keep_exact_consumption_diagnostics() {
    let stream = compressed(b"hi");
    for suffix in [vec![0], stream.clone()] {
        let mut stored = stream.clone();
        stored.extend(suffix);
        let error = text_preview(&stored, 1000).unwrap_err();
        let expected = format!(
            "ancillary zlib consumed {} bytes; expected exactly {}",
            stream.len(),
            stored.len()
        );
        assert_failure(&error, FailureClass::Integrity, &expected);
    }
}

#[test]
fn empty_truncated_checksum_and_raw_streams_keep_integrity_failures() {
    let empty = text_preview(b"", 1000).unwrap_err();
    assert_failure(
        &empty,
        FailureClass::Integrity,
        "ancillary zlib BufError consumed 0/0; expected StreamEnd",
    );
    let stream = compressed(b"hi");
    for end in 0..stream.len() {
        assert!(text_preview(&stream[..end], 1000).is_err());
    }
    let mut corrupt = stream.clone();
    let last = corrupt.len() - 1;
    corrupt[last] ^= 1;
    for bytes in [&corrupt[..], &stream[2..stream.len() - 4], b"\x1f\x8b"] {
        let error = text_preview(bytes, 1000).unwrap_err();
        assert_eq!(error.class, FailureClass::Integrity);
        assert!(error.cause.starts_with("PNG ancillary zlib error "));
    }
}
