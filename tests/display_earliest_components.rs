//! Complete old component, property, decal and bucket layouts retain versioned field absence.
use bessemer::FailureClass;
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
#[allow(
    dead_code,
    reason = "Shared whole-document helpers are covered by document tests."
)]
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
    reason = "Shared formatting also serves view-specific tests."
)]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

const GROUPS: &[&[u8]] = &[
    include_bytes!("fixtures/display-lists/earliest-component-part.bin"),
    include_bytes!("fixtures/display-lists/earliest-component-assembly.bin"),
    include_bytes!("fixtures/display-lists/earliest-component-properties.bin"),
    include_bytes!("fixtures/display-lists/earliest-component-holders.bin"),
    include_bytes!("fixtures/display-lists/earliest-component-buckets-3d.bin"),
    include_bytes!("fixtures/display-lists/earliest-component-buckets-2d.bin"),
    include_bytes!("fixtures/display-lists/earliest-component-decals.bin"),
];

fn cases() -> Vec<OldCase> {
    GROUPS
        .iter()
        .flat_map(|bytes| complete_cases::decode(bytes))
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

fn expected_trace(archive: &DisplayArchive, case: OldCase) -> Vec<u8> {
    if (160..192).contains(&case.scenario) {
        return bucket_trace::trace(archive);
    }
    if case.expected.is_empty() {
        return Vec::new();
    }
    let mut trace = material_trace::trace(archive);
    trace.extend(scene_trace::trace(archive));
    trace.extend(item_trace::trace(archive));
    trace
}

#[test]
fn complete_layouts_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 102);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        let actual = String::from_utf8(expected_trace(&archive, case)).unwrap();
        assert_eq!(
            actual,
            std::str::from_utf8(case.expected).unwrap(),
            "{} {:?} {}",
            case.version,
            case.kind,
            case.scenario
        );
        fields +=
            actual.lines().count() + archive_support::check_scalar_table(&archive, case.scalars);
        assert_eq!(
            DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
    }
    assert_eq!(fields, 36978);
}

#[test]
fn all_component_metadata_members_obey_their_individual_gates() {
    for case in cases()
        .into_iter()
        .filter(|case| case.kind == DisplayDocumentKind::Assembly && case.scenario == 208)
    {
        let archive = parsed(case);
        for (suffix, threshold) in [
            (".comp_ref", 0x12c3),
            (".spool_ref", 0x12c3),
            (".comp_description", 0x12c3),
            (".suppressed", 0x12d0),
            (".comp_type", 0x12d3),
            (".depended_comp_names.count", 0x12df),
            (".equ_dep_comp_names.count", 0x12e2),
            (".folder_comp", 0x12e8),
        ] {
            let path = format!("root.tessellation.components[0].viewer_comp_data{suffix}");
            assert_eq!(
                archive
                    .contents()
                    .fields
                    .iter()
                    .any(|field| field.path.as_ref() == path),
                case.version > threshold,
                "{} {suffix}",
                case.version
            );
        }
    }
}

#[test]
fn embedded_fonts_bos_flags_and_decal_face_words_follow_their_stored_boundaries() {
    for case in cases() {
        let archive = parsed(case);
        if matches!(case.scenario, 0 | 2 | 16 | 18) && !case.expected.is_empty() {
            let fonts = archive
                .contents()
                .fields
                .iter()
                .filter(|field| {
                    field.path.ends_with(".standard_width")
                        && field.path.contains(".line_fonts[")
                        && field
                            .path
                            .starts_with("root.scene.concept.feature_list.nodes[1].")
                })
                .count();
            assert_eq!(fonts, if case.version < 0x12cf { 25 } else { 26 });
        }
        if matches!(case.scenario, 160 | 176) {
            assert_eq!(has_field(&archive, ".bos"), case.version > 0x12dd);
        }
        if case.expected.is_empty() {
            assert_eq!(
                has_field(&archive, ".decal_faces[0].unknown_1c[3]"),
                case.version > 0x12db
            );
        }
    }
}

