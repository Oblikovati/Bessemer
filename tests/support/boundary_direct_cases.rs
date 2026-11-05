pub(crate) struct BoundaryDirectCase<'a> {
    pub context: u32,
    pub version: u32,
    pub style: usize,
    pub edited: bool,
    pub raw: &'a [u8],
}

fn boundary_direct_word(cursor: &mut &[u8]) -> u32 {
    let (word, tail) = cursor.split_at(4);
    *cursor = tail;
    u32::from_le_bytes(word.try_into().unwrap())
}

fn boundary_direct_case<'a>(cursor: &mut &'a [u8]) -> BoundaryDirectCase<'a> {
    let context = boundary_direct_word(cursor);
    let version = boundary_direct_word(cursor);
    let style = boundary_direct_word(cursor) as usize;
    let edited = boundary_direct_word(cursor);
    assert!(edited <= 1);
    let length = boundary_direct_word(cursor) as usize;
    let (raw, tail) = cursor.split_at(length);
    *cursor = tail;
    BoundaryDirectCase {
        context,
        version,
        style,
        edited: edited == 1,
        raw,
    }
}

pub(crate) fn boundary_direct_cases() -> Vec<BoundaryDirectCase<'static>> {
    let mut cursor = include_bytes!("../fixtures/boundary-direct-data/cases.bin").as_slice();
    assert_eq!(&cursor[..4], b"BBD1");
    cursor = &cursor[4..];
    assert_eq!(boundary_direct_word(&mut cursor), 95);
    let cases = (0..95).map(|_| boundary_direct_case(&mut cursor)).collect();
    assert!(cursor.is_empty());
    cases
}
