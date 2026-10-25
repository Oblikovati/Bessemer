use super::external_face_coverage::coverage;
use super::external_face_errors::{component_target_error, link_error, missing_component};
use super::{DisplayArchive, DisplayField, DisplayFieldValue, ExternalFaceLink, ExternalFaceLinks};
use crate::tessellation::{
    BlockLinkCoverage, FaceTessellationBlock, FacetBlockComponent, PartComponentInfo,
    TessellationDirectory,
};
use crate::{FailureClass, Result};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy)]
enum ComponentTarget<'a> {
    Unique(&'a PartComponentInfo),
    Ambiguous,
}

struct FaceSources<'a> {
    components: BTreeMap<&'a str, ComponentTarget<'a>>,
    blocks: BTreeMap<(&'a str, i32), &'a FacetBlockComponent>,
}

impl DisplayArchive {
    /// Checks all named directory links and resolves every requested external facet exactly.
    /// Unrequested empty-name declarations are counted, without inferring absent geometry.
    /// Any missing, ambiguous or invalid required target prevents publication of partial links.
    ///
    /// ```no_run
    /// # use bessemer::{display_lists::*, tessellation::*};
    /// # fn example(archive: &DisplayArchive, directory: &TessellationDirectory, blocks: &std::collections::BTreeMap<String, FaceTessellationBlock>) -> bessemer::Result<()> {
    /// let checked = archive.check_external_face_links(directory, blocks)?;
    /// let facets = checked.links();
    /// # Ok(())
    /// # }
    /// ```
    pub fn check_external_face_links<'a>(
        &'a self,
        directory: &'a TessellationDirectory,
        blocks: &'a BTreeMap<String, FaceTessellationBlock>,
    ) -> Result<ExternalFaceLinks<'a>> {
        let checked_blocks = check_blocks(directory, blocks)?;
        let sources = face_sources(directory, blocks);
        let mut links = Vec::new();
        for field in &self.contents().fields {
            if let Some(link) = resolve_field(field, &sources)? {
                links.push(link);
            }
        }
        let coverage = coverage(directory, blocks, &checked_blocks, &links)?;
        Ok(ExternalFaceLinks { coverage, links })
    }
}

fn check_blocks(
    directory: &TessellationDirectory,
    blocks: &BTreeMap<String, FaceTessellationBlock>,
) -> Result<Vec<BlockLinkCoverage>> {
    require_named_blocks(directory, blocks)?;
    blocks
        .iter()
        .map(|(name, block)| {
            directory
                .check_block_links(name, block)
                .map_err(|mut error| {
                    error.operation = "check external face links";
                    error.entity = format!("FaceTessellations/{name}");
                    error
                })
        })
        .collect()
}

fn require_named_blocks(
    directory: &TessellationDirectory,
    blocks: &BTreeMap<String, FaceTessellationBlock>,
) -> Result<()> {
    let names: BTreeSet<_> = directory
        .contents()
        .entries
        .iter()
        .map(|entry| entry.block_name.text())
        .filter(|name| !name.is_empty())
        .collect();
    for name in names {
        if !blocks.contains_key(name) {
            return Err(link_error(
                FailureClass::Missing,
                &format!("FaceTessellations/{name}"),
                format!("missing block {name:?}; expected a supplied block named by the directory"),
            ));
        }
    }
    Ok(())
}

fn face_sources<'a>(
    directory: &'a TessellationDirectory,
    blocks: &'a BTreeMap<String, FaceTessellationBlock>,
) -> FaceSources<'a> {
    // check_blocks proved unique component indices before this immutable index is built.
    let indexed = blocks
        .iter()
        .flat_map(|(name, block)| {
            block
                .contents()
                .entries
                .iter()
                .map(move |entry| ((name.as_str(), entry.index_in_block), entry))
        })
        .collect();
    FaceSources {
        components: component_targets(directory),
        blocks: indexed,
    }
}

fn component_targets(directory: &TessellationDirectory) -> BTreeMap<&str, ComponentTarget<'_>> {
    let mut targets: BTreeMap<&str, ComponentTarget<'_>> = BTreeMap::new();
    for entry in &directory.contents().entries {
        let key = entry.component_id.text();
        let target = match targets.get(key).copied() {
            None => ComponentTarget::Unique(entry),
            Some(ComponentTarget::Unique(first)) if same_target(first, entry) => continue,
            _ => ComponentTarget::Ambiguous,
        };
        targets.insert(key, target);
    }
    targets
}

fn same_target(first: &PartComponentInfo, second: &PartComponentInfo) -> bool {
    first.block_name.text() == second.block_name.text()
        && first.index_in_block == second.index_in_block
        && first.hash_key == second.hash_key
}

fn resolve_field<'a>(
    field: &'a DisplayField,
    sources: &FaceSources<'a>,
) -> Result<Option<ExternalFaceLink<'a>>> {
    let DisplayFieldValue::ExternalFacet {
        component_id,
        index,
    } = &field.value
    else {
        return Ok(None);
    };
    let identity = component_id.text();
    let entry = component_entry(identity, &field.path, &sources.components)?;
    let component = sources.blocks.get(&(entry.block_name.text(), entry.index_in_block)).ok_or_else(|| link_error(FailureClass::Missing, &field.path, format!("component {identity:?} block {:?} index {}; expected a located cache component", entry.block_name.text(), entry.index_in_block)))?;
    let facet = component.faces.get(*index as usize).ok_or_else(|| link_error(FailureClass::Invalid, &field.path, format!("component {identity:?} face index {index}; expected a position below {} in block {:?} component {}", component.faces.len(), entry.block_name.text(), entry.index_in_block)))?;
    Ok(Some(ExternalFaceLink {
        display_path: &field.path,
        component_id: identity,
        block_name: entry.block_name.text(),
        index_in_block: entry.index_in_block,
        face_index: *index,
        facet,
    }))
}

fn component_entry<'a>(
    identity: &str,
    path: &str,
    targets: &BTreeMap<&str, ComponentTarget<'a>>,
) -> Result<&'a PartComponentInfo> {
    let target = targets
        .get(identity)
        .ok_or_else(|| missing_component(path, identity))?;
    match target {
        ComponentTarget::Unique(entry) if !entry.block_name.text().is_empty() => Ok(entry),
        ComponentTarget::Unique(_) => Err(component_target_error(path, identity, false)),
        ComponentTarget::Ambiguous => Err(component_target_error(path, identity, true)),
    }
}
