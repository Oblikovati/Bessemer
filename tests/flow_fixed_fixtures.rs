//! Independent raw settings slots before introducing the typed settings body API.
use bessemer::flow::{FlowCarrier, FlowLimits};
use std::io::{Cursor, Read};

#[test]
fn six_reference_payloads_rebuild_every_integer_and_binary64_bit() {
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&include_bytes!("fixtures/flow-fixed-settings/slots.bin.gz")[..])
        .take(50_001)
        .read_to_end(&mut raw)
        .unwrap();
    let mut cursor = Cursor::new(raw.as_slice());
    assert_eq!(number::<4>(&mut cursor), *b"FFS1");
    assert_eq!(u32::from_le_bytes(number(&mut cursor)), 6);
    for _ in 0..6 {
        check_case(&mut cursor);
    }
    assert_eq!(cursor.position(), raw.len() as u64);
}

fn check_case(cursor: &mut Cursor<&[u8]>) {
    let name_length = u32::from_le_bytes(number(cursor)) as usize;
    let mut name = vec![0; name_length];
    cursor.read_exact(&mut name).unwrap();
    assert!(std::str::from_utf8(&name).is_ok());
    let identifier = u32::from_le_bytes(number(cursor));
    let length = u32::from_le_bytes(number(cursor)) as usize;
    let count = u32::from_le_bytes(number(cursor)) as usize;
    let mut payload = vec![0; length];
    cursor.read_exact(&mut payload).unwrap();
    let bits: Vec<u64> = (0..count)
        .map(|_| u64::from_le_bytes(number(cursor)))
        .collect();
    let rebuilt = rebuild_slots(identifier, &bits);
    assert_eq!(payload, rebuilt);
    check_carrier(identifier, &payload);
}

fn rebuild_slots(identifier: u32, bits: &[u64]) -> Vec<u8> {
    match identifier {
        7 => {
            assert_eq!(bits.len(), 100);
            bits.iter()
                .flat_map(|value| u32::try_from(*value).unwrap().to_le_bytes())
                .collect()
        }
        102 => {
            assert_eq!(bits.len(), 320);
            bits.iter().flat_map(|value| value.to_le_bytes()).collect()
        }
        other => panic!("fixture identifier {other}; expected 7 or 102"),
    }
}

fn check_carrier(identifier: u32, payload: &[u8]) {
    let mut wire = 7_u16.to_le_bytes().to_vec();
    wire.extend(identifier.to_le_bytes());
    wire.extend((payload.len() as u64).to_le_bytes());
    wire.extend(payload);
    let limits = FlowLimits {
        maximum_wire_bytes: 3000,
        maximum_records: 1,
        maximum_payload_bytes: 2560,
    };
    let carrier = FlowCarrier::from_bytes(&wire, limits).unwrap();
    assert_eq!(carrier.as_bytes(), wire);
    assert_eq!(
        carrier.latest(identifier).unwrap().unwrap().payload,
        payload
    );
}

fn number<const N: usize>(cursor: &mut Cursor<&[u8]>) -> [u8; N] {
    let mut bytes = [0; N];
    cursor.read_exact(&mut bytes).unwrap();
    bytes
}
