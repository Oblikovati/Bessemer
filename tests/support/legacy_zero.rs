//! Complete owned roots prove every remaining low-version branch down to zero.
use super::*;
#[path = "legacy_zero_compare.rs"]
mod compare;
#[path = "legacy_zero_members.rs"]
mod members;

fn zero_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-zero-roots.bin"
    ))
}

fn control_case(case: OldCase) -> OldCase {
    zero_cases()
        .into_iter()
        .find(|row| row.version == 596 && row.kind == case.kind && row.scenario == case.scenario)
        .unwrap()
}

#[test]
fn all_low_version_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(zero_cases().len(), 1394);
    assert_eq!(
        zero_cases().into_iter().map(check_case).sum::<usize>(),
        377537
    );
}

#[test]
fn omitted_members_leave_every_remaining_value_and_graph_tag_unchanged() {
    for case in zero_cases()
        .into_iter()
        .filter(|case| !matches!(case.scenario, 16..=20))
    {
        compare::check_unchanged_fields(case);
    }
}

#[test]
fn appearance_material_light_pane_bucket_and_edge_members_follow_their_own_gates() {
    for case in zero_cases() {
        members::check_members(case);
    }
}

fn expected_zero_edit(case: OldCase) -> OldCase {
    zero_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

const EDIT_SCENARIOS: [u8; 19] = [
    0, 3, 32, 48, 64, 80, 96, 112, 144, 146, 148, 150, 152, 160, 162, 164, 176, 180, 188,
];

#[test]
fn complete_low_version_edits_match_independent_wire_and_authentically_revert() {
    for case in zero_cases()
        .into_iter()
        .filter(|case| EDIT_SCENARIOS.contains(&case.scenario))
    {
        ordinary_text::check_complete_edit(case, expected_zero_edit(case));
    }
}

fn check_single_word(case: OldCase) {
    let source = parsed(case);
    let edited = source
        .with_contents(legacy_root_items::point_style_contents(&source), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_zero_edit(case).bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn one_global_point_word_can_be_edited_without_changing_any_other_bytes_at_version_zero() {
    for case in zero_cases()
        .into_iter()
        .filter(|case| case.version == 0 && case.scenario == 176)
    {
        check_single_word(case);
    }
}

#[test]
fn complete_low_version_facets_retain_every_independent_array_and_scalar() {
    for case in zero_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 96 | 97 | 112 | 113))
    {
        let archive = parsed(case);
        let facets = resumed_inline::inline_facets(&archive);
        assert_eq!(facets.len(), 2);
        for (index, facet) in facets.into_iter().enumerate() {
            legacy_fontless::check_fontless_facet(
                OldCase {
                    version: 596,
                    ..case
                },
                facet,
                index,
            );
        }
    }
}

#[test]
fn dense_empty_edge_arrays_fit_the_exact_twenty_or_twenty_four_byte_record_stride() {
    for case in zero_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 188 | 189))
    {
        members::check_minimal_edges(case);
    }
}

fn check_zero_transition(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(case.version + 1),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    if case.version == 0 {
        assert_eq!(result.unwrap().to_bytes().unwrap(), case.bytes);
    } else {
        assert!(
            result.is_err(),
            "version {} kind {:?}",
            case.version,
            case.kind
        );
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn version_zero_is_a_valid_context_and_each_new_member_is_required_at_its_boundary() {
    for case in zero_cases().into_iter().filter(|case| {
        matches!(
            case.version,
            0 | 144 | 172 | 211 | 370 | 380 | 437 | 561 | 595
        ) && case.scenario == 112
    }) {
        check_zero_transition(case);
    }
}

#[test]
fn uncounted_window_versions_remain_explicitly_unsupported_after_the_lower_guard_is_removed() {
    let case = zero_cases()
        .into_iter()
        .find(|case| case.version == 0 && case.scenario == 112)
        .unwrap();
    for version in [0x1fae, 0x1fce] {
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(version),
            ..case_context(case)
        };
        let error = DisplayArchive::from_bytes(case.bytes, context, LIMITS).unwrap_err();
        assert_eq!(error.class, FailureClass::Unsupported);
        assert!(error.cause.contains("uncounted"));
    }
}

#[test]
fn incomplete_zero_range_roots_never_publish_partial_results() {
    for case in zero_cases() {
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
fn low_version_inline_facets_and_extended_aliases_reconstruct_deterministically_across_threads() {
    for case in zero_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 112 | 180 | 188))
    {
        resumed_inline::concurrent_inline_case(case);
    }
}

#[test]
fn version_zero_and_control_documents_edit_reopen_and_restore_authentic_bytes() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-zero-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-zero-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-zero-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-zero-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-zero-rich-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-zero-rich-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-zero-rich-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-zero-rich-edited-document-assembly.bin"),
    ),
];
