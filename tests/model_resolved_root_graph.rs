//! Complete stored root snapshots, without feature dependency or geometry inference (#201).
#[path = "support/owned_resolved_document.rs"]
mod owned_resolved_document;
#[path = "support/owned_saved_manager_document.rs"]
mod owned_saved_manager_document;
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::*;
use owned_resolved_document::*;

const ALIASES: &[u8] = include_bytes!("fixtures/spline-equation-state/alias-graphs/graph-0.bin");
const ROOT_LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn root_archive(raw: &[u8]) -> PartResolvedFeaturesArchive {
    let document = owned_saved_manager_document::prepared_saved_manager_document(raw);
    read_owned_resolved_document(&document, ROOT_LIMITS)
}

#[test]
fn root_order_nulls_aliases_and_exact_frames_match_independent_graph_literals() {
    let graph = root_archive(ALIASES).root_graph().unwrap();
    let tags = [65535, 98, 0, 65535, 100];
    let definitions = [Some(0), Some(0), None, Some(3), Some(3)];
    assert_eq!(graph.roots().len(), 5);
    for (ordinal, root) in graph.roots().iter().enumerate() {
        assert_eq!(root.ordinal, ordinal);
        assert_eq!(root.tag.framing.tag, tags[ordinal]);
        assert_eq!(root.tag.framing.extended, None);
        assert_eq!(graph.root(&root.field).unwrap(), root);
        assert_root_target(&graph, root, definitions[ordinal]);
    }
    assert_graph_contract(&graph);
}

fn assert_root_target(
    graph: &ModelResolvedRootGraph,
    root: &ModelResolvedRootOccurrence,
    expected: Option<usize>,
) {
    match (&root.target, expected) {
        (ModelResolvedRootTarget::Null, None) => {
            assert_eq!(root.tag.record, ModelObjectRecord::Null)
        }
        (ModelResolvedRootTarget::Local { definition }, Some(ordinal)) => {
            assert_eq!(definition, &graph.roots()[ordinal].field);
            let class = match &root.tag.record {
                ModelObjectRecord::New { class, .. }
                | ModelObjectRecord::Reference { class, .. } => class,
                other => panic!("{other:?}; expected sgSketch record"),
            };
            assert_eq!(class.name, "sgSketch");
            assert_eq!(class.schema, 7);
        }
        actual => panic!("{actual:?}; expected literal local/null target"),
    }
}

fn assert_graph_contract(graph: &ModelResolvedRootGraph) {
    assert_eq!(graph.source().to_bytes().unwrap(), ALIASES);
    assert_eq!(graph.completion(), ModelEditCompletion::Unchanged);
    assert_eq!(graph.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(graph.coverage(), graph.source().coverage());
    assert_eq!(graph.achieved_stored_value_error(), 0);
    assert_eq!(graph.diagnostics().len(), 3);
    assert_eq!(graph.history().len(), graph.roots().len());
    for (root, copy) in graph.roots().iter().zip(graph.history()) {
        assert_eq!(copy.parents.as_slice(), std::slice::from_ref(&root.field));
        assert_eq!(copy.result, root.field);
    }
}

#[test]
fn empty_lists_complete_unchanged_and_nonroot_selections_fail_explicitly() {
    let source = root_archive(&[0x85, 0xff, 0xff, 0xff, 0, 0]);
    let graph = source.root_graph().unwrap();
    assert!(graph.roots().is_empty());
    assert!(graph.history().is_empty());
    assert_eq!(graph.completion(), ModelEditCompletion::Unchanged);
    let error = graph.root(&source.field_handle(0).unwrap()).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert_eq!(error.operation, "resolved root graph");
    assert!(error.cause.contains("expected"));
}

#[test]
fn configuration_objects_are_classified_as_inherited_targets() {
    let target = inherited_target();
    let mut raw = vec![0x85, 0xff, 0xff, 0xff, 1, 0];
    raw.extend_from_slice(&u16::try_from(target.value()).unwrap().to_le_bytes());
    let graph = root_archive(&raw).root_graph().unwrap();
    assert_eq!(
        graph.roots()[0].target,
        ModelResolvedRootTarget::Inherited(target)
    );
    assert_eq!(graph.source().to_bytes().unwrap(), raw);
}

#[test]
fn foreign_and_changed_revision_handles_do_not_select_old_roots() {
    let source = root_archive(ALIASES);
    let graph = source.root_graph().unwrap();
    let foreign = root_archive(ALIASES).root_graph().unwrap();
    assert!(graph.root(&foreign.roots()[0].field).is_err());
    let mut fields = source.fields().to_vec();
    fields[0].value = ModelFieldValue::I32(-456);
    let edited = source.edit_fields(fields).unwrap();
    let current = edited.archive().root_graph().unwrap();
    assert!(graph.root(&current.roots()[0].field).is_err());
    assert!(current.root(&graph.roots()[0].field).is_err());
    assert_graph_contract(&graph);
}

#[test]
fn eight_workers_produce_identical_snapshots_from_one_immutable_source() {
    let source = root_archive(ALIASES);
    let expected = source.root_graph().unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let source = &source;
            let expected = &expected;
            scope.spawn(move || replay_root_graph(source, expected));
        }
    });
    assert_graph_contract(&expected);
}

