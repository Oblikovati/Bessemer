use super::*;
use bessemer::flow::*;
mod arrays;
mod chunks;
mod contracts;
mod edit;
mod grids;
mod isolation;
mod limits;
mod read;
mod runtime;
#[derive(Clone, Copy, Debug)]
enum EditClass {
    Plain,
    Presence,
    SlotTag,
}
struct EditableField {
    id: FlowCheckpointField,
    value: FlowCheckpointValue,
    interval: (usize, usize),
    class: EditClass,
}
fn options() -> FlowCheckpointOptions {
    FlowCheckpointOptions {
        maximum_component_bytes: 4_000_000,
        maximum_row_positions: 1_000_000,
        maximum_indices: 1_000_000,
        maximum_slots: 1_000_000,
        maximum_containers: 100_000,
        maximum_wide_patterns: 2_000_000,
        maximum_chunks: 100_000,
        maximum_encoded_bytes: 2_000_000,
    }
}
fn kind(kind: ComponentKind) -> FlowCheckpointKind {
    match kind {
        ComponentKind::Graph1 => FlowCheckpointKind::Graph1,
        ComponentKind::Graph2 => FlowCheckpointKind::Graph2,
        ComponentKind::Auxiliary => FlowCheckpointKind::Auxiliary,
        ComponentKind::Trailing => FlowCheckpointKind::TrailingContainers,
        ComponentKind::Chunks => FlowCheckpointKind::ChunkBuffer,
    }
}
fn component(row: &ComponentCase) -> FlowCheckpointComponent {
    FlowCheckpointComponent::from_bytes(
        &row.raw,
        FlowCheckpointContext::LogicalVersionSix,
        kind(row.kind),
        options(),
    )
    .unwrap()
}
fn selected(index: usize, count: usize) -> bool {
    count <= 64 || index == 0 || index == count / 2 || index + 1 == count
}
fn check_field(
    field: FlowCheckpointField,
    value: FlowCheckpointValue,
    interval: (usize, usize),
    class: EditClass,
    selected: bool,
    edits: &mut Vec<EditableField>,
) {
    assert_eq!(field.value().unwrap(), value);
    assert_eq!(field.payload_interval(), interval);
    if selected {
        edits.push(EditableField {
            id: field,
            value,
            interval,
            class,
        });
    }
}

fn reopen(view: &FlowCheckpointComponent) -> FlowCheckpointComponent {
    FlowCheckpointComponent::from_bytes(
        view.as_bytes(),
        view.context(),
        view.kind(),
        view.options(),
    )
    .unwrap()
}
fn wrong_class(value: FlowCheckpointValue) -> FlowCheckpointValue {
    match value {
        FlowCheckpointValue::Byte(_) | FlowCheckpointValue::Signed32(_) => {
            FlowCheckpointValue::Pattern32(0)
        }
        FlowCheckpointValue::Pattern32(_) => FlowCheckpointValue::Pattern64(0),
        FlowCheckpointValue::Pattern64(_) => FlowCheckpointValue::Byte(0),
    }
}
