//! Complete independent wire reconstruction before the typed record-408 reader/editor.
#[path = "flow_block_extras/api.rs"]
mod api;
#[path = "flow_block_extras/fixture.rs"]
mod fixture;
#[path = "flow_block_extras/rebuild.rs"]
mod rebuild;
use fixture::*;
use std::io::Read;

#[test]
fn every_legacy_extra_field_reconstructs_exact_wire_with_explicit_context() {
    let cases = cases();
    assert_eq!(cases.len(), 289);
    for case in cases {
        assert!(case.enabled);
        assert_eq!(case.blocks.len(), case.leaves as usize);
        assert!(!case.tree_order && !case.references && !case.runtime);
        assert_eq!(rebuild::decoded(&case), case.decoded, "{}", case.name);
        check_transport(&case);
    }
}

fn check_transport(case: &ExtraCase) {
    if !case.compressed {
        assert_eq!(case.stored, case.decoded);
        return;
    }
    let mut decoded = Vec::new();
    flate2::read::ZlibDecoder::new(case.stored.as_slice())
        .read_to_end(&mut decoded)
        .unwrap();
    assert_eq!(decoded, case.decoded);
}

#[test]
fn native_counts_pin_every_populated_body_and_all_four_list_totals() {
    let cases = cases();
    let native = &cases[0];
    assert_eq!(
        (native.version, native.leaves, native.decoded.len()),
        (4, 3984, 1210312)
    );
    let bodies: Vec<&ExtraBody> = native
        .blocks
        .iter()
        .filter_map(|b| b.body.as_ref())
        .collect();
    assert_eq!(bodies.len(), 1936);
    assert_eq!(bodies.iter().map(|b| b.first.len()).sum::<usize>(), 4636);
    assert_eq!(bodies.iter().map(|b| b.second.len()).sum::<usize>(), 19880);
    assert_eq!(bodies.iter().map(|b| b.third.len()).sum::<usize>(), 3904);
    assert_eq!(
        bodies.iter().map(|b| b.fourth_bytes.len()).sum::<usize>(),
        25632
    );
}
