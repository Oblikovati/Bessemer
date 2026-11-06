use bessemer::flow::*;
use std::io::{Cursor, Read};

pub struct ArrayCase {
    pub name: String,
    pub raw: Vec<u8>,
    pub ordinal: usize,
    pub selector: i32,
    pub first: Option<Vec<i32>>,
    pub second: Vec<i32>,
    pub decoded: Vec<u8>,
}

pub fn cases() -> Vec<ArrayCase> {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("../fixtures/flow-arrays/arrays.bin.gz")[..])
        .take(1_000_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FMA1");
    let count = u32::from_le_bytes(number(&mut cursor));
    let result = (0..count).map(|_| case(&mut cursor)).collect();
    assert_eq!(cursor.position(), raw.len() as u64);
    result
}

fn case(cursor: &mut Cursor<&[u8]>) -> ArrayCase {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let raw = blob(cursor);
    let ordinal = u32::from_le_bytes(number(cursor)) as usize;
    let selector = i32::from_le_bytes(number(cursor));
    let first = if number::<1>(cursor)[0] == 1 {
        Some(array(cursor))
    } else {
        None
    };
    ArrayCase {
        name,
        raw,
        ordinal,
        selector,
        first,
        second: array(cursor),
        decoded: blob(cursor),
    }
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut bytes = [0; N];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}

fn blob(cursor: &mut Cursor<&[u8]>) -> Vec<u8> {
    let count = usize::try_from(u64::from_le_bytes(number(cursor))).unwrap();
    let mut bytes = vec![0; count];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}

fn array(cursor: &mut Cursor<&[u8]>) -> Vec<i32> {
    let count = u32::from_le_bytes(number(cursor));
    (0..count)
        .map(|_| i32::from_le_bytes(number(cursor)))
        .collect()
}

pub fn options() -> FlowArrayOptions {
    FlowArrayOptions {
        maximum_decoded_bytes: 100_000,
        maximum_elements: 10_000,
        expected_count: None,
    }
}

pub fn carrier(bytes: &[u8]) -> FlowCarrier {
    FlowCarrier::from_bytes(
        bytes,
        FlowLimits {
            maximum_wire_bytes: 4_000_000,
            maximum_records: 100,
            maximum_payload_bytes: 4_000_000,
        },
    )
    .unwrap()
}

pub fn view(case: &ArrayCase) -> FlowMapArrays {
    let source = carrier(&case.raw);
    let id = source.records().nth(case.ordinal).unwrap().id;
    source.record_433_arrays(&id, options()).unwrap()
}

pub fn encoded(decoded: &[u8]) -> Vec<u8> {
    use std::io::Write;
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
    encoder.write_all(decoded).unwrap();
    encoder.finish().unwrap()
}

pub fn payload_carrier(identifier: u32, payload: &[u8]) -> FlowCarrier {
    let mut wire = 8_u16.to_le_bytes().to_vec();
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
    carrier(&wire)
}

pub fn decode_payload(
    payload: &[u8],
    options: FlowArrayOptions,
) -> bessemer::Result<FlowMapArrays> {
    let source = payload_carrier(433, payload);
    let id = source.records().next().unwrap().id;
    source.record_433_arrays(&id, options)
}
