//! Owned complete record-12 raw-byte baseline, independent of runtime object indexing.
#[path = "flow_counted_bytes/api.rs"]
mod api;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
#[path = "flow_state_tables/process.rs"]
mod state_process;
use packet::*;
use std::collections::BTreeSet;

struct CountedCase {
    name: String,
    native_count: Option<i32>,
    raw: Vec<u8>,
    count: i32,
    end: u32,
    values: Vec<u8>,
    complete: bool,
    engineering: bool,
    runtime_mapping: bool,
}
fn cases() -> Vec<CountedCase> {
    let mut cursor = cursor(
        include_bytes!("fixtures/flow-counted-bytes/record12.bin.gz"),
        *b"FC12",
    );
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
fn case(cursor: &mut PacketCursor) -> CountedCase {
    CountedCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native_count: optional(cursor, signed),
        raw: blob(cursor),
        count: signed(cursor),
        end: word(cursor),
        values: blob(cursor),
        complete: boolean(cursor),
        engineering: boolean(cursor),
        runtime_mapping: boolean(cursor),
    }
}
#[test]
fn every_owned_counted_payload_rebuilds_the_complete_original_schema_result() {
    let rows = cases();
    assert_eq!(rows.len(), 681);
    for row in rows {
        assert!(row.count >= 0, "{}", row.name);
        assert_eq!(row.count as usize, row.values.len(), "{}", row.name);
        assert_eq!(row.end as usize, 4 + row.values.len());
        let mut expected = row.count.to_le_bytes().to_vec();
        expected.extend_from_slice(&row.values);
        assert_eq!(row.raw, expected, "{}", row.name);
        assert!(row.complete && !row.engineering && !row.runtime_mapping);
    }
}
#[test]
fn every_byte_bit_complement_and_boundary_length_has_owned_framing() {
    let rows = cases();
    let values: BTreeSet<_> = rows
        .iter()
        .flat_map(|row| row.values.iter().copied())
        .collect();
    assert_eq!(values, (0..=u8::MAX).collect());
    let singletons: BTreeSet<_> = rows
        .iter()
        .filter(|row| row.values.len() == 1)
        .map(|row| row.values[0])
        .collect();
    assert_eq!(singletons, values);
    let lengths: BTreeSet<_> = rows.iter().map(|row| row.values.len()).collect();
    for length in [0, 1, 2, 3, 255, 256, 257, 4095, 4096, 65535, 65536] {
        assert!(lengths.contains(&length));
    }
    for bit in 0..8 {
        assert!(values.contains(&(1 << bit)) && values.contains(&!(1 << bit)));
    }
}
#[test]
fn audited_native_cfld_has_exact_count_and_all_zero_raw_bytes_without_boolean_projection() {
    let rows = cases();
    let native: Vec<_> = rows
        .iter()
        .filter(|row| row.native_count.is_some())
        .collect();
    assert_eq!(native.len(), 1);
    let row = native[0];
    assert_eq!(row.name, "native-cfld");
    assert_eq!(
        (row.native_count, row.count, row.end, row.raw.len()),
        (Some(3984), 3984, 3988, 3988)
    );
    assert_eq!(row.values, vec![0; 3984]);
    assert_eq!(row.raw[..4], 3984i32.to_le_bytes());
    assert!(!row.engineering && !row.runtime_mapping);
}
