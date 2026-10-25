//! The seven-version discarded edge word remains a physical archive field.
use super::*;
use bessemer::tessellation::FacetArchive;

fn discard_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-discard-roots.bin"
    ))
}

#[test]
fn complete_discard_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(discard_cases().len(), 262);
    assert_eq!(
        discard_cases().into_iter().map(check_case).sum::<usize>(),
        79282
    );
}

fn check_discard_words(case: OldCase) {
    let archive = parsed(case);
    let fields = &archive.contents().fields;
    let records = discard_record_count(&archive);
    let words: Vec<_> = fields
        .iter()
        .filter(|member| member.path.ends_with(".discarded_v0744"))
        .collect();
    assert_eq!(
        words.len(),
        usize::from((1860..=1866).contains(&case.version)) * records
    );
    let edited = case.scenario == 4 || case.scenario % 16 == 1;
    for word in words {
        assert_eq!(
            word.value,
            DisplayFieldValue::I32(i32::MAX - i32::from(edited))
        );
    }
}

fn check_hidden_silhouettes(case: OldCase) {
    let archive = parsed(case);
    legacy_font::check_font_gates(case);
    legacy_zone::check_zone_members(case);
    for member in &archive.contents().fields {
        if let Some(prefix) = member.path.strip_suffix(".has_2d_info") {
            assert_eq!(
                field(&archive, &format!("{prefix}.silh_h.count")).is_some(),
                case.version > 1857
            );
        }
    }
}

#[test]
fn every_edge_record_retains_the_extra_word_only_in_the_documented_interval() {
    for case in discard_cases() {
        check_discard_words(case);
        check_hidden_silhouettes(case);
    }
}

fn expected_discard_edit(case: OldCase) -> OldCase {
    discard_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_discarded_word_text_and_facet_edits_match_independent_bytes_and_revert() {
    for case in discard_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        ordinary_text::check_complete_edit(case, expected_discard_edit(case));
    }
}

fn check_discard_facets(case: OldCase) {
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
fn complete_old_discard_models_keep_independently_verified_inline_facets() {
    for case in discard_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_discard_facets(case);
    }
}

#[test]
fn truncated_discard_roots_never_publish_partial_results() {
    for case in discard_cases() {
        for length in (0..case.bytes.len())
            .step_by(149)
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
fn incompatible_contexts_reject_missing_or_extra_discarded_words_and_silhouette_lists() {
    for case in discard_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 1857 | 1859 | 1866) && case.scenario == 96)
    {
        let source = parsed(case);
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(case.version + 1),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(source.contents().clone(), context, LIMITS).is_err());
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
    }
}

fn check_discard_field_budget(case: OldCase) {
    let source = parsed(case);
    let count = source.contents().fields.len();
    let exact = DisplayArchiveLimits {
        maximum_fields: count,
        ..LIMITS
    };
    let rebuilt =
        DisplayArchive::new(source.contents().clone(), case_context(case), exact).unwrap();
    assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
    let short = DisplayArchiveLimits {
        maximum_fields: count - 1,
        ..LIMITS
    };
    assert!(DisplayArchive::new(source.contents().clone(), case_context(case), short).is_err());
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn exact_field_budgets_include_every_discarded_word_in_nested_buckets() {
    for case in discard_cases()
        .into_iter()
        .filter(|case| case.version == 1860 && case.scenario == 96)
    {
        check_discard_field_budget(case);
    }
}

#[test]
fn old_silhouette_and_discard_interval_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-discard-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-discard-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-discard-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-discard-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-discard-interval-document-part.bin"),
        include_bytes!(
            "../fixtures/display-lists/legacy-discard-interval-edited-document-part.bin"
        ),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-discard-interval-document-assembly.bin"),
        include_bytes!(
            "../fixtures/display-lists/legacy-discard-interval-edited-document-assembly.bin"
        ),
    ),
];

fn discard_record_count(archive: &DisplayArchive) -> usize {
    archive
        .contents()
        .fields
        .iter()
        .filter(|member| {
            member.path.contains(".edge_vis_data[") && member.path.ends_with(".unknown_0")
        })
        .count()
}
