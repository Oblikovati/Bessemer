//! Native file context controls tangent lists independently of display-list version.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, archive_scalar_trace};
#[path = "support/file_context_cases.rs"]
mod file_cases;
use file_cases::FileContextCase;
#[path = "../examples/support/bucket_trace.rs"]
mod bucket_trace;
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
#[allow(dead_code, reason = "Shared view helpers have dedicated view tests.")]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;
const VIEW_BUCKET: &str = "root.views[0].view_bucket";

fn cases() -> Vec<FileContextCase> {
    [
        include_bytes!("fixtures/display-lists/file-context-9000.bin").as_slice(),
        include_bytes!("fixtures/display-lists/file-context-16000.bin").as_slice(),
        include_bytes!("fixtures/display-lists/file-context-19000.bin").as_slice(),
        include_bytes!("fixtures/display-lists/file-context-shaded.bin").as_slice(),
    ]
    .into_iter()
    .flat_map(file_cases::decode)
    .collect()
}

fn selected(version: u32, file_version: u32, scenario: u8) -> FileContextCase {
    cases()
        .into_iter()
        .find(|case| {
            case.version == version
                && case.file_version == file_version
                && case.scenario == scenario
                && case.kind == DisplayDocumentKind::Part
        })
        .unwrap()
}

fn case_context(case: FileContextCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        file_version: case.file_version,
        kind: case.kind,
        face_tessellation_components: Default::default(),
    }
}

fn parsed(case: FileContextCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

fn complete_trace(archive: &DisplayArchive) -> Vec<u8> {
    [
        reference_trace::trace(archive),
        scene_trace::trace(archive),
        item_trace::trace(archive),
        bucket_trace::trace(archive),
        light_trace::trace(archive),
        material_trace::trace(archive),
    ]
    .concat()
}

#[test]
fn independent_context_roots_reconstruct_all_fields_and_bytes() {
    assert_eq!(cases().len(), 86);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        assert_eq!(archive.context(), case_context(case));
        assert_eq!(
            complete_trace(&archive),
            case.expected,
            "{} {} {:?} {}",
            case.version,
            case.file_version,
            case.kind,
            case.scenario
        );
        fields += case.expected.iter().filter(|byte| **byte == b'\n').count();
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(fields, 23795);
}

#[test]
fn tangent_list_omission_propagates_to_child_buckets_and_preserves_coordinate_scope() {
    for case in cases()
        .into_iter()
        .filter(|case| case.scenario == 0 || case.scenario == 16)
    {
        let archive = parsed(case);
        for prefix in [VIEW_BUCKET, &format!("{VIEW_BUCKET}.sub_buckets[1]")] {
            for label in ["tan_v", "tan_h"] {
                let path = format!("{prefix}.bucket.{label}.count");
                assert_eq!(
                    archive
                        .contents()
                        .fields
                        .iter()
                        .any(|field| field.path.as_ref() == path),
                    case.file_version >= 0x224
                );
            }
        }
        if case.file_version < 0x224 {
            continue;
        }
        let prefix = format!("{VIEW_BUCKET}.bucket.tan_v.items[0]");
        let points = archive
            .contents()
            .fields
            .iter()
            .filter(|field| field.path.starts_with(&format!("{prefix}.points")))
            .count();
        assert_eq!(points, if case.scenario == 16 { 2 } else { 3 });
    }
}

#[test]
fn exact_context_budgets_charge_only_stored_tangent_polygons_and_coordinates() {
    for case in cases() {
        let archive = parsed(case);
        let exact = DisplayArchiveLimits {
            maximum_bytes: case.bytes.len(),
            maximum_fields: archive.contents().fields.len(),
            maximum_objects: case.objects,
            maximum_entries: case.entries,
            ..LIMITS
        };
        assert!(
            DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok(),
            "{} {} {:?} {}",
            case.version,
            case.file_version,
            case.kind,
            case.scenario
        );
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
        for limits in [
            DisplayArchiveLimits {
                maximum_bytes: exact.maximum_bytes - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_fields: exact.maximum_fields - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_objects: exact.maximum_objects - 1,
                ..exact
            },
            DisplayArchiveLimits {
                maximum_entries: exact.maximum_entries - 1,
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

fn edited_mode(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, &format!("{VIEW_BUCKET}.display_mode"), 23);
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn context_specific_edits_match_independent_bytes_and_keep_sources_owned() {
    for version in [9000, 16000, 19000] {
        for file_version in [0, 0x224] {
            let case = selected(version, file_version, 16);
            let mut source = case.bytes.to_vec();
            let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
            source.fill(0);
            std::thread::scope(|scope| {
                let workers: Vec<_> = (0..8)
                    .map(|_| scope.spawn(|| edited_mode(&archive).to_bytes().unwrap()))
                    .collect();
                for worker in workers {
                    assert_eq!(
                        worker.join().unwrap(),
                        selected(version, file_version, 17).bytes
                    );
                }
            });
            assert_eq!(archive.to_bytes().unwrap(), case.bytes);
        }
    }
}
#[test]
fn file_context_is_preserved_when_wire_is_equal_and_cannot_reinterpret_present_lists() {
    for file_version in [0, 0x223, 0x224, 18000, 18999, 19000, u32::MAX] {
        let case = selected(19000, file_version, 4);
        let archive = parsed(case);
        assert_eq!(archive.context().file_version, file_version);
        assert_eq!(
            archive.contents(),
            parsed(selected(19000, 19000, 4)).contents()
        );
    }
    for version in [9000, 16000, 19000] {
        for file_version in [0, 0x224] {
            let case = selected(version, file_version, 0);
            let archive = parsed(case);
            let other = DisplayArchiveContext {
                file_version: if file_version == 0 { 0x224 } else { 0 },
                ..case_context(case)
            };
            assert!(DisplayArchive::new(archive.contents().clone(), other, LIMITS).is_err());
            assert_eq!(archive.to_bytes().unwrap(), case.bytes);
        }
    }
}

#[test]
fn context_specific_roots_reject_prefixes_impossible_counts_and_trailing_bytes() {
    for (version, file_version) in [(9000, 0), (19000, 0x224)] {
        let case = selected(version, file_version, 0);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{version} {file_version} prefix {end}"
            );
        }
        let archive = parsed(case);
        let mut contents = archive.contents().clone();
        let suffix = if file_version == 0 { "silh" } else { "tan_v" };
        archive_support::set_integer(
            &mut contents,
            &format!("{VIEW_BUCKET}.bucket.{suffix}.count"),
            i32::MAX,
        );
        assert!(archive.with_contents(contents, LIMITS).is_err());
        let mut trailing = case.bytes.to_vec();
        trailing.push(0);
        assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn file_context_edits_reopen_preserve_other_streams_and_authentically_revert() {
    let initial = include_bytes!("fixtures/blank-part.SLDPRT");
    let native = archive_support::decode_document(initial, DisplayDocumentKind::Part);
    let original = bessemer::Document::from_bytes(initial).unwrap();
    for file_version in [0, 0x224] {
        let case = selected(19000, file_version, 16);
        let archive = parsed(case);
        let edited = edited_mode(&archive);
        assert_eq!(
            archive_support::check_scalar_table(
                &edited,
                &format!("{VIEW_BUCKET}.display_mode=i:23\n")
            ),
            1
        );
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
        assert_eq!(
            edited
                .with_contents(archive.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
}
