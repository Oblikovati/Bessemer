use super::*;

fn configuration_document() -> Document {
    let mut document = Document::from_bytes(OWNED).unwrap();
    document
        .replace_stream("Contents/Config-0", CONFIGURATION.to_vec())
        .unwrap();
    document
        .replace_stream(
            "Contents/Config-0-ResolvedFeatures",
            CONFIGURATION_FEATURES.to_vec(),
        )
        .unwrap();
    document
}

fn read_configuration(document: &Document) -> PartConfigurationArchive {
    PartConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn whole_document_fixture_has_a_complete_empty_feature_archive_with_the_new_inherited_slots() {
    let document = configuration_document();
    let source = read_configuration(&document);
    let manager = document.read_stream("Contents/CMgr").unwrap();
    let raw = document
        .read_stream("Contents/Config-0-ResolvedFeatures")
        .unwrap();
    assert_eq!(raw, CONFIGURATION_FEATURES);
    let features = PartResolvedFeaturesArchive::from_bytes(&raw, source, &manager, LIMITS).unwrap();
    assert_eq!(features.to_bytes().unwrap(), CONFIGURATION_FEATURES);
    assert_eq!(features.object_slots(), (100, 100));
    assert_eq!(features.coverage().wire_bytes, 10);
}

fn handles(source: &PartConfigurationArchive) -> Vec<ModelFieldHandle> {
    source
        .fields()
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.contains("object_map[") && field.path.ends_with(".mask"))
        .map(|(index, _)| source.field_handle(index).unwrap())
        .collect()
}

fn base_offsets() -> Vec<usize> {
    include_str!("../fixtures/dimension-stored-state/configuration-dimensions.tsv")
        .lines()
        .map(|line| line.split('\t').next().unwrap().parse::<usize>().unwrap() - 16)
        .collect()
}

fn assert_configuration_history(
    source: &PartConfigurationArchive,
    edit: &PartConfigurationEdit,
    selected: usize,
    changed: bool,
) {
    let handles = [
        source.field_handle(selected).unwrap(),
        edit.archive().field_handle(selected).unwrap(),
    ];
    edits::assert_field_history(
        (edit.completion(), edit.unchanged_fields(), edit.changes()),
        source.fields().len(),
        handles,
        changed,
    );
    assert_eq!(
        edit.archive()
            .resolve_field(&source.field_handle(selected).unwrap())
            .is_err(),
        changed
    );
}

fn assert_configuration_request(
    source: &PartConfigurationArchive,
    handle: &ModelFieldHandle,
    base: usize,
    request: ModelDimensionStoredEdit,
) {
    let dimension = source.dimension_state(handle).unwrap();
    let (expected, relative) = edits::literal_edit(CONFIGURATION, base, request);
    let edit = source.edit_dimension_state(&dimension, request).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), expected);
    edits::assert_unselected(
        source.fields(),
        edit.archive().fields(),
        handle.index() + relative,
    );
    assert_configuration_history(
        source,
        &edit,
        handle.index() + relative,
        expected != CONFIGURATION,
    );
    assert_configuration_view(edit.archive(), handle.index(), base, &expected);
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(source.to_bytes().unwrap(), CONFIGURATION);
}

#[test]
fn complete_configuration_dimensions_match_original_offsets_and_all_role_edits() {
    let source = read_configuration(&configuration_document());
    let handles = handles(&source);
    assert_eq!(handles.len(), 2);
    for (handle, base) in handles.iter().zip(base_offsets()) {
        for request in edits::requests() {
            assert_configuration_request(&source, handle, base, request);
        }
    }
}

fn update_document(document: &mut Document, counter: u8) {
    let source = read_configuration(document);
    let view = source.dimension_state(&handles(&source)[0]).unwrap();
    let edit = source
        .edit_dimension_state(
            &view,
            ModelDimensionStoredEdit::FilletRefPointChangeNumber(counter),
        )
        .unwrap();
    document
        .replace_stream("Contents/Config-0", edit.archive().to_bytes().unwrap())
        .unwrap();
}

#[test]
fn dimension_configuration_edits_reopen_preserve_other_streams_and_revert_the_whole_document() {
    let mut document = configuration_document();
    let original = document.to_bytes().unwrap();
    update_document(&mut document, 0);
    let mut reopened = Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
    let source = read_configuration(&reopened);
    let view = source.dimension_state(&handles(&source)[0]).unwrap();
    assert_eq!(view.fillet_ref_point_change_number(), 0);
    assert_other_streams(&Document::from_bytes(&original).unwrap(), &reopened);
    update_document(&mut reopened, 7);
    assert_eq!(reopened.to_bytes().unwrap(), original);
}

#[test]
fn configuration_foreign_stale_and_counter_failures_preserve_complete_sources() {
    let source = read_configuration(&configuration_document());
    let foreign = read_configuration(&configuration_document());
    let handle = handles(&source).remove(0);
    let view = source.dimension_state(&handle).unwrap();
    assert!(foreign.dimension_state(&handle).is_err());
    assert!(
        foreign
            .edit_dimension_state(&view, ModelDimensionStoredEdit::RedundantId(0))
            .is_err()
    );
    assert_configuration_failures(&source, &view, &handle);
    assert_eq!(source.to_bytes().unwrap(), CONFIGURATION);
    assert_eq!(foreign.to_bytes().unwrap(), CONFIGURATION);
}

fn assert_configuration_view(
    source: &PartConfigurationArchive,
    index: usize,
    base: usize,
    expected: &[u8],
) {
    let result = source
        .dimension_state(&source.field_handle(index).unwrap())
        .unwrap();
    let mask = u32::from_le_bytes(expected[base + 16..base + 20].try_into().unwrap());
    let identifier = i32::from_le_bytes(expected[base + 20..base + 24].try_into().unwrap());
    assert_view(&result, mask, identifier);
}

fn assert_other_streams(baseline: &Document, reopened: &Document) {
    for entry in baseline.streams() {
        if entry.name != "Contents/Config-0" {
            assert_eq!(
                reopened.read_stream(entry.name).unwrap(),
                baseline.read_stream(entry.name).unwrap()
            );
        }
    }
}

fn assert_configuration_failures(
    source: &PartConfigurationArchive,
    view: &ModelDimensionStoredState,
    handle: &ModelFieldHandle,
) {
    for value in 8..=255 {
        let request = ModelDimensionStoredEdit::FilletRefPointChangeNumber(value);
        assert!(source.edit_dimension_state(view, request).is_err());
    }
    let changed = source
        .edit_dimension_state(view, ModelDimensionStoredEdit::RedundantId(0))
        .unwrap();
    assert!(changed.archive().dimension_state(handle).is_err());
    assert!(
        changed
            .archive()
            .edit_dimension_state(view, ModelDimensionStoredEdit::RedundantId(1))
            .is_err()
    );
}
