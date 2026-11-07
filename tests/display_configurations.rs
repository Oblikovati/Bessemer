//! Complete per-configuration graphs share identities and force assembly facets inline.
use bessemer::display_lists::*;
#[path = "support/display_archive.rs"]
mod archive_support;
use archive_support::{LIMITS, context};
#[path = "support/old_display_cases.rs"]
mod complete_cases;
use complete_cases::OldCase as ConfigCase;

const CONFIG: &str = "root.configurations";

fn cases() -> Vec<ConfigCase> {
    complete_cases::decode(include_bytes!("fixtures/display-lists/configurations.bin"))
}

fn selected(kind: DisplayDocumentKind, scenario: u8) -> ConfigCase {
    cases()
        .into_iter()
        .find(|case| case.version == 19000 && case.kind == kind && case.scenario == scenario)
        .unwrap()
}

fn case_context(case: ConfigCase) -> DisplayArchiveContext {
    let ids = if case.kind == DisplayDocumentKind::Assembly && case.version >= 18000 {
        ["component 鋼".to_owned()].into()
    } else {
        Default::default()
    };
    DisplayArchiveContext {
        version: DisplayVersion::new(case.version),
        kind: case.kind,
        face_tessellation_components: std::sync::Arc::new(ids),
        ..context()
    }
}

fn parsed(case: ConfigCase) -> DisplayArchive {
    DisplayArchive::from_bytes(case.bytes, case_context(case), LIMITS).unwrap()
}

#[test]
fn all_configuration_snapshots_match_independent_fields_and_forced_bytes() {
    assert_eq!(cases().len(), 15);
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
    assert_eq!(compared, 222);
}

fn pointer(archive: &DisplayArchive, suffix: &str) -> DisplayObjectPointer {
    let path = format!("{CONFIG}.{suffix}");
    let field = archive
        .contents()
        .fields
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    let DisplayFieldValue::ObjectTag(tag) = &field.value else {
        panic!("object tag {path}")
    };
    tag.target.clone()
}

#[test]
fn configuration_aliases_owner_cycles_and_view_objects_share_the_global_map() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let archive = parsed(selected(kind, 0));
        let member = if kind == DisplayDocumentKind::Part {
            "tessellation"
        } else {
            "shd_data[0]"
        };
        let DisplayObjectPointer::New(identity) =
            pointer(&archive, &format!("configs[0].{member}"))
        else {
            panic!("new config object")
        };
        assert_eq!(
            pointer(&archive, &format!("configs[1].{member}")),
            DisplayObjectPointer::Reference(identity)
        );
        if kind == DisplayDocumentKind::Assembly {
            assert_eq!(
                pointer(&archive, "configs[0].shd_data[0].owner"),
                DisplayObjectPointer::Reference(identity)
            );
        }
        let DisplayObjectPointer::New(view) = pointer(&archive, "view_objects.entries[0].views[0]")
        else {
            panic!("new view object")
        };
        assert_eq!(
            pointer(&archive, "view_objects.entries[1].views[0]"),
            DisplayObjectPointer::Reference(view)
        );
    }
}

#[test]
fn assembly_configuration_facets_are_inline_despite_matching_external_directory_ids() {
    let case = selected(DisplayDocumentKind::Assembly, 0);
    let archive = parsed(case);
    let facets: Vec<_> = archive
        .contents()
        .fields
        .iter()
        .filter_map(|field| match &field.value {
            DisplayFieldValue::Facet(facet) if field.path.starts_with(CONFIG) => {
                Some(facet.as_ref())
            }
            _ => None,
        })
        .collect();
    let expected = bessemer::tessellation::FacetArchive::from_bytes(
        include_bytes!("fixtures/tessellation/facet-4a38-1.bin"),
        bessemer::tessellation::TessellationContext {
            archive_version: bessemer::tessellation::ArchiveVersion::new(19000),
            mode: bessemer::tessellation::FacetArchiveMode::Full,
        },
        bessemer::tessellation::TessellationLimits {
            maximum_components: 0,
            maximum_faces: 1,
            maximum_array_elements: 1000,
        },
    )
    .unwrap();
    assert_eq!(facets, vec![expected.contents(), expected.contents()]);
    assert!(
        archive
            .contents()
            .fields
            .iter()
            .all(|field| !matches!(field.value, DisplayFieldValue::ExternalFacet { .. }))
    );
    let without_directory = DisplayArchiveContext {
        face_tessellation_components: Default::default(),
        ..case_context(case)
    };
    let independent = DisplayArchive::from_bytes(case.bytes, without_directory, LIMITS).unwrap();
    assert_eq!(independent.contents(), archive.contents());
}

