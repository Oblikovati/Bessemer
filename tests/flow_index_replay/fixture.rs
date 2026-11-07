use super::packet::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RecordSpan {
    pub identifier: u32,
    pub start: u64,
    pub span: u64,
}
pub(super) struct TerminalProjection {
    pub ordinal: u32,
    pub count: i32,
    pub decoded: Vec<u8>,
    pub entries: Vec<u8>,
    pub end: u32,
    pub compressed: u32,
    pub stored: u32,
    pub footer_start: u32,
    pub distance: u64,
    pub identifier: u32,
    pub width: u8,
    pub complete: bool,
}
pub(super) struct ReplayProjection {
    pub forced: bool,
    pub terminal_adopted: bool,
    pub active: Vec<RecordSpan>,
    pub saved: Vec<(u64, u64)>,
    pub terminal: Option<TerminalProjection>,
    pub version: u16,
    pub physical_count: u32,
    pub qualifications: [bool; 3],
}
pub(super) enum ReplayOutcome {
    Rejected { forced: bool, cause: String },
    Completed(ReplayProjection),
}
pub(super) struct ReplayCase {
    pub name: String,
    pub family: String,
    pub raw: Vec<u8>,
    pub version: u16,
    pub physical: Vec<RecordSpan>,
    pub outcomes: [ReplayOutcome; 2],
}
fn wide(cursor: &mut PacketCursor) -> u64 {
    u64::from_le_bytes(number(cursor))
}
fn record_span(cursor: &mut PacketCursor) -> RecordSpan {
    RecordSpan {
        identifier: word(cursor),
        start: wide(cursor),
        span: wide(cursor),
    }
}
fn saved_pair(cursor: &mut PacketCursor) -> (u64, u64) {
    (wide(cursor), wide(cursor))
}
fn terminal(cursor: &mut PacketCursor) -> TerminalProjection {
    TerminalProjection {
        ordinal: word(cursor),
        count: signed(cursor),
        decoded: blob(cursor),
        entries: blob(cursor),
        end: word(cursor),
        compressed: word(cursor),
        stored: word(cursor),
        footer_start: word(cursor),
        distance: wide(cursor),
        identifier: word(cursor),
        width: byte(cursor),
        complete: boolean(cursor),
    }
}
fn outcome(cursor: &mut PacketCursor) -> ReplayOutcome {
    let forced = boolean(cursor);
    if !boolean(cursor) {
        return ReplayOutcome::Rejected {
            forced,
            cause: String::from_utf8(blob(cursor)).unwrap(),
        };
    }
    ReplayOutcome::Completed(projection(cursor, forced))
}
fn projection(cursor: &mut PacketCursor, forced: bool) -> ReplayProjection {
    ReplayProjection {
        forced,
        terminal_adopted: boolean(cursor),
        active: counted(cursor, record_span),
        saved: counted(cursor, saved_pair),
        terminal: optional(cursor, terminal),
        version: u16::from_le_bytes(number(cursor)),
        physical_count: word(cursor),
        qualifications: std::array::from_fn(|_| boolean(cursor)),
    }
}
fn case(cursor: &mut PacketCursor) -> ReplayCase {
    ReplayCase {
        name: String::from_utf8(blob(cursor)).unwrap(),
        family: String::from_utf8(blob(cursor)).unwrap(),
        raw: blob(cursor),
        version: u16::from_le_bytes(number(cursor)),
        physical: counted(cursor, record_span),
        outcomes: std::array::from_fn(|_| outcome(cursor)),
    }
}
fn packet(raw: &[u8]) -> Vec<ReplayCase> {
    let mut cursor = cursor(raw, *b"FLR1");
    let rows = counted(&mut cursor, case);
    finish(&cursor);
    rows
}
pub(super) fn cases() -> Vec<ReplayCase> {
    let packets: [&[u8]; 4] = [
        include_bytes!("../fixtures/flow-index-replay/native.bin.gz"),
        include_bytes!("../fixtures/flow-index-replay/scan7.bin.gz"),
        include_bytes!("../fixtures/flow-index-replay/scan8.bin.gz"),
        include_bytes!("../fixtures/flow-index-replay/terminal.bin.gz"),
    ];
    packets.into_iter().flat_map(packet).collect()
}
