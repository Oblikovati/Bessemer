//! Early roots own one global item list, independently of the number of views.
use super::*;

fn root_item_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/legacy-root-items-roots.bin"
    ))
}

pub(super) fn edited_old_root(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = source.contents().clone();
    assert_eq!(contents.fields.len(), expected.contents().fields.len());
    for (target, desired) in contents.fields.iter_mut().zip(&expected.contents().fields) {
        assert_eq!(target.path, desired.path);
        if matches!(target.value, DisplayFieldValue::ObjectTag(_)) {
            assert_eq!(target.value, desired.value);
            continue;
        }
        if target.value != desired.value {
            target.value = desired.value.clone();
        }
    }
    contents
}

#[test]
fn complete_global_item_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(root_item_cases().len(), 282);
    assert_eq!(
        root_item_cases().into_iter().map(check_case).sum::<usize>(),
        76807
    );
}

fn corresponding_view_items(case: OldCase) -> OldCase {
    root_item_cases()
        .into_iter()
        .find(|row| row.version == 773 && row.kind == case.kind && row.scenario == case.scenario)
        .unwrap()
}

fn non_item_fields(archive: &DisplayArchive) -> Vec<&DisplayField> {
    archive
        .contents()
        .fields
        .iter()
        .filter(|member| {
            !member.path.starts_with("root.old_display_items.")
                && !member.path.starts_with("root.views[0].display_items.")
        })
        .collect()
}

fn normalized_items(archive: &DisplayArchive) -> Vec<(String, &DisplayFieldValue)> {
    archive
        .contents()
        .fields
        .iter()
        .filter_map(|member| {
            member
                .path
                .strip_prefix("root.old_display_items.")
                .or_else(|| member.path.strip_prefix("root.views[0].display_items."))
                .map(|suffix| (suffix.to_owned(), &member.value))
        })
        .collect()
}

#[test]
fn moving_the_complete_list_keeps_every_item_value_and_all_other_graph_fields_unchanged() {
    for case in root_item_cases().into_iter().filter(|case| {
        case.version < 773 && case.scenario < 160 && !matches!(case.scenario, 16..=20)
    }) {
        let old = parsed(case);
        let modern = parsed(corresponding_view_items(case));
        assert_eq!(normalized_items(&old), normalized_items(&modern));
        assert_eq!(non_item_fields(&old), non_item_fields(&modern));
    }
}

fn check_root_item_layout(case: OldCase) {
    let archive = parsed(case);
    check_item_counts(&archive, case.version);
    if case.version < 773 {
        check_item_position(&archive);
    }
}

fn check_item_counts(archive: &DisplayArchive, version: u32) {
    assert_eq!(
        field(archive, "root.old_display_items.count_word").is_some(),
        version < 773
    );
    let Some(DisplayFieldValue::I32(views)) = field(archive, "root.view_count") else {
        panic!("expected view count");
    };
    for index in 0..*views {
        let path = format!("root.views[{index}].display_items.count_word");
        assert_eq!(field(archive, &path).is_some(), version >= 773);
    }
}

fn check_item_position(archive: &DisplayArchive) {
    let fields = &archive.contents().fields;
    let items = fields
        .iter()
        .position(|member| member.path.as_ref() == "root.old_display_items.count_word")
        .unwrap();
    let environment = fields
        .iter()
        .position(|member| member.path.as_ref() == "root.user_model_env")
        .unwrap();
    assert!(items < environment);
}

#[test]
fn root_and_per_view_item_ownership_follow_the_actual_view_count_and_version() {
    for case in root_item_cases() {
        check_root_item_layout(case);
    }
}

fn expected_root_item_edit(case: OldCase) -> OldCase {
    root_item_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

#[test]
fn complete_global_item_text_geometry_light_and_facet_edits_match_independent_bytes_and_revert() {
    for case in root_item_cases()
        .into_iter()
        .filter(|case| EDIT_SCENARIOS.contains(&case.scenario))
    {
        ordinary_text::check_complete_edit(case, expected_root_item_edit(case));
    }
}

const EDIT_SCENARIOS: [u8; 17] = [
    0, 3, 32, 48, 64, 80, 96, 112, 144, 146, 148, 150, 152, 160, 162, 164, 176,
];

#[test]
fn populated_global_items_are_valid_without_any_view_and_shared_buckets_do_not_duplicate_them() {
    for case in root_item_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 160..=163))
    {
        let archive = parsed(case);
        let count = if case.scenario <= 161 { 0 } else { 2 };
        assert_eq!(
            field(&archive, "root.view_count"),
            Some(&DisplayFieldValue::I32(count))
        );
        assert!(normalized_items(&archive).len() > 10);
        if count == 2 {
            check_shared_bucket(&archive);
        }
    }
}

