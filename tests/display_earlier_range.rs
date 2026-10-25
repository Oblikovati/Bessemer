//! Earlier roots pin collector, ink and configuration-tail version boundaries.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase as RangeCase;
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(dead_code, reason = "Shared view helpers have dedicated view tests.")]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

const ARCHIVE_ROOT_GROUPS: &[&[u8]] = &[
    include_bytes!("fixtures/display-lists/earlier-range-part.bin"),
    include_bytes!("fixtures/display-lists/earlier-range-assembly.bin"),
    include_bytes!("fixtures/display-lists/precollector-range-part.bin"),
    include_bytes!("fixtures/display-lists/precollector-range-assembly.bin"),
    include_bytes!("fixtures/display-lists/precollector-configurations.bin"),
    include_bytes!("fixtures/display-lists/component-transition-part.bin"),
    include_bytes!("fixtures/display-lists/component-transition-assembly.bin"),
    include_bytes!("fixtures/display-lists/transient-viewers.bin"),
    include_bytes!("fixtures/display-lists/grouped-component-part.bin"),
    include_bytes!("fixtures/display-lists/grouped-component-assembly.bin"),
    include_bytes!("fixtures/display-lists/early-assembly-configurations.bin"),
    include_bytes!("fixtures/display-lists/window-boundary-part.bin"),
    include_bytes!("fixtures/display-lists/window-boundary-assembly.bin"),
    include_bytes!("fixtures/display-lists/early-minimal-annotations.bin"),
    include_bytes!("fixtures/display-lists/early-view-roots-part.bin"),
    include_bytes!("fixtures/display-lists/early-view-roots-assembly.bin"),
    include_bytes!("fixtures/display-lists/earliest-minimal-annotations.bin"),
    include_bytes!("fixtures/display-lists/historical-roots-part.bin"),
    include_bytes!("fixtures/display-lists/historical-roots-assembly.bin"),
    include_bytes!("fixtures/display-lists/early-appearance-roots-part.bin"),
    include_bytes!("fixtures/display-lists/early-appearance-roots-assembly.bin"),
    include_bytes!("fixtures/display-lists/earliest-scene-part.bin"),
    include_bytes!("fixtures/display-lists/earliest-scene-assembly.bin"),
];

fn cases() -> Vec<RangeCase> {
    let groups = ARCHIVE_ROOT_GROUPS.iter();
    groups
        .flat_map(|bytes| complete_cases::decode(bytes))
        .collect()
}

fn selected(version: u32, kind: DisplayDocumentKind, scenario: u8) -> RangeCase {
    cases()
        .into_iter()
        .find(|case| case.version == version && case.kind == kind && case.scenario == scenario)
        .unwrap()
}

fn case_context(case: RangeCase) -> DisplayArchiveContext {
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        ..context()
    }
}

fn parsed(case: RangeCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

#[test]
fn all_earlier_range_roots_match_independent_material_scene_component_fields_and_bytes() {
    assert_eq!(cases().len(), 528);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        let mut trace = material_trace::trace(&archive);
        trace.extend(scene_trace::trace(&archive));
        trace.extend(item_trace::trace(&archive));
        assert_eq!(trace, case.expected, "{} {:?}", case.version, case.kind);
        fields += case
            .expected
            .iter()
            .filter(|value| **value == b'\n')
            .count();
        fields += archive_support::check_scalar_table(&archive, case.scalars);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(fields, 161481);
}

fn has_field(archive: &DisplayArchive, path: &str) -> bool {
    archive
        .contents()
        .fields
        .iter()
        .any(|field| field.path.as_ref() == path)
}

fn edited_ambient(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.visual_properties.material.ambient")
        .unwrap();
    assert!(matches!(field.value, DisplayFieldValue::F64(_)));
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn truncated_roots_extra_bytes_and_unproven_contexts_do_not_publish_partial_results() {
    let (version, scenario) = (12000, 128);
    let case = selected(version, DisplayDocumentKind::Part, scenario);
    for end in 0..case.bytes.len() {
        assert!(
            DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
            "prefix {end}"
        );
    }
    let archive = parsed(case);
    let mut contents = archive.contents().clone();
    archive_support::set_integer(&mut contents, "root.view_count", i32::MAX);
    assert!(archive.with_contents(contents, LIMITS).is_err());
    let mut trailing = case.bytes.to_vec();
    trailing.push(0);
    assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());

    let case = selected(12000, DisplayDocumentKind::Part, 128);
    for version in [0, 2010] {
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(version),
            ..case_context(case)
        };
        let error = DisplayArchive::from_bytes(case.bytes, context, LIMITS).unwrap_err();
        assert!(error.to_string().contains("Unsupported"));
    }
}

