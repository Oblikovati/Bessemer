//! Resolved older archive branches remain usable without unresolved facet payloads.
use bessemer::FailureClass;
use bessemer::display_lists::*;
#[allow(
    dead_code,
    reason = "Shared helpers also serve other archive integration crates."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, archive_scalar_trace, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase;
#[path = "../examples/support/archive_trace.rs"]
mod archive_trace;
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
#[allow(
    dead_code,
    reason = "Shared formatting also serves view-specific integration crates."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

fn cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!("fixtures/display-lists/pre-facet-roots.bin"))
}

fn case_context(case: OldCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        face_tessellation_components: if (3..=4).contains(&case.scenario)
            || (case.kind == DisplayDocumentKind::Assembly && case.scenario >= 64)
        {
            std::sync::Arc::new(["Assem1".to_owned()].into())
        } else {
            Default::default()
        },
        ..context()
    }
}

fn parsed(case: OldCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

fn field<'a>(archive: &'a DisplayArchive, path: &str) -> Option<&'a DisplayFieldValue> {
    archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .map(|field| &field.value)
}

fn check_case(case: OldCase) -> usize {
    let archive = parsed(case);
    let actual = archive_trace::trace(&archive);
    assert_eq!(
        std::str::from_utf8(&actual).unwrap(),
        std::str::from_utf8(case.expected).unwrap()
    );
    let fields = actual.iter().filter(|byte| **byte == b'\n').count()
        + archive_support::check_scalar_table(&archive, case.scalars);
    let forced =
        DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS).unwrap();
    assert_eq!(forced.to_bytes().unwrap(), case.bytes);
    fields
}

#[test]
fn complete_resolved_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 94);
    assert_eq!(cases().into_iter().map(check_case).sum::<usize>(), 7230);
}

fn check_empty_geometry(case: OldCase) {
    let archive = parsed(case);
    let facets = archive
        .contents()
        .fields
        .iter()
        .filter(|field| matches!(field.value, DisplayFieldValue::Facet(_)))
        .count();
    assert_eq!(facets, 0);
    if case.scenario == 2 {
        return check_absent_shading(&archive);
    }
    if case.kind == DisplayDocumentKind::Part {
        assert_eq!(
            field(&archive, "root.tessellation.bodies.count_word"),
            Some(&DisplayFieldValue::U16(0))
        );
        return;
    }
    check_null_part(&archive);
}

fn check_absent_shading(archive: &DisplayArchive) {
    assert!(field(archive, "root.visual_properties").is_none());
    assert_eq!(
        field(archive, "root.has_shaded_data"),
        Some(&DisplayFieldValue::I32(0))
    );
}

fn check_null_part(archive: &DisplayArchive) {
    let path = "root.tessellation.components[0].comp_part_tess_data";
    let DisplayFieldValue::ObjectTag(tag) = field(archive, path).unwrap() else {
        panic!("expected null part pointer")
    };
    assert_eq!(tag.target, DisplayObjectPointer::Null);
}

#[test]
fn absent_shaded_data_empty_parts_and_null_component_parts_remain_distinct() {
    for case in cases()
        .into_iter()
        .filter(|case| !(3..=4).contains(&case.scenario))
    {
        check_empty_geometry(case);
    }
}

#[test]
fn pen_layer_parabola_tail_and_assembly_word_follow_separate_boundaries() {
    for case in cases() {
        let archive = parsed(case);
        assert_eq!(
            field(
                &archive,
                "root.views[0].display_items.items[0].data.layer_id"
            )
            .is_some(),
            case.version > 0xc57
        );
        for member in ["unknown_80", "unknown_88"] {
            let path = format!("root.views[0].display_items.items[1].data.{member}");
            assert_eq!(field(&archive, &path).is_some(), case.version > 0xc58);
        }
        let path = "root.tessellation.components[0].unknown_320";
        assert_eq!(
            field(&archive, path).is_some(),
            case.kind == DisplayDocumentKind::Assembly
                && case.scenario != 2
                && case.version > 0xc6e
        );
    }
}

fn edited_case(case: OldCase) -> OldCase {
    cases()
        .into_iter()
        .find(|edited| {
            edited.version == case.version && edited.kind == case.kind && edited.scenario == 1
        })
        .unwrap()
}

fn edited_contents(source: &DisplayArchive, expected: &DisplayArchive) -> DisplayArchiveContents {
    let mut contents = source.contents().clone();
    for field in &mut contents.fields {
        if field.path.ends_with(".width.standard_width") || field.path.ends_with(".data.layer_id") {
            field.value = self::field(expected, &field.path).unwrap().clone();
        }
    }
    contents
}

