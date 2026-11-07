use super::packet::*;
use super::types::*;

pub fn cases() -> Vec<FieldCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-field-values/values.bin.gz"),
        *b"FFV1",
    );
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}

fn case(cursor: &mut PacketCursor) -> FieldCase {
    FieldCase {
        frame: frame(cursor),
        version: signed(cursor),
        projection: projection(cursor),
        body_end: word(cursor),
        byte_counts: std::array::from_fn(|_| word(cursor)),
        flags: std::array::from_fn(|_| boolean(cursor)),
    }
}

fn frame(cursor: &mut PacketCursor) -> FieldFrame {
    FieldFrame {
        name: String::from_utf8(blob(cursor)).unwrap(),
        compressed: boolean(cursor),
        restriction: boolean(cursor),
        context: context(cursor),
        stored: blob(cursor),
        decoded: blob(cursor),
    }
}

fn context(cursor: &mut PacketCursor) -> FieldContext {
    match byte(cursor) {
        0 => FieldContext::Legacy {
            table_count: word(cursor),
            object_widths: counted(cursor, word),
            expected: optional(cursor, expected_widths),
        },
        1 => FieldContext::Regions {
            rows: counted(cursor, word),
            widths: counted(cursor, word),
            active: counted(cursor, boolean),
        },
        kind => panic!("context kind {kind}; expected legacy zero or regions one"),
    }
}

fn expected_widths(cursor: &mut PacketCursor) -> Vec<Option<u32>> {
    counted(cursor, |cursor| optional(cursor, word))
}

fn projection(cursor: &mut PacketCursor) -> FieldProjection {
    match byte(cursor) {
        0 => FieldProjection::Legacy {
            table: counted(cursor, signed),
            checked: boolean(cursor),
            objects: counted(cursor, object),
        },
        1 => FieldProjection::Regions {
            count_checked: boolean(cursor),
            widths_checked: boolean(cursor),
            regions: counted(cursor, region),
        },
        kind => panic!("projection kind {kind}; expected global zero or region one"),
    }
}

fn object(cursor: &mut PacketCursor) -> FieldObject {
    let ordinal = word(cursor);
    let width = word(cursor);
    let start = word(cursor);
    let end = word(cursor);
    FieldObject {
        ordinal,
        width,
        start,
        end,
        bits: (0..width).map(|_| wide(cursor)).collect(),
    }
}

fn region(cursor: &mut PacketCursor) -> FieldRegion {
    FieldRegion {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        rows: word(cursor),
        width: word(cursor),
        active: boolean(cursor),
        bits: counted(cursor, wide),
    }
}

pub fn width_cases() -> Vec<WidthCase> {
    let mut cursor = cursor(
        include_bytes!("../fixtures/flow-field-values/widths.bin.gz"),
        *b"FWP1",
    );
    let result = counted(&mut cursor, width_case);
    finish(&cursor);
    result
}

fn width_case(cursor: &mut PacketCursor) -> WidthCase {
    WidthCase {
        code: signed(cursor),
        cached: optional(cursor, signed_wide),
        index: optional(cursor, signed),
        numeric: optional(cursor, |cursor| counted(cursor, width_number)),
        expected: optional(cursor, word),
    }
}

fn width_number(cursor: &mut PacketCursor) -> WidthNumber {
    match byte(cursor) {
        0 => WidthNumber::Signed(signed_wide(cursor)),
        1 => WidthNumber::FloatBits(wide(cursor)),
        kind => panic!("numeric kind {kind}; expected signed zero or binary64 one"),
    }
}

fn wide(cursor: &mut PacketCursor) -> u64 {
    u64::from_le_bytes(number(cursor))
}
fn signed_wide(cursor: &mut PacketCursor) -> i64 {
    i64::from_le_bytes(number(cursor))
}
