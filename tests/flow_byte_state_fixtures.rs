//! Complete owned record-316 whole-zlib bodies before typed byte-state API work.
#[path = "flow_field_values/packet.rs"]
mod packet;
use packet::*;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;

struct ByteStateCase {
    name: String,
    native_count: Option<i32>,
    stored: Vec<u8>,
    decoded: Vec<u8>,
    header: i32,
    count: i32,
    end: u32,
    values: Vec<u8>,
    dispatch: bool,
}
fn cases() -> Vec<ByteStateCase> {
    let mut cursor = cursor(
        include_bytes!("fixtures/flow-byte-states/record316.bin.gz"),
        *b"FBS1",
    );
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
fn case(cursor: &mut PacketCursor) -> ByteStateCase {
    ByteStateCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native_count: optional(cursor, signed),
        stored: blob(cursor),
        decoded: blob(cursor),
        header: signed(cursor),
        count: signed(cursor),
        end: word(cursor),
        values: blob(cursor),
        dispatch: boolean(cursor),
    }
}
fn inflate_exact(stored: &[u8]) -> Vec<u8> {
    let mut decoder = flate2::read::ZlibDecoder::new(stored);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    assert_eq!(decoder.total_in() as usize, stored.len());
    decoded
}
#[test]
fn all_owned_whole_zlib_payloads_rebuild_complete_source_headers_counts_values_and_eof() {
    let rows = cases();
    assert_eq!(rows.len(), 965);
    for row in rows {
        assert!(row.count >= 0 && !row.dispatch, "{}", row.name);
        assert_eq!(row.count as usize, row.values.len());
        assert_eq!(row.end as usize, row.decoded.len());
        let mut raw = row.header.to_le_bytes().to_vec();
        raw.extend_from_slice(&row.count.to_le_bytes());
        raw.extend_from_slice(&row.values);
        assert_eq!(raw, row.decoded, "{}", row.name);
        assert_eq!(inflate_exact(&row.stored), raw, "{}", row.name);
    }
}
#[test]
fn header_single_bits_complements_and_every_raw_byte_have_owned_coverage() {
    let rows = cases();
    let headers: BTreeSet<_> = rows.iter().map(|row| row.header as u32).collect();
    for bit in 0..32 {
        assert!(headers.contains(&(1u32 << bit)) && headers.contains(&!(1u32 << bit)));
    }
    for header in [0, 1, 0x7fffffff, 0x80000000, 0xffffffff] {
        assert!(headers.contains(&header));
    }
    let bytes: BTreeSet<_> = rows
        .iter()
        .flat_map(|row| row.values.iter().copied())
        .collect();
    assert_eq!(bytes, (0..=u8::MAX).collect());
    let lengths: BTreeSet<_> = rows.iter().map(|row| row.values.len()).collect();
    for length in [0, 1, 2, 3, 255, 256, 257, 4095, 4096, 65535, 65536] {
        assert!(lengths.contains(&length));
    }
}
#[test]
fn every_frozen_compression_level_strategy_and_window_preserves_its_original_body() {
    let rows = cases();
    for level in 0..10 {
        check_compression(&rows, &format!("level-{level}"));
    }
    for strategy in 0..5 {
        for window in [9, 15] {
            check_compression(&rows, &format!("strategy-{strategy}-window-{window}"));
        }
    }
}
fn check_compression(rows: &[ByteStateCase], name: &str) {
    let row = rows.iter().find(|row| row.name == name).unwrap();
    assert_eq!(row.header, i32::MIN);
    assert_eq!(row.count, 1024);
    assert_eq!(
        row.values,
        (0..1024)
            .map(|index| (index & 255) as u8)
            .collect::<Vec<_>>()
    );
    assert_eq!(inflate_exact(&row.stored), row.decoded);
}
#[test]
fn native_fld_keeps_its_complete_stream_and_independent_four_value_census() {
    let rows = cases();
    let native: Vec<_> = rows
        .iter()
        .filter(|row| row.native_count.is_some())
        .collect();
    assert_eq!(native.len(), 1);
    let row = native[0];
    assert_eq!((row.header, row.count), (0, 5532));
    assert_eq!(row.native_count, Some(5532));
    assert_eq!(row.end, 5540);
    assert_eq!(row.stored.len(), 265);
    let mut frequencies = BTreeMap::<u8, usize>::new();
    for value in &row.values {
        *frequencies.entry(*value).or_default() += 1;
    }
    assert_eq!(
        frequencies,
        BTreeMap::from([(3, 90), (4, 90), (236, 1476), (253, 3876)])
    );
    assert_eq!(inflate_exact(&row.stored), row.decoded);
}
