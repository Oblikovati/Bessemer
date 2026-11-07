use super::*;
/// Borrowed complete numeric container, preserving its three raw arrays.
#[derive(Clone, Copy, Debug)]
pub struct FlowCheckpointContainer<'a> {
    pub(super) owner: &'a FlowCheckpointComponent,
    pub(super) layout: &'a ContainerLayout,
}
impl FlowCheckpointComponent {
    /// Borrows one of exactly three trailing containers; e.g. `let container = view.trailing_container(0)?`.
    pub fn trailing_container(&self, index: usize) -> Result<FlowCheckpointContainer<'_>> {
        let Layout::Trailing(containers) = &self.revision.layout else {
            return Err(wrong_profile(self, "TrailingContainers"));
        };
        let index = checked_index(index, containers.len(), "trailing container")?;
        Ok(container(self, &containers[index]))
    }
}
impl<'a> FlowCheckpointContainer<'a> {
    /// Returns the nonnegative stored count N; e.g. `let count = container.stored_count()`.
    pub fn stored_count(self) -> usize {
        self.layout.first.count
    }
    /// Returns the complete body, including the count; e.g. `let span = container.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.layout.start, self.layout.end)
    }
    /// Borrows the first N raw 64-bit values; e.g. `let raw = container.first_patterns().value(0)?`.
    pub fn first_patterns(self) -> FlowCheckpointPatternArray<'a> {
        array(self.owner, self.layout.first)
    }
    /// Borrows N raw 32-bit indices; e.g. `let field = container.indices().field(0)?`.
    pub fn indices(self) -> FlowCheckpointPatternArray<'a> {
        array(self.owner, self.layout.indices)
    }
    /// Borrows the final 2N raw 64-bit values; e.g. `let count = container.second_patterns().len()`.
    pub fn second_patterns(self) -> FlowCheckpointPatternArray<'a> {
        array(self.owner, self.layout.second)
    }
}
pub(super) fn container<'a>(
    owner: &'a FlowCheckpointComponent,
    layout: &'a ContainerLayout,
) -> FlowCheckpointContainer<'a> {
    FlowCheckpointContainer { owner, layout }
}
