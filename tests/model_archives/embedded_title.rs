use super::kernel_frame_tests::context;
use crate::parasolid::TransmitFile;

const UNMARKED: &[u8] = include_bytes!("../fixtures/model-archives/kernel-transmit-unmarked.bin");

#[test]
fn embedded_partition_context_preserves_native_unmarked_titles_and_forced_construction() {
    let context = context();
    let parsed =
        TransmitFile::from_embedded_partition(UNMARKED, &context.base, context.transmit_limits)
            .unwrap();
    assert_eq!(parsed.to_bytes().unwrap(), UNMARKED);
    let forced =
        TransmitFile::new_embedded_partition(&context.base, parsed.contents().clone()).unwrap();
    assert_eq!(forced.to_bytes().unwrap(), UNMARKED);
    assert_eq!(
        parsed
            .with_contents(parsed.contents().clone())
            .unwrap()
            .to_bytes()
            .unwrap(),
        UNMARKED
    );
}

#[test]
fn standalone_marked_context_still_rejects_unmarked_decode_and_construction() {
    let context = context();
    assert!(TransmitFile::from_bytes(UNMARKED, &context.base, context.transmit_limits).is_err());
    let parsed =
        TransmitFile::from_embedded_partition(UNMARKED, &context.base, context.transmit_limits)
            .unwrap();
    assert!(TransmitFile::new(&context.base, parsed.contents().clone()).is_err());
}

#[test]
fn embedded_context_survives_edits_and_rejects_a_contradictory_delta_title() {
    let context = context();
    let parsed =
        TransmitFile::from_embedded_partition(UNMARKED, &context.base, context.transmit_limits)
            .unwrap();
    let mut contents = parsed.contents().clone();
    contents.title = ": TRANSMIT FILE changed title".into();
    let changed = parsed.with_contents(contents.clone()).unwrap();
    let reopened = TransmitFile::from_embedded_partition(
        &changed.to_bytes().unwrap(),
        &context.base,
        context.transmit_limits,
    )
    .unwrap();
    assert_eq!(reopened.contents(), &contents);
    contents.title = ": TRANSMIT FILE (deltas)".into();
    assert!(parsed.with_contents(contents).is_err());
    assert_eq!(parsed.to_bytes().unwrap(), UNMARKED);
}
