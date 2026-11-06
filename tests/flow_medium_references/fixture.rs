use super::{packet::*, types::*};
pub fn cases() -> Vec<ReferenceCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-medium-references/bodies.bin.gz"),
        *b"FMR1",
    );
    let result = counted(&mut cursor, reference);
    finish(&cursor);
    result
}
fn reference(cursor: &mut PacketCursor) -> ReferenceCase {
    ReferenceCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        raw: blob(cursor),
        kind: signed(cursor),
        keys: counted(cursor, blob),
        key: blob(cursor),
        state: optional(cursor, byte),
        loaded_state: optional(cursor, byte),
        references: counted(cursor, blob),
        end: word(cursor),
        tail: blob(cursor),
        runtime: boolean(cursor),
    }
}
