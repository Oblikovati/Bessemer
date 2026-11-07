//! Viewer-query prefixes retain opaque decoded suffixes and never claim full payload framing.
#[path = "flow_viewer_prefix/api.rs"]
mod api;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
#[path = "flow_state_tables/process.rs"]
mod state_process;
#[path = "flow_state_tables/streams.rs"]
mod state_streams;
use packet::*;
use std::collections::BTreeSet;
struct ViewerArray {
    start: u32,
    end: u32,
    count: i32,
    raw: Vec<u8>,
}
struct ViewerFrame {
    native_version: Option<i32>,
    stored: Vec<u8>,
    decoded: Vec<u8>,
    version: i32,
    initial: i32,
    second: i32,
    output: i32,
}
struct ViewerExtent {
    prefix: u32,
    known: u32,
    unknown: u32,
    stored_bytes: u32,
    decoded_bytes: u32,
    outer_bytes: u32,
}
struct ViewerCase {
    name: String,
    frame: ViewerFrame,
    arrays: Vec<ViewerArray>,
    triplet: Vec<u8>,
    final_word: i32,
    extent: ViewerExtent,
    metadata: [i32; 2],
    tail: Vec<u8>,
    profile: String,
}
fn array(cursor: &mut PacketCursor) -> ViewerArray {
    ViewerArray {
        start: word(cursor),
        end: word(cursor),
        count: signed(cursor),
        raw: blob(cursor),
    }
}
fn frame(cursor: &mut PacketCursor) -> ViewerFrame {
    ViewerFrame {
        native_version: optional(cursor, signed),
        stored: blob(cursor),
        decoded: blob(cursor),
        version: signed(cursor),
        initial: signed(cursor),
        second: signed(cursor),
        output: signed(cursor),
    }
}
fn extent(cursor: &mut PacketCursor) -> ViewerExtent {
    ViewerExtent {
        prefix: word(cursor),
        known: word(cursor),
        unknown: word(cursor),
        stored_bytes: word(cursor),
        decoded_bytes: word(cursor),
        outer_bytes: word(cursor),
    }
}
fn profile(cursor: &mut PacketCursor) -> String {
    assert!(!boolean(cursor) && !boolean(cursor) && !boolean(cursor));
    String::from_utf8(blob(cursor)).unwrap()
}
fn case(cursor: &mut PacketCursor) -> ViewerCase {
    ViewerCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        frame: frame(cursor),
        arrays: counted(cursor, array),
        triplet: blob(cursor),
        final_word: signed(cursor),
        extent: extent(cursor),
        metadata: [signed(cursor), signed(cursor)],
        tail: blob(cursor),
        profile: profile(cursor),
    }
}
fn cases() -> Vec<ViewerCase> {
    let mut cursor = cursor(
        include_bytes!("fixtures/flow-viewer-prefix/record121.bin.gz"),
        *b"FVP1",
    );
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
fn rebuilt(row: &ViewerCase) -> Vec<u8> {
    let mut raw = row.frame.initial.to_le_bytes().to_vec();
    raw.extend_from_slice(&row.frame.second.to_le_bytes());
    for array in &row.arrays {
        raw.extend_from_slice(&array.count.to_le_bytes());
    }
    let mut start = 20;
    for array in &row.arrays {
        assert!(array.count > 0);
        assert_eq!(array.raw.len(), array.count as usize * 8);
        assert_eq!(array.start as usize, start);
        start += array.raw.len();
        assert_eq!(array.end as usize, start);
        raw.extend_from_slice(&array.raw);
    }
    assert_eq!(row.triplet.len(), 24);
    raw.extend_from_slice(&row.triplet);
    raw.extend_from_slice(&row.final_word.to_le_bytes());
    assert_eq!(raw.len(), row.extent.prefix as usize);
    raw.extend_from_slice(&row.tail);
    raw
}
#[test]
fn every_owned_prefix_rebuilds_raw_arrays_scalars_metadata_and_unknown_suffix() {
    let rows = cases();
    assert_eq!(rows.len(), 525);
    for row in rows {
        check_transport(&row);
        check_extent(&row);
        check_metadata(&row);
        assert_eq!(rebuilt(&row), row.frame.decoded, "{}", row.name);
    }
}
fn check_transport(row: &ViewerCase) {
    let frame = &row.frame;
    let extent = &row.extent;
    assert!(frame.version >= 2);
    assert_eq!(row.arrays.len(), 3);
    assert_eq!(extent.outer_bytes, 4);
    assert_eq!(row.profile, "viewer_inflate_metadata_query");
    assert_eq!(&frame.stored[..4], &frame.version.to_le_bytes());
    assert_eq!(frame.stored.len(), extent.stored_bytes as usize);
    assert_eq!(
        state_streams::inflate_exact(&frame.stored[4..]),
        frame.decoded
    );
}
fn check_extent(row: &ViewerCase) {
    let extent = &row.extent;
    assert_eq!(row.frame.decoded.len(), extent.decoded_bytes as usize);
    assert_eq!(extent.known, extent.prefix);
    assert_eq!(extent.unknown as usize, row.tail.len());
    assert_eq!(
        row.frame.decoded.len(),
        extent.prefix as usize + row.tail.len()
    );
}
fn check_metadata(row: &ViewerCase) {
    let expected = if row.frame.second == 1 {
        2
    } else {
        row.frame.second
    };
    assert_eq!(row.frame.output, expected);
    assert_eq!(row.metadata, [row.final_word, row.frame.output]);
}
#[test]
fn every_scalar_bit_and_wide_pattern_and_positive_length_boundary_is_owned() {
    let rows = cases();
    check_scalar_bits(&rows);
    let arrays: Vec<_> = rows.iter().flat_map(|row| &row.arrays).collect();
    let patterns: BTreeSet<_> = arrays
        .iter()
        .flat_map(|array| array.raw.as_chunks::<8>().0.iter())
        .map(|raw| u64::from_le_bytes(*raw))
        .collect();
    let discarded: BTreeSet<_> = rows
        .iter()
        .flat_map(|row| row.triplet.as_chunks::<8>().0.iter())
        .map(|raw| u64::from_le_bytes(*raw))
        .collect();
    check_wide_bits(&patterns, &discarded);
    let lengths: BTreeSet<_> = arrays.iter().map(|array| array.count).collect();
    assert!(lengths.is_superset(&BTreeSet::from([
        1, 2, 255, 256, 257, 4095, 4096, 65535, 65536
    ])));
}
fn check_wide_bits(patterns: &BTreeSet<u64>, discarded: &BTreeSet<u64>) {
    for bit in 0..64 {
        assert!(patterns.contains(&(1u64 << bit)) && patterns.contains(&!(1u64 << bit)));
        assert!(discarded.contains(&(1u64 << bit)) && discarded.contains(&!(1u64 << bit)));
    }
}
fn check_scalar_bits(rows: &[ViewerCase]) {
    let selectors: [fn(&ViewerCase) -> i32; 3] = [
        |row| row.frame.initial,
        |row| row.frame.second,
        |row| row.final_word,
    ];
    for selector in selectors {
        let values: BTreeSet<_> = rows.iter().map(|row| selector(row) as u32).collect();
        for bit in 0..32 {
            assert!(values.contains(&(1u32 << bit)) && values.contains(&!(1u32 << bit)));
        }
    }
}
#[test]
fn opaque_tail_boundaries_include_no_tail_and_nearly_complete_native_suffix_without_full_decode_claim()
 {
    let rows = cases();
    let tails: BTreeSet<_> = rows.iter().map(|row| row.extent.unknown).collect();
    assert!(tails.is_superset(&BTreeSet::from([
        0, 1, 2, 3, 7, 8, 31, 32, 255, 256, 4096, 65535, 141226, 141227
    ])));
    for row in rows {
        assert_eq!(&row.frame.decoded[row.extent.prefix as usize..], row.tail);
    }
}
#[test]
fn native_prefix_retains_616_known_bytes_and_all_141227_opaque_decoded_bytes() {
    let rows = cases();
    let native: Vec<_> = rows
        .iter()
        .filter(|row| row.frame.native_version.is_some())
        .collect();
    assert_eq!(native.len(), 1);
    let row = native[0];
    let extents = (
        row.extent.prefix,
        row.extent.unknown,
        row.extent.decoded_bytes,
    );
    assert_eq!(extents, (616, 141227, 141843));
    assert_eq!((row.frame.version, row.frame.native_version), (3, Some(3)));
    let lengths: Vec<_> = row.arrays.iter().map(|array| array.count).collect();
    assert_eq!(lengths, [45, 13, 13]);
    assert_eq!(row.metadata, [1, 2]);
    assert_eq!(row.extent.stored_bytes, 33601);
}
