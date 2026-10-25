//! Complete inline facets resume before the unresolved object-backed interval.
use super::*;
use bessemer::tessellation::{
    ArchiveVersion, FacetArchive, FacetArchiveMode, FacetContainer, TessellationContext,
    TessellationLimits,
};

fn resumed_cases() -> Vec<OldCase> {
    complete_cases::decode(include_bytes!(
        "../fixtures/display-lists/resumed-inline-roots.bin"
    ))
}

#[test]
fn complete_resumed_roots_match_independent_fields_and_forced_bytes() {
    assert_eq!(resumed_cases().len(), 130);
    assert_eq!(
        resumed_cases().into_iter().map(check_case).sum::<usize>(),
        28105
    );
}

fn facet_context() -> TessellationContext {
    TessellationContext {
        archive_version: ArchiveVersion::new(2596),
        mode: FacetArchiveMode::Full,
    }
}

const FACET_BYTES: [[&[u8]; 2]; 2] = [
    [
        include_bytes!("../fixtures/display-lists/resumed-inline-facet-0-0.bin"),
        include_bytes!("../fixtures/display-lists/resumed-inline-facet-0-1.bin"),
    ],
    [
        include_bytes!("../fixtures/display-lists/resumed-inline-facet-1-0.bin"),
        include_bytes!("../fixtures/display-lists/resumed-inline-facet-1-1.bin"),
    ],
];

fn independent_facet(enabled: usize, edited: usize) -> FacetArchive {
    let limits = TessellationLimits {
        maximum_components: 0,
        maximum_faces: 1,
        maximum_array_elements: 1000,
    };
    FacetArchive::from_bytes(FACET_BYTES[enabled][edited], facet_context(), limits).unwrap()
}

pub(super) fn inline_facets(archive: &DisplayArchive) -> Vec<&FacetContainer> {
    archive
        .contents()
        .fields
        .iter()
        .filter_map(|field| match &field.value {
            DisplayFieldValue::Facet(value) => Some(value.as_ref()),
            _ => None,
        })
        .collect()
}

pub(super) fn check_inline_payloads(case: OldCase) {
    let archive = parsed(case);
    let facets = inline_facets(&archive);
    assert_eq!(facets.len(), 2);
    let edited = usize::from(case.scenario % 16 == 1);
    for (index, facet) in facets.into_iter().enumerate() {
        let expected = independent_facet(index, edited);
        assert_eq!(facet, expected.contents());
        let forced = FacetArchive::new(facet_context(), facet.clone()).unwrap();
        assert_eq!(forced.to_bytes().unwrap(), FACET_BYTES[index][edited]);
        assert_eq!(facet.stamp, Some(0xdeadbeef - edited as u32));
        assert!(facet.legacy_triangles.is_some() && facet.old_tolerance.is_some());
    }
}

#[test]
fn both_part_and_assembly_inline_faces_match_complete_independent_facet_records() {
    for case in resumed_cases()
        .into_iter()
        .filter(|case| case.scenario >= 96)
    {
        check_inline_payloads(case);
    }
}

pub(super) fn resumed_edited_contents(
    source: &DisplayArchive,
    expected: &DisplayArchive,
) -> DisplayArchiveContents {
    let mut contents = legacy_edges::edge_edited_contents(source, expected);
    for target in &mut contents.fields {
        if resumed_edit_field(target) {
            target.value = field(expected, &target.path).unwrap().clone();
        }
    }
    contents
}

fn resumed_edit_field(field: &DisplayField) -> bool {
    matches!(field.value, DisplayFieldValue::Facet(_))
        || (field.path.starts_with("root.old_lights.")
            && !matches!(field.value, DisplayFieldValue::ObjectTag(_)))
        || field.path.contains(".dl_ghost_bucket.items[")
        || resumed_polygon_field(&field.path)
        || (field.path.starts_with("root.textured_planes.")
            && matches!(field.value, DisplayFieldValue::Transform(_)))
        || [10, 12, 13, 14, 15, 16, 17, 18].into_iter().any(|index| {
            field
                .path
                .contains(&format!("display_items.items[{index}].data."))
        })
}

