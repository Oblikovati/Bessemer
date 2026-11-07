use super::{packet::*, *};
fn offset(cursor: &mut PacketCursor) -> usize {
    word(cursor) as usize
}
fn wide(cursor: &mut PacketCursor) -> u64 {
    u64::from_le_bytes(number(cursor))
}
fn container(cursor: &mut PacketCursor) -> ContainerProjection {
    ContainerProjection {
        start: offset(cursor),
        end: offset(cursor),
        count: offset(cursor),
        first: counted(cursor, wide),
        indices: counted(cursor, word),
        second: counted(cursor, wide),
    }
}
fn slot(cursor: &mut PacketCursor) -> SlotProjection {
    SlotProjection {
        ordinal: offset(cursor),
        tag: byte(cursor),
        canonical: boolean(cursor),
        container: optional(cursor, container),
    }
}
fn slots(cursor: &mut PacketCursor) -> Vec<SlotProjection> {
    counted(cursor, slot)
}
fn row(cursor: &mut PacketCursor) -> RowProjection {
    RowProjection {
        start: offset(cursor),
        end: offset(cursor),
        count: offset(cursor),
        indices: counted(cursor, word),
        slots: optional(cursor, slots),
    }
}
fn presence(cursor: &mut PacketCursor) -> PresenceProjection {
    PresenceProjection {
        ordinal: offset(cursor),
        flag: byte(cursor),
        row: optional(cursor, row),
    }
}
fn group(cursor: &mut PacketCursor) -> GroupProjection {
    GroupProjection {
        ordinal: offset(cursor),
        length: offset(cursor),
        presence: blob(cursor),
        rows: counted(cursor, presence),
    }
}
fn grid(cursor: &mut PacketCursor) -> GridProjection {
    GridProjection {
        start: offset(cursor),
        end: offset(cursor),
        dimensions: std::array::from_fn(|_| signed(cursor)),
        members: std::array::from_fn(|_| signed(cursor)),
        groups: counted(cursor, group),
    }
}
fn chunk(cursor: &mut PacketCursor) -> EncodedChunk {
    EncodedChunk {
        ordinal: offset(cursor),
        stored: offset(cursor),
        encoded: blob(cursor),
    }
}
fn chunks(cursor: &mut PacketCursor) -> ChunkProjection {
    ChunkProjection {
        version: signed(cursor),
        count: offset(cursor),
        declared: wide(cursor),
        chunks: counted(cursor, chunk),
        codec_decoded: boolean(cursor),
        decoded_size_verified: boolean(cursor),
    }
}
fn kind(cursor: &mut PacketCursor) -> ComponentKind {
    match byte(cursor) {
        1 => ComponentKind::Graph1,
        2 => ComponentKind::Graph2,
        3 => ComponentKind::Auxiliary,
        4 => ComponentKind::Trailing,
        5 => ComponentKind::Chunks,
        value => panic!("kind {value}; expected owned component kind 1..=5"),
    }
}
fn projection(kind: ComponentKind, cursor: &mut PacketCursor) -> ComponentProjection {
    match kind {
        ComponentKind::Graph1 => ComponentProjection::Graph1(counted(cursor, wide)),
        ComponentKind::Graph2 | ComponentKind::Auxiliary => ComponentProjection::Grid(grid(cursor)),
        ComponentKind::Trailing => ComponentProjection::Trailing(counted(cursor, container)),
        ComponentKind::Chunks => ComponentProjection::Chunks(chunks(cursor)),
    }
}
fn case(cursor: &mut PacketCursor) -> ComponentCase {
    let name = String::from_utf8(blob(cursor)).unwrap();
    let kind = kind(cursor);
    ComponentCase {
        name,
        kind,
        raw: blob(cursor),
        end: offset(cursor),
        qualifications: std::array::from_fn(|_| boolean(cursor)),
        projection: projection(kind, cursor),
    }
}
fn packet(raw: &[u8]) -> Vec<ComponentCase> {
    let mut cursor = cursor(raw, *b"FCC1");
    let result = counted(&mut cursor, case);
    finish(&cursor);
    result
}
pub(super) fn cases() -> Vec<ComponentCase> {
    [
        include_bytes!("../fixtures/flow-checkpoint-components/graph1.bin.gz").as_slice(),
        include_bytes!("../fixtures/flow-checkpoint-components/graph2.bin.gz"),
        include_bytes!("../fixtures/flow-checkpoint-components/graph2-containers.bin.gz"),
        include_bytes!("../fixtures/flow-checkpoint-components/auxiliary.bin.gz"),
        include_bytes!("../fixtures/flow-checkpoint-components/trailing.bin.gz"),
        include_bytes!("../fixtures/flow-checkpoint-components/chunks.bin.gz"),
    ]
    .into_iter()
    .flat_map(packet)
    .collect()
}
