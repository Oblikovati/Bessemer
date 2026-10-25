use crate::tessellation::FacetContainer;

/// Exact archive-link coverage; this does not certify topology or mesh geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalFaceLinkCoverage {
    /// Supplied blocks checked for version, unique indices and directory index/hash links.
    pub checked_blocks: usize,
    /// Named directory references whose index and hash matched a supplied block.
    pub checked_directory_references: usize,
    /// Empty-name declarations retained outside the named-block check scope.
    pub directory_entries_without_block_names: usize,
    /// Component records available across all supplied blocks.
    pub available_block_components: usize,
    /// Supplied component records not named by the directory.
    pub unreferenced_block_components: usize,
    /// Serialized external facet references resolved to existing cache positions.
    pub checked_face_references: usize,
    /// Distinct block/index/face positions referenced by the display archive.
    pub unique_referenced_faces: usize,
    /// Facet records stored across all supplied block components.
    pub available_block_faces: usize,
    /// Stored facet records not selected by the display archive.
    pub unreferenced_block_faces: usize,
}

/// One resolved external facet, borrowing immutable archive, directory and block snapshots.
#[derive(Debug)]
pub struct ExternalFaceLink<'a> {
    /// Display field that requested this facet.
    pub display_path: &'a str,
    /// Exact component ID used for the directory lookup.
    pub component_id: &'a str,
    /// Literal directory block name, without a storage prefix.
    pub block_name: &'a str,
    /// Stored component index in that block; this is not its vector offset.
    pub index_in_block: i32,
    /// Zero-based facet position within the selected component.
    pub face_index: u32,
    /// Complete immutable cached facet; no geometry is copied or healed.
    pub facet: &'a FacetContainer,
}

/// Complete requested-face resolution, published only after all required link checks pass.
#[derive(Debug)]
pub struct ExternalFaceLinks<'a> {
    pub(super) coverage: ExternalFaceLinkCoverage,
    pub(super) links: Vec<ExternalFaceLink<'a>>,
}

impl<'a> ExternalFaceLinks<'a> {
    /// Returns exact checked and unreferenced record counts.
    ///
    /// ```no_run
    /// # fn example(links: &bessemer::display_lists::ExternalFaceLinks<'_>) {
    /// let counts = links.coverage();
    /// # }
    /// ```
    pub fn coverage(&self) -> ExternalFaceLinkCoverage {
        self.coverage
    }

    /// Borrows resolved facets in their original display-field order.
    ///
    /// ```no_run
    /// # fn example(links: &bessemer::display_lists::ExternalFaceLinks<'_>) {
    /// for link in links.links() { let facet = link.facet; }
    /// # }
    /// ```
    pub fn links(&self) -> &[ExternalFaceLink<'a>] {
        &self.links
    }
}
