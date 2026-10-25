//! Older display contexts can retain complete references to independently versioned face caches.
use super::*;
use bessemer::tessellation::{
    ArchiveString, FaceTessellationBlock, TessellationDirectory, TextEncoding,
};
use std::collections::BTreeMap;
#[path = "pre_facet_external_documents.rs"]
mod documents;
#[path = "tessellation.rs"]
mod tess_support;

fn external_cases() -> Vec<OldCase> {
    cases()
        .into_iter()
        .filter(|case| (3..=4).contains(&case.scenario))
        .collect()
}

fn directory() -> TessellationDirectory {
    let original = tess_support::directory();
    let mut contents = original.contents().clone();
    contents.entries[0].component_id = ArchiveString::new("Assem1", TextEncoding::Utf16).unwrap();
    original.with_contents(contents).unwrap()
}

fn blocks() -> BTreeMap<String, FaceTessellationBlock> {
    [("000-000-007".to_owned(), tess_support::block())].into()
}

fn external_fields(archive: &DisplayArchive) -> Vec<(&str, u32)> {
    archive
        .contents()
        .fields
        .iter()
        .filter_map(|field| {
            let DisplayFieldValue::ExternalFacet {
                component_id,
                index,
            } = &field.value
            else {
                return None;
            };
            Some((component_id.text(), *index))
        })
        .collect()
}

#[test]
fn older_assemblies_preserve_both_explicit_external_faces_and_non_null_part_objects() {
    assert_eq!(external_cases().len(), 16);
    for case in external_cases() {
        let archive = parsed(case);
        assert_eq!(external_fields(&archive), [("Assem1", 0), ("Assem1", 1)]);
        let path = "root.tessellation.components[0].comp_part_tess_data";
        let DisplayFieldValue::ObjectTag(tag) = field(&archive, path).unwrap() else {
            panic!("expected part object")
        };
        assert!(matches!(tag.target, DisplayObjectPointer::New(_)));
        assert_eq!(
            tag.runtime_class.as_ref().unwrap().name,
            "uoTempPartTessData_c"
        );
    }
}

fn check_links(case: OldCase) {
    let archive = parsed(case);
    let (directory, blocks) = (directory(), blocks());
    let result = archive
        .check_external_face_links(&directory, &blocks)
        .unwrap();
    assert_eq!(result.coverage().checked_face_references, 2);
    assert_eq!(result.coverage().unique_referenced_faces, 2);
    assert_eq!(result.links().len(), 2);
    for (position, link) in result.links().iter().enumerate() {
        assert_eq!(link.component_id, "Assem1");
        assert_eq!(link.block_name, "000-000-007");
        assert_eq!(link.index_in_block, 42);
        assert_eq!(link.face_index as usize, position);
        assert!(std::ptr::eq(
            link.facet,
            &blocks["000-000-007"].contents().entries[0].faces[position]
        ));
    }
}

#[test]
fn declared_cache_versions_resolve_older_display_faces_without_copying_or_guessing_geometry() {
    for case in external_cases() {
        check_links(case);
    }
}

#[test]
fn absent_directory_context_and_missing_cache_blocks_never_publish_complete_geometry() {
    for case in external_cases()
        .into_iter()
        .filter(|case| case.version < 3196)
    {
        let without_ids = DisplayArchiveContext {
            face_tessellation_components: Default::default(),
            ..case_context(case)
        };
        let failure = DisplayArchive::from_bytes(case.bytes, without_ids, LIMITS).unwrap_err();
        assert_eq!(failure.class, FailureClass::Unsupported);
        let archive = parsed(case);
        let failure = archive
            .check_external_face_links(&directory(), &BTreeMap::new())
            .unwrap_err();
        assert_eq!(failure.class, FailureClass::Missing);
        assert_eq!(archive.to_bytes().unwrap(), case.bytes);
    }
}

#[test]
fn independent_edits_keep_external_face_identity_and_authentically_revert() {
    for case in external_cases()
        .into_iter()
        .filter(|case| case.scenario == 3)
    {
        let source = parsed(case);
        let expected = cases()
            .into_iter()
            .find(|other| other.version == case.version && other.scenario == 4)
            .unwrap();
        let edited = source
            .with_contents(edited_contents(&source, &parsed(expected)), LIMITS)
            .unwrap();
        assert_eq!(edited.to_bytes().unwrap(), expected.bytes);
        assert_eq!(external_fields(&edited), external_fields(&source));
        let reverted = edited
            .with_contents(source.contents().clone(), LIMITS)
            .unwrap();
        assert_eq!(reverted.to_bytes().unwrap(), case.bytes);
        assert_eq!(source.to_bytes().unwrap(), case.bytes);
    }
}
