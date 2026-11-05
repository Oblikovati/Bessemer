use super::{LIMITS, ResolvedCorpusCase, configuration_for_case, next_case};
use crate::archive_codec::ArchiveCursor;
use crate::model_archives::{
    CompressedKernelContext, ModelFieldValue, PartConfigurationArchive, PartResolvedFeaturesArchive,
};

fn native_equation_fields(source: &PartResolvedFeaturesArchive) -> Vec<usize> {
    source
        .fields()
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            let selected = field.path.contains(".splines[") && field.path.ends_with(".flags_32c");
            match field.value {
                ModelFieldValue::U32(flags) if selected && flags & 4 != 0 => Some(index),
                _ => None,
            }
        })
        .collect()
}

fn inspect_native_equation_case(
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
) -> usize {
    let (configuration, source) = native_equation_archives(case, kernel);
    let expected = native_equation_fields(&source);
    let states = source.spline_equation_states().unwrap();
    assert_eq!(
        states.len(),
        expected.len(),
        "{}; expected every physical equation flag",
        case.label
    );
    assert_eq!(configuration.to_bytes().unwrap(), case.configuration);
    assert_eq!(source.to_bytes().unwrap(), case.features);
    states.len()
}

fn native_equation_archives(
    case: &ResolvedCorpusCase<'_>,
    kernel: Option<CompressedKernelContext>,
) -> (PartConfigurationArchive, PartResolvedFeaturesArchive) {
    let configuration = configuration_for_case(case, kernel).unwrap();
    let source = PartResolvedFeaturesArchive::from_bytes(
        case.features,
        configuration.clone(),
        case.manager,
        LIMITS,
    )
    .unwrap();
    (configuration, source)
}

#[test]
#[ignore = "requires the retained906-case packet and explicit kernel schema; measures native equation population"]
fn native_equation_projection_preserves_every_retained_carrier_and_matches_physical_saved_fields() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(cursor.u32().unwrap(), 906);
    let mut equations = 0;
    for index in 0..906 {
        let count = inspect_native_equation_case(&next_case(&mut cursor), kernel.clone());
        equations += count;
        println!("EQUATION_NATIVE\t{index}\t{count}");
    }
    assert_eq!(cursor.remaining(), 0);
    assert_eq!(
        equations, 0,
        "frozen906-case corpus; expected the recorded zero equation population"
    );
    println!("EQUATION_NATIVE_TOTAL\t906\t{equations}");
}

#[test]
fn physical_equation_scan_detects_owned_positive_and_absent_equations() {
    let configuration = crate::model_archives::context_snapshot_tests::owned_configuration();
    let originals: [&[u8]; 2] = [
        include_bytes!("../fixtures/spline-equation-state/owned-0-0.bin"),
        include_bytes!("../fixtures/spline-equation-state/owned-0-6.bin"),
    ];
    for (raw, count) in originals.into_iter().zip([1, 0]) {
        let source = PartResolvedFeaturesArchive::from_bytes(
            raw,
            configuration.clone(),
            include_bytes!("../fixtures/sheet-metal/manager-0.bin"),
            LIMITS,
        )
        .unwrap();
        assert_eq!(native_equation_fields(&source).len(), count);
        assert_eq!(source.spline_equation_states().unwrap().len(), count);
    }
}
