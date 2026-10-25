//! Separate polygon flags and double coordinates persist until the packed-word transition.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase as BucketCase;
#[path = "../examples/support/bucket_trace.rs"]
mod bucket_trace;
#[allow(
    dead_code,
    reason = "Bucket traces reuse the scalar/transform formatter; item traces have dedicated tests."
)]
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[allow(dead_code, reason = "Shared view helpers have dedicated view tests.")]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

const ROOT: &str = "root.views[0].view_bucket";

fn cases() -> Vec<BucketCase> {
    let mut cases =
        complete_cases::decode(include_bytes!("fixtures/display-lists/middle-buckets.bin"));
    cases.extend(complete_cases::decode(include_bytes!(
        "fixtures/display-lists/lower-buckets.bin"
    )));
    cases
}

fn selected(version: u32, scenario: u8) -> BucketCase {
    cases()
        .into_iter()
        .find(|case| case.version == version && case.scenario == scenario)
        .unwrap()
}

fn case_context(case: BucketCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        ..context()
    }
}

fn parsed(case: BucketCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

#[test]
fn both_coordinate_scopes_match_independent_fields_on_each_side_of_the_flag_transition() {
    assert_eq!(cases().len(), 30);
    let mut fields = 0;
    for case in cases() {
        assert!(case.scalars.is_empty());
        assert_eq!(case.kind, DisplayDocumentKind::Part);
        let archive = parsed(case);
        assert_eq!(
            bucket_trace::trace(&archive),
            case.expected,
            "{} {}",
            case.version,
            case.scenario
        );
        fields += case
            .expected
            .iter()
            .filter(|value| **value == b'\n')
            .count();
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(fields, 8814);
}

#[test]
fn raw_boolean_words_double_coordinates_and_derived_flags_remain_distinct_from_packed_words() {
    for case in cases().into_iter().filter(|case| case.scenario % 16 == 0) {
        let archive = parsed(case);
        let prefix = format!("{ROOT}.bucket.silh.items[0]");
        let has_smooth = archive
            .contents()
            .fields
            .iter()
            .any(|field| field.path.as_ref() == format!("{prefix}.smooth"));
        assert_eq!(has_smooth, case.version <= 0x41a0);
        assert!(
            archive
                .contents()
                .fields
                .iter()
                .any(|field| field.path.as_ref() == format!("{prefix}.points[0]")
                    && matches!(field.value, DisplayFieldValue::F64(_)))
        );
        if case.version <= 0x41a0 {
            let expected = format!(
                "{prefix}.smooth=i:-17\n{prefix}.has_arc_data=i:-3\n{prefix}.has_silh_data=i:-2\n"
            );
            assert_eq!(archive_support::check_scalar_table(&archive, &expected), 3);
            assert!(
                String::from_utf8(bucket_trace::trace(&archive))
                    .unwrap()
                    .contains(&format!("{prefix}.flags=i:758\n"))
            );
        }
    }
}

fn edited_mode(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, &format!("{ROOT}.display_mode"), 23);
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independent_bucket_edit_reopens_with_explicit_context_and_reverts() {
    for version in [14000, 16000] {
        let case = selected(version, 16);
        let archive = parsed(case);
        let edited = edited_mode(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(version, 17).bytes);
        assert_eq!(
            edited
                .with_contents(archive.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
        let initial = include_bytes!("fixtures/blank-part.SLDPRT");
        let native = archive_support::decode_document(initial, DisplayDocumentKind::Part);
        let mut document = bessemer::Document::from_bytes(initial).unwrap();
        document
            .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
            .unwrap();
        let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
        assert_eq!(
            DisplayArchive::from_bytes(
                &reopened.read_stream("Contents/DisplayLists").unwrap(),
                case_context(case),
                LIMITS
            )
            .unwrap()
            .contents(),
            edited.contents()
        );
        document
            .replace_stream("Contents/DisplayLists", native.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), initial);
    }
}

#[test]
fn exact_budgets_charge_double_coordinates_arc_payloads_and_shared_object_graphs() {
    for case in cases() {
        let archive = parsed(case);
        let empty = case.scenario % 16 == 2;
        let entries = if empty {
            45
        } else if case.scenario >= 16 {
            86
        } else {
            91
        };
        let objects = if empty { 43 } else { 45 };
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok());
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
        for limits in [
            DisplayArchiveLimits {
                maximum_entries: entries - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_objects: objects - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_fields: exact.maximum_fields - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_bytes: exact.maximum_bytes - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), limits).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limits)
                    .is_err()
            );
        }
    }
}

#[test]
fn malformed_coordinate_counts_and_arc_shape_changes_fail_without_mutating_inputs() {
    let case = selected(16000, 16);
    let archive = parsed(case);
    for (suffix, value) in [
        ("count", i32::MAX),
        ("has_arc_data", 0),
        ("has_user_line_property", 0),
    ] {
        let mut contents = archive.contents().clone();
        let path = format!("{ROOT}.bucket.silh.items[0].{suffix}");
        if suffix == "has_user_line_property" {
            let field = contents
                .fields
                .iter_mut()
                .find(|field| field.path.as_ref() == path)
                .unwrap();
            field.value = DisplayFieldValue::U8(0);
        } else {
            archive_support::set_integer(&mut contents, &path, value);
        }
        assert!(archive.with_contents(contents, LIMITS).is_err());
    }
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn every_populated_prefix_fails_and_owned_parallel_edits_match_the_independent_result() {
    for version in [14000, 16000] {
        let case = selected(version, 16);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "prefix {end}"
            );
        }
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_mode(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), selected(version, 17).bytes);
            }
        });
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn lower_detailing_and_arc_tail_fields_follow_their_distinct_inclusive_boundaries() {
    for case in cases() {
        let archive = parsed(case);
        let fields = &archive.contents().fields;
        let has = |path: &str| fields.iter().any(|field| field.path.as_ref() == path);
        assert_eq!(has(&format!("{ROOT}.unknown_178")), case.version >= 0x374b);
        assert_eq!(
            has(&format!(
                "{ROOT}.bucket.needs_post_process_bucket_for_detailing_mode"
            )),
            case.version >= 0x372a
        );
        assert!(has(&format!("{ROOT}.bucket.dl_model_bucket.count_word")));
        assert_eq!(
            has(&format!("{ROOT}.bucket.silh.items[0].arc_data.unknown_6c")),
            case.scenario % 16 != 2 && case.version >= 0x3724
        );
    }
}
