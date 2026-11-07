use super::{ExternalFaceLink, ExternalFaceLinkCoverage, external_face_errors::link_error};
use crate::{
    FailureClass, Result,
    tessellation::{BlockLinkCoverage, FaceTessellationBlock, TessellationDirectory},
};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn coverage(
    directory: &TessellationDirectory,
    blocks: &BTreeMap<String, FaceTessellationBlock>,
    checked: &[BlockLinkCoverage],
    links: &[ExternalFaceLink<'_>],
) -> Result<ExternalFaceLinkCoverage> {
    let mut counts = initial_coverage(blocks.len());
    directory_counts(&mut counts, directory, checked)?;
    counts.checked_face_references = links.len();
    counts.unique_referenced_faces = unique_faces(links);
    counts.available_block_faces = sum_counts(blocks.values().flat_map(|block| {
        block
            .contents()
            .entries
            .iter()
            .map(|entry| entry.faces.len())
    }))?;
    counts.unreferenced_block_faces =
        unreferenced_faces(counts.available_block_faces, counts.unique_referenced_faces)?;
    Ok(counts)
}

fn initial_coverage(block_count: usize) -> ExternalFaceLinkCoverage {
    ExternalFaceLinkCoverage {
        checked_blocks: block_count,
        checked_directory_references: 0,
        directory_entries_without_block_names: 0,
        available_block_components: 0,
        unreferenced_block_components: 0,
        checked_face_references: 0,
        unique_referenced_faces: 0,
        available_block_faces: 0,
        unreferenced_block_faces: 0,
    }
}

fn directory_counts(
    counts: &mut ExternalFaceLinkCoverage,
    directory: &TessellationDirectory,
    checked: &[BlockLinkCoverage],
) -> Result<()> {
    counts.checked_directory_references =
        sum_counts(checked.iter().map(|value| value.checked_references))?;
    counts.available_block_components =
        sum_counts(checked.iter().map(|value| value.available_entries))?;
    counts.unreferenced_block_components =
        sum_counts(checked.iter().map(|value| value.unreferenced_entries))?;
    counts.directory_entries_without_block_names = directory
        .contents()
        .entries
        .iter()
        .filter(|entry| entry.block_name.text().is_empty())
        .count();
    Ok(())
}

fn unique_faces(links: &[ExternalFaceLink<'_>]) -> usize {
    links
        .iter()
        .map(|link| (link.block_name, link.index_in_block, link.face_index))
        .collect::<BTreeSet<_>>()
        .len()
}

fn unreferenced_faces(available: usize, unique: usize) -> Result<usize> {
    available.checked_sub(unique).ok_or_else(|| {
        link_error(
            FailureClass::Invalid,
            "coverage",
            format!("{unique} unique references; expected at most {available} stored facets"),
        )
    })
}

fn sum_counts(mut values: impl Iterator<Item = usize>) -> Result<usize> {
    values.try_fold(0usize, |total, value| {
        total.checked_add(value).ok_or_else(|| {
            link_error(
                FailureClass::Invalid,
                "coverage",
                format!("count total {total} plus {value}; expected a sum fitting usize"),
            )
        })
    })
}
