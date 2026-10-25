//! Exact cache identities and requested positions are checked before borrowed facets are published.
use bessemer::FailureClass;
use bessemer::display_lists::{
    DisplayArchive, DisplayArchiveContext, DisplayDocumentKind, ExternalFaceLink,
    ExternalFaceLinkCoverage, ExternalFaceLinks,
};
use bessemer::tessellation::*;
use std::collections::BTreeMap;
#[allow(
    dead_code,
    reason = "Shared helpers also serve scalar and archive edit tests."
)]
#[path = "support/display_archive.rs"]
mod archive_support;
#[path = "support/tessellation.rs"]
mod tess_support;

const BLOCK_NAME: &str = "000-000-007";

fn archive() -> DisplayArchive {
    let context = DisplayArchiveContext {
        kind: DisplayDocumentKind::Assembly,
        face_tessellation_components: std::sync::Arc::new(["Assem1".to_owned()].into()),
        ..archive_support::context()
    };
    DisplayArchive::from_bytes(
        include_bytes!("fixtures/display-lists/external-shaded-assembly.bin"),
        context,
        archive_support::LIMITS,
    )
    .unwrap()
}

fn directory() -> TessellationDirectory {
    let original = tess_support::directory();
    let mut contents = original.contents().clone();
    contents.entries[0].component_id = ArchiveString::new("Assem1", TextEncoding::Utf16).unwrap();
    original.with_contents(contents).unwrap()
}

fn blocks() -> BTreeMap<String, FaceTessellationBlock> {
    [(BLOCK_NAME.to_owned(), tess_support::block())].into()
}

fn empty_name() -> ArchiveString {
    ArchiveString::new("", TextEncoding::Utf16).unwrap()
}

#[test]
fn exact_links_select_stored_component_indices_and_borrow_the_correct_facets() {
    let (archive, directory, blocks) = (archive(), directory(), blocks());
    let linked = archive
        .check_external_face_links(&directory, &blocks)
        .unwrap();
    assert_eq!(
        linked.coverage(),
        ExternalFaceLinkCoverage {
            checked_blocks: 1,
            checked_directory_references: 1,
            directory_entries_without_block_names: 0,
            available_block_components: 1,
            unreferenced_block_components: 0,
            checked_face_references: 2,
            unique_referenced_faces: 2,
            available_block_faces: 2,
            unreferenced_block_faces: 0,
        }
    );
    assert_eq!(linked.links().len(), 2);
    for (position, link) in linked.links().iter().enumerate() {
        assert_eq!(link.component_id, "Assem1");
        assert_eq!(link.block_name, BLOCK_NAME);
        assert_eq!(link.index_in_block, 42);
        assert_eq!(link.face_index as usize, position);
        assert!(link.display_path.ends_with(".facet_container"));
        assert!(std::ptr::eq(
            link.facet,
            &blocks[BLOCK_NAME].contents().entries[0].faces[position]
        ));
    }
}

#[test]
fn duplicate_declarations_with_the_same_target_do_not_reload_or_duplicate_geometry() {
    let (archive, directory, blocks) = (archive(), directory(), blocks());
    let mut contents = directory.contents().clone();
    contents.entries.push(contents.entries[0].clone());
    let duplicated = directory.with_contents(contents).unwrap();
    let linked = archive
        .check_external_face_links(&duplicated, &blocks)
        .unwrap();
    assert_eq!(linked.coverage().checked_directory_references, 2);
    assert_eq!(linked.coverage().available_block_components, 1);
    assert_eq!(linked.coverage().unique_referenced_faces, 2);
    assert_eq!(linked.links().len(), 2);
}

#[test]
fn unused_cache_components_blocks_and_unlocated_declarations_remain_explicit() {
    let (archive, directory, mut blocks) = (archive(), directory(), blocks());
    let mut contents = blocks[BLOCK_NAME].contents().clone();
    let mut extra = contents.entries[0].clone();
    extra.index_in_block = 43;
    extra.hash_key += 1;
    contents.entries.push(extra);
    blocks.insert(
        BLOCK_NAME.into(),
        blocks[BLOCK_NAME].with_contents(contents).unwrap(),
    );
    blocks.insert("unused".into(), tess_support::block());
    let mut contents = directory.contents().clone();
    let mut unlocated = contents.entries[0].clone();
    unlocated.component_id = ArchiveString::new("unused component", TextEncoding::Utf16).unwrap();
    unlocated.block_name = empty_name();
    unlocated.index_in_block = -1;
    contents.entries.push(unlocated);
    let directory = directory.with_contents(contents).unwrap();
    let linked = archive
        .check_external_face_links(&directory, &blocks)
        .unwrap();
    assert_eq!(linked.coverage().checked_blocks, 2);
    assert_eq!(linked.coverage().directory_entries_without_block_names, 1);
    assert_eq!(linked.coverage().unreferenced_block_components, 2);
    assert_eq!(linked.coverage().available_block_faces, 6);
    assert_eq!(linked.coverage().unreferenced_block_faces, 4);
}

#[test]
fn ambiguous_component_targets_never_select_the_first_directory_row() {
    let (archive, directory, mut blocks) = (archive(), directory(), blocks());
    let mut contents = blocks[BLOCK_NAME].contents().clone();
    let mut second = contents.entries[0].clone();
    second.index_in_block = 43;
    second.hash_key += 1;
    contents.entries.push(second.clone());
    blocks.insert(
        BLOCK_NAME.into(),
        blocks[BLOCK_NAME].with_contents(contents).unwrap(),
    );
    let mut contents = directory.contents().clone();
    let mut alternative = contents.entries[0].clone();
    alternative.index_in_block = second.index_in_block;
    alternative.hash_key = second.hash_key;
    contents.entries.push(alternative);
    let directory = directory.with_contents(contents).unwrap();
    let error = archive
        .check_external_face_links(&directory, &blocks)
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Unsupported);
    assert!(error.cause.contains("Assem1"));
    assert!(error.cause.contains("unambiguous block/index/hash"));
}

