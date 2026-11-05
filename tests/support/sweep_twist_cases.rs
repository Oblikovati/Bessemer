pub struct OwnedTwistCase {
    pub label: &'static str,
    pub slots: usize,
    pub object_slots: Vec<u32>,
    pub raw: &'static [u8],
    pub payloads: Vec<&'static [u8]>,
}

fn take_twist<'a>(cursor: &mut &'a [u8], count: usize) -> &'a [u8] {
    let (value, tail) = cursor.split_at(count);
    *cursor = tail;
    value
}

fn twist_u16(cursor: &mut &[u8]) -> usize {
    u16::from_le_bytes(take_twist(cursor, 2).try_into().unwrap()) as usize
}

fn twist_u32(cursor: &mut &[u8]) -> u32 {
    u32::from_le_bytes(take_twist(cursor, 4).try_into().unwrap())
}

fn owned_case(cursor: &mut &'static [u8]) -> OwnedTwistCase {
    let length = twist_u16(cursor);
    let label = std::str::from_utf8(take_twist(cursor, length)).unwrap();
    let slots = twist_u32(cursor) as usize;
    let count = twist_u16(cursor);
    let object_slots = (0..count).map(|_| twist_u32(cursor)).collect();
    let length = twist_u32(cursor) as usize;
    let raw = take_twist(cursor, length);
    let count = twist_u16(cursor);
    let payloads = (0..count).map(|_| take_twist(cursor, 17)).collect();
    OwnedTwistCase {
        label,
        slots,
        object_slots,
        raw,
        payloads,
    }
}

pub fn owned_twist_cases() -> Vec<OwnedTwistCase> {
    let mut cursor = include_bytes!("../fixtures/sweep-twist/owned-cases.bin").as_slice();
    assert_eq!(take_twist(&mut cursor, 4), b"BWO1");
    assert_eq!(twist_u32(&mut cursor), 48);
    let result = (0..48).map(|_| owned_case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    result
}
