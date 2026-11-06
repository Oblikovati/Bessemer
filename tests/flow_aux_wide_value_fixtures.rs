//! Complete owned record444 bodies retain raw values discarded by the native reader.
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_state_tables/streams.rs"]
mod state_streams;
use packet::*;
use std::collections::BTreeSet;

struct AuxWideCase {
    name: String,
    native_count: Option<i32>,
    stored: Vec<u8>,
    decoded: Vec<u8>,
    version: i32,
    legacy: Option<i32>,
    count: i32,
    end: u32,
    values: Vec<u8>,
}
fn case(cursor: &mut PacketCursor) -> AuxWideCase {
    let result = AuxWideCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native_count: optional(cursor, signed),
        stored: blob(cursor),
        decoded: blob(cursor),
        version: signed(cursor),
        legacy: optional(cursor, signed),
        count: signed(cursor),
        end: word(cursor),
        values: blob(cursor),
    };
    assert!(boolean(cursor));
    result
}
fn cases() -> Vec<AuxWideCase> {
    let mut cursor = cursor(
        include_bytes!("fixtures/flow-aux-wide-values/record444.bin.gz"),
        *b"FAW1",
    );
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
fn rebuilt(row: &AuxWideCase) -> Vec<u8> {
    let mut raw = row.version.to_le_bytes().to_vec();
    if let Some(legacy) = row.legacy {
        raw.extend_from_slice(&legacy.to_le_bytes());
    }
    raw.extend_from_slice(&row.count.to_le_bytes());
    raw.extend_from_slice(&row.values);
    raw
}
#[test]
fn every_owned_whole_zlib_body_rebuilds_exact_version_legacy_count_and_patterns() {
    let rows = cases();
    assert_eq!(rows.len(), 559);
    for row in rows {
        assert!((0..=1).contains(&row.version));
        assert_eq!(row.legacy.is_some(), row.version == 0);
        assert!(row.count >= 0);
        assert_eq!(row.values.len(), row.count as usize * 8);
        assert_eq!(row.end as usize, row.decoded.len());
        assert_eq!(
            row.decoded.len(),
            if row.version == 0 { 12 } else { 8 } + row.values.len()
        );
        assert_eq!(rebuilt(&row), row.decoded, "{}", row.name);
        assert_eq!(
            state_streams::inflate_exact(&row.stored),
            row.decoded,
            "{}",
            row.name
        );
    }
}
#[test]
fn both_versions_retain_all_raw_wide_bits_and_count_boundaries() {
    let rows = cases();
    for version in 0..=1 {
        let subset: Vec<_> = rows.iter().filter(|row| row.version == version).collect();
        let patterns: BTreeSet<_> = subset
            .iter()
            .flat_map(|row| row.values.as_chunks::<8>().0.iter())
            .map(|raw| u64::from_le_bytes(*raw))
            .collect();
        check_wide_patterns(&patterns);
        let counts: BTreeSet<_> = subset.iter().map(|row| row.count).collect();
        assert!(counts.is_superset(&BTreeSet::from([
            0, 1, 2, 255, 256, 257, 4095, 4096, 65535, 65536
        ])));
    }
}
fn check_wide_patterns(patterns: &BTreeSet<u64>) {
    for bit in 0..64 {
        assert!(patterns.contains(&(1u64 << bit)) && patterns.contains(&!(1u64 << bit)));
    }
    for expected in [
        0,
        1 << 63,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000042,
        0x7ff0000000000001,
        u64::MAX,
    ] {
        assert!(patterns.contains(&expected));
    }
}
#[test]
fn discarded_legacy_signed_single_bits_and_complements_are_owned() {
    let values: BTreeSet<_> = cases()
        .iter()
        .filter_map(|row| row.legacy.map(|value| value as u32))
        .collect();
    for bit in 0..32 {
        assert!(values.contains(&(1u32 << bit)) && values.contains(&!(1u32 << bit)));
    }
}
#[test]
fn native_sample_retains_all_values_and_the_discarded_legacy_member() {
    let rows = cases();
    let native: Vec<_> = rows
        .iter()
        .filter(|row| row.native_count.is_some())
        .collect();
    assert_eq!(native.len(), 1);
    let row = native[0];
    assert_eq!(
        (
            row.version,
            row.legacy,
            row.count,
            row.end,
            row.native_count
        ),
        (0, Some(6), 180, 1452, Some(180))
    );
    assert_eq!(row.stored.len(), 32);
    assert_eq!(row.values, vec![0; 180 * 8]);
}
