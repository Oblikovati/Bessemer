//! Independent complete primary-class record439 fields and intervals.
#[path = "flow_primary_table/api.rs"]
mod api;
#[path = "flow_primary_table/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_field_values/packet_wide.rs"]
mod packet_wide;
#[path = "flow_primary_table/patches.rs"]
mod patches;
#[path = "flow_primary_table/rebuild.rs"]
mod rebuild;
#[path = "flow_primary_table/types.rs"]
mod types;
use packet::*;
use packet_wide::wide;
use types::*;
#[test]
fn every_primary_table_rebuilds_all_optional_bodies_typed_rows_blocks_and_nested_intervals() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 5685);
    for case in cases {
        assert_eq!(rebuild::record(&case), case.raw, "{}", case.name);
    }
}
#[test]
fn native_primary_table_is_a_complete_empty_version_two_payload_without_populated_native_credit() {
    let cases = fixture::cases();
    let native = &cases[0];
    assert_eq!(native.name, "native-cpt");
    assert_eq!(native.version, 2);
    assert!(native.items.is_empty());
    assert_eq!(native.raw, [2, 0, 0, 0, 0, 0, 0, 0]);
    assert!(native.complete);
    assert!(!native.engineering);
}
#[test]
fn all_raw_byte_roles_and_four_ragged_typed_layouts_have_complete_owned_coverage() {
    let cases = fixture::cases();
    let items: Vec<_> = cases.iter().flat_map(|case| &case.items).collect();
    let kinds: std::collections::BTreeSet<_> = items.iter().map(|item| item.kind).collect();
    let element_kinds: std::collections::BTreeSet<_> =
        items.iter().map(|item| item.element_kind).collect();
    assert_eq!((kinds.len(), element_kinds.len()), (256, 256));
    let groups: Vec<_> = items.iter().flat_map(|item| &item.groups).collect();
    let modes: std::collections::BTreeSet<_> = groups.iter().map(|group| group.mode).collect();
    let presence: std::collections::BTreeSet<_> =
        groups.iter().map(|group| group.presence).collect();
    let flags: std::collections::BTreeSet<_> = groups
        .iter()
        .filter_map(|group| group.body.as_ref())
        .map(|body| body.flag)
        .collect();
    assert_eq!(modes.len(), 257);
    assert_eq!(presence.len(), 256);
    assert_eq!(flags.len(), 257);
    assert!(modes.contains(&None) && flags.contains(&None));
    check_typed_layouts(&groups);
}
fn check_typed_layouts(groups: &[&PrimaryGroup]) {
    let elements: Vec<_> = groups
        .iter()
        .filter_map(|group| group.elements.as_ref())
        .flatten()
        .collect();
    let kinds: std::collections::BTreeSet<_> =
        elements.iter().map(|element| element.kind).collect();
    assert_eq!(kinds, [1, 2, 3, 4].into());
    for element in elements {
        let width = [0, 4, 8, 12, 16][element.kind as usize];
        for row in &element.rows {
            assert_eq!(row.end - row.start, 4 + row_count(&row.values) * width);
        }
    }
    assert!(groups.iter().any(|group| typed_without_body(group)));
}
fn typed_without_body(group: &PrimaryGroup) -> bool {
    group.body.is_none()
        && group
            .elements
            .as_ref()
            .is_some_and(|elements| !elements.is_empty())
}

fn row_count(values: &RowValues) -> u32 {
    match values {
        RowValues::Words(values) => values.len() as u32,
        RowValues::Pairs(values) => values.len() as u32,
        RowValues::Wide(values) => values.len() as u32,
        RowValues::Mixed(values) => values.len() as u32,
    }
}
#[test]
fn primary_body_arrays_preserve_every_single_bit_complement_and_special_wide_pattern() {
    let cases = fixture::cases();
    let mut words = std::collections::BTreeSet::new();
    let mut wides = std::collections::BTreeSet::new();
    for body in cases
        .iter()
        .flat_map(|case| &case.items)
        .flat_map(|item| &item.groups)
        .filter_map(|group| group.body.as_ref())
    {
        words.extend(body.first.iter().chain(&body.second).copied());
        wides.extend(body.wide6.iter().chain(&body.wide3).copied());
        if let Some(legacy) = &body.legacy {
            wides.extend(legacy.iter().copied());
        }
    }
    check_patterns(&words, &wides);
}
fn check_patterns(
    words: &std::collections::BTreeSet<u32>,
    wides: &std::collections::BTreeSet<u64>,
) {
    for bit in 0..32 {
        assert!(words.contains(&(1u32 << bit)));
        assert!(words.contains(&!(1u32 << bit)));
    }
    for bit in 0..64 {
        assert!(wides.contains(&(1u64 << bit)));
        assert!(wides.contains(&!(1u64 << bit)));
    }
    for pattern in [
        0,
        0x8000000000000000,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000042,
        0x7ff0000000000001,
    ] {
        assert!(wides.contains(&pattern));
    }
}
