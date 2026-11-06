//! Independent complete pool/body wire reconstruction before the typed record-421 API.
#[path = "flow_pointer_bodies/fixture.rs"]
mod fixture;
#[path = "flow_pointer_bodies/rebuild.rs"]
mod rebuild;
use fixture::*;
use std::io::Read;

#[test]
fn every_pointer_body_field_reconstructs_wire_with_explicit_context_and_exclusions() {
    let cases = cases();
    assert_eq!(cases.len(), 2109);
    for case in cases {
        assert_eq!(
            rebuild::decoded(&case),
            case.frame.decoded,
            "{}",
            case.frame.name
        );
        assert_eq!(case.exclusions, [false; 4]);
        check_transport(&case.frame);
    }
}

fn check_transport(frame: &PointerFrame) {
    if !frame.compressed {
        assert_eq!(frame.stored, frame.decoded);
        return;
    }
    let mut decoded = Vec::new();
    flate2::read::ZlibDecoder::new(frame.stored.as_slice())
        .read_to_end(&mut decoded)
        .unwrap();
    assert_eq!(decoded, frame.decoded);
}

#[test]
fn native_pool_totals_and_group_extents_are_frozen_independently() {
    let cases = cases();
    let native = &cases[0];
    assert_eq!(
        (native.frame.leaf, native.frame.decoded.len()),
        (3984, 352136)
    );
    assert_eq!(native.frame.counts, [9338, 6542, 5532]);
    assert_eq!(
        native.pools.iter().map(|pool| pool.count).sum::<u32>(),
        10144
    );
    let extents: Vec<(u32, u32)> = native
        .groups
        .iter()
        .map(|group| (group.start, group.end))
        .collect();
    assert_eq!(
        extents,
        [(247444, 285750), (285750, 319616), (319616, 352136)]
    );
}
