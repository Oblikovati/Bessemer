//! Composer 7.9 XML/geode equality indexing and isolated lexical reference edits (14.91).
mod catalog;
mod index;
mod types;

use crate::archive_codec::ArchiveRevision;
use crate::composer::geometry::{ComposerGeometryArchive, ComposerSceneRevision};
use crate::metadata::{ComposerXmlDocument, XmlAttribute};
use crate::{BessemerError, FailureClass, Result};
use catalog::GuidCatalog;
use std::sync::Arc;
pub use types::*;

/// Immutable complete XML plus an exact local identifier index on one geometry revision.
#[derive(Debug, Clone)]
pub struct ComposerXmlLinks {
    xml: ComposerXmlDocument,
    catalog: Arc<GuidCatalog>,
    references: Arc<[ComposerXmlReference]>,
    unreferenced: Arc<[ComposerGeodeId]>,
    revision: ArchiveRevision,
    scene_revision: ComposerSceneRevision,
    limits: ComposerLinkLimits,
}

impl ComposerXmlLinks {
    /// Read and index complete XML, e.g. `ComposerXmlLinks::from_bytes(xml, &scene, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        scene: &ComposerGeometryArchive,
        limits: ComposerLinkLimits,
    ) -> Result<Self> {
        let xml = ComposerXmlDocument::from_bytes(bytes, limits.xml).map_err(link_error)?;
        let catalog = Arc::new(catalog::catalog(scene, limits)?);
        Self::from_document(xml, catalog, scene.revision().clone(), limits)
    }

    fn from_document(
        xml: ComposerXmlDocument,
        catalog: Arc<GuidCatalog>,
        scene_revision: ComposerSceneRevision,
        limits: ComposerLinkLimits,
    ) -> Result<Self> {
        require_root(&xml)?;
        let revision = ArchiveRevision::default();
        let indexed = index::references(&xml, &catalog, &revision, limits)?;
        Ok(Self {
            xml,
            catalog,
            references: indexed.references,
            unreferenced: indexed.unreferenced,
            revision,
            scene_revision,
            limits,
        })
    }

    /// Every occurrence in element preorder, e.g. `index.references()`.
    pub fn references(&self) -> &[ComposerXmlReference] {
        &self.references
    }

    /// Original nonempty keys in first-seen order, e.g. `index.guid_groups()`.
    pub fn guid_groups(&self) -> &[ComposerGuidGroup] {
        &self.catalog.groups
    }

    /// Empty or absent binary GUIDs, e.g. `index.unkeyed_geodes()`.
    pub fn unkeyed_geodes(&self) -> &[ComposerGeodeId] {
        &self.catalog.unkeyed
    }

    /// Keyed geodes with no equality occurrence, e.g. `index.unreferenced_geodes()`.
    pub fn unreferenced_geodes(&self) -> &[ComposerGeodeId] {
        &self.unreferenced
    }

    /// Geometry snapshot identity used for every candidate, e.g. `index.scene_revision()`.
    pub fn scene_revision(&self) -> &ComposerSceneRevision {
        &self.scene_revision
    }

    /// Complete root attributes without namespace declarations, e.g. `index.root_attributes()`.
    pub fn root_attributes(&self) -> impl Iterator<Item = &XmlAttribute> {
        self.xml.attributes(0)
    }

    /// Exact stored bytes, including comments, unknown nodes and BOM, e.g. `index.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.xml.bytes().to_vec()
    }

    /// Complete local-index check coverage, e.g. `index.coverage().references`.
    pub fn coverage(&self) -> ComposerLinkCoverage {
        ComposerLinkCoverage {
            mode: ComposerLinkCheckMode::ExactLocalEquality,
            xml_bytes: self.xml.bytes().len(),
            elements: self.xml.element_count(),
            geodes: self.catalog.geodes,
            references: self.references.len(),
        }
    }

    /// Runtime/full-project exclusions, e.g. `index.diagnostics()`.
    pub fn diagnostics(&self) -> [ComposerLinkDiagnostic; 4] {
        LINK_DIAGNOSTICS
    }

    /// Read through a revision-bound identity, e.g. `index.reference(&id)?`.
    pub fn reference(&self, id: &ComposerXmlReferenceId) -> Result<&ComposerXmlReference> {
        if id.revision == self.revision && id.index < self.references.len() {
            return Ok(&self.references[id.index]);
        }
        Err(invalid(format!(
            "reference index {} from another/missing revision; expected this index's {} references",
            id.index,
            self.references.len()
        )))
    }

    /// Change one existing attribute and reindex, e.g. `index.edit_reference(&id, "guid-text")?`.
    pub fn edit_reference(
        &self,
        id: &ComposerXmlReferenceId,
        value: &str,
    ) -> Result<ComposerLinkEdit> {
        let reference = self.reference(id)?;
        if reference.identifier.as_ref() == value {
            return Ok(edit_outcome(self.clone(), id.clone(), id.clone(), false));
        }
        let index = self.reindexed(reference.element_ordinal, value)?;
        let result = index.references[id.index].id.clone();
        require_reference_text(&index.references[id.index], value)?;
        Ok(edit_outcome(index, id.clone(), result, true))
    }

    fn reindexed(&self, ordinal: usize, value: &str) -> Result<Self> {
        budget(
            "replacement UTF-8 bytes",
            value.len(),
            self.limits.xml.maximum_bytes,
        )?;
        let xml = self
            .xml
            .with_ident_geom(ordinal, value, self.limits.xml)
            .map_err(link_error)?;
        Self::from_document(
            xml,
            self.catalog.clone(),
            self.scene_revision.clone(),
            self.limits,
        )
    }
}