fn replay_root_graph(source: &PartResolvedFeaturesArchive, expected: &ModelResolvedRootGraph) {
    for _ in 0..16 {
        let actual = source.root_graph().unwrap();
        assert_eq!(actual.roots(), expected.roots());
        assert_eq!(actual.history(), expected.history());
        assert_graph_contract(&actual);
    }
}

#[test]
fn all_published_root_snapshot_types_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<ModelResolvedRootGraph>();
    require::<ModelResolvedRootOccurrence>();
    require::<ModelResolvedRootTarget>();
    require::<ModelResolvedRootCopy>();
    require::<ModelResolvedRootDiagnostic>();
}

#[test]
fn every_physical_field_selection_matches_root_membership_without_mutating_the_graph() {
    let source = root_archive(ALIASES);
    let graph = source.root_graph().unwrap();
    for index in 0..source.fields().len() {
        let handle = source.field_handle(index).unwrap();
        let expected = graph.roots().iter().find(|root| root.field == handle);
        match (graph.root(&handle), expected) {
            (Ok(actual), Some(expected)) => assert_eq!(actual, expected),
            (Err(error), None) => {
                assert_eq!(error.class, bessemer::FailureClass::Invalid);
                assert_eq!(error.operation, "resolved root graph");
                assert!(error.cause.contains("expected"));
            }
            actual => panic!("{actual:?} at field {index}; expected physical root membership"),
        }
    }
    assert_graph_contract(&graph);
}

#[test]
fn mutating_caller_owned_occurrence_copies_cannot_change_the_published_snapshot() {
    let graph = root_archive(ALIASES).root_graph().unwrap();
    let mut copies = graph.roots().to_vec();
    copies[0].ordinal = usize::MAX;
    copies[0].target = ModelResolvedRootTarget::Null;
    copies[0].tag.record = ModelObjectRecord::Null;
    assert_eq!(graph.roots()[0].ordinal, 0);
    assert!(matches!(
        graph.roots()[0].target,
        ModelResolvedRootTarget::Local { .. }
    ));
    assert!(matches!(
        graph.roots()[0].tag.record,
        ModelObjectRecord::New { .. }
    ));
    assert_graph_contract(&graph);
}

fn inherited_target() -> ModelObjectId {
    owned_root_configuration()
        .fields()
        .iter()
        .find_map(|field| match &field.value {
            ModelFieldValue::ObjectTag(tag) => match tag.record {
                ModelObjectRecord::New { index, .. } => Some(index),
                _ => None,
            },
            _ => None,
        })
        .unwrap()
}

