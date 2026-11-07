use super::*;
pub struct StateCase {
    pub name: String,
    pub identifier: u32,
    pub version: i32,
    pub context: Option<i32>,
    pub raw: Vec<u8>,
    pub fields: Vec<stored_fields::StoredField>,
    pub scopes: Vec<(String, u32, u32)>,
    pub counts: Vec<u32>,
    pub projections: Vec<(String, bool)>,
    pub quotients: Vec<(String, Option<i32>)>,
    pub complete: bool,
    pub runtime: bool,
    pub engineering: bool,
}
pub fn cases() -> Vec<StateCase> {
    let mut result = Vec::new();
    for compressed in [
        &include_bytes!("../fixtures/flow-state-tables/record129.bin.gz")[..],
        &include_bytes!("../fixtures/flow-state-tables/record138.bin.gz")[..],
        &include_bytes!("../fixtures/flow-state-tables/record139.bin.gz")[..],
        &include_bytes!("../fixtures/flow-state-tables/record417.bin.gz")[..],
        &include_bytes!("../fixtures/flow-state-tables/record432.bin.gz")[..],
    ] {
        let mut cursor = cursor(compressed, *b"FST2");
        result.extend(counted(&mut cursor, case));
        finish(&cursor);
    }
    result
}
fn case(cursor: &mut PacketCursor) -> StateCase {
    StateCase {
        name: text(cursor),
        identifier: word(cursor),
        version: signed(cursor),
        context: optional(cursor, signed),
        raw: blob(cursor),
        fields: counted(cursor, field),
        scopes: counted(cursor, scope),
        counts: counted(cursor, word),
        projections: counted(cursor, projection),
        quotients: counted(cursor, quotient),
        complete: boolean(cursor),
        runtime: boolean(cursor),
        engineering: boolean(cursor),
    }
}
fn text(cursor: &mut PacketCursor) -> String {
    String::from_utf8(blob(cursor)).unwrap()
}
fn field(cursor: &mut PacketCursor) -> stored_fields::StoredField {
    stored_fields::StoredField {
        path: text(cursor),
        offset: word(cursor),
        encoding: byte(cursor),
        count: word(cursor),
        raw: blob(cursor),
    }
}
fn scope(cursor: &mut PacketCursor) -> (String, u32, u32) {
    (text(cursor), word(cursor), word(cursor))
}
fn projection(cursor: &mut PacketCursor) -> (String, bool) {
    (text(cursor), boolean(cursor))
}
fn quotient(cursor: &mut PacketCursor) -> (String, Option<i32>) {
    (text(cursor), optional(cursor, signed))
}
