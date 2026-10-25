//! Old walls retain null material pointers; unresolved non-null bodies never publish archives.
use bessemer::FailureClass;
use bessemer::display_lists::*;
#[allow(
    dead_code,
    reason = "Shared document helpers are also used by other integration crates."
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
    reason = "Shared view formatting is tested separately through view fixtures."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

fn cases() -> Vec<OldCase> {
    [
        include_bytes!("fixtures/display-lists/earliest-wall-part.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-wall-assembly.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-wall-properties.bin").as_slice(),
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

fn has_field(archive: &DisplayArchive, suffix: &str) -> bool {
    archive
        .contents()
        .fields
        .iter()
        .any(|field| field.path.ends_with(suffix))
}

#[test]
fn all_complete_old_wall_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 42);
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
    assert_eq!(fields, 15404);
}

#[test]
fn all_three_wall_kinds_keep_old_null_pointers_and_new_visual_properties_distinct() {
    for case in cases().into_iter().filter(|case| case.scenario == 192) {
        let archive = parsed(case);
        for index in 0..3 {
            let prefix = format!("root.scene.walls[{index}].wall");
            assert_eq!(
                has_field(&archive, &format!("{prefix}.pw_material")),
                case.version < 0x117f
            );
            assert_eq!(
                has_field(&archive, &format!("{prefix}.viz_prop")),
                case.version >= 0x117f
            );
            assert_eq!(
                has_field(&archive, &format!("{prefix}.unknown_10")),
                case.version >= 0x117f
            );
            if case.version < 0x117f {
                let field = archive
                    .contents()
                    .fields
                    .iter()
                    .find(|field| field.path.as_ref() == format!("{prefix}.pw_material"))
                    .unwrap();
                let DisplayFieldValue::ObjectTag(tag) = &field.value else {
                    panic!("expected a null wall pointer")
                };
                assert_eq!(tag.target, DisplayObjectPointer::Null);
            }
        }
    }
}

#[test]
fn shader_surface_and_face_words_follow_their_own_version_conditions() {
    for case in cases().into_iter().filter(|case| case.scenario == 192) {
        let archive = parsed(case);
        for (suffix, present) in [
            (".rv_app_props.axis_direction1", case.version >= 0x1165),
            (".rv_app_props.flags", case.version >= 0x1165),
            (".rv_app_props.refraction_roughness", case.version > 0x1179),
            (".rv_app_props.appearance_u_dir[2]", case.version > 0x1179),
        ] {
            assert_eq!(
                has_field(&archive, suffix),
                present,
                "{} {suffix}",
                case.version
            );
        }
        let face = if case.kind == DisplayDocumentKind::Part {
            "root.tessellation.bodies[0].faces[0]"
        } else {
            "root.tessellation.components[0].comp_part_tess_data.bodies[0].faces[0]"
        };
        assert_eq!(
            has_field(&archive, &format!("{face}.unknown_68")),
            case.version >= 0x117d
        );
    }
}

fn budgets() -> Vec<(usize, usize)> {
    use complete_cases::framing::{require_finished, take_u32};
    let mut bytes = include_bytes!("fixtures/display-lists/earliest-wall-budgets.bin").as_slice();
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
fn null_wall_alias_changes_do_not_shift_object_or_collection_budgets() {
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
fn cross_version_construction_does_not_infer_wall_or_appearance_defaults() {
    for (before, after) in [
        (0x1164, 0x1165),
        (0x1179, 0x117a),
        (0x117c, 0x117d),
        (0x117e, 0x117f),
    ] {
        let old = selected(before, DisplayDocumentKind::Part, 192);
        let new = selected(after, DisplayDocumentKind::Part, 192);
        assert!(
            DisplayArchive::new(parsed(old).contents().clone(), case_context(new), LIMITS).is_err()
        );
        assert!(
            DisplayArchive::new(parsed(new).contents().clone(), case_context(old), LIMITS).is_err()
        );
    }
}

#[test]
fn unresolved_non_null_wall_materials_fail_explicitly_without_publishing_partial_results() {
    let context = case_context(selected(4400, DisplayDocumentKind::Part, 192));
    let source = include_bytes!("fixtures/display-lists/earliest-wall-unresolved.bin");
    let error = DisplayArchive::from_bytes(source, context, LIMITS).unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("moPWMaterialProperties_c"));
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
fn concurrent_old_wall_edits_match_independent_bytes_and_keep_owned_sources() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(4400, kind, 192);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), selected(4400, kind, 193).bytes);
            }
        });
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn every_old_wall_prefix_is_rejected_without_synthesizing_missing_fields() {
    for version in [
        4400, 0x1164, 0x1165, 0x1179, 0x117a, 0x117c, 0x117d, 0x117e, 0x117f,
    ] {
        let case = selected(version, DisplayDocumentKind::Part, 192);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{version} prefix {end}"
            );
        }
    }
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
    assert_eq!(context.version.value(), 4400);
    DisplayArchive::from_bytes(
        &document.read_stream("Contents/DisplayLists").unwrap(),
        context,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn complete_documents_resolve_old_wall_contexts_reopen_edits_and_restore_authentic_bytes() {
    for (kind, source) in [
        (
            DisplayDocumentKind::Part,
            include_bytes!("fixtures/display-lists/earliest-wall-document-part.bin").as_slice(),
        ),
        (
            DisplayDocumentKind::Assembly,
            include_bytes!("fixtures/display-lists/earliest-wall-document-assembly.bin").as_slice(),
        ),
    ] {
        let mut document = bessemer::Document::from_bytes(source).unwrap();
        let archive = document_archive(&document, kind);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(4400, kind, 193).bytes);
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
