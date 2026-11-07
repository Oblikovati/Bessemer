//! Literal Cosworks persistence framing, malformed input and ownership checks (#127).
use bessemer::simulation::SimulationPersistText;
type TextGolden<'a> = (&'a [u8], Option<Vec<u16>>);
#[path = "simulation_persistence/corpus.rs"]
mod corpus;
#[path = "simulation_persistence/edits.rs"]
mod edits;
#[path = "simulation_persistence/ownership.rs"]
mod ownership;
#[path = "simulation_persistence/projections.rs"]
mod projections;
#[path = "simulation_persistence/studies.rs"]
mod studies;
#[path = "simulation_persistence/versions.rs"]
mod versions;

#[test]
fn null_empty_odd_and_surrogate_frames_retain_literal_bits() {
    let cases: &[TextGolden<'_>] = &[
        (&[0, 0, 0, 0], None),
        (&[2, 0, 0, 0, 0, 0], Some(vec![])),
        (&[3, 0, 0, 0, 0x81, 0, 0], None),
        (
            &[8, 0, 0, 0, 0, 0xd8, 0, 0, 0xff, 0xdf, 0, 0],
            Some(vec![0xd800, 0, 0xdfff]),
        ),
    ];
    for (bytes, units) in cases {
        let frame = SimulationPersistText::read_from(bytes, 0, 16).unwrap();
        assert_eq!(frame.to_bytes(), *bytes);
        assert_eq!(frame.header(), &bytes[..4]);
        assert_eq!(frame.payload(), &bytes[4..]);
        assert_eq!(frame.declared_bytes() as usize, bytes.len() - 4);
        assert_eq!(frame.is_null(), bytes.len() == 4);
        assert_eq!(&frame.utf16_units(), units);
    }
}

#[test]
fn constructors_preserve_null_and_embedded_nuls_without_cstring_truncation() {
    let null = SimulationPersistText::from_units(None, 0).unwrap();
    assert_eq!(null.to_bytes(), [0, 0, 0, 0]);
    let empty = SimulationPersistText::from_units(Some(&[]), 2).unwrap();
    assert_eq!(empty.to_bytes(), [2, 0, 0, 0, 0, 0]);
    let frame = SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 8).unwrap();
    assert_eq!(frame.to_bytes(), [8, 0, 0, 0, 0, 0xd8, 0, 0, 65, 0, 0, 0]);
    assert!(SimulationPersistText::from_units(Some(&[1]), 3).is_err());
    assert!(SimulationPersistText::from_payload(Some(&[]), 2).is_err());
    assert!(SimulationPersistText::from_payload(Some(&[1, 0]), 2).is_err());
    assert_eq!(
        SimulationPersistText::from_payload(Some(&[0x81, 0, 0]), 3)
            .unwrap()
            .payload(),
        [0x81, 0, 0]
    );
}

#[test]
fn every_truncated_prefix_and_invalid_shape_fails_without_input_mutation() {
    let bytes = [6, 0, 0, 0, 65, 0, 66, 0, 0, 0];
    for end in 0..bytes.len() {
        assert!(SimulationPersistText::read_from(&bytes[..end], 0, 16).is_err());
    }
    assert!(SimulationPersistText::read_from(&bytes, 0, 5).is_err());
    assert!(SimulationPersistText::read_from(&bytes, usize::MAX, 16).is_err());
    assert!(SimulationPersistText::read_from(&[1, 0, 0, 0, 0], 0, 16).is_err());
    assert!(SimulationPersistText::read_from(&[2, 0, 0, 0, 0, 1], 0, 16).is_err());
    assert!(SimulationPersistText::read_from(&[255, 255, 255, 255], 0, 16).is_err());
    assert_eq!(bytes, [6, 0, 0, 0, 65, 0, 66, 0, 0, 0]);
}

#[test]
fn nonzero_offsets_retain_complete_owned_parent_and_isolate_returned_buffers() {
    let mut bytes = vec![0xaa, 0xbb, 2, 0, 0, 0, 0, 0, 0xcc];
    let frame = SimulationPersistText::read_from(&bytes, 2, 2).unwrap();
    bytes.fill(0);
    drop(bytes);
    assert_eq!(frame.source_parent(), [0xaa, 0xbb, 2, 0, 0, 0, 0, 0, 0xcc]);
    assert_eq!(frame.source_offsets(), (2, 6, 8));
    assert_eq!(frame.coverage().source_bytes, 9);
    assert_eq!(frame.coverage().consumed_bytes, 6);
    assert_eq!(frame.coverage().opaque_bytes, 3);
    let mut exported = frame.to_bytes();
    exported.fill(1);
    assert_eq!(frame.to_bytes(), [2, 0, 0, 0, 0, 0]);
}

#[test]
fn owned_frames_are_send_sync_and_concurrent_reads_match_serial() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SimulationPersistText>();
    let frame = SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 8).unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let frame = &frame;
            scope.spawn(move || {
                for _ in 0..32 {
                    assert_eq!(frame.to_bytes(), [8, 0, 0, 0, 0, 0xd8, 0, 0, 65, 0, 0, 0]);
                    assert_eq!(frame.utf16_units(), Some(vec![0xd800, 0, 65]));
                }
            });
        }
    });
}