#[test]
fn edits_match_independent_bytes_and_keep_the_input_through_success_failure_and_revert() {
    for case in cases().into_iter().filter(|case| case.scenario == 0) {
        let source = parsed(case);
        let expected_case = edited_case(case);
        let expected = parsed(expected_case);
        let contents = edited_contents(&source, &expected);
        assert_short_edit_budget(&source, &contents);
        let edited = source.with_contents(contents, LIMITS).unwrap();
        assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
        let reverted = edited
            .with_contents(source.contents().clone(), LIMITS)
            .unwrap();
        assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    }
}

fn assert_short_edit_budget(source: &DisplayArchive, contents: &DisplayArchiveContents) {
    let limits = DisplayArchiveLimits {
        maximum_bytes: 1,
        ..LIMITS
    };
    assert!(source.with_contents(contents.clone(), limits).is_err());
}

#[test]
fn unresolved_facet_objects_fail_at_the_payload_instead_of_rejecting_every_old_root() {
    let supported = complete_cases::decode(include_bytes!(
        "fixtures/display-lists/earliest-visual-part.bin"
    ));
    let case = supported
        .into_iter()
        .find(|case| case.version == 3196 && case.scenario == 208)
        .unwrap();
    let source = parsed(case);
    let old = DisplayArchiveContext {
        version: DisplayVersion::new(3195),
        ..case_context(case)
    };
    let error = DisplayArchive::from_bytes(case.bytes, old.clone(), LIMITS).unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("unresolved object range"));
    assert!(DisplayArchive::new(source.contents().clone(), old, LIMITS).is_err());
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn truncated_and_extended_complete_roots_never_publish_partial_results() {
    for case in cases() {
        for length in (0..case.bytes.len())
            .step_by(43)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
        let mut trailing = case.bytes.to_vec();
        trailing.push(0);
        assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
    }
}

fn opposite_version(version: u32) -> u32 {
    match version {
        0xc57 => 0xc58,
        0xc58 => 0xc57,
        0xc59 => 0xc58,
        0xc6e => 0xc6f,
        _ => 0xc6e,
    }
}

fn check_boundary(case: OldCase) {
    let archive = parsed(case);
    let version = opposite_version(case.version);
    let target = DisplayArchiveContext {
        version: DisplayVersion::new(version),
        ..case_context(case)
    };
    let changed =
        case.version < 0xc6e || (case.kind == DisplayDocumentKind::Assembly && case.scenario != 2);
    let result = DisplayArchive::new(archive.contents().clone(), target, LIMITS);
    assert_eq!(result.is_err(), changed);
    if let Ok(unchanged) = result {
        assert_eq!(unchanged.to_bytes().unwrap(), case.bytes);
    }
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn old_field_shapes_reject_incompatible_context_and_keep_unchanged_document_branches() {
    for case in cases()
        .into_iter()
        .filter(|case| [0xc57, 0xc58, 0xc59, 0xc6e, 0xc6f].contains(&case.version))
    {
        check_boundary(case);
    }
}

#[test]
fn all_five_old_light_classes_keep_their_identity_in_complete_earlier_roots() {
    let names = [
        "moAmbientLight_c",
        "moLightFeature_c",
        "moDirectionLight_c",
        "moSpotLight_c",
        "moPointLight_c",
    ];
    let selected = cases().into_iter().filter(|case| case.scenario >= 16);
    assert_eq!(selected.clone().count(), 30);
    for case in selected {
        let archive = parsed(case);
        let value = field(&archive, "root.old_lights.items[1]").unwrap();
        let DisplayFieldValue::ObjectTag(tag) = value else {
            panic!("expected old-light object")
        };
        assert_eq!(
            tag.runtime_class.as_ref().unwrap().name,
            names[usize::from(case.scenario - 16)]
        );
    }
}

#[path = "support/pre_facet_external.rs"]
mod external_faces;
#[path = "support/legacy_addins.rs"]
mod legacy_addins;
#[path = "support/legacy_cameras.rs"]
mod legacy_cameras;
#[path = "support/legacy_panes.rs"]
mod legacy_panes;
#[path = "support/legacy_threads.rs"]
mod legacy_threads;
#[path = "support/legacy_visuals.rs"]
mod legacy_visuals;
#[path = "support/legacy_widths.rs"]
mod legacy_widths;
