//! Complete native-document PNG edits and rollback without unrelated stream changes.
use bessemer::{
    Document,
    preview::{PngLimits, PngPreview},
};
const PART: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");
const LIMITS: PngLimits = PngLimits {
    maximum_pixels: 4_000_000,
    maximum_inflated_bytes: 64 * 1024 * 1024,
    maximum_chunks: 100_000,
};

#[test]
fn full_document_sample_edit_reopens_and_preserves_every_other_stream() {
    let mut document = Document::from_bytes(PART).unwrap();
    let original = document.read_stream("PreviewPNG").unwrap();
    let preview = PngPreview::from_bytes(&original, LIMITS).unwrap();
    let mut samples = preview.raster().samples.clone();
    samples[0] ^= 1;
    let edited = preview.with_samples(samples.clone(), LIMITS).unwrap();
    document
        .replace_stream("PreviewPNG", edited.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    let decoded =
        PngPreview::from_bytes(&reopened.read_stream("PreviewPNG").unwrap(), LIMITS).unwrap();
    assert_eq!(decoded.raster().samples, samples);
    for stream in document
        .streams()
        .filter(|stream| stream.name != "PreviewPNG")
    {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            document.read_stream(stream.name).unwrap()
        );
    }
    document
        .replace_stream("PreviewPNG", original.clone())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), PART);
    assert_eq!(preview.to_bytes().unwrap(), original);
}

#[test]
fn forced_chunk_reconstruction_preserves_the_entire_native_document() {
    let mut document = Document::from_bytes(PART).unwrap();
    let original = document.read_stream("PreviewPNG").unwrap();
    let preview = PngPreview::from_bytes(&original, LIMITS).unwrap();
    let rebuilt = PngPreview::new(preview.contents().clone(), LIMITS).unwrap();
    document
        .replace_stream("PreviewPNG", rebuilt.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), PART);
}
