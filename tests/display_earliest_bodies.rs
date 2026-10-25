//! Old body-property pointers have no synthetic wrapper object or property flags.
use bessemer::display_lists::*;
#[allow(
    dead_code,
    reason = "Shared document helpers are exercised by other integration crates."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, archive_scalar_trace, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase;
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(
    dead_code,
    reason = "Shared view formatting is exercised by view-specific tests."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

fn cases() -> Vec<OldCase> {
    [
        include_bytes!("fixtures/display-lists/earliest-body-part.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-body-assembly.bin").as_slice(),
    ]
    .into_iter()
    .flat_map(complete_cases::decode)
    .collect()
}

fn case_context(case: OldCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..context()
    }
}

fn parsed(case: OldCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

fn selected(version: u32, kind: DisplayDocumentKind, scenario: u8) -> OldCase {
    cases()
        .into_iter()
        .find(|case| (case.version, case.kind, case.scenario) == (version, kind, scenario))
        .unwrap()
}

fn part_prefix(case: OldCase) -> &'static str {
    if case.kind == DisplayDocumentKind::Part {
        "root.tessellation"
    } else {
        "root.tessellation.components[0].comp_part_tess_data"
    }
}

fn has_field(archive: &DisplayArchive, path: &str) -> bool {
    archive
        .contents()
        .fields
        .iter()
        .any(|field| field.path.as_ref() == path)
}

fn object_target(archive: &DisplayArchive, path: &str) -> DisplayObjectPointer {
    let field = archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &field.value else {
        panic!("expected object tag at {path}")
    };
    tag.target.clone()
}

#[test]
fn all_body_property_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 26);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        let mut actual = material_trace::trace(&archive);
        actual.extend(scene_trace::trace(&archive));
        actual.extend(item_trace::trace(&archive));
        assert_eq!(
            std::str::from_utf8(&actual).unwrap(),
            std::str::from_utf8(case.expected).unwrap(),
            "{} {:?} {}",
            case.version,
            case.kind,
            case.scenario
        );
        fields += actual.iter().filter(|byte| **byte == b'\n').count()
            + archive_support::check_scalar_table(&archive, case.scalars);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(fields, 8303);
}

#[test]
fn direct_maps_preserve_duplicate_keys_aliases_nulls_and_the_absence_of_wrapper_objects() {
    for case in cases().into_iter().filter(|case| case.version < 0x10f6) {
        let archive = parsed(case);
        let body = format!("{}.bodies[0]", part_prefix(case));
        let pointer = format!("{body}.body_prop_infos[0].value");
        assert!(!has_field(&archive, &pointer));
        assert!(!has_field(&archive, &format!("{pointer}.unknown_8")));
        let direct = object_target(&archive, &format!("{pointer}.viz_prop"));
        if case.scenario == 226 {
            assert_eq!(direct, DisplayObjectPointer::Null);
            continue;
        }
        let DisplayObjectPointer::New(visual) = object_target(&archive, "root.visual_properties")
        else {
            panic!("expected loaded visual properties")
        };
        assert_eq!(direct, DisplayObjectPointer::Reference(visual));
        assert_eq!(
            object_target(
                &archive,
                &format!("{body}.body_prop_infos[1].value.viz_prop")
            ),
            DisplayObjectPointer::Null
        );
        let expected = format!(
            "{body}.body_prop_infos.count=i:2\n{body}.body_prop_infos[0].key=i:-1\n{body}.body_prop_infos[1].key=i:-1\n"
        );
        assert_eq!(archive_support::check_scalar_table(&archive, &expected), 3);
    }
}

#[test]
fn wrapped_maps_and_state_words_follow_their_individual_boundaries() {
    for case in cases().into_iter().filter(|case| case.scenario == 224) {
        let archive = parsed(case);
        let part = part_prefix(case);
        assert_eq!(
            has_field(
                &archive,
                &format!("{part}.bodies[0].body_prop_infos[0].value")
            ),
            case.version >= 0x10f6
        );
        assert_eq!(
            has_field(&archive, &format!("{part}.unknown_168")),
            case.version >= 0x10f0
        );
        assert_eq!(
            has_field(
                &archive,
                "root.views[0].display_items.items[0].data.ole_list_absent"
            ),
            case.version > 0x1115
        );
        if case.kind == DisplayDocumentKind::Assembly {
            assert_eq!(
                has_field(&archive, "root.tessellation.components[0].unknown_370"),
                case.version >= 0x10f0
            );
            assert_eq!(
                has_field(
                    &archive,
                    "root.tessellation.components[0].viewer_comp_data.is_virtual_comp"
                ),
                case.version > 0x110d
            );
        }
    }
}

