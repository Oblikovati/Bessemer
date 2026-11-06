use super::*;
pub fn cases() -> Vec<ProfileCase> {
    let mut cursor = cursor(
        include_bytes!("../../fixtures/flow-medium-references/profiles.bin.gz"),
        *b"FCP1",
    );
    let result = counted(&mut cursor, profile);
    finish(&cursor);
    result
}
fn profile(cursor: &mut PacketCursor) -> ProfileCase {
    ProfileCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        raw: blob(cursor),
        prefix: blob(cursor),
        backend_start: word(cursor),
        backend_end: word(cursor),
        first: counted(cursor, first),
        second: counted(cursor, second),
        footer: footer(cursor),
    }
}
fn first(cursor: &mut PacketCursor) -> ProfileFirst {
    ProfileFirst {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        kind: signed(cursor),
        key: blob(cursor),
        body: fixture::body(cursor),
    }
}
fn second(cursor: &mut PacketCursor) -> ProfileSecond {
    ProfileSecond {
        ordinal: word(cursor),
        start: word(cursor),
        end: word(cursor),
        kind: signed(cursor),
        key: blob(cursor),
        state: byte(cursor),
        loaded: byte(cursor),
        references: counted(cursor, blob),
    }
}
fn footer(cursor: &mut PacketCursor) -> ProfileFooter {
    ProfileFooter {
        keys: counted(cursor, blob),
        attributes: counted(cursor, signed),
        final_counts: [word(cursor), word(cursor)],
        end: word(cursor),
        complete: boolean(cursor),
        engineering: boolean(cursor),
        fresh: boolean(cursor),
        runtime: boolean(cursor),
    }
}