fn check_shared_bucket(archive: &DisplayArchive) {
    let Some(DisplayFieldValue::ObjectTag(first)) = field(archive, "root.views[0].view_bucket")
    else {
        panic!("expected first bucket tag");
    };
    let Some(DisplayFieldValue::ObjectTag(second)) = field(archive, "root.views[1].view_bucket")
    else {
        panic!("expected second bucket tag");
    };
    let DisplayObjectPointer::New(identity) = first.target else {
        panic!("expected loaded first bucket");
    };
    assert_eq!(second.target, DisplayObjectPointer::Reference(identity));
}

#[test]
fn extended_empty_and_populated_global_counts_keep_their_physical_headers() {
    for case in root_item_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 164 | 165 | 176 | 177))
    {
        let archive = parsed(case);
        assert_eq!(
            field(&archive, "root.old_display_items.count_word"),
            Some(&DisplayFieldValue::U16(u16::MAX))
        );
        let count = u32::from(case.scenario >= 176);
        assert_eq!(
            field(&archive, "root.old_display_items.count_dword"),
            Some(&DisplayFieldValue::U32(count))
        );
    }
}

#[test]
fn a_single_global_point_style_edit_matches_independent_wire_and_authentically_reverts() {
    for case in root_item_cases()
        .into_iter()
        .filter(|case| case.scenario == 176)
    {
        check_point_style_edit(case);
    }
}

fn point_style_contents(source: &DisplayArchive) -> DisplayArchiveContents {
    let mut contents = source.contents().clone();
    let path = "root.old_display_items.items[0].data.point_style";
    let target = contents
        .fields
        .iter_mut()
        .find(|member| member.path.as_ref() == path)
        .unwrap();
    assert_eq!(target.value, DisplayFieldValue::I32(-1));
    target.value = DisplayFieldValue::I32(23);
    contents
}

fn check_point_style_edit(case: OldCase) {
    let source = parsed(case);
    let edited = source
        .with_contents(point_style_contents(&source), LIMITS)
        .unwrap();
    assert_eq!(
        edited.to_bytes().unwrap(),
        expected_root_item_edit(case).bytes
    );
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn complete_inline_facets_remain_exact_after_the_list_moves_before_the_environment() {
    for case in root_item_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 96 | 97 | 112 | 113))
    {
        let archive = parsed(case);
        let facets = resumed_inline::inline_facets(&archive);
        assert_eq!(facets.len(), 2);
        for (index, facet) in facets.into_iter().enumerate() {
            legacy_curvature::check_curvature_facet(
                OldCase {
                    version: 773,
                    ..case
                },
                facet,
                index,
            );
        }
    }
}

#[test]
fn changing_item_ownership_context_rejects_missing_or_extra_list_fields() {
    for case in root_item_cases()
        .into_iter()
        .filter(|case| matches!(case.version, 772 | 773) && matches!(case.scenario, 0 | 2 | 112))
    {
        let source = parsed(case);
        let context = DisplayArchiveContext {
            version: DisplayVersion::new(if case.version == 772 { 773 } else { 772 }),
            ..case_context(case)
        };
        assert!(DisplayArchive::new(source.contents().clone(), context, LIMITS).is_err());
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn incomplete_root_item_streams_never_publish_partial_results() {
    for case in root_item_cases() {
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
fn global_items_and_inline_facets_reconstruct_deterministically_across_threads() {
    for case in root_item_cases()
        .into_iter()
        .filter(|case| case.scenario == 112)
    {
        resumed_inline::concurrent_inline_case(case);
    }
}

#[test]
fn global_and_per_view_item_documents_edit_reopen_and_revert() {
    for (kind, raw, expected) in DOCUMENTS {
        resumed_inline::check_owned_document(kind, raw, expected);
    }
}

const DOCUMENTS: [(DisplayDocumentKind, &[u8], &[u8]); 4] = [
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-root-items-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-root-items-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-root-items-document-assembly.bin"),
        include_bytes!("../fixtures/display-lists/legacy-root-items-edited-document-assembly.bin"),
    ),
    (
        DisplayDocumentKind::Part,
        include_bytes!("../fixtures/display-lists/legacy-root-items-rich-document-part.bin"),
        include_bytes!("../fixtures/display-lists/legacy-root-items-rich-edited-document-part.bin"),
    ),
    (
        DisplayDocumentKind::Assembly,
        include_bytes!("../fixtures/display-lists/legacy-root-items-rich-document-assembly.bin"),
        include_bytes!(
            "../fixtures/display-lists/legacy-root-items-rich-edited-document-assembly.bin"
        ),
    ),
];
