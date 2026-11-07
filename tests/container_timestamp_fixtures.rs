//! Frozen native timestamp arithmetic and independently located container headers.
#[path = "flow_field_values/packet.rs"]
mod packet;
use bessemer::Document;
use packet::*;

const NATIVE: [&[u8]; 3] = [
    include_bytes!("fixtures/blank-part.SLDPRT"),
    include_bytes!("fixtures/blank-assembly.SLDASM"),
    include_bytes!("fixtures/blank-drawing.SLDDRW"),
];
struct TimestampRequest {
    calendar: Option<[u32; 6]>,
    override_word: u32,
    expected: u32,
}
struct NativeTimestamp {
    fixture: usize,
    name: String,
    local: usize,
    central: usize,
    packed: u32,
}
fn calendar(cursor: &mut PacketCursor) -> [u32; 6] {
    std::array::from_fn(|_| word(cursor))
}
fn request(cursor: &mut PacketCursor) -> TimestampRequest {
    TimestampRequest {
        calendar: optional(cursor, calendar),
        override_word: word(cursor),
        expected: signed(cursor) as u32,
    }
}
fn native(cursor: &mut PacketCursor) -> NativeTimestamp {
    NativeTimestamp {
        fixture: usize::from(byte(cursor)),
        name: String::from_utf8(blob(cursor)).unwrap(),
        local: u64::from_le_bytes(number(cursor)) as usize,
        central: u64::from_le_bytes(number(cursor)) as usize,
        packed: word(cursor),
    }
}
fn requests() -> Vec<TimestampRequest> {
    let mut cursor = cursor(
        include_bytes!("fixtures/container-timestamps/requests.bin.gz"),
        *b"ZTR1",
    );
    let rows = counted(&mut cursor, request);
    finish(&cursor);
    rows
}
fn natives() -> Vec<NativeTimestamp> {
    let mut cursor = cursor(
        include_bytes!("fixtures/container-timestamps/native.bin.gz"),
        *b"ZTN1",
    );
    let rows = counted(&mut cursor, native);
    finish(&cursor);
    rows
}
fn arithmetic_oracle(row: &TimestampRequest) -> u32 {
    if row.override_word != 0 {
        return row.override_word;
    }
    let Some([second, minute, hour, day, month, year]) = row.calendar else {
        return 0;
    };
    let origin = if year > 1980 {
        1980
    } else if year > 80 {
        80
    } else {
        0
    };
    let year = u64::from(year - origin);
    let date = (year << 25) + ((u64::from(month) + 1) << 21) + (u64::from(day) << 16);
    let time = (u64::from(hour) << 11) + (u64::from(minute) << 5) + u64::from(second >> 1);
    date as u32 | time as u32
}
#[test]
fn all_22398_explicit_reference_requests_match_independent_weighted_unsigned_arithmetic() {
    let rows = requests();
    assert_eq!(rows.len(), 22398);
    for row in &rows {
        assert_eq!(arithmetic_oracle(row), row.expected);
    }
    for axis in 0..6 {
        let observed: std::collections::BTreeSet<_> = rows
            .iter()
            .filter_map(|row| row.calendar.map(|words| words[axis]))
            .collect();
        for bit in 0..32 {
            assert!(observed.contains(&(1u32 << bit)));
            assert!(observed.contains(&!(1u32 << bit)));
        }
    }
}
#[test]
fn all_113_native_stream_names_and_local_central_timestamp_intervals_match_reference_bytes() {
    let rows = natives();
    assert_eq!(rows.len(), 113);
    for row in rows {
        let raw = NATIVE[row.fixture];
        assert_eq!(
            u32::from_le_bytes(raw[row.local..row.local + 4].try_into().unwrap()),
            row.packed
        );
        assert_eq!(
            u32::from_le_bytes(raw[row.central..row.central + 4].try_into().unwrap()),
            row.packed
        );
        let document = Document::from_bytes(raw).unwrap();
        assert!(document.streams().any(|stream| stream.name == row.name));
        assert_eq!(document.to_bytes().unwrap(), raw);
    }
}
#[test]
fn existing_container_parser_retains_all_timestamp_bits_and_requires_both_stored_copies() {
    let rows = natives();
    for (fixture, source) in NATIVE.iter().enumerate() {
        let row = rows.iter().find(|row| row.fixture == fixture).unwrap();
        for packed in raw_patterns() {
            let mut raw = source.to_vec();
            raw[row.local..row.local + 4].copy_from_slice(&packed.to_le_bytes());
            raw[row.central..row.central + 4].copy_from_slice(&packed.to_le_bytes());
            assert_eq!(Document::from_bytes(&raw).unwrap().to_bytes().unwrap(), raw);
            raw[row.local] ^= 1;
            assert!(Document::from_bytes(&raw).is_err());
        }
    }
}
fn raw_patterns() -> Vec<u32> {
    let mut values = vec![0, u32::MAX, 0xff65ffdf, 0xd8210000, 0xa0210000];
    for bit in 0..32 {
        values.extend([1 << bit, !(1 << bit)]);
    }
    values
}