#[test]
fn aliases_can_target_nested_local_definitions_without_becoming_root_definitions() {
    let original = include_bytes!("fixtures/shared-sketch-handles/owned-0-1.bin");
    let source = root_archive(original);
    let (field_index, object) = nested_target(&source);
    assert_eq!(&original[4..6], &[1, 0]);
    let mut raw = original.to_vec();
    raw[4..6].copy_from_slice(&2u16.to_le_bytes());
    raw.extend_from_slice(&u16::try_from(object.value()).unwrap().to_le_bytes());
    let graph = root_archive(&raw).root_graph().unwrap();
    assert_eq!(graph.roots().len(), 2);
    let ModelResolvedRootTarget::Local { definition } = &graph.roots()[1].target else {
        panic!("expected local nested object {object:?}");
    };
    assert_eq!(definition.index(), field_index);
    assert_ne!(definition, &graph.roots()[0].field);
    assert_eq!(graph.source().to_bytes().unwrap(), raw);
}

fn nested_target(source: &PartResolvedFeaturesArchive) -> (usize, ModelObjectId) {
    source
        .fields()
        .iter()
        .enumerate()
        .find_map(|(index, field)| {
            if field.path.as_ref() == "root.features[0]" {
                return None;
            }
            match &field.value {
                ModelFieldValue::ObjectTag(tag) => match tag.record {
                    ModelObjectRecord::New { index: object, .. } => Some((index, object)),
                    _ => None,
                },
                _ => None,
            }
        })
        .unwrap()
}

fn owned_root_configuration() -> PartConfigurationArchive {
    let document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        ROOT_LIMITS,
    )
    .unwrap()
}

fn root_stream_result(
    raw: &[u8],
    configuration: &PartConfigurationArchive,
    limits: ModelArchiveLimits,
) -> bessemer::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_bytes(
        raw,
        configuration.clone(),
        include_bytes!("fixtures/sheet-metal/manager-0.bin"),
        limits,
    )
}

#[test]
fn every_incomplete_alias_stream_and_appended_byte_fails_without_publishing_partial_roots() {
    let configuration = owned_root_configuration();
    let source = root_stream_result(ALIASES, &configuration, ROOT_LIMITS).unwrap();
    let graph = source.root_graph().unwrap();
    for end in 0..ALIASES.len() {
        assert!(
            root_stream_result(&ALIASES[..end], &configuration, ROOT_LIMITS).is_err(),
            "prefix{end}; expected incomplete stream failure"
        );
    }
    for value in 0..=u8::MAX {
        let mut appended = ALIASES.to_vec();
        appended.push(value);
        assert!(
            root_stream_result(&appended, &configuration, ROOT_LIMITS).is_err(),
            "appended{value}; expected exact EOF failure"
        );
    }
    assert_graph_contract(&graph);
    assert_eq!(source.to_bytes().unwrap(), ALIASES);
}

#[test]
fn exact_source_byte_and_field_caps_pass_and_one_below_fails_without_changing_the_snapshot() {
    let configuration = owned_root_configuration();
    let source = root_stream_result(ALIASES, &configuration, ROOT_LIMITS).unwrap();
    let exact = ModelArchiveLimits {
        maximum_bytes: ALIASES.len(),
        maximum_fields: source.fields().len(),
        ..ROOT_LIMITS
    };
    let accepted = root_stream_result(ALIASES, &configuration, exact).unwrap();
    assert_graph_contract(&accepted.root_graph().unwrap());
    for limits in insufficient_root_caps(exact) {
        let error = root_stream_result(ALIASES, &configuration, limits).unwrap_err();
        assert!(error.cause.contains("expected"));
    }
    assert_graph_contract(&source.root_graph().unwrap());
}

fn insufficient_root_caps(exact: ModelArchiveLimits) -> [ModelArchiveLimits; 2] {
    [
        ModelArchiveLimits {
            maximum_bytes: exact.maximum_bytes - 1,
            ..exact
        },
        ModelArchiveLimits {
            maximum_fields: exact.maximum_fields - 1,
            ..exact
        },
    ]
}
