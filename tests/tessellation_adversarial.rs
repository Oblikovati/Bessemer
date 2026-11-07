//! Truncation, budgets, physical field presence, and immutable failure boundaries.
#[path = "support/tessellation.rs"]
mod support;
use bessemer::tessellation::*;
use support::*;

#[test]
fn every_incomplete_facet_and_directory_prefix_fails() {
    for end in 0..MODERN.len() {
        assert!(
            FacetArchive::from_bytes(&MODERN[..end], modern_context(), LIMITS).is_err(),
            "facet prefix {end}"
        );
    }
    for end in 0..DIRECTORY.len() {
        assert!(
            TessellationDirectory::from_bytes(&DIRECTORY[..end], modern_context(), LIMITS).is_err(),
            "directory prefix {end}"
        );
    }
    let mut trailing = MODERN.to_vec();
    trailing.push(0);
    assert!(FacetArchive::from_bytes(&trailing, modern_context(), LIMITS).is_err());
}

#[test]
fn block_eof_is_valid_only_at_complete_component_boundaries() {
    for end in 1..BLOCK.len() {
        assert!(
            FaceTessellationBlock::from_bytes(&BLOCK[..end], modern_context(), LIMITS).is_err(),
            "block prefix {end}"
        );
    }
    let empty = FaceTessellationBlock::from_bytes(&[], modern_context(), LIMITS).unwrap();
    assert!(empty.contents().entries.is_empty());
    assert!(empty.to_bytes().unwrap().is_empty());
    let doubled: Vec<u8> = BLOCK.iter().chain(BLOCK).copied().collect();
    assert_eq!(
        FaceTessellationBlock::from_bytes(&doubled, modern_context(), LIMITS)
            .unwrap()
            .contents()
            .entries
            .len(),
        2
    );
}

#[test]
fn required_budgets_and_impossible_counts_fail_before_payload_allocation() {
    let mut limits = LIMITS;
    limits.maximum_components = 0;
    assert!(TessellationDirectory::from_bytes(DIRECTORY, modern_context(), limits).is_err());
    assert!(FaceTessellationBlock::from_bytes(BLOCK, modern_context(), limits).is_err());
    limits = LIMITS;
    limits.maximum_faces = 0;
    assert!(FacetArchive::from_bytes(MODERN, modern_context(), limits).is_err());
    limits = LIMITS;
    limits.maximum_array_elements = 0;
    assert!(FacetArchive::from_bytes(MODERN, modern_context(), limits).is_err());
    for count in [i32::MIN, -1, i32::MAX] {
        let mut bytes = DIRECTORY.to_vec();
        bytes[8..12].copy_from_slice(&count.to_le_bytes());
        assert!(TessellationDirectory::from_bytes(&bytes, modern_context(), LIMITS).is_err());
        let mut bytes = BLOCK.to_vec();
        bytes[12..16].copy_from_slice(&count.to_le_bytes());
        assert!(FaceTessellationBlock::from_bytes(&bytes, modern_context(), LIMITS).is_err());
    }
}

#[test]
fn unresolved_legacy_objects_fail_explicitly_and_empty_results_remain_complete() {
    for version in [0xa25, 0xb00, 0xc7b] {
        let context = context(version, FacetArchiveMode::Full);
        let error = FacetArchive::from_bytes(&[0; 8], context, LIMITS).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
        assert!(
            FaceTessellationBlock::from_bytes(&[], context, LIMITS)
                .unwrap()
                .contents()
                .entries
                .is_empty()
        );
    }
    let mut bytes = DIRECTORY.to_vec();
    bytes[..4].copy_from_slice(&3i32.to_le_bytes());
    assert_eq!(
        TessellationDirectory::from_bytes(&bytes, modern_context(), LIMITS)
            .unwrap_err()
            .class,
        bessemer::FailureClass::Unsupported
    );
}

#[test]
fn constructed_records_require_the_exact_version_and_flag_selected_shape() {
    let source = parse(MODERN);
    let mut face = source.contents().clone();
    face.stamp = None;
    assert!(source.with_contents(face).is_err());
    let mut face = source.contents().clone();
    face.texture_coords.as_mut().unwrap().flag = 0;
    assert!(source.with_contents(face).is_err());
    let mut face = source.contents().clone();
    face.edge_points = None;
    assert!(source.with_contents(face).is_err());
    let mut face = source.contents().clone();
    face.vertex_layout = FacetVertexLayout::Compact(DynamicArray::Absent);
    assert!(source.with_contents(face).is_err());
    let mut face = source.contents().clone();
    face.curvature = FacetCurvature::Legacy(DynamicArray::Absent);
    assert!(source.with_contents(face).is_err());
    assert_eq!(source.to_bytes().unwrap(), MODERN);
}

#[test]
fn block_link_mismatch_missing_duplicate_and_context_errors_are_typed() {
    let source = block();
    for (index, hash) in [(41, 0x123456789abcdef0), (42, 1)] {
        let mut contents = source.contents().clone();
        contents.entries[0].index_in_block = index;
        contents.entries[0].hash_key = hash;
        let changed = source.with_contents(contents).unwrap();
        assert!(
            directory()
                .check_block_links("000-000-007", &changed)
                .is_err()
        );
    }
    let mut contents = source.contents().clone();
    contents.entries.push(contents.entries[0].clone());
    assert!(
        directory()
            .check_block_links("000-000-007", &source.with_contents(contents).unwrap())
            .is_err()
    );
    let wrong = FaceTessellationBlock::new(
        context(1, FacetArchiveMode::Full),
        FaceTessellationBlockContents { entries: vec![] },
    )
    .unwrap();
    assert!(
        directory()
            .check_block_links("000-000-007", &wrong)
            .is_err()
    );
}

#[test]
fn byte_mutation_never_publishes_unrebuildable_successes() {
    for index in 0..MODERN.len() {
        let mut mutated = MODERN.to_vec();
        mutated[index] ^= 0xff;
        if let Ok(archive) = FacetArchive::from_bytes(&mutated, modern_context(), LIMITS) {
            assert_eq!(
                FacetArchive::new(archive.context(), archive.contents().clone())
                    .unwrap()
                    .to_bytes()
                    .unwrap(),
                mutated,
                "mutation {index}"
            );
        }
    }
}

#[test]
fn invalid_layout_and_directory_representations_include_the_offending_context() {
    let source = parse(MODERN);
    for mode in [0, 4] {
        let mut face = source.contents().clone();
        let FacetVertexLayout::Styled { mode: stored, .. } = &mut face.vertex_layout else {
            panic!("styled fixture")
        };
        *stored = mode;
        assert!(
            source
                .with_contents(face)
                .unwrap_err()
                .cause
                .contains(&format!("mode {mode}"))
        );
    }
    let mut face = source.contents().clone();
    face.vertex_layout = FacetVertexLayout::Legacy(DynamicArray::Absent);
    assert!(source.with_contents(face).is_err());
    let source = directory();
    let mut contents = source.contents().clone();
    contents.entries[0].tessellation_numbers = ComponentTessellationNumbers::Absent;
    assert!(source.with_contents(contents).is_err());
}
