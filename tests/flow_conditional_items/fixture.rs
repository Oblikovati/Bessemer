use super::*;
pub fn cases() -> Vec<ConditionalCase> {
    let mut result = Vec::new();
    for compressed in [
        &include_bytes!("../fixtures/flow-conditional-items/record126.bin.gz")[..],
        &include_bytes!("../fixtures/flow-conditional-items/record315.bin.gz")[..],
        &include_bytes!("../fixtures/flow-conditional-items/record128.bin.gz")[..],
    ] {
        let mut cursor = cursor(compressed, *b"FCI1");
        result.extend(counted(&mut cursor, case));
        finish(&cursor);
    }
    result
}
fn case(cursor: &mut PacketCursor) -> ConditionalCase {
    ConditionalCase {
        name: text(cursor),
        identifier: word(cursor),
        version: signed(cursor),
        scale: word(cursor),
        outer_version: optional(cursor, signed),
        stored: blob(cursor),
        decoded: blob(cursor),
        fields: counted(cursor, field),
        scopes: counted(cursor, scope),
        counts: counted(cursor, word),
        projections: counted(cursor, projection),
        complete: boolean(cursor),
        runtime: boolean(cursor),
        engineering: boolean(cursor),
    }
}
fn text(cursor: &mut PacketCursor) -> String {
    String::from_utf8(blob(cursor)).unwrap()
}
fn field(cursor: &mut PacketCursor) -> ConditionalField {
    ConditionalField {
        path: text(cursor),
        offset: word(cursor),
        encoding: byte(cursor),
        count: word(cursor),
        raw: blob(cursor),
    }
}
fn scope(cursor: &mut PacketCursor) -> ConditionalScope {
    ConditionalScope {
        path: text(cursor),
        start: word(cursor),
        end: word(cursor),
    }
}
fn projection(cursor: &mut PacketCursor) -> (String, bool) {
    (text(cursor), boolean(cursor))
}
