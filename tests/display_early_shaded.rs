//! Version-7000/9000 views, early display items and complete shaded roots.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod old_cases;
use old_cases::OldCase;
#[path = "../examples/support/item_trace.rs"]
mod item_trace;
#[path = "../examples/support/light_trace.rs"]
mod light_trace;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(dead_code, reason = "Shared view helpers have dedicated view tests.")]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

fn cases() -> Vec<OldCase> {
    old_cases::decode(include_bytes!("fixtures/display-lists/early-shaded.bin"))
}

fn selected(version: u32, scenario: u8) -> OldCase {
    cases()
        .into_iter()
        .find(|case| {
            case.version == version
                && case.kind == DisplayDocumentKind::Part
                && case.scenario == scenario
        })
        .unwrap()
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

fn assert_trace(actual: &[u8], expected: &[u8]) {
    let actual = std::str::from_utf8(actual)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    let expected = std::str::from_utf8(expected)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(actual, expected, "trace row {index}");
    }
    assert_eq!(actual.len(), expected.len());
}

#[test]
fn early_part_assembly_and_unshaded_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 7);
    for case in cases() {
        let archive = parsed(case);
        let mut trace = material_trace::trace(&archive);
        trace.extend(scene_trace::trace(&archive));
        trace.extend(light_trace::trace(&archive));
        trace.extend(item_trace::trace(&archive));
        assert_trace(&trace, case.expected);
        assert_eq!(
            archive_support::check_scalar_table(&archive, case.scalars),
            if case.kind == DisplayDocumentKind::Assembly {
                32
            } else {
                0
            }
        );
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
}

#[test]
fn absent_collections_are_distinct_from_present_empty_and_populated_views() {
    for case in cases() {
        let archive = parsed(case);
        let fields = &archive.contents().fields;
        let values = fields
            .iter()
            .filter_map(|field| match &field.value {
                DisplayFieldValue::NamedViews(value) => Some(value.is_some()),
                DisplayFieldValue::SpatialViews(value) => Some(value.is_some()),
                DisplayFieldValue::AnnotationViews(value) => Some(value.is_some()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(values, vec![case.version == 9000; 3]);
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == "root.user_model_env.unknown_1c4"),
            case.version == 9000
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.as_ref() == "root.units_of_measurement.length_units"),
            case.version == 9000 && case.scenario != 2
        );
        assert!(
            !fields
                .iter()
                .any(|field| field.path.ends_with(".tool_tip_name")
                    || field.path.ends_with(".material.unknown_7c"))
        );
    }
}

#[test]
fn early_annotation_text_and_spline_fields_follow_wire_gates() {
    for version in [7000, 9000] {
        let archive = parsed(selected(version, 2));
        let fields = &archive.contents().fields;
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with(".is_scale_invariant")),
            version == 9000
        );
        assert!(
            !fields
                .iter()
                .any(|field| field.path.ends_with(".text_scale"))
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with(".display_name")),
            version == 9000
        );
        assert_eq!(
            fields
                .iter()
                .any(|field| field.path.ends_with("edw_annot_data.count")),
            version == 9000
        );
        let table = "root.views[0].display_items.items[4].data.unknown_18=i:-5\n";
        assert_eq!(archive_support::check_scalar_table(&archive, table), 1);
    }
    let archive = parsed(selected(9000, 2));
    assert_eq!(
        archive_support::check_scalar_table(
            &archive,
            "root.views[0].display_items.items[5].data.edw_annot_data.count=i:5\nroot.views[0].display_items.items[5].data.edw_annot_data[4].unknown_i=i:-8\n"
        ),
        2
    );
    let record = [-7_i32, 0, 0, 0, -8, 0]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect::<Vec<_>>();
    let mut tail = record.repeat(5);
    tail.extend(0_i32.to_le_bytes());
    assert_eq!(tail.len(), 124);
    assert!(selected(9000, 2).bytes.ends_with(&tail));
}

fn edited_material(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    let field = contents
        .fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.visual_properties.material.ambient")
        .unwrap();
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn early_material_edit_reopens_with_explicit_context_and_reverts_all_bytes() {
    let case = selected(9000, 0);
    let archive = parsed(case);
    let edited = edited_material(&archive);
    assert_eq!(edited.to_bytes().unwrap(), selected(9000, 1).bytes);
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

#[test]
fn early_global_budgets_cover_absent_views_and_old_facet_elements() {
    for case in cases() {
        let archive = parsed(case);
        let prefix = if case.version == 7000 { 52 } else { 64 };
        let entries = prefix
            + if case.scenario == 2 {
                0
            } else {
                36 + usize::from(case.kind == DisplayDocumentKind::Assembly) * 8
            };
        let objects = if case.scenario == 2 {
            42
        } else if case.kind == DisplayDocumentKind::Assembly {
            54
        } else {
            52
        };
        let exact = DisplayArchiveLimits {
            maximum_entries: entries,
            maximum_objects: objects,
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok());
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
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
                maximum_bytes: exact.maximum_bytes - 1,
                ..exact
            },
        ] {
            assert!(DisplayArchive::from_bytes(case.bytes, case_context(case), limited).is_err());
            assert!(
                archive
                    .with_contents(archive.contents().clone(), limited)
                    .is_err()
            );
        }
    }
}

#[test]
fn every_early_unshaded_prefix_and_changed_presence_shape_fails_without_partial_results() {
    for version in [7000, 9000] {
        let case = selected(version, 2);
        let archive = parsed(case);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{version} prefix {end}"
            );
        }
        let mut contents = archive.contents().clone();
        archive_support::set_integer(&mut contents, "root.view_count", i32::MAX);
        assert!(archive.with_contents(contents, LIMITS).is_err());
        let wrong = DisplayArchiveContext {
            version: DisplayVersion::new(if version == 7000 { 9000 } else { 7000 }),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(archive.contents().clone(), wrong, LIMITS).is_err());
    }
}

#[test]
fn early_results_own_source_storage_and_parallel_edits_match_the_independent_bytes() {
    let case = selected(9000, 0);
    let mut source = case.bytes.to_vec();
    let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
    source.fill(0);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edited_material(&archive).to_bytes().unwrap()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), selected(9000, 1).bytes);
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
