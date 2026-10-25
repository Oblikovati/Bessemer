//! Complete legacy bucket roots exercise float32 coordinates and serialized boolean flags.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "../examples/support/bucket_trace.rs"]
mod bucket_trace;
#[path = "../examples/support/framing.rs"]
mod framing;
#[allow(
    dead_code,
    reason = "Bucket traces reuse the scalar/transform formatter; item traces have dedicated tests."
)]
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[allow(dead_code, reason = "Shared view helpers have dedicated view tests.")]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

#[derive(Clone, Copy)]
struct BucketCase {
    version: u32,
    two_dimensional: bool,
    scenario: u8,
    bytes: &'static [u8],
    expected: &'static [u8],
}

fn bucket_blob(remaining: &mut &'static [u8]) -> &'static [u8] {
    let length = framing::take_u32(remaining).unwrap() as usize;
    framing::take(remaining, length).unwrap()
}

fn read_case(remaining: &mut &'static [u8]) -> BucketCase {
    let version = framing::take_u32(remaining).unwrap();
    let two_dimensional = framing::take(remaining, 1).unwrap()[0] != 0;
    let scenario = framing::take(remaining, 1).unwrap()[0];
    let bytes = bucket_blob(remaining);
    let expected = bucket_blob(remaining);
    BucketCase {
        version,
        two_dimensional,
        scenario,
        bytes,
        expected,
    }
}

fn cases() -> Vec<BucketCase> {
    let mut remaining: &'static [u8] = include_bytes!("fixtures/display-lists/legacy-buckets.bin");
    let count = framing::take_u32(&mut remaining).unwrap();
    framing::require_count(count, remaining.len(), 14).unwrap();
    let cases = (0..count).map(|_| read_case(&mut remaining)).collect();
    framing::require_finished(remaining).unwrap();
    cases
}

fn selected(version: u32, two: bool, scenario: u8) -> BucketCase {
    cases()
        .into_iter()
        .find(|case| {
            case.version == version && case.two_dimensional == two && case.scenario == scenario
        })
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

fn assert_trace(actual: &[u8], expected: &[u8]) {
    let actual = std::str::from_utf8(actual)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    let expected = std::str::from_utf8(expected)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(actual, expected, "trace row {index}");
    }
    assert_eq!(actual.len(), expected.len());
}

#[test]
fn all_supported_legacy_bucket_snapshots_match_independent_flags_fields_and_bytes() {
    assert_eq!(cases().len(), 10);
    for case in cases() {
        let archive = parsed(case);
        assert_trace(&bucket_trace::trace(&archive), case.expected);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
}

const ROOT: &str = "root.views[0].view_bucket";

#[test]
fn legacy_raw_flags_and_interpreted_flags_preserve_noncanonical_values() {
    for (version, flags) in [(7000, 838), (9000, 838), (10000, 966), (11000, 710)] {
        let archive = parsed(selected(version, false, 0));
        let trace = String::from_utf8(bucket_trace::trace(&archive)).unwrap();
        assert!(trace.contains(&format!("{ROOT}.bucket.silh.items[0].flags=i:{flags}\n")));
        assert!(trace.contains(&format!("{ROOT}.bucket.silh.items[1].flags=i:2\n")));
        assert!(trace.contains(&format!("{ROOT}.bucket.silh.items[2].flags=i:2\n")));
        let raw = "root.views[0].view_bucket.bucket.silh.items[0].smooth=i:-17\nroot.views[0].view_bucket.bucket.silh.items[0].new_boundary=i:-3\nroot.views[0].view_bucket.bucket.silh.items[0].new_region=i:0\nroot.views[0].view_bucket.bucket.silh.items[0].bos=i:6\n";
        assert_eq!(archive_support::check_scalar_table(&archive, raw), 4);
        let path = format!("{ROOT}.bucket.component_str_map[0].key");
        let value = &archive
            .contents()
            .fields
            .iter()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value;
        assert!(if version <= 0x23d7 {
            matches!(value, DisplayFieldValue::U16(65535))
        } else {
            matches!(value, DisplayFieldValue::I32(-7))
        });
    }
}

#[test]
fn coordinate_scopes_and_omitted_tails_match_old_layouts_without_default_fields() {
    for case in cases().into_iter().filter(|case| case.scenario == 0) {
        let archive = parsed(case);
        let fields = &archive.contents().fields;
        let prefix = format!("{ROOT}.bucket.silh.items[0].points_f32[");
        assert_eq!(
            fields
                .iter()
                .filter(|field| field.path.starts_with(&prefix))
                .count(),
            if case.two_dimensional { 4 } else { 6 }
        );
        let thread = format!("{ROOT}.bucket.cthread_maps[0].entries[0].value.items[0].points_f32[");
        assert_eq!(
            fields
                .iter()
                .filter(|field| field.path.starts_with(&thread))
                .count(),
            3
        );
        let halo = format!("{ROOT}.bucket.halo_repair.items[0].points_f32[");
        assert_eq!(
            fields
                .iter()
                .filter(|field| field.path.starts_with(&halo))
                .count(),
            3
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with(".hatch_bucket_set_info.count")),
            case.version == 11000
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with(".surf_cut_body")),
            case.version != 7000
        );
        for leaf in [
            "has_original_view_bucket_in_detailing_mode",
            "unknown_178",
            "bucket_data_for_detailing_mode_saved",
            "needs_post_process_bucket_for_detailing_mode",
            "edge_persistent_id",
        ] {
            assert!(
                !fields
                    .iter()
                    .any(|field| field.path.rsplit('.').next() == Some(leaf)),
                "{leaf}"
            );
        }
    }
    let archive = parsed(selected(7000, false, 0));
    let path = format!("{ROOT}.bucket.silh.items[0].points_f32[5]");
    let value = &archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value;
    let DisplayFieldValue::F32(value) = value else {
        panic!("stored float")
    };
    assert_eq!(value.bits(), 0x7f800042);
}

