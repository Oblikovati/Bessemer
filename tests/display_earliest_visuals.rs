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
#[path = "../examples/support/bucket_trace.rs"]
mod bucket_trace;
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
        include_bytes!("fixtures/display-lists/earliest-width-items.bin").as_slice(),
        include_bytes!("fixtures/display-lists/earliest-light-state-kinds.bin").as_slice(),
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
    assert_eq!(cases().len(), 548);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        let mut actual = material_trace::trace(&archive);
        actual.extend(scene_trace::trace(&archive));
        actual.extend(item_trace::trace(&archive));
        if case.version < 0xf41 && case.scenario < 208 {
            actual.extend(bucket_trace::trace(&archive));
        }
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
    assert_eq!(fields, 96706);
}

#[test]
fn implicit_visual_pointers_preserve_aliases_nulls_and_absent_map_headers() {
    for case in cases()
        .into_iter()
        .filter(|case| case.version <= 0x10ae && (case.version > 0xf5e || case.scenario >= 208))
    {
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

#[test]
fn early_appearance_layouts_keep_the_old_word_and_omit_unstored_rendering_fields() {
    for case in cases()
        .into_iter()
        .filter(|case| case.version > 0xf5e || case.scenario >= 208)
    {
        let archive = parsed(case);
        let suffix_present = |suffix: &str| {
            archive
                .contents()
                .fields
                .iter()
                .any(|field| field.path.ends_with(suffix))
        };
        assert_eq!(
            suffix_present(".rv_app_props.color_form"),
            case.version >= 0x1052
        );
        assert_eq!(
            suffix_present(".rv_app_props.reflectivity"),
            case.version >= 0x1052
        );
        assert_eq!(
            suffix_present(".unknown_old_5c"),
            (0xe55..0x1052).contains(&case.version)
        );
        assert_eq!(
            suffix_present(".default_appearance"),
            case.version >= 0x1052
        );
    }
}

#[test]
fn early_appearance_fields_cannot_be_reinterpreted_across_the_boundary() {
    for case in cases()
        .into_iter()
        .filter(|case| [0x1051, 0x1052].contains(&case.version))
    {
        let archive = parsed(case);
        let version = if case.version == 0x1051 {
            0x1052
        } else {
            0x1051
        };
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(version),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(archive.contents().clone(), context, LIMITS).is_err());
        assert!(
            DisplayArchive::from_bytes(
                case.bytes,
                case_context(case),
                DisplayArchiveLimits {
                    maximum_bytes: case.bytes.len() - 1,
                    ..LIMITS
                }
            )
            .is_err()
        );
    }
}

fn check_old_part_tail(archive: &DisplayArchive, case: OldCase) {
    let part = part_prefix(case);
    let expected = case.version >= 0xfd8;
    for suffix in [
        ".bodies[0].faces[0].unknown_34",
        ".bodies[0].faces[1].unknown_34",
        ".bodies[0].decals.count",
        ".decals.count",
    ] {
        assert_eq!(has_field(archive, &format!("{part}{suffix}")), expected);
    }
}

#[test]
fn root_size_face_state_and_decal_headers_follow_their_independent_boundaries() {
    for case in cases() {
        let archive = parsed(case);
        assert_eq!(
            has_field(&archive, "root.max_model_size"),
            case.version > 0xfca
        );
        check_old_part_tail(&archive, case);
        if case.kind == DisplayDocumentKind::Assembly {
            let path = "root.tessellation.components[0].decals.count";
            assert_eq!(has_field(&archive, path), case.version >= 0xfd8);
        }
    }
}

fn opposite_tessellation_boundary(version: u32) -> u32 {
    match version {
        0xfca => 0xfcb,
        0xfcb => 0xfca,
        0xfd7 => 0xfd8,
        _ => 0xfd7,
    }
}

#[test]
fn old_tessellation_fields_cannot_be_published_under_the_other_layout() {
    let boundaries = [0xfca, 0xfcb, 0xfd7, 0xfd8];
    for case in cases()
        .into_iter()
        .filter(|case| boundaries.contains(&case.version))
    {
        let archive = parsed(case);
        let version = opposite_tessellation_boundary(case.version);
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(version),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(archive.contents().clone(), context, LIMITS).is_err());
    }
}

fn check_early_prefix_samples(case: OldCase) {
    let tail = case.bytes.len() - 8..case.bytes.len();
    for length in (0..case.bytes.len()).step_by(97).chain(tail) {
        let prefix = &case.bytes[..length];
        assert!(DisplayArchive::from_bytes(prefix, case_context(case), LIMITS).is_err());
    }
}

#[test]
fn early_tessellation_rejects_sampled_prefixes_and_trailing_bytes() {
    let earliest = cases()
        .into_iter()
        .filter(|case| case.version == 4000 && case.scenario == 208);
    for case in earliest {
        check_early_prefix_samples(case);
        let mut extended = case.bytes.to_vec();
        extended.push(0);
        assert!(DisplayArchive::from_bytes(&extended, case_context(case), LIMITS).is_err());
    }
}

#[path = "support/earliest_environment.rs"]
mod earlier_environment;

#[path = "support/earliest_widths.rs"]
mod earlier_widths;

#[path = "support/earliest_materials.rs"]
mod earlier_materials;

#[path = "support/earliest_state.rs"]
mod earlier_state;

#[path = "support/earliest_component_state.rs"]
mod earlier_component_state;

#[path = "support/earliest_shader.rs"]
mod earlier_shader;

#[path = "support/earliest_light_states.rs"]
mod earlier_light_states;

#[path = "support/earliest_caps.rs"]
mod earlier_caps;
