use super::*;
type Selector = fn(&FlowCheckpointComponent) -> FlowCheckpointField;
#[test]
fn freshly_reopened_reference_edits_reverse_all_value_classes_and_both_protected_byte_rules() {
    for row in cases()
        .into_iter()
        .filter(|row| row.name.starts_with("reference-"))
    {
        fresh_reversal(&row, representative, EditClass::Plain);
        if row.kind == ComponentKind::Graph2 {
            fresh_reversal(&row, presence, EditClass::Presence);
            fresh_reversal(&row, tag, EditClass::SlotTag);
            fresh_reversal(&row, container_word, EditClass::Plain);
        }
        if row.kind == ComponentKind::Chunks {
            fresh_reversal(&row, declared, EditClass::Plain);
        }
    }
}
fn fresh_reversal(row: &ComponentCase, select: Selector, class: EditClass) {
    let source = component(row);
    let id = select(&source);
    let field = retained_specification(id, class);
    let changed = edit::check_edit(&source, &field, edit::replacement(&field));
    let result = changed.result();
    let reopened = reopen(result);
    let fresh = select(&reopened);
    assert_ne!(&fresh, changed.result_field());
    assert!(
        reopened
            .replace_field(changed.result_field(), field.value)
            .is_err()
    );
    let reverse = reopened.replace_field(&fresh, field.value).unwrap();
    assert_eq!(reverse.result().as_bytes(), source.as_bytes());
    assert_eq!(reverse.result().coverage(), source.coverage());
}
fn retained_specification(id: FlowCheckpointField, class: EditClass) -> EditableField {
    EditableField {
        interval: id.payload_interval(),
        value: id.value().unwrap(),
        id,
        class,
    }
}

fn representative(view: &FlowCheckpointComponent) -> FlowCheckpointField {
    match view.kind() {
        FlowCheckpointKind::Graph1 => view.graph1_patterns().unwrap().field(0).unwrap(),
        FlowCheckpointKind::Graph2 => view.grid().unwrap().member(0).unwrap(),
        FlowCheckpointKind::Auxiliary => auxiliary_index(view),
        FlowCheckpointKind::TrailingContainers => view
            .trailing_container(0)
            .unwrap()
            .second_patterns()
            .field(1)
            .unwrap(),
        FlowCheckpointKind::ChunkBuffer => view
            .chunk_buffer()
            .unwrap()
            .chunk(0)
            .unwrap()
            .encoded_fields()
            .field(0)
            .unwrap(),
    }
}
fn auxiliary_index(view: &FlowCheckpointComponent) -> FlowCheckpointField {
    view.grid()
        .unwrap()
        .group(2)
        .unwrap()
        .row(0)
        .unwrap()
        .unwrap()
        .indices()
        .field(0)
        .unwrap()
}

fn presence(view: &FlowCheckpointComponent) -> FlowCheckpointField {
    view.grid().unwrap().group(0).unwrap().presence(0).unwrap()
}
fn reference_slot(view: &FlowCheckpointComponent) -> FlowCheckpointSlot<'_> {
    view.grid()
        .unwrap()
        .group(0)
        .unwrap()
        .row(0)
        .unwrap()
        .unwrap()
        .slots()
        .unwrap()
        .slot(1)
        .unwrap()
}
fn tag(view: &FlowCheckpointComponent) -> FlowCheckpointField {
    reference_slot(view).tag()
}
fn container_word(view: &FlowCheckpointComponent) -> FlowCheckpointField {
    reference_slot(view)
        .container()
        .unwrap()
        .first_patterns()
        .field(0)
        .unwrap()
}
fn declared(view: &FlowCheckpointComponent) -> FlowCheckpointField {
    view.chunk_buffer().unwrap().declared_raw_bytes()
}
#[test]
fn caller_buffers_and_other_edits_cannot_mutate_published_components_or_retained_fields() {
    for mut row in cases()
        .into_iter()
        .filter(|row| row.name.starts_with("reference-"))
    {
        let source = component(&row);
        let original = source.as_bytes().to_vec();
        let field = representative(&source);
        let old = field.value().unwrap();
        row.raw.fill(0xff);
        let alternate = wrong_class(old);
        assert!(source.replace_field(&field, alternate).is_err());
        let changed = source.replace_field(&field, changed_value(old)).unwrap();
        assert_eq!(source.as_bytes(), original);
        assert_ne!(changed.result().as_bytes(), original);
        assert_eq!(field.value().unwrap(), old);
        drop(changed);
        drop(source);
        assert_eq!(field.value().unwrap(), old);
    }
}
fn changed_value(value: FlowCheckpointValue) -> FlowCheckpointValue {
    match value {
        FlowCheckpointValue::Byte(value) => FlowCheckpointValue::Byte(value ^ 0xff),
        FlowCheckpointValue::Pattern32(value) => FlowCheckpointValue::Pattern32(!value),
        FlowCheckpointValue::Pattern64(value) => FlowCheckpointValue::Pattern64(!value),
        FlowCheckpointValue::Signed32(value) => FlowCheckpointValue::Signed32(!value),
    }
}
#[test]
fn every_public_checkpoint_type_is_send_sync_and_field_debug_stays_bounded_on_large_sources() {
    value_traits();
    view_traits();
    let row = cases()
        .into_iter()
        .find(|row| row.name == "trailing-container-1-65536")
        .unwrap();
    let source = component(&row);
    let field = source
        .trailing_container(1)
        .unwrap()
        .first_patterns()
        .field(0)
        .unwrap();
    assert!(format!("{field:?}").len() < 256);
    let foreign = component(&row)
        .replace_field(&field, FlowCheckpointValue::Pattern64(0))
        .unwrap_err();
    assert!(foreign.cause.len() < 512);
}
fn value_traits() {
    traits::<FlowCheckpointContext>();
    traits::<FlowCheckpointKind>();
    traits::<FlowCheckpointOptions>();
    traits::<FlowCheckpointCounts>();
    traits::<FlowCheckpointCheckMode>();
    traits::<FlowCheckpointCoverage>();
    traits::<FlowCheckpointDiagnostic>();
    traits::<FlowCheckpointValue>();
    traits::<FlowCheckpointComponent>();
    traits::<FlowCheckpointField>();
    traits::<FlowCheckpointByteHistory>();
    traits::<FlowCheckpointComponentEdit>();
}

fn traits<T: Send + Sync>() {}
fn view_traits() {
    traits::<FlowCheckpointPatternArray<'static>>();
    traits::<FlowCheckpointContainer<'static>>();
    traits::<FlowCheckpointGrid<'static>>();
    traits::<FlowCheckpointGroup<'static>>();
    traits::<FlowCheckpointRow<'static>>();
    traits::<FlowCheckpointSlots<'static>>();
    traits::<FlowCheckpointSlot<'static>>();
    traits::<FlowCheckpointChunks<'static>>();
    traits::<FlowCheckpointChunk<'static>>();
    traits::<FlowPayloadRelation>();
}
