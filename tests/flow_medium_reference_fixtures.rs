//! Explicit kind-key context and complete reference-body framing, without native map semantics.
#[path = "flow_medium_references/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_medium_references/rebuild.rs"]
mod rebuild;
#[path = "flow_medium_references/types.rs"]
mod types;
#[test]
fn every_reference_body_reconstructs_all_raw_keys_ordered_duplicates_and_native_state_projection() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 2580);
    for case in cases {
        assert_eq!(rebuild::body(&case), case.raw, "{}", case.name);
    }
}
#[test]
fn identical_wire_keeps_distinct_kind_dependent_state_and_tail_boundaries() {
    let cases = fixture::cases();
    let first = &cases[cases.len() - 2];
    let second = &cases[cases.len() - 1];
    assert_eq!(first.raw, second.raw);
    assert_eq!((first.kind, second.kind), (1, 2));
    assert_eq!(first.state, Some(0));
    assert_eq!(second.state, None);
    assert_eq!(first.end, 23);
    assert_eq!(second.end, 22);
    assert!(first.tail.is_empty());
    assert_eq!(second.tail, [0]);
}
#[test]
fn native_reference_body_is_complete_but_retains_its_exact_following_outer_record() {
    let cases = fixture::cases();
    let native = &cases[0];
    let exact = &cases[1];
    assert_eq!(
        (native.raw.len(), native.end, native.tail.len()),
        (71, 39, 32)
    );
    assert_eq!(native.references.len(), 1);
    assert_eq!(native.keys.len(), 1);
    assert_eq!(native.references[0], native.keys[0]);
    assert_eq!(native.state, Some(1));
    assert_eq!(exact.raw, &native.raw[..39]);
    assert!(exact.tail.is_empty());
}
