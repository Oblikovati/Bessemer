//! Versioned decal bodies use existing appearance codecs and preserve unsigned count framing.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase as DecalCase;

fn cases() -> Vec<DecalCase> {
    let mut cases = complete_cases::decode(include_bytes!("fixtures/display-lists/decals.bin"));
    cases.extend(complete_cases::decode(include_bytes!(
        "fixtures/display-lists/early-decal-transition.bin"
    )));
    cases
}

fn selected(scenario: u8) -> DecalCase {
    cases()
        .into_iter()
        .find(|case| {
            case.version == 19000
                && case.kind == DisplayDocumentKind::Part
                && case.scenario == scenario
        })
        .unwrap()
}

fn case_context(case: DecalCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..context()
    }
}

fn parsed(case: DecalCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

#[test]
fn all_versioned_decal_lists_and_maps_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 18);
    let mut compared = 0;
    for case in cases() {
        assert!(case.expected.is_empty());
        let archive = parsed(case);
        compared += archive_support::check_scalar_table(&archive, case.scalars);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(compared, 9436);
}

fn first_decal(archive: &DisplayArchive) -> String {
    archive
        .contents()
        .fields
        .iter()
        .find_map(|field| field.path.strip_suffix(".mask_invert").map(str::to_owned))
        .unwrap()
}

#[test]
fn raw_flags_palette_ids_mirror_omission_duplicate_map_keys_and_empty_collections_are_preserved() {
    for case in cases().into_iter().filter(|case| case.scenario == 0) {
        let archive = parsed(case);
        let prefix = first_decal(&archive);
        let expected = format!(
            "{prefix}.unknown_0=i:255\n{prefix}.mask_type=i:254\n{prefix}.mask_invert=i:-17\n{prefix}.color_mask[0]=i:4294967295\n{prefix}.decal_faces[0].face_id=i:4294967295\n{prefix}.decal_faces[0].unknown_1c[3]=i:-11\n"
        );
        assert_eq!(archive_support::check_scalar_table(&archive, &expected), 6);
        let mirror = format!("{prefix}.mirror_offset_x");
        assert_eq!(
            archive
                .contents()
                .fields
                .iter()
                .any(|field| field.path.as_ref() == mirror),
            case.version > 0x1bbe
        );
        if case.version >= 10000 && case.kind == DisplayDocumentKind::Part {
            let expected = "root.tessellation.decals_map[0].key=i:-7\nroot.tessellation.decals_map[1].key=i:-7\nroot.tessellation.decals_map[1].value.count=i:0\n";
            assert_eq!(archive_support::check_scalar_table(&archive, expected), 3);
        }
    }
    let empty = parsed(selected(2));
    assert!(
        empty
            .contents()
            .fields
            .iter()
            .all(|field| !field.path.ends_with(".color_mask[0]")
                && !field.path.contains(".decal_faces[0]."))
    );
    assert!(
        parsed(selected(3))
            .contents()
            .fields
            .iter()
            .all(|field| !field.path.ends_with(".mask_invert"))
    );
}

fn edited_mask(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        &format!("{}.mask_invert", first_decal(archive)),
        23,
    );
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independently_encoded_mask_edit_reopens_preserves_other_streams_and_reverts() {
    let case = selected(0);
    let archive = parsed(case);
    let edited = edited_mask(&archive);
    assert_eq!(edited.to_bytes().unwrap(), selected(1).bytes);
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
        archive_support::decode_document(&reopened.to_bytes().unwrap(), DisplayDocumentKind::Part)
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

fn budget(case: DecalCase) -> (usize, usize) {
    let assembly = case.kind == DisplayDocumentKind::Assembly;
    match (case.version, case.scenario) {
        (0x239c | 0x239d, _) => (
            if assembly { 155 } else { 144 } + 2 * usize::from(case.version >= 0x239d),
            if assembly { 54 } else { 52 },
        ),
        (7000, _) => {
            if assembly {
                (126, 54)
            } else {
                (108, 52)
            }
        }
        (9000, _) => {
            if assembly {
                (138, 54)
            } else {
                (120, 52)
            }
        }
        (10000, _) => {
            if assembly {
                (127, 54)
            } else {
                (109, 52)
            }
        }
        (11000, _) => {
            if assembly {
                (131, 54)
            } else {
                (113, 52)
            }
        }
        (18000, _) => (80, 49),
        (_, 2) => (140, 49),
        (_, 3) => (134, 49),
        _ => {
            if assembly {
                (63, 46)
            } else {
                (156, 49)
            }
        }
    }
}

#[test]
fn exact_budgets_charge_each_decal_palette_face_and_map_entry_without_new_objects() {
    for case in cases() {
        let archive = parsed(case);
        let (entries, objects) = budget(case);
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(
            DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok(),
            "{} {:?}",
            case.version,
            case.kind
        );
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
fn impossible_unsigned_counts_negative_face_counts_and_every_prefix_fail_closed() {
    let case = selected(0);
    let archive = parsed(case);
    let prefix = first_decal(&archive);
    for path in [
        format!("{prefix}.color_mask.count"),
        "root.tessellation.decals_map.count".to_owned(),
        "root.tessellation.bodies[0].decals.count".to_owned(),
    ] {
        let mut contents = archive.contents().clone();
        let field = contents
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap();
        assert!(matches!(field.value, DisplayFieldValue::U32(_)));
        field.value = DisplayFieldValue::U32(u32::MAX);
        assert!(archive.with_contents(contents, LIMITS).is_err());
    }
    for value in [-1, i32::MAX] {
        let mut contents = archive.contents().clone();
        archive_support::set_integer(&mut contents, &format!("{prefix}.decal_faces.count"), value);
        assert!(archive.with_contents(contents, LIMITS).is_err());
    }
    let minimal = selected(2);
    for end in 0..minimal.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&minimal.bytes[..end], case_context(minimal), LIMITS)
                .is_err(),
            "prefix {end}"
        );
    }
    let mut trailing = case.bytes.to_vec();
    trailing.push(0);
    assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
}

#[test]
fn published_decal_archives_own_sources_and_concurrent_edits_match_independent_bytes() {
    let case = selected(0);
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_mask(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), selected(1).bytes);
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
#[test]
fn populated_part_decals_switch_from_a_list_to_a_map_at_the_exact_boundary() {
    for case in cases()
        .into_iter()
        .filter(|case| matches!(case.version, 0x239c | 0x239d))
    {
        let archive = parsed(case);
        let part = if case.kind == DisplayDocumentKind::Part {
            "root.tessellation"
        } else {
            "root.tessellation.components[0].comp_part_tess_data"
        };
        let contains = |suffix: &str| {
            archive
                .contents()
                .fields
                .iter()
                .any(|field| field.path.as_ref() == format!("{part}.{suffix}"))
        };
        assert_eq!(contains("decals.count"), case.version < 0x239d);
        assert_eq!(contains("decals_map[0].key"), case.version >= 0x239d);
        if case.version >= 0x239d {
            assert_eq!(
                archive_support::check_scalar_table(
                    &archive,
                    &format!("{part}.decals_map[0].key=i:-7\n{part}.decals_map[1].key=i:-7\n")
                ),
                2
            );
        }
    }
}
