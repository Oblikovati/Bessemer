//! Complete document replacement using the owned native blank-part fixture.
use bessemer::{Document, preview::DibPreview};
const PART: &[u8] = include_bytes!("fixtures/blank-part.SLDPRT");

#[test]
fn palette_edit_preserves_pixels_every_other_stream_and_reverts_the_document() {
    let mut document = Document::from_bytes(PART).unwrap();
    let original = document.read_stream("Preview").unwrap();
    let preview = DibPreview::from_bytes(&original, 4_000_000).unwrap();
    let mut contents = preview.contents().clone();
    contents.palette[0].red ^= 0xff;
    let edited = preview.with_contents(contents.clone(), 4_000_000).unwrap();
    assert_eq!(
        edited.decode_pixels(4_000_000).unwrap(),
        preview.decode_pixels(4_000_000).unwrap()
    );
    document
        .replace_stream("Preview", edited.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    assert_eq!(
        DibPreview::from_bytes(&reopened.read_stream("Preview").unwrap(), 4_000_000)
            .unwrap()
            .contents(),
        &contents
    );
    for stream in document.streams().filter(|stream| stream.name != "Preview") {
        assert_eq!(
            reopened.read_stream(stream.name).unwrap(),
            document.read_stream(stream.name).unwrap()
        );
    }
    document.replace_stream("Preview", original).unwrap();
    assert_eq!(document.to_bytes().unwrap(), PART);
}

#[test]
fn complete_pixel_replacement_reopens_and_preserves_the_original_preview() {
    let mut document = Document::from_bytes(PART).unwrap();
    let original = document.read_stream("Preview").unwrap();
    let preview = DibPreview::from_bytes(&original, 4_000_000).unwrap();
    let count =
        preview.contents().header.width as usize * preview.contents().header.height as usize;
    let pixels = vec![0; count];
    let edited = preview.with_indexed_pixels(&pixels, 4_000_000).unwrap();
    document
        .replace_stream("Preview", edited.to_bytes().unwrap())
        .unwrap();
    let reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    let preview_pixels =
        DibPreview::from_bytes(&reopened.read_stream("Preview").unwrap(), 4_000_000)
            .unwrap()
            .decode_pixels(4_000_000)
            .unwrap();
    assert_eq!(
        preview_pixels,
        bessemer::preview::DibPixels::Indexed(pixels.into_iter().map(Some).collect())
    );
    assert_eq!(preview.to_bytes().unwrap(), original);
}
