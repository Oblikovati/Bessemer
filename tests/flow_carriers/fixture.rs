use bessemer::flow::FlowLimits;
use std::io::Read;

pub struct CarrierCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub records: Vec<(u32, usize, usize)>,
}

pub fn limits() -> FlowLimits {
    FlowLimits {
        maximum_wire_bytes: 4_000_000,
        maximum_records: 100,
        maximum_payload_bytes: 4_000_000,
    }
}

pub fn cases() -> Vec<CarrierCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/flow-carriers/carriers.bin.gz")[..])
        .take(4_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = raw.as_slice();
    assert_eq!(take(&mut cursor, 4), b"FLC1");
    let count = word(&mut cursor);
    let result = (0..count).map(|_| case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    result
}

fn case(cursor: &mut &[u8]) -> CarrierCase {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let raw = blob(cursor);
    let count = word(cursor);
    let records = (0..count)
        .map(|_| (word(cursor), wide(cursor), wide(cursor)))
        .collect();
    CarrierCase { name, raw, records }
}

fn take<'a>(cursor: &mut &'a [u8], length: usize) -> &'a [u8] {
    let (head, tail) = cursor.split_at(length);
    *cursor = tail;
    head
}

fn blob(cursor: &mut &[u8]) -> Vec<u8> {
    let length = wide(cursor);
    take(cursor, length).to_vec()
}

fn word(cursor: &mut &[u8]) -> u32 {
    u32::from_le_bytes(take(cursor, 4).try_into().unwrap())
}

fn wide(cursor: &mut &[u8]) -> usize {
    usize::try_from(u64::from_le_bytes(take(cursor, 8).try_into().unwrap())).unwrap()
}
