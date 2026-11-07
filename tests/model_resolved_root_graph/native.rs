use super::{LIMITS, ResolvedCorpusCase, configuration_for_case, next_case};
use crate::archive_codec::ArchiveCursor;
use crate::model_archives::{
    CompressedKernelContext, ModelFieldValue, ModelObjectRecord, ModelResolvedRootGraph,
    ModelResolvedRootOccurrence, ModelResolvedRootTarget, PartResolvedFeaturesArchive,
};
#[path = "native_exports.rs"]
pub(crate) mod exports;

fn assert_native_root_target(
    graph: &ModelResolvedRootGraph,
    root: &ModelResolvedRootOccurrence,
) -> usize {
    match &root.target {
        ModelResolvedRootTarget::Null => {
            assert_eq!(root.tag.record, ModelObjectRecord::Null);
            0
        }
        ModelResolvedRootTarget::Inherited(index) => {
            assert!((index.value() as usize) < graph.source().object_slots().0);
            assert!(
                matches!(root.tag.record, ModelObjectRecord::Reference { index: actual, .. } if actual == *index)
            );
            1
        }
        ModelResolvedRootTarget::Local { definition } => {
            assert_local_root_target(graph, root, definition);
            2
        }
    }
}

fn assert_local_root_target(
    graph: &ModelResolvedRootGraph,
    root: &ModelResolvedRootOccurrence,
    definition: &crate::model_archives::ModelFieldHandle,
) {
    let field = graph.source().resolve_field(definition).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &field.value else {
        panic!("{field:?}; expected object definition");
    };
    let ModelObjectRecord::New { index, class } = &tag.record else {
        panic!("{tag:?}; expected defining New tag");
    };
    let (actual, actual_class) = root_record_identity(&root.tag.record);
    assert_eq!(actual, *index);
    assert_eq!(actual_class, class);
}

fn root_record_identity(
    record: &ModelObjectRecord,
) -> (
    crate::model_archives::ModelObjectId,
    &crate::model_archives::ModelClass,
) {
    match record {
        ModelObjectRecord::New { index, class } | ModelObjectRecord::Reference { index, class } => {
            (*index, class)
        }
        actual => panic!("{actual:?}; expected nonnull local target"),
    }
}

fn inspect_native_root_case(
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
) -> [usize; 4] {
    let configuration = configuration_for_case(case, kernel).unwrap();
    let source = PartResolvedFeaturesArchive::from_bytes(
        case.features,
        configuration.clone(),
        case.manager,
        LIMITS,
    )
    .unwrap();
    let graph = source.root_graph().unwrap();
    let mut totals = [graph.roots().len(), 0, 0, 0];
    assert_eq!(graph.roots().len(), source.root_count());
    for (ordinal, root) in graph.roots().iter().enumerate() {
        assert_native_root_occurrence(&graph, root, ordinal);
        totals[1 + assert_native_root_target(&graph, root)] += 1;
    }
    assert_eq!(configuration.to_bytes().unwrap(), case.configuration);
    assert_eq!(source.to_bytes().unwrap(), case.features);
    exports::export_root_case(case, &graph);
    totals
}

fn assert_native_root_occurrence(
    graph: &ModelResolvedRootGraph,
    root: &ModelResolvedRootOccurrence,
    ordinal: usize,
) {
    let source = graph.source();
    assert_eq!(root.ordinal, ordinal);
    assert_eq!(
        source.fields()[root.field.index()].path.as_ref(),
        format!("root.features[{ordinal}]")
    );
    assert_eq!(
        source.resolve_field(&root.field).unwrap().value,
        ModelFieldValue::ObjectTag(Box::new(root.tag.clone()))
    );
    assert_eq!(graph.root(&root.field).unwrap(), root);
}

#[test]
#[ignore = "requires retained906-case packet and explicit kernel schema; validates stored root metadata, not geometry"]
fn every_retained_native_root_preserves_physical_tags_and_loaded_target_links() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(cursor.u32().unwrap(), 906);
    let mut totals = [0; 4];
    for index in 0..906 {
        let actual = inspect_native_root_case(&next_case(&mut cursor), kernel.clone());
        accumulate_root_totals(&mut totals, actual);
        println!(
            "ROOT_NATIVE\t{index}\t{}\t{}\t{}\t{}",
            actual[0], actual[1], actual[2], actual[3]
        );
    }
    assert_eq!(cursor.remaining(), 0);
    println!(
        "ROOT_NATIVE_TOTAL\t906\t{}\t{}\t{}\t{}",
        totals[0], totals[1], totals[2], totals[3]
    );
}

fn accumulate_root_totals(totals: &mut [usize; 4], actual: [usize; 4]) {
    for (total, count) in totals.iter_mut().zip(actual) {
        *total += count;
    }
}
