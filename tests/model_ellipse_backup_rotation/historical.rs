use super::*;
use bessemer::Document;
use bessemer::header::ArchiveVersion;

pub(super) fn native_ellipse_archive(
    path: &str,
    configuration: &str,
    version: u32,
    features: &[u8],
) -> PartResolvedFeaturesArchive {
    let mut document = Document::from_bytes(&std::fs::read(path).unwrap()).unwrap();
    document
        .replace_stream(
            &format!("{configuration}-ResolvedFeatures"),
            features.to_vec(),
        )
        .unwrap();
    native_ellipse_from_document(&document, configuration, version)
}

pub(super) fn native_ellipse_from_document(
    document: &Document,
    configuration: &str,
    version: u32,
) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::from_stream_name(configuration).unwrap(),
        ArchiveVersion::new(version),
        Some(native_kernel_context::native_test_kernel_context()),
        LIMITS,
    )
    .unwrap()
}

fn historical_ellipse_context(case: &EllipseCase) -> PartResolvedFeaturesArchive {
    if case.version == 19000 {
        return current_ellipse_archive(case);
    }
    let columns: Vec<_> = include_str!("../fixtures/bitmap-appearance/native-contexts.tsv")
        .lines()
        .find(|line| line.split('\t').next().unwrap().parse::<u32>().unwrap() == case.version)
        .unwrap()
        .split('\t')
        .collect();
    let source = native_ellipse_archive(columns[1], columns[2], case.version, case.raw);
    assert_eq!(source.object_slots().1, case.slots);
    assert_eq!(source.to_bytes().unwrap(), case.raw);
    source
}

#[test]
#[ignore = "Requires the two retained native contexts and installed kernel schema"]
fn all_supported_versions_project_edit_and_reverse_every_complete_owned_context() {
    let mut operations = 0;
    for case in ellipse_cases() {
        let source = historical_ellipse_context(&case);
        verify_ellipse_projection(&source, &case);
        contracts::verify_non_ellipse_handles(&source);
        operations += verify_ellipse_case_requests(&source, &case);
    }
    assert_eq!(operations, 1080);
}
