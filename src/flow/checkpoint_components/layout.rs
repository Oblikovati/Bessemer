use super::*;
#[derive(Debug)]
pub(super) struct CheckpointRevision {
    pub wire: Arc<[u8]>,
    pub context: FlowCheckpointContext,
    pub kind: FlowCheckpointKind,
    pub options: FlowCheckpointOptions,
    pub layout: Layout,
    pub counts: FlowCheckpointCounts,
    pub diagnostics: Vec<FlowCheckpointDiagnostic>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ValueShape {
    Byte,
    Signed32,
    Pattern32,
    Pattern64,
}
impl ValueShape {
    pub fn width(self) -> usize {
        match self {
            Self::Byte => 1,
            Self::Signed32 | Self::Pattern32 => 4,
            Self::Pattern64 => 8,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ValueRule {
    Any,
    Presence,
    SlotTag,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PatternSpan {
    pub start: usize,
    pub count: usize,
    pub shape: ValueShape,
}
impl PatternSpan {
    pub fn end(self) -> usize {
        self.start + self.count * self.shape.width()
    }
}
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Layout {
    Graph1(PatternSpan),
    Grid(GridLayout),
    Trailing(Vec<ContainerLayout>),
    Chunks(Vec<ChunkLayout>),
}
#[derive(Debug, PartialEq, Eq)]
pub(super) struct ContainerLayout {
    pub start: usize,
    pub end: usize,
    pub first: PatternSpan,
    pub indices: PatternSpan,
    pub second: PatternSpan,
}
#[derive(Debug, PartialEq, Eq)]
pub(super) struct GridLayout {
    pub dimensions: [i32; 3],
    pub groups: Vec<GroupLayout>,
}
#[derive(Debug, PartialEq, Eq)]
pub(super) struct GroupLayout {
    pub presence: PatternSpan,
    pub rows: Vec<Option<RowLayout>>,
}
#[derive(Debug, PartialEq, Eq)]
pub(super) struct RowLayout {
    pub start: usize,
    pub end: usize,
    pub indices: PatternSpan,
    pub slots: Option<Vec<SlotLayout>>,
}
#[derive(Debug, PartialEq, Eq)]
pub(super) struct SlotLayout {
    pub start: usize,
    pub container: Option<ContainerLayout>,
}
#[derive(Debug, PartialEq, Eq)]
pub(super) struct ChunkLayout {
    pub start: usize,
    pub end: usize,
    pub encoded: PatternSpan,
}
