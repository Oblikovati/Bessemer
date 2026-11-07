//! Complete reference IMS-definition frames retain every array and physical object ordinal.
#[path = "flow_result_definitions/api.rs"]
mod api;
#[path = "flow_result_definitions/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_result_definitions/rebuild.rs"]
mod rebuild;
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
#[path = "flow_state_tables/process.rs"]
mod state_process;
#[path = "flow_state_tables/streams.rs"]
mod streams;
use fixture::*;
use std::collections::BTreeSet;

#[test]
fn all_frozen_definition_frames_rebuild_every_decoded_byte_and_exact_compression() {
    for row in cases() {
        assert_eq!(
            streams::inflate_exact(&row.stored),
            row.decoded,
            "{}",
            row.name
        );
        assert_eq!(rebuild::reconstruct(&row), row.decoded, "{}", row.name);
        assert_eq!(row.version, 1);
        assert_eq!(row.end as usize, row.decoded.len());
        assert!(row.complete && !row.runtime);
    }
}

#[test]
fn native_definition_keeps_full_7932_patterns_and_complete_five_collection_framing() {
    let row = cases().into_iter().find(|row| row.native).unwrap();
    assert_eq!(row.stored.len(), 6767);
    assert_eq!(row.decoded.len(), 31798);
    assert_eq!(row.groups.len(), 5);
    assert_eq!(row.groups[0].objects.len(), 1);
    assert!(row.groups[1..].iter().all(|group| group.objects.is_empty()));
    let object = &row.groups[0].objects[0];
    assert_eq!(object.identifier, 0);
    assert_eq!(object.body.version, 2);
    assert_eq!(object.body.allocation, Some(0));
    assert_eq!(object.body.arrays[0].as_ref().unwrap().count, 3876);
    assert_eq!(object.body.arrays[1].as_ref().unwrap().count, 4056);
    assert!(object.body.arrays[2..].iter().all(Option::is_none));
}

#[test]
fn every_unsigned_bit_complement_allocation_byte_and_definition_version_is_owned() {
    let mut identifiers = BTreeSet::new();
    let mut patterns: [BTreeSet<u32>; 4] = std::array::from_fn(|_| BTreeSet::new());
    let mut flags: [BTreeSet<u8>; 2] = std::array::from_fn(|_| BTreeSet::new());
    let mut versions = BTreeSet::new();
    for row in cases() {
        for object in &row.groups[0].objects {
            identifiers.insert(object.identifier as u32);
            versions.insert(object.body.version);
            collect_patterns(&object.body, &mut patterns, &mut flags);
        }
    }
    assert_eq!(versions, BTreeSet::from([0, 1, 2, 3]));
    check_bits(&identifiers);
    patterns.iter().for_each(check_bits);
    assert_eq!(flags, std::array::from_fn(|_| (0..=255).collect()));
}

fn collect_patterns(
    body: &ImsBody,
    patterns: &mut [BTreeSet<u32>; 4],
    flags: &mut [BTreeSet<u8>; 2],
) {
    if let Some(value) = body.allocation {
        flags[(body.version - 2) as usize].insert(value);
    }
    for (ordinal, array) in body.arrays.iter().enumerate() {
        if let Some(array) = array {
            patterns[ordinal].extend(
                array
                    .raw
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|word| u32::from_le_bytes(*word)),
            );
        }
    }
}

fn check_bits(values: &BTreeSet<u32>) {
    assert!(values.contains(&0) && values.contains(&u32::MAX));
    for bit in 0..32 {
        assert!(values.contains(&(1 << bit)));
        assert!(values.contains(&!(1 << bit)));
    }
}

#[test]
fn object_counts_duplicate_identifiers_and_array_length_boundaries_preserve_physical_order() {
    let rows = cases();
    let counts: BTreeSet<_> = rows.iter().map(|row| row.groups[0].objects.len()).collect();
    assert!(BTreeSet::from([0, 1, 2, 3, 16, 64, 257]).is_subset(&counts));
    let row = rows.iter().find(|row| row.name == "objects-257").unwrap();
    for (ordinal, object) in row.groups[0].objects.iter().enumerate() {
        assert_eq!(object.identifier, if ordinal % 2 == 0 { 7 } else { -1 });
        assert_eq!(object.body.version, ordinal as i32 % 4);
    }
    for ordinal in 0..4 {
        check_lengths(&rows, ordinal);
    }
}

fn check_lengths(rows: &[DefinitionCase], ordinal: usize) {
    let lengths: BTreeSet<_> = rows
        .iter()
        .flat_map(|row| &row.groups[0].objects)
        .filter_map(|object| {
            object.body.arrays[ordinal]
                .as_ref()
                .map(|array| array.count)
        })
        .collect();
    assert!(
        BTreeSet::from([
            0, 1, 2, 3, 7, 8, 15, 16, 31, 32, 63, 64, 255, 256, 257, 65535, 65536
        ])
        .is_subset(&lengths)
    );
}