fn edited_smooth(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        &format!("{ROOT}.bucket.silh.items[0].smooth"),
        0,
    );
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn legacy_flag_edit_matches_independent_bytes_reopens_and_reverts_the_document() {
    let case = selected(10000, true, 0);
    let archive = parsed(case);
    let edited = edited_smooth(&archive);
    assert_eq!(edited.to_bytes().unwrap(), selected(10000, true, 1).bytes);
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
    let original = bessemer::Document::from_bytes(initial).unwrap();
    let mut document = original.clone();
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
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            reopened.read_stream(stream.name).unwrap()
        );
    }
    document
        .replace_stream("Contents/DisplayLists", native.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), initial);
}

#[test]
fn exact_budgets_include_legacy_coordinates_maps_and_object_bodies() {
    for case in cases() {
        let archive = parsed(case);
        let prefix = match case.version {
            7000 => 52,
            9000 => 64,
            _ => 51,
        };
        let extra = if case.scenario == 2 {
            0
        } else {
            28 + 5 * if case.two_dimensional { 2 } else { 3 }
                + usize::from(case.version == 11000) * 2
        };
        let entries = prefix + extra;
        let objects = if case.scenario == 2 { 43 } else { 45 };
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok());
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
        for limited in [
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
            assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), limited).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limited)
                    .is_err()
            );
        }
    }
}

#[test]
fn legacy_truncations_invalid_counts_and_changed_coordinate_dimensions_are_rejected() {
    let case = selected(10000, true, 0);
    let archive = parsed(case);
    for end in 0..case.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
            "prefix {end}"
        );
    }
    for suffix in [
        "bucket.silh.count",
        "bucket.silh.items[0].count",
        "bucket.silh.items[0].type_data.count",
        "bucket.cthread_maps[0].count",
        "sub_buckets.count",
    ] {
        let mut contents = archive.contents().clone();
        archive_support::set_integer(&mut contents, &format!("{ROOT}.{suffix}"), i32::MAX);
        assert!(archive.with_contents(contents, LIMITS).is_err(), "{suffix}");
    }
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, &format!("{ROOT}.bucket.has_2d_info"), 0);
    assert!(archive.with_contents(contents, LIMITS).is_err());
}

#[test]
fn legacy_aliases_and_nonpositive_counts_keep_identity_and_empty_semantics() {
    let archive = parsed(selected(7000, false, 0));
    let alias = format!("{ROOT}.sub_buckets[0]");
    let value = &archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == alias)
        .unwrap()
        .value;
    let DisplayFieldValue::ObjectTag(tag) = value else {
        panic!("object tag")
    };
    assert_eq!(
        tag.target,
        DisplayObjectPointer::Reference(DisplayObjectId::new(47).unwrap())
    );
    let mut contents = archive.contents().clone();
    for field in &mut contents.fields {
        if field.path.ends_with(".count")
            && matches!(field.value,DisplayFieldValue::I32(count) if count<=0)
        {
            field.value = DisplayFieldValue::I32(i32::MIN);
        }
    }
    let changed = archive.with_contents(contents, LIMITS).unwrap();
    assert_eq!(bucket_trace::trace(&changed), bucket_trace::trace(&archive));
    assert!(
        parsed(selected(7000, false, 2))
            .contents()
            .fields
            .iter()
            .all(|field| !field.path.contains(".silh.items["))
    );
}

#[test]
fn legacy_results_own_sources_and_parallel_edits_match_the_independent_wire_result() {
    let case = selected(10000, true, 0);
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_smooth(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), selected(10000, true, 1).bytes);
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