#[test]
fn populated_ole_arrays_keep_unsigned_words_and_relocated_scene_aliases_keep_identity() {
    for case in cases().into_iter().filter(|case| case.scenario == 224) {
        let archive = parsed(case);
        if case.version > 0x1115 {
            let prefix = "root.views[0].display_items.items[0].data.ole_object_num_list";
            assert_eq!(
                archive_support::check_scalar_table(
                    &archive,
                    &format!("{prefix}.count=i:2\n{prefix}[0]=i:0\n{prefix}[1]=i:4294967295\n")
                ),
                3
            );
        }
        let DisplayObjectPointer::New(name) = object_target(&archive, "root.scene.concept.name")
        else {
            panic!("expected loaded scene name")
        };
        assert_eq!(
            object_target(&archive, "root.scene.floor_selection"),
            DisplayObjectPointer::Reference(name)
        );
        assert_eq!(
            object_target(&archive, "root.scene.concept.merged_feature_list.nodes[0]"),
            DisplayObjectPointer::Reference(name)
        );
    }
}

fn budgets() -> Vec<(usize, usize)> {
    use complete_cases::framing::{require_finished, take_u32};
    let mut bytes = include_bytes!("fixtures/display-lists/earliest-body-budgets.bin").as_slice();
    let count = take_u32(&mut bytes).unwrap();
    let rows = (0..count)
        .map(|_| {
            (
                take_u32(&mut bytes).unwrap() as usize,
                take_u32(&mut bytes).unwrap() as usize,
            )
        })
        .collect();
    require_finished(bytes).unwrap();
    rows
}

#[test]
fn exact_budgets_charge_real_map_entries_ole_words_and_loaded_wrapper_objects() {
    assert_eq!(budgets().len(), cases().len());
    for (case, (entries, objects)) in cases().into_iter().zip(budgets()) {
        let archive = parsed(case);
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(
            DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok(),
            "{} {:?} {}",
            case.version,
            case.kind,
            case.scenario
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
                maximum_bytes: case.bytes.len() - 1,
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
fn cross_version_construction_does_not_synthesize_wrappers_or_state() {
    for (before, after) in [
        (0x10ef, 0x10f0),
        (0x10f5, 0x10f6),
        (0x110d, 0x110e),
        (0x1115, 0x1116),
    ] {
        let old = selected(before, DisplayDocumentKind::Assembly, 224);
        let new = selected(after, DisplayDocumentKind::Assembly, 224);
        assert!(
            DisplayArchive::new(parsed(old).contents().clone(), case_context(new), LIMITS).is_err()
        );
        assert!(
            DisplayArchive::new(parsed(new).contents().clone(), case_context(old), LIMITS).is_err()
        );
    }
}

fn edited_ambient(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.visual_properties.material.ambient")
        .unwrap();
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn concurrent_direct_map_edits_match_independent_bytes_and_keep_owned_sources() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(4300, kind, 224);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), selected(4300, kind, 225).bytes);
            }
        });
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn every_new_layout_prefix_and_impossible_ole_count_is_rejected() {
    for version in [
        4300, 0x10ef, 0x10f0, 0x10f5, 0x10f6, 0x110d, 0x110e, 0x1115, 0x1116,
    ] {
        let case = selected(version, DisplayDocumentKind::Part, 224);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{version} prefix {end}"
            );
        }
    }
    let case = selected(0x1116, DisplayDocumentKind::Part, 224);
    let archive = parsed(case);
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| {
            field.path.as_ref()
                == "root.views[0].display_items.items[0].data.ole_object_num_list.count"
        })
        .unwrap();
    field.value = DisplayFieldValue::U32(u32::MAX);
    assert!(archive.with_contents(contents, LIMITS).is_err());
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[path = "support/tessellation.rs"]
mod tessellation_support;

fn document_archive(document: &bessemer::Document, kind: DisplayDocumentKind) -> DisplayArchive {
    let context = DisplayArchiveContext::from_document(
        document,
        kind,
        19000,
        tessellation_support::modern_context(),
        tessellation_support::LIMITS,
        100000,
    )
    .unwrap();
    assert_eq!(context.version.value(), 4300);
    DisplayArchive::from_bytes(
        &document.read_stream("Contents/DisplayLists").unwrap(),
        context,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn complete_documents_reopen_direct_map_edits_and_restore_authentic_files() {
    for (kind, source) in [
        (
            DisplayDocumentKind::Part,
            include_bytes!("fixtures/display-lists/earliest-body-document-part.bin").as_slice(),
        ),
        (
            DisplayDocumentKind::Assembly,
            include_bytes!("fixtures/display-lists/earliest-body-document-assembly.bin").as_slice(),
        ),
    ] {
        let mut document = bessemer::Document::from_bytes(source).unwrap();
        let archive = document_archive(&document, kind);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(4300, kind, 225).bytes);
        document
            .replace_stream("Contents/DisplayLists", edited.to_bytes().unwrap())
            .unwrap();
        let reopened = bessemer::Document::from_bytes(&document.to_bytes().unwrap()).unwrap();
        assert_eq!(
            document_archive(&reopened, kind).contents(),
            edited.contents()
        );
        assert_eq!(
            reopened.read_stream("Unrelated/Payload").unwrap(),
            b"retained\0payload"
        );
        document
            .replace_stream("Contents/DisplayLists", archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), source);
    }
}
