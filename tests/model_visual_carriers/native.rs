use super::{LIMITS, configuration_for_case, next_case};
use crate::archive_codec::ArchiveCursor;
use crate::model_archives::{ModelField, ModelFieldValue, PartResolvedFeaturesArchive};

fn native_visual_prefix(path: &str) -> Option<&str> {
    path.strip_suffix(".unknown_0").filter(|prefix| {
        prefix.ends_with(".entity_visual_properties") || prefix.ends_with(".visual_properties")
    })
}

fn inspect_native_visual_fields(fields: &[ModelField]) -> usize {
    let mut count = 0;
    for (index, field) in fields.iter().enumerate() {
        let Some(prefix) = native_visual_prefix(&field.path) else {
            continue;
        };
        assert_native_visual_group(&fields[index..index + 7], prefix);
        count += 1;
    }
    count
}

fn assert_native_visual_group(fields: &[ModelField], prefix: &str) {
    let suffixes = [
        ".unknown_0",
        ".unknown_2",
        ".unknown_4",
        ".unknown_8",
        ".layer",
        ".line_width.standard_width",
        ".line_width.custom_width",
    ];
    for (field, suffix) in fields.iter().zip(suffixes) {
        assert_eq!(field.path.strip_prefix(prefix), Some(suffix));
    }
    assert!(matches!(fields[0].value, ModelFieldValue::U16(_)));
    assert!(matches!(fields[1].value, ModelFieldValue::U16(_)));
    assert!(matches!(fields[2].value, ModelFieldValue::U8(_)));
    assert!(matches!(fields[3].value, ModelFieldValue::U32(_)));
    assert!(matches!(fields[4].value, ModelFieldValue::U16(_)));
    assert!(matches!(fields[5].value, ModelFieldValue::I16(_)));
    assert!(matches!(fields[6].value, ModelFieldValue::F32(_)));
}

fn inspect_native_visual_case(
    case: &super::ResolvedCorpusCase<'_>,
    kernel: Option<crate::model_archives::CompressedKernelContext>,
) -> (usize, usize) {
    let source = configuration_for_case(case, kernel).unwrap();
    let configuration = inspect_native_visual_fields(source.fields());
    let features =
        PartResolvedFeaturesArchive::from_bytes(case.features, source, case.manager, LIMITS)
            .unwrap();
    assert_eq!(features.to_bytes().unwrap(), case.features);
    (
        configuration,
        inspect_native_visual_fields(features.fields()),
    )
}

#[test]
#[ignore = "requires the complete retained906-case packet and explicit kernel schema"]
fn native_visual_carriers_bind_all906_streams_to_original_physical_record_counts() {
    let bytes = std::fs::read("target/resolved-native-probe.bin").unwrap();
    let kernel = crate::model_archives::native_corpus_tests::corpus_kernel_context();
    let mut cursor = ArchiveCursor::new(&bytes);
    assert_eq!(cursor.take(4).unwrap(), b"BRF1");
    assert_eq!(cursor.u32().unwrap(), 906);
    let mut totals = [0; 2];
    for index in 0..906 {
        let (configuration, features) =
            inspect_native_visual_case(&next_case(&mut cursor), kernel.clone());
        totals[0] += configuration;
        totals[1] += features;
        println!("VISUAL_NATIVE\t{index}\t{configuration}\t{features}");
    }
    assert_eq!(cursor.remaining(), 0);
    assert_eq!(totals[1], 127773); // #181 / C-1959: original physical shared-codec reads.
    println!("VISUAL_NATIVE_TOTAL\t{}\t{}", totals[0], totals[1]);
}
