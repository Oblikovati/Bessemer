use super::{FaceTessellationBlock, PartComponentInfo, TessellationDirectory, invalid};
use crate::Result;
use std::collections::{BTreeMap, BTreeSet};

/// Archive identity checks only; these counts do not certify mesh geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockLinkCoverage {
    /// Directory references whose index and hash matched.
    pub checked_references: usize,
    /// Complete entries stored in the selected block.
    pub available_entries: usize,
    /// Entries not referenced by this directory for this block name.
    pub unreferenced_entries: usize,
}

impl TessellationDirectory {
    /// Checks references to one named block without discarding unreferenced cache entries.
    ///
    /// ```no_run
    /// # use bessemer::tessellation::*;
    /// # fn example(directory: &TessellationDirectory, block: &FaceTessellationBlock) -> bessemer::Result<()> {
    /// let coverage = directory.check_block_links("000-000-007", block)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn check_block_links(
        &self,
        block_name: &str,
        block: &FaceTessellationBlock,
    ) -> Result<BlockLinkCoverage> {
        require_version(self, block)?;
        let hashes = block_hashes(block)?;
        check_references(&self.contents().entries, block_name, &hashes)
    }
}

fn check_references(
    entries: &[PartComponentInfo],
    block_name: &str,
    hashes: &BTreeMap<i32, u64>,
) -> Result<BlockLinkCoverage> {
    let mut referenced = BTreeSet::new();
    let mut checked_references = 0;
    for entry in entries
        .iter()
        .filter(|entry| entry.block_name.text() == block_name)
    {
        require_hash(entry.index_in_block, entry.hash_key, hashes)?;
        referenced.insert(entry.index_in_block);
        checked_references += 1;
    }
    Ok(BlockLinkCoverage {
        checked_references,
        available_entries: hashes.len(),
        unreferenced_entries: hashes.len() - referenced.len(),
    })
}

fn require_version(directory: &TessellationDirectory, block: &FaceTessellationBlock) -> Result<()> {
    let expected = directory.context().archive_version;
    let actual = block.context().archive_version;
    if expected == actual {
        return Ok(());
    }
    Err(invalid(format!(
        "block archive version {}; expected directory version {}",
        actual.value(),
        expected.value()
    )))
}

fn block_hashes(block: &FaceTessellationBlock) -> Result<BTreeMap<i32, u64>> {
    let mut hashes = BTreeMap::new();
    for entry in &block.contents().entries {
        if hashes
            .insert(entry.index_in_block, entry.hash_key)
            .is_some()
        {
            return Err(invalid(format!(
                "duplicate block index {}; expected unique indices",
                entry.index_in_block
            )));
        }
    }
    Ok(hashes)
}

fn require_hash(index: i32, expected: u64, hashes: &BTreeMap<i32, u64>) -> Result<()> {
    let actual = hashes.get(&index).ok_or_else(|| {
        invalid(format!(
            "missing block index {index}; expected directory target with hash {expected:#x}"
        ))
    })?;
    if *actual == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "block index {index} hash {actual:#x}; expected directory hash {expected:#x}"
    )))
}
