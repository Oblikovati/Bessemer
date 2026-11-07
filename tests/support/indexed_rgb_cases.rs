pub struct IndexedRgbCase<'a> {
    pub original: &'a [u8],
    pub edited: &'a [u8],
    pub indices: &'a [u8],
    pub edited_indices: &'a [u8],
}

fn indexed_rgb_blob<'a>(cursor: &mut &'a [u8]) -> &'a [u8] {
    let (size, remaining) = cursor.split_first_chunk::<4>().unwrap();
    let length = u32::from_le_bytes(*size) as usize;
    let (blob, tail) = remaining.split_at(length);
    *cursor = tail;
    blob
}

fn indexed_rgb_case<'a>(cursor: &mut &'a [u8]) -> IndexedRgbCase<'a> {
    IndexedRgbCase {
        original: indexed_rgb_blob(cursor),
        edited: indexed_rgb_blob(cursor),
        indices: indexed_rgb_blob(cursor),
        edited_indices: indexed_rgb_blob(cursor),
    }
}

fn one_entry_cases() -> Vec<IndexedRgbCase<'static>> {
    [
        include_bytes!("../fixtures/preview-indexed-rgb/one-entry-1.bin").as_slice(),
        include_bytes!("../fixtures/preview-indexed-rgb/one-entry-4.bin").as_slice(),
        include_bytes!("../fixtures/preview-indexed-rgb/one-entry-8.bin").as_slice(),
    ]
    .into_iter()
    .map(|original| IndexedRgbCase {
        original,
        edited: original,
        indices: &[0],
        edited_indices: &[0],
    })
    .collect()
}

pub fn indexed_rgb_cases() -> Vec<IndexedRgbCase<'static>> {
    let packet = include_bytes!("../fixtures/preview-indexed-rgb/cases.bin");
    assert_eq!(&packet[..4], b"BIX1");
    assert_eq!(u32::from_le_bytes(packet[4..8].try_into().unwrap()), 864);
    let mut cursor = &packet[8..];
    let mut cases: Vec<IndexedRgbCase<'static>> =
        (0..864).map(|_| indexed_rgb_case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    cases.extend(one_entry_cases());
    cases
}
