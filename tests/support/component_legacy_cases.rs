pub(crate) struct ComponentLegacyCase<'a> {
    pub context: u32,
    pub version: u32,
    pub presence: i32,
    pub kind: usize,
    pub edited: bool,
    pub raw: &'a [u8],
}

fn component_legacy_word(cursor: &mut &[u8]) -> u32 {
    let (word, tail) = cursor.split_at(4);
    *cursor = tail;
    u32::from_le_bytes(word.try_into().unwrap())
}

fn component_legacy_case<'a>(cursor: &mut &'a [u8]) -> ComponentLegacyCase<'a> {
    let context = component_legacy_word(cursor);
    let version = component_legacy_word(cursor);
    let presence = component_legacy_word(cursor) as i32;
    let kind = component_legacy_word(cursor) as usize;
    assert!(kind < 3);
    let edited = component_legacy_word(cursor);
    assert!(edited <= 1);
    let count = component_legacy_word(cursor) as usize;
    let (raw, tail) = cursor.split_at(count);
    *cursor = tail;
    ComponentLegacyCase {
        context,
        version,
        presence,
        kind,
        edited: edited == 1,
        raw,
    }
}

pub(crate) fn component_legacy_cases() -> Vec<ComponentLegacyCase<'static>> {
    let mut cursor = include_bytes!("../fixtures/component-legacy-reference/cases.bin").as_slice();
    assert_eq!(&cursor[..4], b"BLR1");
    cursor = &cursor[4..];
    assert_eq!(component_legacy_word(&mut cursor), 297);
    let cases = (0..297)
        .map(|_| component_legacy_case(&mut cursor))
        .collect();
    assert!(cursor.is_empty());
    cases
}
