use super::{LIMITS, configuration_for_case, next_case};
use crate::archive_codec::ArchiveCursor;
use crate::model_archives::{
    ModelDimensionStoredState, ModelField, ModelFieldValue, PartConfigurationArchive,
    PartResolvedFeaturesArchive,
};

fn inspect_dimensions(
    fields: &[ModelField],
    view: impl Fn(usize) -> ModelDimensionStoredState,
) -> usize {
    let mut count = 0;
    for (index, _) in fields
        .iter()
        .enumerate()
        .filter(|(_, field)| field.path.ends_with(".am_object_id"))
    {
        let dimension = view(index + 2);
        assert_dimension_words(fields, index + 2, &dimension);
        count += 1;
    }
    count
}

fn assert_dimension_words(
    fields: &[ModelField],
    index: usize,
    dimension: &ModelDimensionStoredState,
) {
    let ModelFieldValue::I32(mask) = fields[index].value else {
        panic!("expected physical signed mask")
    };
    let ModelFieldValue::I32(identifier) = fields[index + 1].value else {
        panic!("expected physical signed redundant identifier")
    };
    assert_eq!(dimension.mask(), mask);
    assert_eq!(dimension.redundant_id(), identifier);
    assert_eq!(dimension.enforce_bend_angle_range(), mask & 2 != 0);
    assert_eq!(
        u32::from(dimension.fillet_ref_point_change_number()),
        (mask as u32 >> 2) & 7
    );
    assert_eq!(dimension.mask_unresolved_bits(), mask as u32 & 0xffffffe1);
}

fn configuration_dimensions(source: &PartConfigurationArchive) -> usize {
    inspect_dimensions(source.fields(), |index| {
        source
            .dimension_state(&source.field_handle(index).unwrap())
            .unwrap()
    })
}

fn feature_dimensions(source: &PartResolvedFeaturesArchive) -> usize {
    inspect_dimensions(source.fields(), |index| {
        source
            .dimension_state(&source.field_handle(index).unwrap())
            .unwrap()
    })
}

fn inspect_native_dimensions(
    case: &super::ResolvedCorpusCase<'_>,
    kernel: Option<crate::model_archives::CompressedKernelContext>,
) -> [usize; 2] {
    let configuration = configuration_for_case(case, kernel).unwrap();
    let features = PartResolvedFeaturesArchive::from_bytes(
        case.features,
        configuration.clone(),
        case.manager,
        LIMITS,
    )
    .unwrap();
    let counts = [
        configuration_dimensions(&configuration),
        feature_dimensions(&features),
    ];
    assert_eq!(configuration.to_bytes().unwrap(), case.configuration);
    assert_eq!(features.to_bytes().unwrap(), case.features);
    counts
}

#[test]
#[ignore = "requires the complete retained906-case packet and explicit kernel schema"]
fn native_dimension_views_preserve_all906_carriers_and_original_redundant_word_denominator() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(cursor.u32().unwrap(), 906);
    let mut totals = [0; 2];
    for index in 0..906 {
        let counts = inspect_native_dimensions(&next_case(&mut cursor), kernel.clone());
        totals[0] += counts[0];
        totals[1] += counts[1];
        println!("DIMENSION_NATIVE\t{index}\t{}\t{}", counts[0], counts[1]);
    }
    assert_eq!(cursor.remaining(), 0);
    // #179 / C-1923: 305,272 gained bytes are one signed identifier for each feature dimension.
    assert_eq!(totals[1], 305272 / 4);
    println!("DIMENSION_TOTAL\t{}\t{}", totals[0], totals[1]);
}
