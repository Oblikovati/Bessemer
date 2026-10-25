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
        include_bytes!("fixtures/display-lists/earliest-visual-part.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-visual-assembly.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-visual-properties.bin").as_slice(),
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
    assert_eq!(cases().len(), 54);
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
    assert_eq!(fields, 18242);
}

#[test]
fn implicit_visual_pointers_preserve_aliases_nulls_and_absent_map_headers() {
    for case in cases().into_iter().filter(|case| case.version <= 0x10ae) {
        let archive = parsed(case);
        let part = part_prefix(case);
        let paths = [
            format!("{part}.model_viz_props"),
            format!("{part}.bodies[0].faces[0].face_viz_props"),
            format!("{part}.bodies[0].faces[1].face_viz_props"),
            format!("{part}.bodies[0].body_prop_infos"),
        ];
        for prefix in paths {
            assert!(!has_field(&archive, &format!("{prefix}.count")));
            assert!(!has_field(&archive, &format!("{prefix}[0].key")));
        }
        let path = format!("{part}.bodies[0].body_prop_infos[0].value.viz_prop");
        let expected = if case.scenario == 210 {
            DisplayObjectPointer::Null
        } else {
            let DisplayObjectPointer::New(id) = object_target(&archive, "root.visual_properties")
            else {
                panic!("expected visual identity")
            };
            DisplayObjectPointer::Reference(id)
        };
        assert_eq!(object_target(&archive, &path), expected);
    }
}

#[test]
fn viewer_and_table_words_follow_independent_version_boundaries() {
    for case in cases() {
        let archive = parsed(case);
        let table = "root.views[0].display_items.items[0].data";
        assert_eq!(
            has_field(&archive, &format!("{table}.unknown_0c")),
            case.version > 0x10b2
        );
        if case.kind == DisplayDocumentKind::Assembly {
            assert_eq!(
                has_field(
                    &archive,
                    "root.tessellation.components[0].viewer_comp_data.display_state"
                ),
                case.version > 0x10ae
            );
        }
    }
}
fn budgets() -> Vec<(usize, usize)> {
    use complete_cases::framing::{require_finished, take_u32};
    let mut bytes = include_bytes!("fixtures/display-lists/earliest-visual-budgets.bin").as_slice();
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
            assert!(
                DisplayArchive::from_bytes(case.bytes, case_context(case), limits).is_err(),
                "{} {:?} {} {limits:?}",
                case.version,
                case.kind,
                case.scenario
            );
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limits)
                    .is_err()
            );
        }
    }
}

#[test]
fn holder_state_words_begin_at_the_reference_boundary() {
    for case in cases().into_iter().filter(|case| case.scenario == 16) {
        let archive = parsed(case);
        let state = archive
            .contents()
            .fields
            .iter()
            .filter(|field| field.path.ends_with(".from_sub_assembly"))
            .count();
        assert_eq!(state > 0, case.version >= 0x106a);
    }
}
