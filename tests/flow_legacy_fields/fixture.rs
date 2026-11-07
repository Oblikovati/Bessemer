use super::*;
pub(super) struct ScalarQuery {
    pub nonzero: bool,
    pub call: bool,
    pub status: Option<i32>,
    pub faults: bool,
    pub virtual_result: bool,
}
pub(super) struct WordCase {
    pub name: String,
    pub identifier: u32,
    pub raw: Vec<u8>,
    pub patterns: Vec<u64>,
    pub end: usize,
    pub query: Option<ScalarQuery>,
    pub runtime: bool,
    pub engineering: bool,
}
pub(super) struct QueryCase {
    pub slot7: u64,
    pub slot10: u64,
    pub query: ScalarQuery,
}
pub(super) struct SelectionCase {
    pub name: String,
    pub records: Vec<(u32, Vec<u8>)>,
    pub completed: bool,
    pub identifier: Option<u32>,
    pub cause: String,
}
fn text(cursor: &mut PacketCursor) -> String {
    String::from_utf8(blob(cursor)).unwrap()
}
fn wide(cursor: &mut PacketCursor) -> u64 {
    u64::from_le_bytes(number(cursor))
}
fn query(cursor: &mut PacketCursor) -> ScalarQuery {
    ScalarQuery {
        nonzero: boolean(cursor),
        call: boolean(cursor),
        status: optional(cursor, signed),
        faults: boolean(cursor),
        virtual_result: boolean(cursor),
    }
}
fn word_case(cursor: &mut PacketCursor) -> WordCase {
    WordCase {
        name: text(cursor),
        identifier: word(cursor),
        raw: blob(cursor),
        patterns: counted(cursor, wide),
        end: word(cursor) as usize,
        query: optional(cursor, query),
        runtime: boolean(cursor),
        engineering: boolean(cursor),
    }
}
fn query_case(cursor: &mut PacketCursor) -> QueryCase {
    QueryCase {
        slot7: wide(cursor),
        slot10: wide(cursor),
        query: query(cursor),
    }
}
fn record(cursor: &mut PacketCursor) -> (u32, Vec<u8>) {
    (word(cursor), blob(cursor))
}
fn selection(cursor: &mut PacketCursor) -> SelectionCase {
    SelectionCase {
        name: text(cursor),
        records: counted(cursor, record),
        completed: boolean(cursor),
        identifier: optional(cursor, word),
        cause: text(cursor),
    }
}
pub(super) fn words() -> Vec<WordCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-legacy-field-containers/words.bin.gz"),
        *b"FLW1",
    );
    let result = counted(&mut cursor, word_case);
    finish(&cursor);
    result
}
pub(super) fn queries() -> Vec<QueryCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-legacy-field-containers/queries.bin.gz"),
        *b"FLQ1",
    );
    let result = counted(&mut cursor, query_case);
    finish(&cursor);
    result
}
pub(super) fn groups() -> Vec<ConditionalCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-legacy-field-containers/groups.bin.gz"),
        *b"FCI1",
    );
    let result = counted(&mut cursor, case_reader::case);
    finish(&cursor);
    result
}
pub(super) fn selections() -> Vec<SelectionCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-legacy-field-containers/selections.bin.gz"),
        *b"FLS1",
    );
    let result = counted(&mut cursor, selection);
    finish(&cursor);
    result
}
