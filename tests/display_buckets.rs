//! Independent modern bucket streams cover scoped dimensions, object identities and raw flags.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "../examples/support/bucket_trace.rs"]
mod bucket_trace;
#[allow(
    dead_code,
    reason = "Bucket traces reuse the item scalar/transform formatter; full item traces have dedicated tests."
)]
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[allow(
    dead_code,
    reason = "Shared trace helpers are exercised by view-specific tests."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;
const TWO: &[u8] = include_bytes!("fixtures/display-lists/bucket-2d.bin");
const THREE: &[u8] = include_bytes!("fixtures/display-lists/bucket-3d.bin");
const EMPTY: &[u8] = include_bytes!("fixtures/display-lists/bucket-empty.bin");
const EDITED: &[u8] = include_bytes!("fixtures/display-lists/bucket-edited.bin");
const ROOT: &str = "root.views[0].view_bucket";

fn parsed(bytes: &[u8]) -> DisplayArchive {
    DisplayArchive::from_bytes(bytes, context(), LIMITS).unwrap()
}

#[test]
fn both_coordinate_scopes_and_empty_buckets_match_independent_fields_and_bytes() {
    let fixtures: [(&[u8], &[u8]); 4] = [
        (TWO, include_bytes!("fixtures/display-lists/bucket-2d.txt")),
        (
            THREE,
            include_bytes!("fixtures/display-lists/bucket-3d.txt"),
        ),
        (
            EMPTY,
            include_bytes!("fixtures/display-lists/bucket-empty.txt"),
        ),
        (
            EDITED,
            include_bytes!("fixtures/display-lists/bucket-edited.txt"),
        ),
    ];
    for (bytes, expected) in fixtures {
        let archive = parsed(bytes);
        assert_eq!(
            String::from_utf8(bucket_trace::trace(&archive)).unwrap(),
            String::from_utf8(expected.to_vec()).unwrap()
        );
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), context(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );
    }
}

#[test]
fn scoped_dimensions_nonpositive_counts_and_raw_framing_are_preserved() {
    let archive = parsed(TWO);
    let table = "root.views[0].view_bucket.bucket.has_2d_info=i:-4\nroot.views[0].view_bucket.bucket.silh.count=i:4\nroot.views[0].view_bucket.bucket.silh.items[1].count=i:0\nroot.views[0].view_bucket.bucket.silh.items[2].count=i:-3\nroot.views[0].view_bucket.bucket.wire.items[0].type_data.count=i:-2\nroot.views[0].view_bucket.bucket.component_str_map.count_word=i:65535\nroot.views[0].view_bucket.bucket.component_str_map.count_dword=i:2\nroot.views[0].view_bucket.sub_buckets.count=i:2\n";
    assert_eq!(archive_support::check_scalar_table(&archive, table), 8);
    let mut contents = archive.contents().clone();
    for field in &mut contents.fields {
        if field.path.ends_with(".count")
            && matches!(field.value, DisplayFieldValue::I32(value) if value <= 0)
        {
            field.value = DisplayFieldValue::I32(i32::MIN);
        }
    }
    let changed = archive.with_contents(contents, LIMITS).unwrap();
    assert_eq!(bucket_trace::trace(&changed), bucket_trace::trace(&archive));
    for (label, count) in [
        ("silh.items[0]", 4),
        ("cthread_maps[0].entries[0].value.items[0]", 3),
        ("halo_repair.items[0]", 3),
    ] {
        let prefix = format!("{ROOT}.bucket.{label}.points[");
        assert_eq!(
            archive
                .contents()
                .fields
                .iter()
                .filter(|field| field.path.starts_with(&prefix))
                .count(),
            count
        );
    }
}

#[test]
fn field_edit_matches_independent_bytes_reopens_other_streams_and_reverts() {
    let archive = parsed(TWO);
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, &format!("{ROOT}.display_mode"), 23);
    let edited = archive.with_contents(contents, LIMITS).unwrap();
    assert_eq!(edited.to_bytes().unwrap(), EDITED);
    assert_eq!(
        edited
            .with_contents(archive.contents().clone(), LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        TWO
    );
    let original =
        bessemer::Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let mut document = original.clone();
    document
        .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
        .unwrap();
    let written = document.to_bytes().unwrap();
    assert_eq!(
        archive_support::decode_document(&written, DisplayDocumentKind::Part).contents(),
        edited.contents()
    );
    let reopened = bessemer::Document::from_bytes(&written).unwrap();
    for stream in original
        .streams()
        .filter(|stream| stream.name != "Contents/DisplayLists")
    {
        assert_eq!(
            original.read_stream(stream.name).unwrap(),
            reopened.read_stream(stream.name).unwrap()
        );
    }
}

#[test]
fn exact_global_budgets_cover_polygons_maps_objects_and_nested_buckets() {
    for (bytes, entries, objects) in [(TWO, 86, 45), (THREE, 91, 45), (EMPTY, 45, 43)] {
        let archive = parsed(bytes);
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: bytes.len(),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(bytes, context(), exact).is_ok());
        assert!(DisplayArchive::new(archive.contents().clone(), context(), exact).is_ok());
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
                maximum_bytes: bytes.len() - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(bytes, context(), limited).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limited)
                    .is_err()
            );
        }
    }
}