fn budgets() -> Vec<(usize, usize)> {
    use complete_cases::framing::{require_finished, take_u32};
    let mut bytes =
        include_bytes!("fixtures/display-lists/earliest-component-budgets.bin").as_slice();
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
fn exact_global_budgets_include_versioned_fonts_polygons_and_decal_arrays() {
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
        assert_eq!(
            DisplayArchive::from_bytes(case.bytes, case_context(case), exact)
                .unwrap()
                .contents(),
            archive.contents()
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
fn old_component_shapes_cannot_be_republished_with_new_field_requirements() {
    for (before, after) in [
        (0x12c3, 0x12c4),
        (0x12d0, 0x12d1),
        (0x12d3, 0x12d4),
        (0x12df, 0x12e0),
        (0x12e2, 0x12e3),
        (0x12e8, 0x12e9),
    ] {
        let old = selected(before, DisplayDocumentKind::Assembly, 208);
        let new = selected(after, DisplayDocumentKind::Assembly, 208);
        assert!(
            DisplayArchive::new(parsed(old).contents().clone(), case_context(new), LIMITS).is_err()
        );
        assert!(
            DisplayArchive::new(parsed(new).contents().clone(), case_context(old), LIMITS).is_err()
        );
    }
    let case = selected(4800, DisplayDocumentKind::Part, 208);
    let unsupported = DisplayArchiveContext {
        version: DisplayVersion::new(2129),
        ..case_context(case)
    };
    assert_eq!(
        DisplayArchive::from_bytes(case.bytes, unsupported, LIMITS)
            .unwrap_err()
            .class,
        FailureClass::Unsupported
    );
}

#[test]
fn font_bucket_and_decal_shapes_do_not_synthesize_new_fields() {
    for (before, after, scenario) in [(0x12ce, 0x12cf, 0), (0x12dd, 0x12de, 160)] {
        let old = selected(before, DisplayDocumentKind::Part, scenario);
        let new = selected(after, DisplayDocumentKind::Part, scenario);
        assert!(
            DisplayArchive::new(parsed(old).contents().clone(), case_context(new), LIMITS).is_err()
        );
        assert!(
            DisplayArchive::new(parsed(new).contents().clone(), case_context(old), LIMITS).is_err()
        );
    }
    let old = cases()
        .into_iter()
        .find(|case| {
            case.version == 0x12db
                && case.kind == DisplayDocumentKind::Part
                && case.expected.is_empty()
        })
        .unwrap();
    let new = cases()
        .into_iter()
        .find(|case| {
            case.version == 0x12dc
                && case.kind == DisplayDocumentKind::Part
                && case.expected.is_empty()
        })
        .unwrap();
    assert!(
        DisplayArchive::new(parsed(old).contents().clone(), case_context(new), LIMITS).is_err()
    );
    assert!(
        DisplayArchive::new(parsed(new).contents().clone(), case_context(old), LIMITS).is_err()
    );
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
    assert_eq!(context.version.value(), 4800);
    DisplayArchive::from_bytes(
        &document.read_stream("Contents/DisplayLists").unwrap(),
        context,
        LIMITS,
    )
    .unwrap()
}

#[test]
fn complete_documents_resolve_context_reopen_independent_edits_and_authentically_revert() {
    for (kind, source) in [
        (
            DisplayDocumentKind::Part,
            include_bytes!("fixtures/display-lists/earliest-component-document-part.bin")
                .as_slice(),
        ),
        (
            DisplayDocumentKind::Assembly,
            include_bytes!("fixtures/display-lists/earliest-component-document-assembly.bin")
                .as_slice(),
        ),
    ] {
        let mut document = bessemer::Document::from_bytes(source).unwrap();
        let archive = document_archive(&document, kind);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(4800, kind, 209).bytes);
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
fn populated_old_dependencies_preserve_duplicate_keys_and_match_independent_edits() {
    let case = selected(0x12e3, DisplayDocumentKind::Assembly, 210);
    let archive = parsed(case);
    let prefix = "root.tessellation.components[0].viewer_comp_data";
    let expected = format!(
        "{prefix}.depended_comp_names.count=i:2\n{prefix}.equ_dep_comp_names.count=i:2\n{prefix}.equ_dep_comp_names[0].key=s:6b6579\n{prefix}.equ_dep_comp_names[1].key=s:6b6579\n{prefix}.equ_dep_comp_names[0].values.count=i:2\n{prefix}.equ_dep_comp_names[1].values.count=i:0\n"
    );
    assert_eq!(archive_support::check_scalar_table(&archive, &expected), 6);
    assert_eq!(
        edited_ambient(&archive).to_bytes().unwrap(),
        selected(0x12e3, DisplayDocumentKind::Assembly, 211).bytes
    );
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn old_component_results_own_inputs_and_parallel_edits_match_independent_bytes() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(4800, kind, 208);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_ambient(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), selected(4800, kind, 209).bytes);
            }
        });
        let changed = edited_ambient(&archive);
        let reopened =
            DisplayArchive::from_bytes(&changed.to_bytes().unwrap(), case_context(case), LIMITS)
                .unwrap();
        assert_eq!(reopened.contents(), changed.contents());
        assert_eq!(
            changed
                .with_contents(archive.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn every_new_component_prefix_and_impossible_dependency_count_fails_without_mutating_input() {
    for version in [4800, 0x12c4, 0x12d1, 0x12d4, 0x12e0, 0x12e3, 0x12e9] {
        let case = selected(version, DisplayDocumentKind::Assembly, 208);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{version} prefix {end}"
            );
        }
    }
    let case = selected(4850, DisplayDocumentKind::Assembly, 208);
    let archive = parsed(case);
    for leaf in ["depended_comp_names.count", "equ_dep_comp_names.count"] {
        let mut contents = archive.contents().clone();
        archive_support::set_integer(
            &mut contents,
            &format!("root.tessellation.components[0].viewer_comp_data.{leaf}"),
            i32::MAX,
        );
        assert!(archive.with_contents(contents, LIMITS).is_err());
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}