#[test]
fn published_earlier_range_roots_own_sources_and_parallel_edits_match_independent_bytes() {
    let (version, scenario) = (12000, 128);
    let case = selected(version, DisplayDocumentKind::Assembly, scenario);
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(
                worker.join().unwrap(),
                selected(version, DisplayDocumentKind::Assembly, scenario + 1).bytes
            );
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn collector_and_ink_scopes_follow_their_independent_caller_bounds() {
    for case in cases() {
        let archive = parsed(case);
        let (part, viewer) = if case.kind == DisplayDocumentKind::Part {
            ("root.tessellation", "root.absent")
        } else {
            (
                "root.tessellation.components[0].comp_part_tess_data",
                "root.tessellation.components[0].viewer_comp_data",
            )
        };
        assert_eq!(
            has_field(
                &archive,
                &format!("{part}.ref_geom_feature_data_collector.count_low")
            ),
            case.version > 0x2f5d
        );
        assert_eq!(
            has_field(
                &archive,
                &format!("{viewer}.ref_geom_data_collector.count_low")
            ),
            case.kind == DisplayDocumentKind::Assembly && case.version > 0x2f44
        );
        assert_eq!(
            has_field(&archive, "root.markup_ink_views.view_count"),
            case.version > 0x2f80
        );
    }
}

#[test]
fn exact_earlier_root_budgets_cover_missing_collectors_ink_and_configuration_tails() {
    for case in cases() {
        let archive = parsed(case);
        let mut entries = if case.kind == DisplayDocumentKind::Part {
            135
        } else {
            140
        };
        entries -= usize::from(case.version <= 0x2f80);
        entries -= 2 * usize::from(case.version <= 0x2bd0);
        entries -=
            4 * usize::from(case.version <= 0x2791) + 4 * usize::from(case.version <= 0x27ad);
        entries -= if case.kind == DisplayDocumentKind::Assembly && case.version <= 0x2f44 {
            4
        } else {
            0
        };
        entries -= 2 * usize::from(case.version <= 0x1fd4);
        entries += match case.scenario % 128 {
            6 | 7 => 5,
            2 => 99,
            4 | 5 => {
                let (minimum, visibility) = if case.kind == DisplayDocumentKind::Part {
                    (0x2b61, 0x2b7f)
                } else {
                    (0x2b6f, 0x2b77)
                };
                2 + 4 * usize::from(case.version > minimum)
                    + usize::from(case.version > visibility)
                    + 3 * usize::from(case.version > 0x2fa5)
            }
            _ => 0,
        };
        if case.scenario >= 160 {
            entries = early_simple_text::early_entries(case);
        }
        if case.scenario == 168 {
            entries = 163;
        }
        if case.scenario % 128 == 8 {
            entries = 152;
        }
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: if case.scenario % 128 == 8 || case.scenario == 168 {
                42
            } else if case.kind == DisplayDocumentKind::Part {
                52
            } else {
                54 + usize::from(matches!(case.scenario % 128, 6 | 7))
            },
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
                maximum_objects: exact.maximum_objects - 1,
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
            assert!(
                DisplayArchive::from_bytes(case.bytes, case_context(case), limits).is_err(),
                "{} {:?} {} limits {limits:?}",
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
fn all_seven_earlier_reference_kinds_match_independent_fields_and_bytes() {
    let geometry = complete_cases::decode(include_bytes!(
        "fixtures/display-lists/earlier-geometry.bin"
    ));
    assert_eq!(geometry.len(), 9);
    let mut fields = 0;
    for case in geometry {
        let archive = parsed(case);
        assert_eq!(
            reference_trace::trace(&archive),
            case.expected,
            "{}",
            case.version
        );
        fields += case.expected.iter().filter(|byte| **byte == b'\n').count();
        let exact = DisplayArchiveLimits {
            maximum_entries: 143 - usize::from(case.version <= 0x2f80),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok());
        assert!(
            DisplayArchive::from_bytes(
                case.bytes,
                case_context(case),
                DisplayArchiveLimits {
                    maximum_entries: exact.maximum_entries - 1,
                    ..exact
                }
            )
            .is_err()
        );

        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
        let prefix =
            "root.tessellation.components[0].viewer_comp_data.ref_geom_data_collector.entries";
        assert_eq!(
            has_field(&archive, &format!("{prefix}[2].unknown_98[0]")),
            case.version > 0x2f5b
        );
        assert_eq!(
            has_field(&archive, &format!("{prefix}[3].unknown_114")),
            case.version > 0x2f66
        );
        assert_eq!(
            has_field(&archive, &format!("{prefix}[5].unknown_c0.dims[0]")),
            (0x2fb9..0x3200).contains(&case.version)
        );
        assert_eq!(
            has_field(&archive, &format!("{prefix}[6].unknown_38[0]")),
            case.version > 0x2fb8
        );
    }
    assert_eq!(fields, 1030);
}

#[path = "../examples/support/reference_trace.rs"]
mod reference_trace;

#[test]
fn edited_earlier_roots_reopen_in_documents_and_restore_authentic_bytes() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(12000, kind, 128);
        let edited = edited_ambient(&parsed(case));
        assert_eq!(edited.to_bytes().unwrap(), selected(12000, kind, 129).bytes);
        let initial: &[u8] = if kind == DisplayDocumentKind::Part {
            include_bytes!("fixtures/blank-part.SLDPRT")
        } else {
            include_bytes!("fixtures/blank-assembly.SLDASM")
        };
        let native = archive_support::decode_document(initial, kind);
        let mut document = bessemer::Document::from_bytes(initial).unwrap();
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
        document
            .replace_stream("Contents/DisplayLists", native.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), initial);
    }
}

#[test]
fn older_cpoint_object_roles_are_explicit_nulls_and_configuration_tail_bounds_are_distinct() {
    for case in complete_cases::decode(include_bytes!(
        "fixtures/display-lists/earlier-geometry.bin"
    ))
    .into_iter()
    .filter(|case| (0x2fb9..0x3200).contains(&case.version))
    {
        let archive = parsed(case);
        let prefix = "root.tessellation.components[0].viewer_comp_data.ref_geom_data_collector.entries[5].unknown_c0";
        for suffix in [
            "dims[0]",
            "dims[1]",
            "dims[2]",
            "dim70",
            "params[0]",
            "params[1]",
        ] {
            let field = archive
                .contents()
                .fields
                .iter()
                .find(|field| field.path.as_ref() == format!("{prefix}.{suffix}"))
                .unwrap();
            assert!(
                matches!(&field.value,DisplayFieldValue::ObjectTag(tag) if matches!(tag.target,DisplayObjectPointer::Null))
            );
        }
    }
    for case in cases().into_iter().filter(|case| case.scenario == 132) {
        let archive = parsed(case);
        assert_eq!(
            has_field(&archive, "root.configurations.unknown_tail.count_low"),
            case.version > 0x2fa5
        );
    }
}

#[path = "support/early_configuration_cases.rs"]
mod early_configurations;

#[path = "support/component_transition_cases.rs"]
mod component_transitions;
#[path = "support/grouped_component_cases.rs"]
mod grouped_components;

#[path = "../examples/support/annotation_trace.rs"]
mod annotation_trace;
#[path = "support/early_simple_text_cases.rs"]
mod early_simple_text;
#[path = "support/early_view_oracles.rs"]
mod early_view_oracles;
#[path = "../examples/support/named_trace.rs"]
mod named_trace;
#[path = "../examples/support/spatial_trace.rs"]
mod spatial_trace;
#[path = "support/window_boundary_cases.rs"]
mod window_boundaries;

#[path = "support/historical_root_cases.rs"]
mod historical_roots;

#[path = "support/early_appearance_cases.rs"]
mod early_appearance;

#[path = "support/earliest_scene_cases.rs"]
mod earliest_scene;
