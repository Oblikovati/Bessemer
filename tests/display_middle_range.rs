//! Complete part and assembly roots pin every 16000..18000 material/component/facet gate.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::archive_scalar_trace;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase as RangeCase;
#[path = "../examples/support/material_trace.rs"]
mod material_trace;
#[path = "../examples/support/scene_trace.rs"]
mod scene_trace;
#[allow(dead_code, reason = "Shared view helpers have dedicated view tests.")]
#[path = "../examples/support/view_trace.rs"]
mod view_trace;

fn cases() -> Vec<RangeCase> {
    complete_cases::decode(include_bytes!("fixtures/display-lists/middle-range.bin"))
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
fn all_middle_range_roots_match_independent_material_scene_component_fields_and_bytes() {
    assert_eq!(cases().len(), 42);
    let mut fields = 0;
    for case in cases() {
        let archive = parsed(case);
        let mut trace = material_trace::trace(&archive);
        trace.extend(scene_trace::trace(&archive));
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
    assert_eq!(fields, 13200);
}

fn has_field(archive: &DisplayArchive, path: &str) -> bool {
    archive
        .contents()
        .fields
        .iter()
        .any(|field| field.path.as_ref() == path)
}

#[test]
fn version_gates_omit_fields_instead_of_synthesizing_defaults() {
    for case in cases() {
        let archive = parsed(case);
        assert_eq!(
            has_field(&archive, "root.visual_properties.has_dspbr"),
            case.version > 0x3f08
        );
        assert_eq!(
            has_field(&archive, "root.scene.hdr_exposure"),
            case.version > 0x42ec
        );
        assert!(!has_field(&archive, "root.scene.field_of_view"));
        let assembly = case.kind == DisplayDocumentKind::Assembly;
        assert_eq!(
            has_field(&archive, "root.envelope_system_options.unknown_color"),
            assembly && case.version > 0x42da
        );
        let prefix = "root.tessellation.components[0].viewer_comp_data";
        assert_eq!(
            has_field(&archive, &format!("{prefix}.routing_assembly")),
            assembly && case.version > 0x42e7
        );
        assert_eq!(
            has_field(&archive, &format!("{prefix}.top_level_speed_pak")),
            assembly && case.version >= 0x42f3
        );
        assert_eq!(
            archive
                .contents()
                .fields
                .iter()
                .filter(|field| matches!(field.value, DisplayFieldValue::Facet(_)))
                .count(),
            2
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
    assert!(matches!(field.value, DisplayFieldValue::F64(_)));
    field.value = DisplayFieldValue::F64(StoredDouble::from_bits(23.0_f64.to_bits()));
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn independent_ambient_edit_reopens_in_each_document_kind_and_reverts() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(16000, kind, 0);
        let archive = parsed(case);
        let edited = edited_ambient(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(16000, kind, 1).bytes);
        assert_eq!(
            edited
                .with_contents(archive.contents().clone(), LIMITS)
                .unwrap()
                .to_bytes()
                .unwrap(),
            case.bytes
        );
        let initial: &[u8] = if kind == DisplayDocumentKind::Part {
            include_bytes!("fixtures/blank-part.SLDPRT")
        } else {
            include_bytes!("fixtures/blank-assembly.SLDASM")
        };
        let native = archive_support::decode_document(initial, kind);
        let original = bessemer::Document::from_bytes(initial).unwrap();
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
    }
}

#[test]
fn full_root_budgets_include_both_inline_facets_at_the_vertex_layout_boundary() {
    for case in cases() {
        let archive = parsed(case);
        let entries = if case.kind == DisplayDocumentKind::Part {
            149
        } else {
            154
        } - if case.version <= 0x3ee7 { 8 } else { 0 };
        let objects = if case.kind == DisplayDocumentKind::Part {
            52
        } else {
            54
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
            assert!(
                DisplayArchive::from_bytes(case.bytes, case_context(case), limits).is_err(),
                "{} {:?} limits {limits:?}",
                case.version,
                case.kind
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
fn truncated_roots_extra_bytes_and_unproven_contexts_do_not_publish_partial_results() {
    let case = selected(16000, DisplayDocumentKind::Part, 0);
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
    for version in [0, 15999, 19001, u32::MAX] {
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(version),
            ..case_context(case)
        };
        let error = DisplayArchive::from_bytes(case.bytes, context, LIMITS).unwrap_err();
        assert!(error.to_string().contains("Unsupported"));
    }
}

#[test]
fn published_middle_range_roots_own_sources_and_parallel_edits_match_independent_bytes() {
    let case = selected(16000, DisplayDocumentKind::Assembly, 0);
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
                selected(16000, DisplayDocumentKind::Assembly, 1).bytes
            );
        }
    });
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}
