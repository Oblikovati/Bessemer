//! Immutable decoded rasters, source isolation, and deterministic concurrent PNG edits.
#[path = "support/png.rs"]
mod support;
use bessemer::preview::*;
use support::*;

#[test]
fn public_types_are_send_sync_and_source_input_edits_and_failed_operations_are_isolated() {
    fn shared<T: Send + Sync>() {}
    shared::<PngPreview>();
    shared::<PngContents>();
    shared::<PngChunk>();
    shared::<ChunkType>();
    shared::<PngColor>();
    shared::<PngHeader>();
    shared::<PngLimits>();
    shared::<PngRaster>();
    let mut bytes = BASE.to_vec();
    let source = parse(&bytes);
    bytes.fill(0);
    let original = source.contents().clone();
    let mut samples = source.raster().samples.clone();
    samples[0] ^= 1;
    let edited = source.with_samples(samples.clone(), LIMITS).unwrap();
    samples[0] ^= 2;
    assert_ne!(edited.raster().samples[0], samples[0]);
    let mut contents = source.contents().clone();
    contents.chunks[0].payload.clear();
    assert!(source.with_contents(contents, LIMITS).is_err());
    assert_eq!(source.to_bytes().unwrap(), BASE);
    assert_eq!(
        edited
            .with_contents(original, LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        BASE
    );
}

#[test]
fn eight_rounds_of_eight_parallel_edits_equal_serial_bytes_and_samples() {
    let source = parse(BASE);
    let expected: Vec<Vec<u8>> = (0..8).map(|value| edit(&source, value)).collect();
    for _ in 0..8 {
        std::thread::scope(|scope| {
            let tasks: Vec<_> = (0..8)
                .map(|value| {
                    let source = &source;
                    scope.spawn(move || edit(source, value))
                })
                .collect();
            for (task, expected) in tasks.into_iter().zip(&expected) {
                assert_eq!(task.join().unwrap(), *expected);
            }
        });
    }
    assert_eq!(source.to_bytes().unwrap(), BASE);
}

fn edit(source: &PngPreview, value: u16) -> Vec<u8> {
    let mut samples = source.raster().samples.clone();
    samples[0] = value;
    let changed = source.with_samples(samples, LIMITS).unwrap();
    assert_eq!(
        parse(&changed.to_bytes().unwrap()).raster(),
        changed.raster()
    );
    changed.to_bytes().unwrap()
}