#[test]
fn impossible_counts_conditional_shape_changes_and_every_prefix_fail() {
    for bytes in [TWO, THREE, EMPTY] {
        for length in 3187..bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&bytes[..length], context(), LIMITS).is_err(),
                "length {length}"
            );
        }
    }
    let archive = parsed(TWO);
    for suffix in [
        "bucket.silh.count",
        "bucket.silh.items[0].count",
        "bucket.silh.items[0].type_data.count",
        "bucket.cthread_maps[0].count",
        "bucket.hatch_bucket_set_info.count",
        "sub_buckets.count",
    ] {
        let mut contents = archive.contents().clone();
        archive_support::set_integer(&mut contents, &format!("{ROOT}.{suffix}"), i32::MAX);
        assert!(archive.with_contents(contents, LIMITS).is_err(), "{suffix}");
    }
    for suffix in [
        "bucket.has_2d_info",
        "has_xform",
        "has_ghost_dl_xform",
        "has_original_view_bucket_in_detailing_mode",
    ] {
        let mut contents = archive.contents().clone();
        archive_support::set_integer(&mut contents, &format!("{ROOT}.{suffix}"), 0);
        assert!(archive.with_contents(contents, LIMITS).is_err(), "{suffix}");
    }
}

#[test]
fn self_aliases_and_generic_thread_keys_keep_identity_and_declared_roles_stay_strict() {
    let archive = parsed(TWO);
    let contents = archive.contents();
    let alias_path = format!("{ROOT}.sub_buckets[0]");
    let alias = contents
        .fields
        .iter()
        .find(|field| field.path.as_ref() == alias_path)
        .unwrap()
        .value
        .clone();
    let DisplayFieldValue::ObjectTag(tag) = &alias else {
        panic!("object tag")
    };
    assert_eq!(
        tag.target,
        DisplayObjectPointer::Reference(DisplayObjectId::new(47).unwrap())
    );
    let mut changed = contents.clone();
    let path = format!("{ROOT}.visual_properties");
    changed
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = alias.clone();
    assert!(archive.with_contents(changed, LIMITS).is_err());
    let thread = contents
        .fields
        .iter()
        .find(|field| {
            field.path.as_ref() == format!("{ROOT}.bucket.cthread_maps[1].entries[0].key")
        })
        .unwrap();
    assert_eq!(thread.value, alias);
}

#[test]
fn unknown_generic_thread_key_classes_remain_unsupported() {
    let archive = parsed(TWO);
    let mut contents = archive.contents().clone();
    let path = format!("{ROOT}.bucket.cthread_maps[0].entries[0].key");
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &mut field.value else {
        panic!("object tag")
    };
    tag.runtime_class.as_mut().unwrap().name = "UnknownThreadKey_c".into();
    tag.new_class.as_mut().unwrap().name = "UnknownThreadKey_c".into();
    let error = archive.with_contents(contents, LIMITS).unwrap_err();
    assert!(error.to_string().contains("Unsupported"));
    assert!(error.to_string().contains("UnknownThreadKey_c"));
}

#[test]
fn source_ownership_and_parallel_bucket_edits_are_isolated() {
    let mut source = TWO.to_vec();
    let archive = parsed(&source);
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    let mut contents = archive.contents().clone();
                    archive_support::set_integer(
                        &mut contents,
                        &format!("{ROOT}.display_mode"),
                        23,
                    );
                    archive
                        .with_contents(contents, LIMITS)
                        .unwrap()
                        .to_bytes()
                        .unwrap()
                })
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), EDITED);
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), TWO);
}
