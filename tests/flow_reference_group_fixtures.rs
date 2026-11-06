//! Independent complete record-416 field/interval baseline before typed readers/editors.
#[path = "flow_reference_groups/api.rs"]
mod api;
#[path = "flow_reference_groups/fixture.rs"]
mod fixture;
#[path = "flow_reference_groups/rebuild.rs"]
mod rebuild;
use fixture::*;
use std::io::Read;

#[test]
fn every_reference_group_field_reconstructs_wire_under_its_explicit_mapping_context() {
    let cases = cases();
    assert_eq!(cases.len(), 1397);
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

fn check_transport(frame: &ReferenceFrame) {
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
fn native_partition_reference_totals_and_empty_second_group_are_frozen_independently() {
    let native = cases().remove(0);
    check_native_header(&native);
    assert!(!native.frame.mapping);
    assert_eq!(native.stored_header, Some(2));
    assert_eq!((native.groups[0].start, native.groups[0].end), (8, 4460));
    assert_eq!((native.groups[1].start, native.groups[1].end), (4460, 4464));
    assert_eq!(
        native.groups.each_ref().map(|group| group.entries.len()),
        [64, 0]
    );
    assert_eq!(
        native_entry_totals(&native.groups[0].entries),
        [128, 424, 176]
    );
    assert!(
        native.groups.iter().all(|group| group.wide.is_empty()
            && group.mapping.is_empty()
            && group.level.is_none())
    );
    assert_eq!(16 + 64 * 24 + 128 * 4 + (424 + 176) * 4, 4464);
}

fn check_native_header(native: &ReferenceCase) {
    assert_eq!(
        (
            native.version,
            native.frame.stored.len(),
            native.frame.decoded.len()
        ),
        (14, 1978, 4464)
    );
}

fn native_entry_totals(entries: &[ReferenceEntry]) -> [usize; 3] {
    entries.iter().fold([0; 3], |mut totals, entry| {
        totals[0] += entry.references.len();
        totals[1] += entry.first.len();
        totals[2] += entry.second.len();
        totals
    })
}
