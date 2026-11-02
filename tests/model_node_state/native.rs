use super::{LIMITS, configuration_for_case, next_case};
use crate::archive_codec::ArchiveCursor;
use crate::model_archives::{
    ModelField, ModelFieldValue, ModelNodeStoredFlag, ModelNodeStoredState,
    PartResolvedFeaturesArchive,
};

fn inspect_nodes(
    fields: &[ModelField],
    require_clear_global: bool,
    view: impl Fn(usize) -> ModelNodeStoredState,
) -> (usize, usize) {
    let (mut nodes, mut parameters) = (0, 0);
    for (index, _) in fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.ends_with(".flags_28"))
    {
        let node = view(index - 1);
        assert_native_words(fields, index, &node);
        if node.parameter_class().is_some() {
            parameters += 1;
            if require_clear_global {
                assert_clear_global(&node);
            }
        }
        nodes += 1;
    }
    (nodes, parameters)
}

fn assert_native_words(fields: &[ModelField], index: usize, node: &ModelNodeStoredState) {
    let ModelFieldValue::I32(state) = fields[index - 1].value else {
        panic!("expected source state")
    };
    let ModelFieldValue::U32(flags) = fields[index].value else {
        panic!("expected source flags")
    };
    assert_eq!(node.update_state(), state);
    assert_eq!(node.flags_28(), flags);
    assert_eq!(
        node.stored_flag(ModelNodeStoredFlag::NeedsUpdate).unwrap(),
        state as u32 & 1 != 0
    );
}

fn assert_clear_global(node: &ModelNodeStoredState) {
    assert!(
        !node
            .stored_flag(ModelNodeStoredFlag::GlobalVariableParameter)
            .unwrap()
    );
}

#[test]
#[ignore = "requires the complete retained906-case packet and explicit kernel schema"]
fn native_stored_node_views_preserve_all906_archives_and_original_parameter_denominator() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(cursor.u32().unwrap(), 906);
    let mut parameters = 0;
    for index in 0..906 {
        let case = next_case(&mut cursor);
        parameters += inspect_native_case(&case, kernel.clone(), index);
    }
    assert_eq!(cursor.remaining(), 0);
    assert_eq!(parameters, 39487);
}

fn inspect_native_case(
    case: &super::ResolvedCorpusCase<'_>,
    kernel: Option<crate::model_archives::CompressedKernelContext>,
    index: u32,
) -> usize {
    let configuration = configuration_for_case(case, kernel).unwrap();
    let features = PartResolvedFeaturesArchive::from_bytes(
        case.features,
        configuration.clone(),
        case.manager,
        LIMITS,
    )
    .unwrap();
    let base = inspect_configuration(&configuration);
    let extra = inspect_features(&features);
    assert_eq!(configuration.to_bytes().unwrap(), case.configuration);
    assert_eq!(features.to_bytes().unwrap(), case.features);
    println!("NODE_NATIVE\t{index}\t{}\t{}\t{}", base.0, extra.0, extra.1);
    extra.1
}

fn inspect_configuration(
    configuration: &crate::model_archives::PartConfigurationArchive,
) -> (usize, usize) {
    inspect_nodes(configuration.fields(), false, |position| {
        configuration
            .node_state(&configuration.field_handle(position).unwrap())
            .unwrap()
    })
}

fn inspect_features(features: &PartResolvedFeaturesArchive) -> (usize, usize) {
    inspect_nodes(features.fields(), true, |position| {
        features
            .node_state(&features.field_handle(position).unwrap())
            .unwrap()
    })
}
