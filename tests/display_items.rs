//! Independent modern display-item fixtures exercise payload variants inside complete streams.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[allow(
    dead_code,
    reason = "Shared view helpers are exercised in view-specific tests."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;
const PRESENT: &[u8] = include_bytes!("fixtures/display-lists/items-present.bin");
const ABSENT: &[u8] = include_bytes!("fixtures/display-lists/items-absent.bin");
const EDITED: &[u8] = include_bytes!("fixtures/display-lists/items-edited.bin");
const PREFIX: &str = "root.views[0].display_items.items";

fn parsed(bytes: &[u8]) -> DisplayArchive {
    DisplayArchive::from_bytes(bytes, context(), LIMITS).unwrap()
}

#[test]
fn every_modern_item_payload_matches_independent_fields_and_forced_rebuild() {
    let fixtures: [(&[u8], &[u8]); 3] = [
        (
            PRESENT,
            include_bytes!("fixtures/display-lists/items-present.txt"),
        ),
        (
            ABSENT,
            include_bytes!("fixtures/display-lists/items-absent.txt"),
        ),
        (
            EDITED,
            include_bytes!("fixtures/display-lists/items-edited.txt"),
        ),
    ];
    for (bytes, expected) in fixtures {
        let archive = parsed(bytes);
        assert_eq!(
            String::from_utf8(item_trace::trace(&archive)).unwrap(),
            String::from_utf8(expected.to_vec()).unwrap()
        );
        let rebuilt = DisplayArchive::new(archive.contents().clone(), context(), LIMITS).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), bytes);
    }
}

#[test]
fn edits_match_independent_wire_oracle_preserve_other_streams_and_revert() {
    let archive = parsed(PRESENT);
    let original = archive.contents().clone();
    let mut changed = original.clone();
    archive_support::set_integer(&mut changed, &format!("{PREFIX}[0].data.point_style"), 23);
    let edited = archive.with_contents(changed, LIMITS).unwrap();
    assert_eq!(edited.to_bytes().unwrap(), EDITED);
    assert_eq!(archive.to_bytes().unwrap(), PRESENT);
    assert_eq!(
        edited
            .with_contents(original, LIMITS)
            .unwrap()
            .to_bytes()
            .unwrap(),
        PRESENT
    );
    let document =
        bessemer::Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    let mut replacement = document.clone();
    replacement
        .replace_stream("Contents/DisplayLists", EDITED.to_vec())
        .unwrap();
    assert_eq!(
        archive_support::decode_document(
            &replacement.to_bytes().unwrap(),
            DisplayDocumentKind::Part
        )
        .contents(),
        parsed(EDITED).contents()
    );
    for entry in document.streams() {
        if entry.name == "Contents/DisplayLists" {
            continue;
        }
        assert_eq!(
            document.read_stream(entry.name).unwrap(),
            replacement.read_stream(entry.name).unwrap()
        );
    }
}

#[test]
fn count_layouts_raw_flags_and_payload_free_unknown_types_are_retained() {
    let archive = parsed(PRESENT);
    let expected = "root.views[0].display_items.count_word=i:65535\nroot.views[0].display_items.count_dword=i:48\nroot.views[0].display_items.items[0].size_of_element=i:4275878552\nroot.views[0].display_items.items[6].data.has_widths=i:-2\nroot.views[0].display_items.items[46].type=i:-1\nroot.views[0].display_items.items[47].type=i:2147483647\n";
    assert_eq!(archive_support::check_scalar_table(&archive, expected), 6);
    for field in &archive.contents().fields {
        if !field.path.ends_with(".size_of_element") {
            continue;
        }
        let mut changed = archive.contents().clone();
        changed
            .fields
            .iter_mut()
            .find(|candidate| candidate.path == field.path)
            .unwrap()
            .value = DisplayFieldValue::U32(0);
        assert!(archive.with_contents(changed, LIMITS).is_ok());
    }
}

#[test]
fn item_counts_conditionals_and_widths_reject_invalid_shapes() {
    let archive = parsed(PRESENT);
    for (suffix, value) in [
        ("[4].data.points.count", -1),
        ("[5].data.points.count", i32::MAX),
        ("[15].data.width_count", -1),
        ("[15].data.width_count", (1 << 24) + 1),
        ("[16].data.items.count", -1),
        ("[22].data.edw_annot_data.count", i32::MAX),
        ("[24].data.points.count", -1),
        ("[6].data.has_widths", 0),
    ] {
        let mut changed = archive.contents().clone();
        let path = format!("{PREFIX}{suffix}");
        let field = changed
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap();
        field.value = match field.value {
            DisplayFieldValue::I16(_) => DisplayFieldValue::I16(value as i16),
            _ => DisplayFieldValue::I32(value),
        };
        assert!(
            archive.with_contents(changed, LIMITS).is_err(),
            "{path}={value}"
        );
    }
}

#[test]
fn prefixes_and_exact_entry_byte_field_budgets_are_enforced() {
    for (bytes, end, entries) in [(PRESENT, 6388, 125), (ABSENT, 5964, 120)] {
        for length in 3187..end {
            assert!(
                DisplayArchive::from_bytes(&bytes[..length], context(), LIMITS).is_err(),
                "prefix {length}"
            );
        }
        let archive = parsed(bytes);
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_bytes: bytes.len(),
            maximum_fields: archive.contents().fields.len(),
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
                maximum_bytes: bytes.len() - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_fields: exact.maximum_fields - 1,
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
fn source_mutation_and_concurrent_edits_are_isolated() {
    let mut source = PRESENT.to_vec();
    let archive = parsed(&source);
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    let mut changed = archive.contents().clone();
                    archive_support::set_integer(
                        &mut changed,
                        &format!("{PREFIX}[0].data.point_style"),
                        23,
                    );
                    archive
                        .with_contents(changed, LIMITS)
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
    assert_eq!(archive.to_bytes().unwrap(), PRESENT);
}
