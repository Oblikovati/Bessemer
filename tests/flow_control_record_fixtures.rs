//! Strict writer-derived control payloads; synthetic examples do not claim installed native fixtures.
#[path = "flow_control_records/api.rs"]
mod api;
#[path = "flow_control_records/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_control_records/rebuild.rs"]
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
fn every_control_payload_rebuilds_the_complete_body_and_exact_compression_or_footer() {
    let rows = cases();
    assert_eq!(rows.len(), 1071);
    assert_eq!(
        rows.iter().filter(|row| row.identifier == SAVED).count(),
        313
    );
    for row in rows {
        rebuild::check_case(&row);
    }
}
#[test]
fn three_authenticated_writer_examples_are_synthetic_and_preserve_literal_entries() {
    let rows = cases();
    let reference: Vec<_> = rows.iter().filter(|row| row.reference).collect();
    assert_eq!(reference.len(), 3);
    let saved = reference
        .iter()
        .find(|row| row.identifier == SAVED)
        .unwrap();
    assert_eq!(saved.count, 2);
    assert_eq!(saved.stored.len(), 24);
    assert_eq!(words(&saved.entries), [10, 20, 30, 40]);
    for width in [4, 8] {
        check_reference_index(&reference, width);
    }
}
fn words(raw: &[u8]) -> Vec<u64> {
    let (words, remainder) = raw.as_chunks::<8>();
    assert!(remainder.is_empty());
    words.iter().map(|word| u64::from_le_bytes(*word)).collect()
}
fn check_reference_index(rows: &[&ControlCase], width: u8) {
    let row = rows.iter().find(|row| row.width == width).unwrap();
    assert_eq!(row.count, 2);
    assert_eq!(row.stored.len(), 44 + width as usize);
    assert_eq!(row.compressed, 32);
    assert_eq!(row.footer.as_ref().unwrap().start, 40);
    assert_eq!(
        index_entries(&row.entries),
        [(12, 2, 3988), (14, 4010, 3988)]
    );
}
fn index_entries(raw: &[u8]) -> Vec<(u32, u64, u64)> {
    let (entries, remainder) = raw.as_chunks::<20>();
    assert!(remainder.is_empty());
    entries
        .iter()
        .map(|entry| {
            let identifier = u32::from_le_bytes(entry[..4].try_into().unwrap());
            let start = u64::from_le_bytes(entry[4..12].try_into().unwrap());
            let span = u64::from_le_bytes(entry[12..].try_into().unwrap());
            (identifier, start, span)
        })
        .collect()
}
#[test]
fn all_full_width_bits_complements_and_identifier_patterns_are_owned() {
    let mut saved: [BTreeSet<u64>; 2] = std::array::from_fn(|_| BTreeSet::new());
    let mut indexed: [BTreeSet<u64>; 2] = std::array::from_fn(|_| BTreeSet::new());
    let mut identifiers = BTreeSet::new();
    for row in cases() {
        if row.identifier == SAVED {
            collect_saved(&row.entries, &mut saved);
        } else {
            collect_index(&row.entries, &mut identifiers, &mut indexed);
        }
    }
    saved.iter().chain(&indexed).for_each(check_wide_bits);
    assert!(identifiers.contains(&0) && identifiers.contains(&u32::MAX));
    for bit in 0..32 {
        assert!(identifiers.contains(&(1 << bit)) && identifiers.contains(&!(1 << bit)));
    }
}
fn collect_saved(raw: &[u8], values: &mut [BTreeSet<u64>; 2]) {
    for entry in words(raw).as_chunks::<2>().0 {
        values[0].insert(entry[0]);
        values[1].insert(entry[1]);
    }
}
fn collect_index(raw: &[u8], identifiers: &mut BTreeSet<u32>, values: &mut [BTreeSet<u64>; 2]) {
    for (identifier, start, span) in index_entries(raw) {
        identifiers.insert(identifier);
        values[0].insert(start);
        values[1].insert(span);
    }
}
fn check_wide_bits(values: &BTreeSet<u64>) {
    assert!(values.contains(&0) && values.contains(&u64::MAX));
    for bit in 0..64 {
        assert!(values.contains(&(1 << bit)) && values.contains(&!(1 << bit)));
    }
    assert!(values.contains(&0x7ff8000000000042));
}
#[test]
fn empty_large_duplicate_reserved_and_unreconciled_entries_retain_wire_order() {
    let rows = cases();
    let counts: BTreeSet<_> = rows.iter().map(|row| row.count).collect();
    assert!(
        BTreeSet::from([
            0, 1, 2, 3, 7, 8, 15, 16, 31, 32, 63, 64, 255, 256, 257, 65535, 65536
        ])
        .is_subset(&counts)
    );
    let saved = rows
        .iter()
        .find(|row| row.name == "saved-unordered-duplicates")
        .unwrap();
    assert_eq!(words(&saved.entries), [7, 3, 7, 0, 0, u64::MAX, 7, 1]);
    for width in [4, 8] {
        check_duplicate_index(&rows, width);
    }
}

fn check_duplicate_index(rows: &[ControlCase], width: u8) {
    let name = format!("index-{width}-unordered-duplicates");
    let index = rows.iter().find(|row| row.name == name).unwrap();
    assert_eq!(
        index_entries(&index.entries),
        [
            (12, 123, 25),
            (12, 9, 0),
            (0xffffff9d, u64::MAX, 1),
            (12, 0, u64::MAX)
        ]
    );
}
