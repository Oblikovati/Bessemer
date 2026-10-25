//! Complete roots before viewer metadata and polygon type payloads were stored.
use super::*;

fn component_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-component-roots.bin"
    ))
}

fn corresponding_viewer_case(case: OldCase) -> OldCase {
    component_cases()
        .into_iter()
        .find(|row| row.version == 989 && row.kind == case.kind && row.scenario == case.scenario)
        .unwrap()
}

fn check_direct_component_prefix(case: OldCase) {
    let old = parsed(case);
    if field(&old, "root.has_shaded_data") == Some(&DisplayFieldValue::I32(0)) {
        return;
    }
    let modern = parsed(corresponding_viewer_case(case));
    let prefix = "root.tessellation.components[0]";
    for member in ["comp_id_string", "comp_xform", "comp_bbox", "comp_viz_prop"] {
        let direct = field(&old, &format!("{prefix}.{member}"));
        assert!(direct.is_some());
        assert_eq!(
            direct,
            field(&modern, &format!("{prefix}.viewer_comp_data.{member}"))
        );
    }
    assert_eq!(object_tags(&old), object_tags(&modern));
}

fn object_tags(archive: &DisplayArchive) -> Vec<&DisplayFieldValue> {
    archive
        .contents()
        .fields
        .iter()
        .filter_map(|member| {
            matches!(member.value, DisplayFieldValue::ObjectTag(_)).then_some(&member.value)
        })
        .collect()
}

#[test]
fn direct_component_prefix_retains_identity_transform_bounds_and_all_graph_slots() {
    for case in component_cases().into_iter().filter(|case| {
        case.kind == DisplayDocumentKind::Assembly
            && case.version < 931
            && !matches!(case.scenario, 2 | 16..=20 | 34 | 50)
    }) {
        check_direct_component_prefix(case);
    }
}

#[test]
fn old_view_activity_keeps_the_physical_word_while_the_trace_exposes_its_logical_boolean() {
    for version in [915, 916] {
        check_activity_word(version);
    }
}

fn check_activity_word(version: u32) {
    let case = component_cases()
        .into_iter()
        .find(|row| {
            row.version == version && row.kind == DisplayDocumentKind::Part && row.scenario == 112
        })
        .unwrap();
    let archive = parsed(case);
    let path = "root.views[0].view_bucket.flags";
    assert_eq!(field(&archive, path), Some(&DisplayFieldValue::I32(-7)));
    let trace = String::from_utf8(bucket_trace::trace(&archive)).unwrap();
    assert!(
        trace
            .lines()
            .any(|line| line == format!("{path}=i:{}", if version < 916 { 1 } else { -7 }))
    );
    assert_eq!(archive.to_bytes().unwrap(), case.bytes);
}

#[test]
fn complete_early_component_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(component_cases().len(), 402);
    assert_eq!(
        component_cases().into_iter().map(check_case).sum::<usize>(),
        120703
    );
}

pub(super) fn check_component_members(case: OldCase) {
    let archive = parsed(case);
    legacy_model_viz::check_model_viz_members(case);
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".splitter_10.rows") {
            assert_eq!(
                field(&archive, &format!("{prefix}.browser_view_width")).is_some(),
                case.version > 903
            );
        }
        if let Some(prefix) = member.path.strip_suffix(".comp_part_tess_data") {
            check_component_identity(&archive, prefix, case.version);
        }
        if let Some(prefix) = member.path.strip_suffix(".smooth") {
            check_polygon_type(&archive, prefix, case.version);
        }
    }
}

fn check_component_identity(archive: &DisplayArchive, prefix: &str, version: u32) {
    let identity = if version < 931 {
        prefix.to_owned()
    } else {
        format!("{prefix}.viewer_comp_data")
    };
    for member in ["comp_id_string", "comp_xform", "comp_bbox", "comp_viz_prop"] {
        assert!(field(archive, &format!("{identity}.{member}")).is_some());
    }
    assert_eq!(
        field(
            archive,
            &format!("{prefix}.viewer_comp_data.ref_configuration")
        )
        .is_some(),
        version >= 931
    );
}

fn check_polygon_type(archive: &DisplayArchive, prefix: &str, version: u32) {
    let Some(DisplayFieldValue::I32(count)) = field(archive, &format!("{prefix}.count")) else {
        panic!("expected polygon count");
    };
    assert_eq!(
        field(archive, &format!("{prefix}.type")).is_some(),
        *count > 0 && version > 915
    );
}

#[test]
fn early_components_environment_words_and_polygon_types_obey_their_individual_gates() {
    for case in component_cases() {
        check_component_members(case);
    }
}

fn expected_component_edit(case: OldCase) -> OldCase {
    component_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_component_text_light_and_facet_edits_match_independent_bytes_and_revert() {
    for case in component_cases().into_iter().filter(|case| {
        matches!(
            case.scenario,
            0 | 3 | 32 | 48 | 64 | 80 | 96 | 112 | 144 | 146 | 148 | 150 | 152
        )
    }) {
        ordinary_text::check_complete_edit(case, expected_component_edit(case));
    }
}

#[test]
fn early_inline_facets_keep_complete_independent_records_without_edge_arrays() {
    for case in component_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 96 | 97 | 112 | 113))
    {
        let archive = parsed(case);
        let facets = resumed_inline::inline_facets(&archive);
        assert_eq!(facets.len(), 2);
        for (index, facet) in facets.into_iter().enumerate() {
            legacy_model_viz::check_model_viz_facet_record(
                facet,
                OldCase {
                    version: 989,
                    ..case
                },
                index,
            );
        }
    }
}

#[test]
fn truncated_early_component_roots_never_publish_partial_results() {
    for case in component_cases() {
        for length in (0..case.bytes.len())
            .step_by(173)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
    }
}

#[test]
fn new_contexts_require_missing_environment_polygon_and_viewer_fields() {
    for case in component_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 903 | 915 | 930) && case.scenario == 112)
    {
        let source = parsed(case);
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(case.version + 1),
            ..case_context(case)
        };
        check_component_context(&source, context, case);
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
    }
}

fn check_component_context(source: &DisplayArchive, context: DisplayArchiveContext, case: OldCase) {
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    if case.version == 930 && case.kind == DisplayDocumentKind::Part {
        assert_eq!(result.unwrap().to_bytes().unwrap(), case.bytes);
    } else {
        assert!(
            result.is_err(),
            "version {} kind {:?}",
            case.version,
            case.kind
        );
    }
}

#[test]
fn old_component_documents_edit_reopen_and_authentically_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-component-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-component-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-component-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-component-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-component-rich-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-component-rich-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-component-rich-document-assembly.bin"),
        include_bytes!(
            "../fixtures/display-lists/legacy-component-rich-edited-document-assembly.bin"
        ),
    ),
];
