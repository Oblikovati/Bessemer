use super::{OverflowPageImage, OwnedOverflowCase, identity};

pub(super) fn packet_cases() -> Vec<OwnedOverflowCase> {
    let mut cursor = include_bytes!("../fixtures/database-overflow/cases.bin").as_slice();
    assert_eq!(take(&mut cursor, 4), b"HOV1");
    assert_eq!(u32::from_le_bytes(word(&mut cursor)), 117);
    let cases = (0..117)
        .map(|_| {
            let count = u32::from_le_bytes(word(&mut cursor)) as usize;
            decode_case(take(&mut cursor, count))
        })
        .collect();
    assert!(cursor.is_empty());
    cases
}

fn decode_case(mut cursor: &[u8]) -> OwnedOverflowCase {
    let extent = u32::from_le_bytes(word(&mut cursor)) as usize;
    let key_length = u32::from_le_bytes(word(&mut cursor)) as usize;
    let value_length = u64::from_le_bytes(word(&mut cursor)) as usize;
    let wrapped = take(&mut cursor, 1)[0] != 0;
    let count = u32::from_le_bytes(word(&mut cursor)) as usize;
    take(&mut cursor, extent);
    let pages = decode_pages(&mut cursor, count, extent);
    let key = take(&mut cursor, key_length).to_vec();
    let value = take(&mut cursor, value_length).to_vec();
    assert!(cursor.is_empty());
    OwnedOverflowCase {
        extent,
        pages,
        key,
        value,
        wrapped,
    }
}

pub(super) fn decode_pages(
    cursor: &mut &[u8],
    count: usize,
    extent: usize,
) -> Vec<OverflowPageImage> {
    (0..count)
        .map(|_| OverflowPageImage {
            identifier: identity(u64::from_le_bytes(word(cursor))),
            bytes: take(cursor, extent).to_vec(),
        })
        .collect()
}

pub(super) fn word<const N: usize>(cursor: &mut &[u8]) -> [u8; N] {
    take(cursor, N).try_into().unwrap()
}

pub(super) fn take<'a>(cursor: &mut &'a [u8], count: usize) -> &'a [u8] {
    let (selected, remaining) = cursor.split_at(count);
    *cursor = remaining;
    selected
}
