use super::*;
pub struct SelectionCase {
    pub name: String,
    pub native_count: Option<i32>,
    pub inputs: SelectionInputs,
    pub expected: SelectionExpected,
}
pub struct SelectionInputs {
    pub compressed_links: bool,
    pub links_stored: Vec<u8>,
    pub links_decoded: Vec<u8>,
    pub states_stored: Vec<u8>,
    pub states_decoded: Vec<u8>,
    pub version: i32,
    pub entries: Vec<SelectionEntry>,
}
pub struct SelectionEntry {
    pub ordinal: u32,
    pub start: u32,
    pub end: u32,
    pub code: u8,
    pub native_code: u8,
}
pub struct SelectionExpected {
    pub values: Vec<u8>,
    pub link_count: u32,
    pub selected_count: u32,
    pub parsed_count: u32,
    pub value_end: u32,
    pub dense_indices: Vec<i32>,
    pub selected_ordinals: Vec<u32>,
    pub entry_patterns: Vec<Option<u64>>,
    pub fresh: bool,
    pub context_owner: bool,
    pub complete: bool,
    pub value_owner: bool,
    pub scatter: bool,
    pub engineering: bool,
}
pub fn cases() -> Vec<SelectionCase> {
    [
        include_bytes!("../fixtures/flow-selected-values/part0.bin.gz").as_slice(),
        include_bytes!("../fixtures/flow-selected-values/part1.bin.gz").as_slice(),
        include_bytes!("../fixtures/flow-selected-values/part2.bin.gz").as_slice(),
    ]
    .into_iter()
    .flat_map(read_packet)
    .collect()
}
fn read_packet(raw: &[u8]) -> Vec<SelectionCase> {
    let mut cursor = packet::cursor(raw, *b"FSL1");
    for code in 0..=255 {
        assert_eq!(boolean(&mut cursor), ACCEPTED.contains(&(code as u8)));
    }
    for code in 0..=255 {
        assert_eq!(byte(&mut cursor), converted_code(code as u8, 0));
    }
    let rows = counted(&mut cursor, case);
    finish(&cursor);
    rows
}
fn case(cursor: &mut PacketCursor) -> SelectionCase {
    SelectionCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        native_count: optional(cursor, signed),
        inputs: inputs(cursor),
        expected: expected(cursor),
    }
}
fn inputs(cursor: &mut PacketCursor) -> SelectionInputs {
    SelectionInputs {
        compressed_links: boolean(cursor),
        links_stored: blob(cursor),
        links_decoded: blob(cursor),
        states_stored: blob(cursor),
        states_decoded: blob(cursor),
        version: signed(cursor),
        entries: counted(cursor, entry),
    }
}
fn entry(cursor: &mut PacketCursor) -> SelectionEntry {
    SelectionEntry {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        code: byte(cursor),
        native_code: byte(cursor),
    }
}
fn expected(cursor: &mut PacketCursor) -> SelectionExpected {
    SelectionExpected {
        values: blob(cursor),
        link_count: word(cursor),
        selected_count: word(cursor),
        parsed_count: word(cursor),
        value_end: word(cursor),
        dense_indices: counted(cursor, signed),
        selected_ordinals: counted(cursor, word),
        entry_patterns: counted(cursor, optional_wide),
        fresh: boolean(cursor),
        context_owner: boolean(cursor),
        complete: boolean(cursor),
        value_owner: boolean(cursor),
        scatter: boolean(cursor),
        engineering: boolean(cursor),
    }
}
fn optional_wide(cursor: &mut PacketCursor) -> Option<u64> {
    optional(cursor, wide)
}
fn wide(cursor: &mut PacketCursor) -> u64 {
    u64::from_le_bytes(packet::number(cursor))
}