#[test]
fn every_named_block_is_required_even_when_the_display_has_no_external_facets() {
    let inline = DisplayArchive::from_bytes(
        include_bytes!("fixtures/display-lists/inline-shaded-part.bin"),
        archive_support::context(),
        archive_support::LIMITS,
    )
    .unwrap();
    let error = inline
        .check_external_face_links(&directory(), &BTreeMap::new())
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    assert_eq!(error.entity, format!("FaceTessellations/{BLOCK_NAME}"));
    let mut blocks = blocks();
    let mut contents = blocks[BLOCK_NAME].contents().clone();
    contents.entries[0].hash_key ^= 1;
    blocks.insert(
        BLOCK_NAME.into(),
        blocks[BLOCK_NAME].with_contents(contents).unwrap(),
    );
    assert_eq!(
        inline
            .check_external_face_links(&directory(), &blocks)
            .unwrap_err()
            .class,
        FailureClass::Invalid
    );
}

#[test]
fn requested_unlocated_or_undeclared_components_cannot_publish_links() {
    let (archive, directory) = (archive(), directory());
    let mut contents = directory.contents().clone();
    contents.entries[0].block_name = empty_name();
    let unlocated = directory.with_contents(contents).unwrap();
    let error = archive
        .check_external_face_links(&unlocated, &BTreeMap::new())
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("empty block name"));
    let mut contents = directory.contents().clone();
    contents.entries.clear();
    let empty = directory.with_contents(contents).unwrap();
    let error = archive
        .check_external_face_links(&empty, &blocks())
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("matching directory declaration"));
}

#[test]
fn a_late_missing_face_discards_partial_links_and_preserves_every_input() {
    let (archive, directory, mut blocks) = (archive(), directory(), blocks());
    let mut contents = blocks[BLOCK_NAME].contents().clone();
    contents.entries[0].faces.pop();
    blocks.insert(
        BLOCK_NAME.into(),
        blocks[BLOCK_NAME].with_contents(contents).unwrap(),
    );
    let originals = (
        archive.to_bytes().unwrap(),
        directory.to_bytes().unwrap(),
        blocks[BLOCK_NAME].to_bytes().unwrap(),
    );
    let error = archive
        .check_external_face_links(&directory, &blocks)
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("face index 1"));
    assert_eq!(error.operation, "check external face links");
    assert_eq!(
        (
            archive.to_bytes().unwrap(),
            directory.to_bytes().unwrap(),
            blocks[BLOCK_NAME].to_bytes().unwrap()
        ),
        originals
    );
}

#[test]
fn mismatched_versions_and_duplicate_indices_fail_before_face_resolution() {
    let (archive, directory, mut blocks) = (archive(), directory(), blocks());
    let wrong_context = TessellationContext {
        archive_version: ArchiveVersion::new(18000),
        ..directory.context()
    };
    let wrong = TessellationDirectory::new(wrong_context, directory.contents().clone()).unwrap();
    assert!(
        archive
            .check_external_face_links(&wrong, &blocks)
            .unwrap_err()
            .cause
            .contains("expected directory version")
    );
    let mut contents = blocks[BLOCK_NAME].contents().clone();
    contents.entries.push(contents.entries[0].clone());
    blocks.insert(
        BLOCK_NAME.into(),
        blocks[BLOCK_NAME].with_contents(contents).unwrap(),
    );
    assert!(
        archive
            .check_external_face_links(&directory, &blocks)
            .unwrap_err()
            .cause
            .contains("duplicate block index")
    );
}

#[test]
fn completed_empty_link_sets_are_distinct_from_missing_requested_geometry() {
    let inline = DisplayArchive::from_bytes(
        include_bytes!("fixtures/display-lists/inline-shaded-part.bin"),
        archive_support::context(),
        archive_support::LIMITS,
    )
    .unwrap();
    let original = directory();
    let mut contents = original.contents().clone();
    contents.entries.clear();
    let empty = original.with_contents(contents).unwrap();
    let blocks = BTreeMap::new();
    let result = inline.check_external_face_links(&empty, &blocks).unwrap();
    assert!(result.links().is_empty());
    assert_eq!(result.coverage().checked_face_references, 0);
    assert_eq!(result.coverage().checked_blocks, 0);
}

#[test]
fn borrowed_results_are_send_sync_and_concurrent_checks_leave_snapshots_isolated() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ExternalFaceLinkCoverage>();
    assert_send_sync::<ExternalFaceLink<'static>>();
    assert_send_sync::<ExternalFaceLinks<'static>>();
    let (archive, directory, blocks) = (archive(), directory(), blocks());
    let result = archive
        .check_external_face_links(&directory, &blocks)
        .unwrap();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    archive
                        .check_external_face_links(&directory, &blocks)
                        .unwrap()
                        .coverage()
                })
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), result.coverage());
        }
    });
    let mut contents = blocks[BLOCK_NAME].contents().clone();
    contents.entries[0].faces.clear();
    let changed = blocks[BLOCK_NAME].with_contents(contents).unwrap();
    assert!(changed.contents().entries[0].faces.is_empty());
    assert_eq!(result.links().len(), 2);
    assert!(std::ptr::eq(
        result.links()[1].facet,
        &blocks[BLOCK_NAME].contents().entries[0].faces[1]
    ));
}
