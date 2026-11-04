use super::{LIMITS, ResolvedCorpusCase, configuration_for_case, next_case};
use crate::archive_codec::ArchiveCursor;
use crate::model_archives::{
    CompressedKernelContext, ModelField, ModelFieldValue, ModelObjectRecord,
    ModelParameterStoredState, PartResolvedFeaturesArchive,
};
#[path = "native_exports.rs"]
mod exports;

fn physical_parameter_count(fields: &[ModelField]) -> usize {
    fields
        .iter()
        .filter(|field| {
            let ModelFieldValue::ObjectTag(tag) = &field.value else {
                return false;
            };
            let ModelObjectRecord::New { class, .. } = &tag.record else {
                return false;
            };
            matches!(
                class.name.as_str(),
                "moDensityParameter_c"
                    | "moLengthParameter_c"
                    | "moAngleParameter_c"
                    | "moIntegerParameter_c"
                    | "moScalerParameter_c"
                    | "moStringParameter_c"
            )
        })
        .count()
}

fn assert_native_parameter_views(states: &[ModelParameterStoredState], fields: &[ModelField]) {
    assert_eq!(states.len(), physical_parameter_count(fields));
    for parameter in states {
        assert_eq!(parameter.achieved_stored_value_error(), 0);
        assert_eq!(parameter.diagnostics().len(), 3);
        for relation in parameter.history() {
            assert_eq!(
                relation.parents.as_slice(),
                std::slice::from_ref(&relation.result)
            );
            parameter.source().resolve_field(&relation.result).unwrap();
        }
        for name in [parameter.node_name(), parameter.parameter_name()] {
            assert_eq!(
                name.text().is_none(),
                matches!(name.tag().record, ModelObjectRecord::Null)
            );
        }
    }
}

fn inspect_native_parameter_case(
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
) -> [usize; 2] {
    let configuration = configuration_for_case(case, kernel).unwrap();
    let source = PartResolvedFeaturesArchive::from_bytes(
        case.features,
        configuration.clone(),
        case.manager,
        LIMITS,
    )
    .unwrap();
    let configuration_parameters = configuration.stored_parameters().unwrap();
    let feature_parameters = source.stored_parameters().unwrap();
    assert_native_parameter_views(&configuration_parameters, configuration.fields());
    assert_native_parameter_views(&feature_parameters, source.fields());
    exports::export_parameter_case(case, &configuration_parameters, &feature_parameters);
    assert_eq!(configuration.to_bytes().unwrap(), case.configuration);
    assert_eq!(source.to_bytes().unwrap(), case.features);
    [configuration_parameters.len(), feature_parameters.len()]
}

#[test]
#[ignore = "requires retained906-case packet and explicit kernel schema; measures complete stored parameter projections"]
fn every_native_parameter_definition_matches_physical_population_and_keeps_complete_carriers() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(cursor.u32().unwrap(), 906);
    let mut totals = [0; 2];
    for index in 0..906 {
        let counts = inspect_native_parameter_case(&next_case(&mut cursor), kernel.clone());
        totals[0] += counts[0];
        totals[1] += counts[1];
        println!("PARAMETER_NATIVE\t{index}\t{}\t{}", counts[0], counts[1]);
    }
    assert_eq!(cursor.remaining(), 0);
    println!("PARAMETER_NATIVE_TOTAL\t906\t{}\t{}", totals[0], totals[1]);
}
