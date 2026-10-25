//! Zone words, exploded lists and complete textured-plane transforms at their bounds.
use super::*;
use bessemer::tessellation::FacetArchive;

fn zone_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-zone-roots.bin"
    ))
}

#[test]
fn complete_zone_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(zone_cases().len(), 262);
    assert_eq!(
        zone_cases().into_iter().map(check_case).sum::<usize>(),
        79234
    );
}

fn check_zone_members(case: OldCase) {
    let archive = parsed(case);
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".num_user_comp_bodies") {
            assert_eq!(
                field(&archive, &format!("{prefix}.is_a_zone")).is_some(),
                case.version > 1896
            );
        }
        if let Some(prefix) = member.path.strip_suffix(".has_2d_info") {
            for name in ["explode_v", "explode_h"] {
                assert_eq!(
                    field(&archive, &format!("{prefix}.{name}.count")).is_some(),
                    case.version > 1875
                );
            }
        }
    }
    check_zone_plane_count(&archive, case.version);
}

fn check_zone_plane_count(archive: &DisplayArchive, version: u32) {
    let shaded = field(archive, "root.old_lights.has_lights").is_some();
    let count = field(archive, "root.textured_planes.count");
    if version > 1899 && shaded {
        assert_eq!(count, Some(&DisplayFieldValue::I32(2)));
    } else {
        assert!(count.is_none());
    }
}

#[test]
fn zone_exploded_and_plane_fields_follow_their_independent_storage_bounds() {
    for case in zone_cases() {
        legacy_font::check_font_gates(case);
        check_zone_members(case);
    }
}

fn zone_plane_traces() -> Vec<&'static [u8]> {
    let mut remaining =
        include_bytes!("../fixtures/display-lists/legacy-zone-plane-traces.bin").as_slice();
    let mut records = Vec::new();
    while !remaining.is_empty() {
        let (word, tail) = remaining.split_at(4);
        let length = u32::from_le_bytes(word.try_into().unwrap()) as usize;
        let (trace, rest) = tail.split_at(length);
        records.push(trace);
        remaining = rest;
    }
    records
}

fn zone_plane_trace(archive: &DisplayArchive) -> Vec<u8> {
    let mut rows = view_trace::Trace::new();
    for member in &archive.contents().fields {
        if member.path.starts_with("root.textured_planes.")
            && matches!(member.value, DisplayFieldValue::Transform(_))
        {
            item_trace::insert(&mut rows, &member.path, &member.value);
        }
    }
    rows.into_iter()
        .filter(|(_, value)| !value.starts_with("a:"))
        .map(|(path, value)| format!("{path}={value}\n"))
        .collect::<String>()
        .into_bytes()
}

#[test]
fn every_old_plane_transform_matches_independent_source_fields() {
    let traces = zone_plane_traces();
    assert_eq!(traces.len(), zone_cases().len());
    let mut fields = 0;
    for (case, expected) in zone_cases().into_iter().zip(traces) {
        assert_eq!(zone_plane_trace(&parsed(case)), expected);
        fields += expected.iter().filter(|byte| **byte == b'\n').count();
    }
    assert_eq!(fields, 2112);
}

fn expected_zone_edit(case: OldCase) -> OldCase {
    zone_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_zone_plane_and_facet_edits_match_independent_bytes_and_revert() {
    for case in zone_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_zone_edit(case));
    }
}

fn check_zone_facets(case: OldCase) {
    let source = parsed(case);
    let facets = resumed_inline::inline_facets(&source);
    assert_eq!(facets.len(), 2);
    let context = legacy_rendering::facet_context(case.version);
    for (index, facet) in facets.into_iter().enumerate() {
        let expected =
            legacy_font::expected_font_facet(2011, index, usize::from(case.scenario % 16 == 1));
        assert_eq!(facet, expected.contents());
        let rebuilt = FacetArchive::new(context, facet.clone()).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), expected.to_bytes().unwrap());
    }
}

#[test]
fn old_zone_models_retain_complete_independently_verified_inline_faces() {
    for case in zone_cases().into_iter().filter(|case| case.scenario >= 96) {
        check_zone_facets(case);
    }
}

#[test]
fn truncated_zone_roots_never_publish_partial_results() {
    for case in zone_cases() {
        for length in (0..case.bytes.len())
            .step_by(139)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
    }
}

fn check_zone_transition(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(case.version + 1),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    if case.version == 1896 && case.kind == DisplayDocumentKind::Part {
        assert_eq!(result.unwrap().to_bytes().unwrap(), case.bytes);
    } else {
        assert!(result.is_err());
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn new_contexts_require_zone_plane_and_exploded_fields_only_on_reachable_paths() {
    for case in zone_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 1875 | 1896 | 1899) && case.scenario == 96)
    {
        check_zone_transition(case);
    }
}

#[test]
fn old_and_populated_plane_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-zone-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-zone-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-zone-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-zone-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-zone-planes-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-zone-planes-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-zone-planes-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-zone-planes-edited-document-assembly.bin"),
    ),
];
