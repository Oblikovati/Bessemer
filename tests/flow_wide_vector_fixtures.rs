//! Complete raw record148 projections preserve the full length table and empty vectors.
#[path = "flow_wide_vectors/api.rs"]
mod api;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
#[path = "flow_state_tables/process.rs"]
mod state_process;
use packet::*;
use std::collections::BTreeSet;

struct VectorProjection {
    length: i32,
    start: u32,
    end: u32,
    raw: Vec<u8>,
}
struct VectorCase {
    name: String,
    native_count: Option<i32>,
    raw: Vec<u8>,
    version: i32,
    count: i32,
    end: u32,
    arrays: Vec<VectorProjection>,
}
fn vector(cursor: &mut PacketCursor) -> VectorProjection {
    VectorProjection {
        length: signed(cursor),
        start: word(cursor),
        end: word(cursor),
        raw: blob(cursor),
    }
}
fn case(cursor: &mut PacketCursor) -> VectorCase {
    VectorCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native_count: optional(cursor, signed),
        raw: blob(cursor),
        version: signed(cursor),
        count: signed(cursor),
        end: word(cursor),
        arrays: counted(cursor, vector),
    }
}
fn cases() -> Vec<VectorCase> {
    let mut cursor = cursor(
        include_bytes!("fixtures/flow-wide-vectors/record148.bin.gz"),
        *b"FWV1",
    );
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
fn rebuilt(row: &VectorCase) -> Vec<u8> {
    let mut raw = row.version.to_le_bytes().to_vec();
    raw.extend_from_slice(&row.count.to_le_bytes());
    for array in &row.arrays {
        raw.extend_from_slice(&array.length.to_le_bytes());
    }
    for array in &row.arrays {
        raw.extend_from_slice(&array.raw);
    }
    raw
}
fn check_intervals(row: &VectorCase) {
    let mut start = 8 + row.arrays.len() * 4;
    for array in &row.arrays {
        assert!(array.length >= 0);
        assert_eq!(array.raw.len(), array.length as usize * 8);
        assert_eq!(array.start as usize, start);
        start += array.raw.len();
        assert_eq!(array.end as usize, start);
    }
    assert_eq!(start, row.raw.len());
}
#[test]
fn all_owned_vectors_rebuild_lengths_before_bodies_and_exact_physical_intervals() {
    let rows = cases();
    assert_eq!(rows.len(), 394);
    for row in rows {
        assert_eq!(row.version, 0);
        assert_eq!(row.count as usize, row.arrays.len());
        assert_eq!(row.end as usize, row.raw.len());
        check_intervals(&row);
        assert_eq!(rebuilt(&row), row.raw, "{}", row.name);
    }
}
#[test]
fn every_raw_wide_bit_and_boundary_vector_count_and_length_is_owned() {
    let rows = cases();
    let counts: BTreeSet<_> = rows.iter().map(|row| row.count).collect();
    let arrays: Vec<_> = rows.iter().flat_map(|row| &row.arrays).collect();
    let lengths: BTreeSet<_> = arrays.iter().map(|array| array.length).collect();
    let patterns: BTreeSet<_> = arrays
        .iter()
        .flat_map(|array| array.raw.as_chunks::<8>().0.iter())
        .map(|raw| u64::from_le_bytes(*raw))
        .collect();
    check_patterns(&patterns);
    assert!(counts.is_superset(&BTreeSet::from([0, 1, 2, 255, 256, 257, 1024, 4096])));
    assert!(lengths.is_superset(&BTreeSet::from([
        0, 1, 2, 255, 256, 257, 4095, 4096, 65535, 65536
    ])));
}
fn check_patterns(patterns: &BTreeSet<u64>) {
    for bit in 0..64 {
        assert!(patterns.contains(&(1u64 << bit)) && patterns.contains(&!(1u64 << bit)));
    }
    for value in [
        0,
        u64::MAX,
        1 << 63,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000042,
        0x7ff0000000000001,
    ] {
        assert!(patterns.contains(&value));
    }
}
#[test]
fn no_vectors_and_present_empty_vectors_remain_distinct() {
    let rows = cases();
    assert!(rows.iter().any(|row| row.arrays.is_empty()));
    for count in [1, 2, 257, 4096] {
        let row = rows
            .iter()
            .find(|row| row.count == count && row.arrays.iter().all(|array| array.length == 0))
            .unwrap();
        assert_eq!(row.raw.len(), 8 + count as usize * 4);
        assert_eq!(row.arrays.len(), count as usize);
        assert!(row.arrays.iter().all(|array| array.start == array.end));
    }
}
#[test]
fn native_sample_retains_its_complete_single_raw_pattern_and_provenance() {
    let rows = cases();
    let native: Vec<_> = rows
        .iter()
        .filter(|row| row.native_count.is_some())
        .collect();
    assert_eq!(native.len(), 1);
    let row = native[0];
    assert_eq!(
        (row.version, row.count, row.end, row.native_count),
        (0, 1, 20, Some(1))
    );
    assert_eq!(
        (row.arrays[0].start, row.arrays[0].end, row.arrays[0].length),
        (12, 20, 1)
    );
    assert_eq!(row.arrays[0].raw, 4698062975489146916u64.to_le_bytes());
}