fn edited_tail(archive: &DisplayArchive) -> DisplayArchive {
    let mut contents = archive.contents().clone();
    archive_support::set_integer(
        &mut contents,
        &format!("{CONFIG}.unknown_tail.entries[0].values[0]"),
        23,
    );
    archive.with_contents(contents, LIMITS).unwrap()
}

#[test]
fn tail_edit_matches_independent_bytes_reopens_and_reverts_each_document_kind() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(kind, 0);
        let archive = parsed(case);
        let edited = edited_tail(&archive);
        assert_eq!(edited.to_bytes().unwrap(), selected(kind, 1).bytes);
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
        let decoded = DisplayArchive::from_bytes(
            &reopened.read_stream("Contents/DisplayLists").unwrap(),
            case_context(case),
            LIMITS,
        )
        .unwrap();
        assert_eq!(decoded.contents(), edited.contents());
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

fn entry_budget(case: ConfigCase) -> usize {
    if case.version == 10000 {
        return 133;
    }
    if case.version == 11000 {
        return 141;
    }
    if case.version == 18000 {
        return 175;
    }
    match (case.scenario, case.kind) {
        (2 | 3, _) => 53,
        (4, _) => 59,
        (5, DisplayDocumentKind::Part) => 153,
        (5, DisplayDocumentKind::Assembly) => 169,
        (_, DisplayDocumentKind::Part) => 160,
        (_, DisplayDocumentKind::Assembly) => 176,
    }
}

fn object_budget(case: ConfigCase) -> usize {
    if case.version < 18000 {
        return 60;
    }
    if case.version == 18000 {
        return 55;
    }
    match (case.scenario, case.kind) {
        (2..=4, _) => 46,
        (5, DisplayDocumentKind::Part) => 51,
        (5, DisplayDocumentKind::Assembly) => 52,
        (_, DisplayDocumentKind::Part) => 52,
        (_, DisplayDocumentKind::Assembly) => 53,
    }
}