fn require_reference_text(reference: &ComposerXmlReference, expected: &str) -> Result<()> {
    if reference.identifier.as_ref() == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "reparsed reference {:?}; expected exact replacement {expected:?}",
        reference.identifier
    )))
}

fn require_root(xml: &ComposerXmlDocument) -> Result<()> {
    let name = xml.name(0);
    let version = xml.attribute(0, "Version");
    if name.namespace.is_none()
        && name.local.as_ref() == "DocumentSMG"
        && matches!(version, Some("7.9" | "7.900000"))
    {
        return Ok(());
    }
    Err(BessemerError::new(
        "composer XML links",
        "root/version",
        FailureClass::Unsupported,
        format!(
            "root {name:?}, version {version:?}; expected unqualified DocumentSMG with 7.9 or 7.900000"
        ),
    ))
}

fn edit_outcome(
    index: ComposerXmlLinks,
    parent: ComposerXmlReferenceId,
    result: ComposerXmlReferenceId,
    changed: bool,
) -> ComposerLinkEdit {
    ComposerLinkEdit {
        coverage: index.coverage(),
        index,
        completion: if changed {
            crate::display_lists::DisplayEditCompletion::Changed
        } else {
            crate::display_lists::DisplayEditCompletion::Unchanged
        },
        fidelity: ComposerLinkEditFidelity::ExactAttributeText,
        history: ComposerLinkHistory { parent, result },
        diagnostics: LINK_DIAGNOSTICS,
    }
}

const LINK_DIAGNOSTICS: [ComposerLinkDiagnostic; 4] = [
    ComposerLinkDiagnostic::RuntimeResolutionNotChecked,
    ComposerLinkDiagnostic::GraphSideEffectsNotChecked,
    ComposerLinkDiagnostic::ProjectSemanticsNotChecked,
    ComposerLinkDiagnostic::NativeApplicationNotChecked,
];

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "composer XML links",
        "complete local index",
        FailureClass::Invalid,
        cause,
    )
}

fn budget(label: &str, actual: usize, maximum: usize) -> Result<()> {
    if actual <= maximum {
        return Ok(());
    }
    Err(invalid(format!(
        "{label} {actual}; expected at most {maximum}"
    )))
}

fn link_error(mut error: BessemerError) -> BessemerError {
    error.operation = "composer XML links";
    error
}