fn expected_resumed_edit(case: OldCase) -> OldCase {
    resumed_cases()
        .into_iter()
        .find(|row| {
            row.version == case.version
                && row.kind == case.kind
                && row.scenario == case.scenario + 1
        })
        .unwrap()
}

fn check_resumed_edit(case: OldCase) {
    let source = parsed(case);
    let expected_case = expected_resumed_edit(case);
    let expected = parsed(expected_case);
    let edited = source
        .with_contents(resumed_edited_contents(&source, &expected), LIMITS)
        .unwrap();
    assert_eq!(edited.to_bytes().unwrap(), expected_case.bytes);
    let reverted = edited
        .with_contents(source.contents().clone(), LIMITS)
        .unwrap();
    assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn facet_stamp_polygon_and_text_edits_match_independent_bytes_and_authentically_revert() {
    for case in resumed_cases()
        .into_iter()
        .filter(|case| matches!(case.scenario, 0 | 3 | 32 | 48 | 64 | 80 | 96 | 112))
    {
        check_resumed_edit(case);
    }
}

fn check_resumed_transition(case: OldCase) {
    let source = parsed(case);
    let context = DisplayArchiveContext {
        version: DisplayVersion::new(2597),
        ..case_context(case)
    };
    let result = DisplayArchive::new(source.contents().clone(), context, LIMITS);
    if case.scenario >= 96 {
        let failure = result.unwrap_err();
        assert_eq!(failure.class, FailureClass::Unsupported);
        assert!(failure.cause.contains("object"));
    } else {
        assert_eq!(result.unwrap().to_bytes().unwrap(), case.bytes);
    }
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn the_object_interval_rejects_only_inline_payloads_instead_of_empty_and_cached_roots() {
    for case in resumed_cases()
        .into_iter()
        .filter(|case| case.version == 2596)
    {
        check_resumed_transition(case);
    }
}

#[test]
fn declared_external_face_context_cannot_silently_reinterpret_inline_assembly_bytes() {
    for case in resumed_cases()
        .into_iter()
        .filter(|case| case.kind == DisplayDocumentKind::Assembly && case.scenario >= 96)
    {
        let context = DisplayArchiveContext {
            face_tessellation_components: std::sync::Arc::new(["Assem1".to_owned()].into()),
            ..case_context(case)
        };
        assert!(DisplayArchive::from_bytes(case.bytes, context, LIMITS).is_err());
        assert_eq!(parsed(case).to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn incomplete_resumed_inline_roots_never_publish_partial_fields() {
    for case in resumed_cases() {
        for length in (0..case.bytes.len())
            .step_by(83)
            .chain(case.bytes.len() - 8..case.bytes.len())
        {
            assert!(
                DisplayArchive::from_bytes(&case.bytes[..length], case_context(case), LIMITS)
                    .is_err()
            );
        }
    }
}

fn concurrent_inline_case(case: OldCase) {
    let source = parsed(case);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            let archive = &source;
            scope.spawn(move || {
                let rebuilt =
                    DisplayArchive::new(archive.contents().clone(), case_context(case), LIMITS)
                        .unwrap();
                assert_eq!(rebuilt.to_bytes().unwrap(), case.bytes);
            });
        }
    });
    assert_eq!(source.to_bytes().unwrap(), case.bytes);
}

#[test]
fn pre_object_inline_facets_are_deterministic_across_concurrent_constructors() {
    for case in resumed_cases()
        .into_iter()
        .filter(|case| case.scenario == 112)
    {
        concurrent_inline_case(case);
    }
}

#[path = "resumed_inline_documents.rs"]
mod documents;

pub(super) fn check_owned_document(kind: DisplayDocumentKind, raw: &[u8], expected: &[u8]) {
    documents::check_document_edit(kind, raw, expected);
}

fn resumed_polygon_field(path: &str) -> bool {
    path.contains(".view_bucket.")
        && (path.ends_with(".node_id")
            || path.contains(".points_f32[")
            || path.contains(".points["))
}
