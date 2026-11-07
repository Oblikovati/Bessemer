#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ComponentKind {
    Graph1,
    Graph2,
    Auxiliary,
    Trailing,
    Chunks,
}
pub(super) struct ComponentCase {
    pub name: String,
    pub kind: ComponentKind,
    pub raw: Vec<u8>,
    pub end: usize,
    pub qualifications: [bool; 3],
    pub projection: ComponentProjection,
}
pub(super) enum ComponentProjection {
    Graph1(Vec<u64>),
    Grid(GridProjection),
    Trailing(Vec<ContainerProjection>),
    Chunks(ChunkProjection),
}
pub(super) struct ContainerProjection {
    pub start: usize,
    pub end: usize,
    pub count: usize,
    pub first: Vec<u64>,
    pub indices: Vec<u32>,
    pub second: Vec<u64>,
}
pub(super) struct GridProjection {
    pub start: usize,
    pub end: usize,
    pub dimensions: [i32; 3],
    pub members: [i32; 3],
    pub groups: Vec<GroupProjection>,
}
pub(super) struct GroupProjection {
    pub ordinal: usize,
    pub length: usize,
    pub presence: Vec<u8>,
    pub rows: Vec<PresenceProjection>,
}
pub(super) struct PresenceProjection {
    pub ordinal: usize,
    pub flag: u8,
    pub row: Option<RowProjection>,
}
pub(super) struct RowProjection {
    pub start: usize,
    pub end: usize,
    pub count: usize,
    pub indices: Vec<u32>,
    pub slots: Option<Vec<SlotProjection>>,
}
pub(super) struct SlotProjection {
    pub ordinal: usize,
    pub tag: u8,
    pub canonical: bool,
    pub container: Option<ContainerProjection>,
}
pub(super) struct ChunkProjection {
    pub version: i32,
    pub count: usize,
    pub declared: u64,
    pub chunks: Vec<EncodedChunk>,
    pub codec_decoded: bool,
    pub decoded_size_verified: bool,
}
pub(super) struct EncodedChunk {
    pub ordinal: usize,
    pub stored: usize,
    pub encoded: Vec<u8>,
}