#[test]
fn exact_global_budgets_include_configurations_matrices_items_and_nested_facets() {
    for case in cases() {
        let archive = parsed(case);
        let exact = DisplayArchiveLimits {
            maximum_entries: entry_budget(case),
            maximum_objects: object_budget(case),
            maximum_fields: archive.contents().fields.len(),
            maximum_bytes: case.bytes.len(),
            ..LIMITS
        };
        assert!(
            DisplayArchive::from_bytes(case.bytes, case_context(case), exact).is_ok(),
            "{} {:?} {} exact {exact:?}",
            case.version,
            case.kind,
            case.scenario
        );
        assert!(DisplayArchive::new(archive.contents().clone(), case_context(case), exact).is_ok());
        for limits in [
            DisplayArchiveLimits {
                maximum_entries: exact.maximum_entries - 1,
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
                "{} {:?} {} short {limits:?}",
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
fn completed_empty_null_records_zero_view_matrices_and_old_layout_omissions_remain_distinct() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        for scenario in [2, 3] {
            let archive = parsed(selected(kind, scenario));
            let fields: Vec<_> = archive
                .contents()
                .fields
                .iter()
                .filter(|field| field.path.starts_with(CONFIG))
                .collect();
            assert_eq!(fields.len(), 1);
            assert!(
                matches!(fields[0].value,DisplayFieldValue::I32(value) if value==if scenario==2 {-7} else {0})
            );
        }
        let minimal = parsed(selected(kind, 4));
        assert_eq!(
            pointer(&minimal, "view_objects.entries[0].views[0]"),
            DisplayObjectPointer::Null
        );
        let no_views = parsed(selected(kind, 5));
        assert!(
            no_views
                .contents()
                .fields
                .iter()
                .any(|field| field.path.as_ref()
                    == format!("{CONFIG}.view_objects.entries[1].config_id"))
        );
        assert!(
            no_views
                .contents()
                .fields
                .iter()
                .all(|field| !field.path.contains("view_objects.entries[0].views["))
        );
    }
    for case in cases().into_iter().filter(|case| case.version < 18000) {
        let archive = parsed(case);
        assert!(
            archive
                .contents()
                .fields
                .iter()
                .all(|field| !field.path.starts_with(&format!("{CONFIG}.view_"))
                    && !field.path.starts_with(&format!("{CONFIG}.unknown_tail")))
        );
    }
}

#[test]
fn impossible_counts_high_words_and_incomplete_matrices_fail_before_publication() {
    let archive = parsed(selected(DisplayDocumentKind::Part, 0));
    for suffix in ["config_count", "annotation_view_visibility.count"] {
        for value in [-1, i32::MAX] {
            let mut contents = archive.contents().clone();
            archive_support::set_integer(&mut contents, &format!("{CONFIG}.{suffix}"), value);
            assert!(archive.with_contents(contents, LIMITS).is_err());
        }
    }
    for suffix in [
        "unknown_tail.count_high",
        "unknown_tail.entries[0].count_high",
    ] {
        let mut contents = archive.contents().clone();
        let path = format!("{CONFIG}.{suffix}");
        let field = contents
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap();
        field.value = DisplayFieldValue::U32(1);
        let error = archive.with_contents(contents, LIMITS).unwrap_err();
        assert!(error.to_string().contains(if suffix.contains("entries") {
            "4294967299"
        } else {
            "4294967298"
        }));
    }
    let mut contents = archive.contents().clone();
    let index = contents
        .fields
        .iter()
        .position(|field| {
            field.path.as_ref() == format!("{CONFIG}.view_objects.entries[0].config_id")
        })
        .unwrap();
    contents.fields.truncate(index + 1);
    let error = archive.with_contents(contents, LIMITS).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("configuration view dimensions 2 x 1")
    );
    assert_eq!(
        archive.to_bytes().unwrap(),
        selected(DisplayDocumentKind::Part, 0).bytes
    );
}

#[test]
fn every_truncated_minimal_tail_and_trailing_byte_fails_for_both_kinds() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(kind, 4);
        for end in 0..case.bytes.len() {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..end], case_context(case), LIMITS).is_err(),
                "{kind:?} prefix {end}"
            );
        }
        let mut trailing = case.bytes.to_vec();
        trailing.push(0);
        assert!(DisplayArchive::from_bytes(&trailing, case_context(case), LIMITS).is_err());
    }
}

#[test]
fn configuration_graphs_own_sources_and_concurrent_edits_match_the_independent_result() {
    for kind in [DisplayDocumentKind::Part, DisplayDocumentKind::Assembly] {
        let case = selected(kind, 0);
        let mut source = case.bytes.to_vec();
        let archive = DisplayArchive::from_bytes(&source, case_context(case), LIMITS).unwrap();
        source.fill(0);
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|_| scope.spawn(|| edited_tail(&archive).to_bytes().unwrap()))
                .collect();
            for worker in workers {
                assert_eq!(worker.join().unwrap(), selected(kind, 1).bytes);
            }
        });
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn unknown_generic_configuration_bodies_and_wrong_declared_assembly_classes_fail_explicitly() {
    for (kind, member, name, expected) in [
        (
            DisplayDocumentKind::Part,
            "tessellation",
            "UnknownConfiguration_c",
            "Unsupported",
        ),
        (
            DisplayDocumentKind::Assembly,
            "shd_data[0]",
            "moNodeName_c",
            "Invalid",
        ),
    ] {
        let archive = parsed(selected(kind, 0));
        let mut contents = archive.contents().clone();
        let path = format!("{CONFIG}.configs[0].{member}");
        let field = contents
            .fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap();
        let DisplayFieldValue::ObjectTag(tag) = &mut field.value else {
            panic!("object tag")
        };
        tag.runtime_class.as_mut().unwrap().name = name.into();
        tag.new_class.as_mut().unwrap().name = name.into();
        let error = archive.with_contents(contents, LIMITS).unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
        assert!(error.to_string().contains(name));
    }
}
