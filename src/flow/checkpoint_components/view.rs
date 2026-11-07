use super::*;
mod chunks;
mod containers;
mod grids;
pub use chunks::*;
pub use containers::*;
pub use grids::*;
/// Borrowed validated byte, index or wide-pattern array; values keep their serialized classes.
#[derive(Clone, Copy, Debug)]
pub struct FlowCheckpointPatternArray<'a> {
    pub(super) owner: &'a FlowCheckpointComponent,
    pub(super) span: PatternSpan,
}
impl FlowCheckpointComponent {
    /// Borrows the exact immutable standalone component; e.g. `let raw = view.as_bytes()`.
    pub fn as_bytes(&self) -> &[u8] {
        &self.revision.wire
    }
    /// Returns the caller's logical declaration; e.g. `let context = view.context()`.
    pub fn context(&self) -> FlowCheckpointContext {
        self.revision.context
    }
    /// Returns the selected logical layout; e.g. `let kind = view.kind()`.
    pub fn kind(&self) -> FlowCheckpointKind {
        self.revision.kind
    }
    /// Returns the original aggregate bounds; e.g. `let limit = view.options().maximum_slots`.
    pub fn options(&self) -> FlowCheckpointOptions {
        self.revision.options
    }
    /// Borrows graph one's exact seven values; e.g. `let raw = view.graph1_patterns()?.value(0)?`.
    pub fn graph1_patterns(&self) -> Result<FlowCheckpointPatternArray<'_>> {
        match &self.revision.layout {
            Layout::Graph1(span) => Ok(array(self, *span)),
            _ => Err(wrong_profile(self, "Graph1")),
        }
    }
    /// Reports complete component framing and explicit exclusions; e.g. `let coverage = view.coverage()`.
    pub fn coverage(&self) -> FlowCheckpointCoverage {
        FlowCheckpointCoverage {
            mode: FlowCheckpointCheckMode::ExactLogicalComponent,
            context: self.context(),
            kind: self.kind(),
            component_bytes: self.as_bytes().len(),
            counts: self.revision.counts,
            complete_component_framing: true,
            complete_parent_record: false,
            engineering_fields_assigned: false,
            chunk_codec_decoded: false,
            decoded_size_verified: false,
            achieved_byte_error: 0,
        }
    }
    /// Borrows native, writer and codec qualifications; e.g. `let exclusions = view.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowCheckpointDiagnostic] {
        &self.revision.diagnostics
    }
}
impl FlowCheckpointPatternArray<'_> {
    /// Returns the complete stored element count; e.g. `let count = array.len()`.
    pub fn len(self) -> usize {
        self.span.count
    }
    /// Distinguishes complete empty arrays; e.g. `assert!(array.is_empty())`.
    pub fn is_empty(self) -> bool {
        self.span.count == 0
    }
    /// Returns validated element width in bytes; e.g. `let width = array.element_width()`.
    pub fn element_width(self) -> usize {
        self.span.shape.width()
    }
    /// Returns only raw element bytes, excluding count prefixes; e.g. `let span = array.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end())
    }
    /// Returns a checked revision-bound existing element; e.g. `let field = array.field(0)?`.
    pub fn field(self, index: usize) -> Result<FlowCheckpointField> {
        let index = checked_index(index, self.span.count, "pattern array")?;
        Ok(self.owner.field(
            self.span.start + index * self.span.shape.width(),
            self.span.shape,
            ValueRule::Any,
        ))
    }
    /// Reads one exact serialized value; e.g. `let value = array.value(0)?`.
    pub fn value(self, index: usize) -> Result<FlowCheckpointValue> {
        self.field(index)?.value()
    }
}
fn array(owner: &FlowCheckpointComponent, span: PatternSpan) -> FlowCheckpointPatternArray<'_> {
    FlowCheckpointPatternArray { owner, span }
}
fn wrong_profile(owner: &FlowCheckpointComponent, expected: &str) -> BessemerError {
    invalid(
        "component kind",
        format!(
            "kind {:?}; expected {expected} logical layout",
            owner.kind()
        ),
    )
}
