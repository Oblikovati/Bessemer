use super::*;
pub fn cases() -> Vec<KindTwoCase> {
    let mut cursor = cursor(
        include_bytes!("../../fixtures/flow-medium-kind-two/bodies.bin.gz"),
        *b"FMO2",
    );
    let result = counted(&mut cursor, body);
    finish(&cursor);
    result
}
fn body(cursor: &mut PacketCursor) -> KindTwoCase {
    KindTwoCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        raw: blob(cursor),
        prefix: fixture::prefix(cursor),
        scalar: fixture::wide(cursor),
        model: model(cursor),
        footer: footer(cursor),
    }
}
fn model(cursor: &mut PacketCursor) -> ModelCase {
    let kind = signed(cursor);
    let start = word(cursor);
    let end = word(cursor);
    let version = byte(cursor);
    let payload = payload(cursor, kind);
    ModelCase {
        kind,
        start,
        end,
        version,
        payload,
    }
}
fn table(cursor: &mut PacketCursor) -> TableModel {
    TableModel {
        first_count: fixture::count_word(cursor),
        second_count: fixture::count_word(cursor),
        first: counted(cursor, fixture::words),
        second: fixture::words(cursor),
        word_patterns: counted(cursor, word),
        third_count: fixture::count_word(cursor),
        third: fixture::words(cursor),
        member: signed(cursor),
        scalars: fixture::words(cursor),
        states: blob(cursor),
    }
}
fn footer(cursor: &mut PacketCursor) -> KindTwoFooter {
    KindTwoFooter {
        state: byte(cursor),
        loaded: byte(cursor),
        end: word(cursor),
        tail: blob(cursor),
        complete: boolean(cursor),
        installed: boolean(cursor),
        runtime: boolean(cursor),
    }
}

fn payload(cursor: &mut PacketCursor, kind: i32) -> ModelPayload {
    match kind {
        0..=2 => ModelPayload::Curves(CurveModel {
            curves: counted(cursor, fixture::curve),
            scalars: fixture::words(cursor),
            states: blob(cursor),
        }),
        3 => ModelPayload::Scalars(fixture::words(cursor)),
        4 => ModelPayload::Switched(SwitchedModel {
            first: fixture::words(cursor),
            state: byte(cursor),
            last: fixture::wide(cursor),
        }),
        5 => ModelPayload::Table(table(cursor)),
        _ => panic!("model kind {kind}; expected 0..=5"),
    }
}
