//! Earlier bucket layouts retain discarded pointers and explicit coordinate widths.
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

const BUCKET_ROOT_GROUPS: &[&[u8]] = &[
    include_bytes!("fixtures/display-lists/earlier-buckets-3d.bin"),
    include_bytes!("fixtures/display-lists/earlier-buckets-2d.bin"),
    include_bytes!("fixtures/display-lists/precollector-buckets-3d.bin"),
    include_bytes!("fixtures/display-lists/precollector-buckets-2d.bin"),
    include_bytes!("fixtures/display-lists/component-buckets-3d.bin"),
    include_bytes!("fixtures/display-lists/component-buckets-2d.bin"),
    include_bytes!("fixtures/display-lists/grouped-buckets-3d.bin"),
    include_bytes!("fixtures/display-lists/grouped-buckets-2d.bin"),
];

fn cases() -> Vec<BucketCase> {
    let groups = BUCKET_ROOT_GROUPS.iter();
    groups
        .flat_map(|bytes| complete_cases::decode(bytes))
        .collect()
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
fn earlier_buckets_match_independent_fields_and_exact_bytes() {
    assert_eq!(cases().len(), 230);
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
    assert_eq!(fields, 47909);
}

fn edited_mode(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, &format!("{ROOT}.display_mode"), 23);
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independent_bucket_edit_reopens_with_explicit_context_and_reverts() {
    let version = 12000;
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

#[test]
fn exact_budgets_charge_earlier_coordinates_and_discarded_objects() {
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
        let entries = entries
            - usize::from(!empty && case.version <= 0x2faa)
            - 2 * usize::from(!empty && case.version <= 0x2789);
        let objects = if empty {
            43
        } else {
            45 + usize::from((0x2f63..=0x2f95).contains(&case.version))
        };
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
    let case = selected(0x2f96, 16);
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
    let version = 12000;
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

#[test]
fn earlier_coordinate_and_optional_tail_boundaries_are_explicit() {
    for case in cases() {
        let archive = parsed(case);
        let has = |suffix: &str| {
            archive
                .contents()
                .fields
                .iter()
                .any(|field| field.path.as_ref() == format!("{ROOT}.{suffix}"))
        };
        assert_eq!(
            has("bucket.bucket_data_for_detailing_mode_saved"),
            case.version > 0x2f8b
        );
        assert_eq!(
            has("bucket.dl_model_bucket.count_word"),
            case.version > 0x2faa
        );
        if case.scenario % 16 == 2 {
            continue;
        }
        let coordinate = archive
            .contents()
            .fields
            .iter()
            .find(|field| {
                field.path.as_ref()
                    == format!(
                        "{ROOT}.bucket.silh.items[0].{}[0]",
                        if case.version < 0x2f9a {
                            "points_f32"
                        } else {
                            "points"
                        }
                    )
            })
            .unwrap();
        assert_eq!(
            matches!(coordinate.value, DisplayFieldValue::F32(_)),
            case.version < 0x2f9a
        );
        assert_eq!(
            has("bucket.silh.items[0].discarded_edge_data"),
            (0x2f63..=0x2f95).contains(&case.version)
        );
        assert_eq!(
            has("bucket.silh.items[0].arc_data.unknown_68"),
            case.version > 0x2fa9
        );
        assert_eq!(
            has("bucket.cthread_maps[0].entries[0].key.linear_in_2d"),
            case.version > 0x2fac
        );
    }
}

#[test]
fn discarded_edge_body_and_aliases_remain_owned_and_role_checked() {
    let case = selected(0x2f63, 0);
    let archive = parsed(case);
    let path = format!("{ROOT}.bucket.silh.items[0].discarded_edge_data");
    assert!(
        archive
            .contents()
            .fields
            .iter()
            .any(|field| field.path.as_ref() == format!("{path}.start_point[0]"))
    );
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &mut field.value else {
        panic!("expected object tag")
    };
    tag.runtime_class.as_mut().unwrap().name = "uiViewBucket_c".into();
    assert!(archive.with_contents(contents, LIMITS).is_err());
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    assert_eq!(
        archive_support::check_scalar_table(
            &archive,
            &format!("{ROOT}.bucket.silh.items[0].smooth=i:-17\n")
        ),
        1
    );
}

#[test]
fn discarded_edge_alias_cannot_target_a_view_bucket() {
    let case = selected(0x2f95, 0);
    let archive = parsed(case);
    let replacement = archive
        .contents()
        .fields
        .iter()
        .find_map(|field| match &field.value {
            DisplayFieldValue::ObjectTag(tag)
                if matches!(tag.target, DisplayObjectPointer::Reference(_))
                    && tag
                        .runtime_class
                        .as_ref()
                        .is_some_and(|class| class.name == "uiViewBucket_c") =>
            {
                Some(tag.clone())
            }
            _ => None,
        })
        .unwrap();
    let mut contents = archive.contents().clone();
    let path = format!("{ROOT}.bucket.silh.items[3].discarded_edge_data");
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    field.value = DisplayFieldValue::ObjectTag(replacement);
    let error = archive.with_contents(contents, LIMITS).unwrap_err();
    assert!(error.to_string().contains("gcBucketEdgeData_c"), "{error}");
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
#[test]
fn word_component_keys_and_missing_section_flags_keep_their_wire_types() {
    for case in cases().into_iter().filter(|case| case.scenario % 16 == 0) {
        let archive = parsed(case);
        let fields = &archive.contents().fields;
        let key = fields
            .iter()
            .find(|field| field.path.as_ref() == format!("{ROOT}.bucket.component_str_map[0].key"))
            .unwrap();
        assert_eq!(
            key.value,
            if case.version <= 0x23d7 {
                DisplayFieldValue::U16(65529)
            } else {
                DisplayFieldValue::I32(-7)
            }
        );
        assert_eq!(
            fields.iter().any(|field| field.path.as_ref()
                == format!("{ROOT}.bucket.silh.items[0].section_cut_new")),
            case.version >= 0x23b4
        );
    }
    let case = selected(0x23d7, 0);
    let archive = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(0x23d8),
        ..case_context(case)
    };
    assert!(DisplayArchive::new(archive.contents().clone(), context, LIMITS).is_err());
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
