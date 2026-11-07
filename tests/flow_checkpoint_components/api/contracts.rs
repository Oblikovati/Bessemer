use super::*;
#[test]
fn wrong_views_invalid_indices_foreign_stale_fields_and_value_class_errors_are_typed_and_non_destructive()
 {
    for row in cases()
        .into_iter()
        .filter(|row| row.name.starts_with("reference-"))
    {
        let view = component(&row);
        wrong_views(&view);
        check_indices(&view);
        let foreign = component(&row);
        for field in read::fields(&row, &view) {
            reject_foreign(&foreign, &field);
            reject_class(&view, &field);
            reject_structure(&view, &field);
            reject_stale(&view, &field);
        }
        assert_eq!(view.as_bytes(), row.raw);
    }
}
fn wrong_views(view: &FlowCheckpointComponent) {
    assert_eq!(
        view.graph1_patterns().is_ok(),
        view.kind() == FlowCheckpointKind::Graph1
    );
    assert_eq!(
        view.grid().is_ok(),
        matches!(
            view.kind(),
            FlowCheckpointKind::Graph2 | FlowCheckpointKind::Auxiliary
        )
    );
    assert_eq!(
        view.trailing_container(0).is_ok(),
        view.kind() == FlowCheckpointKind::TrailingContainers
    );
    assert_eq!(
        view.chunk_buffer().is_ok(),
        view.kind() == FlowCheckpointKind::ChunkBuffer
    );
}
fn reject_foreign(view: &FlowCheckpointComponent, field: &EditableField) {
    let before = view.as_bytes().to_vec();
    let error = view.replace_field(&field.id, field.value).unwrap_err();
    assert_eq!(error.operation, "Flow logical checkpoint components");
    assert_eq!(error.entity, "field identity");
    assert!(error.cause.len() < 512 && format!("{:?}", field.id).len() < 256);
    assert_eq!(view.as_bytes(), before);
}
fn reject_class(view: &FlowCheckpointComponent, field: &EditableField) {
    let wrong = wrong_class(field.value);
    let error = view.replace_field(&field.id, wrong).unwrap_err();
    assert_eq!(error.entity, "replacement field class");
    assert_eq!(field.id.value().unwrap(), field.value);
}
fn reject_structure(view: &FlowCheckpointComponent, field: &EditableField) {
    let FlowCheckpointValue::Byte(old) = field.value else {
        return;
    };
    let new = match field.class {
        EditClass::Plain => return,
        EditClass::Presence => u8::from(old == 0),
        EditClass::SlotTag => {
            if old <= 1 {
                2
            } else {
                0
            }
        }
    };
    let error = view
        .replace_field(&field.id, FlowCheckpointValue::Byte(new))
        .unwrap_err();
    assert_eq!(error.entity, "replacement byte structure");
    assert_eq!(field.id.value().unwrap(), field.value);
}
fn reject_stale(view: &FlowCheckpointComponent, field: &EditableField) {
    let new = edit::replacement(field);
    if new == field.value {
        return;
    }
    let changed = view.replace_field(&field.id, new).unwrap();
    assert_ne!(changed.source_field(), changed.result_field());
    reject_foreign(changed.result(), field);
    assert_eq!(view.as_bytes(), changed.source().as_bytes());
}
fn check_indices(view: &FlowCheckpointComponent) {
    if let Ok(array) = view.graph1_patterns() {
        array_bounds(array);
    }
    if let Ok(grid) = view.grid() {
        grid_bounds(grid);
    }
    if let Ok(chunks) = view.chunk_buffer() {
        assert!(chunks.chunk(chunks.len()).is_err());
        assert!(chunks.chunk(usize::MAX).is_err());
        for index in 0..chunks.len() {
            array_bounds(chunks.chunk(index).unwrap().encoded_fields());
        }
    }
    if view.kind() == FlowCheckpointKind::TrailingContainers {
        assert!(view.trailing_container(3).is_err());
        assert!(view.trailing_container(usize::MAX).is_err());
        for index in 0..3 {
            container_bounds(view.trailing_container(index).unwrap());
        }
    }
}
fn array_bounds(array: FlowCheckpointPatternArray<'_>) {
    assert!(array.field(array.len()).is_err());
    assert!(array.value(array.len()).is_err());
    assert!(array.field(usize::MAX).is_err());
    assert!(array.value(usize::MAX).is_err());
}
fn container_bounds(container: FlowCheckpointContainer<'_>) {
    array_bounds(container.first_patterns());
    array_bounds(container.indices());
    array_bounds(container.second_patterns());
}
fn grid_bounds(grid: FlowCheckpointGrid<'_>) {
    assert!(grid.group(3).is_err() && grid.group(usize::MAX).is_err());
    assert!(grid.member(3).is_err() && grid.member(usize::MAX).is_err());
    for axis in 0..3 {
        group_bounds(grid.group(axis).unwrap());
    }
}
fn group_bounds(group: FlowCheckpointGroup<'_>) {
    assert!(group.row(group.len()).is_err() && group.row(usize::MAX).is_err());
    assert!(group.presence(group.len()).is_err() && group.presence(usize::MAX).is_err());
    for index in 0..group.len() {
        if let Some(row) = group.row(index).unwrap() {
            row_bounds(row);
        }
    }
}

fn row_bounds(row: FlowCheckpointRow<'_>) {
    array_bounds(row.indices());
    let Some(slots) = row.slots() else {
        return;
    };
    assert!(slots.slot(slots.len()).is_err() && slots.slot(usize::MAX).is_err());
    for index in 0..slots.len() {
        if let Some(container) = slots.slot(index).unwrap().container() {
            container_bounds(container);
        }
    }
}
