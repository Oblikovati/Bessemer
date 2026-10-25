//! High unsigned markers select documented later-version branches without a release-number ceiling.
use bessemer::display_lists::*;
#[allow(
    dead_code,
    reason = "Shared archive helpers also serve other integration crates."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, archive_scalar_trace, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase;
#[path = "../examples/support/bucket_trace.rs"]
mod bucket_trace;
#[path = "support/unsigned_documents.rs"]
mod documents;
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[path = "../examples/support/light_trace.rs"]
mod light_trace;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/reference_trace.rs"]
mod reference_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(
    dead_code,
    reason = "Shared trace formatting also serves view-specific integration crates."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

fn cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "fixtures/display-lists/unsigned-versions.bin"
    ))
}

fn case_context(case: OldCase) -> DisplayArchiveContext {
    let identities = if case.scenario >= 11 {
        vec!["component 鋼".into()]
    } else {
        Vec::new()
    };
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        face_tessellation_components: std::sync::Arc::new(identities.into_iter().collect()),
        ..context()
    }
}

fn trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut fields = reference_trace::trace(archive);
    fields.extend(scene_trace::trace(archive));
    fields.extend(item_trace::trace(archive));
    fields.extend(bucket_trace::trace(archive));
    fields.extend(light_trace::trace(archive));
    fields.extend(material_trace::trace(archive));
    fields
}

fn check_case(case: OldCase) -> usize {
    let archive = DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap();
    let actual = trace(&archive);
    compare_trace(&actual, case.expected, case);
    let fields = actual.iter().filter(|byte| **byte == b'\n').count()
        + archive_support::check_scalar_table(&archive, case.scalars);
    let forced =
        DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS).unwrap();
    assert_eq!(forced.to_bytes().unwrap(), case.bytes);
    fields
}

fn compare_trace(actual: &[u8], expected: &[u8], case: OldCase) {
    let actual = std::str::from_utf8(actual)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    let expected = std::str::from_utf8(expected)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(
            actual, expected,
            "{} {:?} {} line {index}",
            case.version, case.kind, case.scenario
        );
    }
    assert_eq!(actual.len(), expected.len());
}

#[test]
fn all_unsigned_profiles_match_independent_fields_and_complete_forced_bytes() {
    assert_eq!(cases().len(), 85);
    assert_eq!(cases().into_iter().map(check_case).sum::<usize>(), 31270);
}

#[test]
fn high_versions_keep_exact_field_shapes_under_the_same_documented_predicates() {
    for case in cases().into_iter().filter(|case| case.version > 19000) {
        let baseline = cases()
            .into_iter()
            .find(|baseline| baseline.version == 19000 && baseline.scenario == case.scenario)
            .unwrap();
        let actual = DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap();
        let expected =
            DisplayArchive::from_bytes(baseline.bytes, case_context(baseline), LIMITS).unwrap();
        assert_eq!(actual.contents(), expected.contents());
        let rebound =
            DisplayArchive::new(actual.contents().clone(), case_context(baseline), LIMITS).unwrap();
        assert_eq!(rebound.to_bytes().unwrap(), baseline.bytes);
    }
}

#[test]
fn signed_markers_normalize_before_branch_selection() {
    for (marker, expected) in [
        ("-1", u32::MAX),
        ("-2147483648", 0x80000000),
        ("2147483647", 0x7fffffff),
    ] {
        let name = format!("_DL_VERSION_{marker}/DLUpdateStamp");
        let version = DisplayVersion::from_stream_names([name.as_str()]).unwrap();
        assert_eq!(version.value(), expected);
        let mut selected = cases()
            .into_iter()
            .find(|case| case.version == expected && case.scenario == 1)
            .unwrap();
        selected.version = version.value();
        check_case(selected);
    }
}
