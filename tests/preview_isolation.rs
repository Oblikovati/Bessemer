//! Immutable publication and concurrency evidence for decoded previews.
#[path = "support/preview.rs"]
mod support;
use bessemer::preview::*;
use support::*;

#[test]
fn public_types_are_send_sync_and_owned_edits_do_not_alias() {
    fn shared<T: Send + Sync>() {}
    shared::<DibPreview>();
    shared::<DibContents>();
    shared::<DibPixels>();
    shared::<BitmapInfoHeader>();
    shared::<PaletteEntry>();
    let mut bytes = RLE.to_vec();
    let source = parse(&bytes);
    bytes.fill(0);
    let original = source.contents().clone();
    let mut contents = original.clone();
    contents.palette[0].red = 42;
    let edited = source.with_contents(contents.clone(), LIMIT).unwrap();
    contents.palette[0].red = 43;
    assert_eq!(edited.contents().palette[0].red, 42);
    assert_eq!(source.to_bytes().unwrap(), RLE);
    assert_eq!(
        edited
            .with_contents(original, LIMIT)
            .unwrap()
            .to_bytes()
            .unwrap(),
        RLE
    );
}

#[test]
fn eight_rounds_of_concurrent_edits_and_reads_match_serial_bytes() {
    let source = parse(RLE);
    let expected: Vec<Vec<u8>> = (0..8).map(|i| edit(&source, i)).collect();
    for _ in 0..8 {
        std::thread::scope(|scope| {
            let tasks: Vec<_> = (0..8)
                .map(|i| {
                    let source = &source;
                    scope.spawn(move || edit(source, i))
                })
                .collect();
            for (task, expected) in tasks.into_iter().zip(&expected) {
                assert_eq!(task.join().unwrap(), *expected);
            }
        });
    }
    assert_eq!(source.to_bytes().unwrap(), RLE);
}

fn edit(source: &DibPreview, value: u8) -> Vec<u8> {
    source.decode_pixels(LIMIT).unwrap();
    source
        .with_indexed_pixels(&[value % 5; 20], LIMIT)
        .unwrap()
        .to_bytes()
        .unwrap()
}
