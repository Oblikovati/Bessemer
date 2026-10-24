//! Complete document/carrier/transmit operations with independent owned fixtures.
#[path = "support/transmit.rs"]
mod transmit_support;
use bessemer::block_stream::{BlockStream, ChunkFraming};
use bessemer::parasolid::*;
use transmit_support::{LIMITS, base, list_mut};

#[test]
fn part_world_edit_preserves_every_other_field_chunk_and_document_stream() {
    let bytes = include_bytes!("fixtures/blank-part.SLDPRT");
    let original = bessemer::Document::from_bytes(bytes).unwrap();
    let mut document = original.clone();
    let stream = "Contents/Config-0-Partition";
    let carrier_bytes = document.read_stream(stream).unwrap();
    let mut carrier =
        BlockStream::from_bytes(&carrier_bytes, ChunkFraming::LengthPrefixed, 1_000_000).unwrap();
    let source = TransmitFile::from_bytes(carrier.chunks()[0].payload(), &base(), LIMITS).unwrap();
    let offset = source
        .layout_fields(101)
        .unwrap()
        .iter()
        .position(|field| field.name == "highest_id")
        .unwrap();
    let mut contents = source.contents().clone();
    list_mut(&mut contents).records[0].fields[offset] = FieldValues::Integers(vec![123]);
    let edited = source.with_contents(contents).unwrap();
    carrier
        .replace_chunk(0, edited.to_bytes().unwrap())
        .unwrap();
    document
        .replace_stream(stream, carrier.to_bytes().unwrap())
        .unwrap();
    let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    let carrier = BlockStream::from_bytes(
        &reopened.read_stream(stream).unwrap(),
        ChunkFraming::LengthPrefixed,
        1_000_000,
    )
    .unwrap();
    assert_eq!(
        TransmitFile::from_bytes(carrier.chunks()[0].payload(), &base(), LIMITS)
            .unwrap()
            .contents(),
        edited.contents()
    );
    let untouched =
        BlockStream::from_bytes(&carrier_bytes, ChunkFraming::LengthPrefixed, 1_000_000).unwrap();
    assert_eq!(carrier.chunks()[1], untouched.chunks()[1]);
    for other in original.streams().filter(|entry| entry.name != stream) {
        assert_eq!(
            original.read_stream(other.name).unwrap(),
            reopened.read_stream(other.name).unwrap()
        );
    }
    assert_eq!(original.to_bytes().unwrap(), bytes);
}

#[test]
fn forced_transmit_rebuilds_preserve_the_complete_native_document() {
    let bytes = include_bytes!("fixtures/blank-part.SLDPRT");
    let mut document = bessemer::Document::from_bytes(bytes).unwrap();
    let stream = "Contents/Config-0-Partition";
    let mut carrier = BlockStream::from_bytes(
        &document.read_stream(stream).unwrap(),
        ChunkFraming::LengthPrefixed,
        1_000_000,
    )
    .unwrap();
    for index in 0..carrier.chunks().len() {
        let source =
            TransmitFile::from_bytes(carrier.chunks()[index].payload(), &base(), LIMITS).unwrap();
        let rebuilt = TransmitFile::new(&base(), source.contents().clone()).unwrap();
        carrier
            .replace_chunk(index, rebuilt.to_bytes().unwrap())
            .unwrap();
    }
    document
        .replace_stream(stream, carrier.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), bytes);
}
