use super::*;
use crate::metadata::ComposerXmlDocument;
use std::collections::BTreeSet;

pub(super) struct ReferenceSet {
    pub references: Arc<[ComposerXmlReference]>,
    pub unreferenced: Arc<[ComposerGeodeId]>,
}

struct ReferenceBuilder<'a> {
    xml: &'a ComposerXmlDocument,
    catalog: &'a GuidCatalog,
    revision: &'a ArchiveRevision,
    limits: ComposerLinkLimits,
    references: Vec<ComposerXmlReference>,
    referred: BTreeSet<usize>,
    path_units: usize,
}

pub(super) fn references(
    xml: &ComposerXmlDocument,
    catalog: &GuidCatalog,
    revision: &ArchiveRevision,
    limits: ComposerLinkLimits,
) -> Result<ReferenceSet> {
    let mut builder = ReferenceBuilder::new(xml, catalog, revision, limits);
    for ordinal in 0..xml.element_count() {
        if let Some(identifier) = xml.attribute(ordinal, "IdentGeom") {
            builder.push(ordinal, identifier)?;
        }
    }
    Ok(ReferenceSet {
        unreferenced: unreferenced(catalog, &builder.referred),
        references: builder.references.into(),
    })
}

impl<'a> ReferenceBuilder<'a> {
    fn new(
        xml: &'a ComposerXmlDocument,
        catalog: &'a GuidCatalog,
        revision: &'a ArchiveRevision,
        limits: ComposerLinkLimits,
    ) -> Self {
        Self {
            xml,
            catalog,
            revision,
            limits,
            references: Vec::new(),
            referred: BTreeSet::new(),
            path_units: 0,
        }
    }

    fn push(&mut self, ordinal: usize, identifier: &str) -> Result<()> {
        budget(
            "XML references",
            self.references.len() + 1,
            self.limits.maximum_references,
        )?;
        let path = self.record_path(ordinal)?;
        let (status, candidates) = match_identifier(identifier, self.catalog, &mut self.referred);
        let reference = self.make_reference(ordinal, identifier, path, status, candidates);
        self.references.push(reference);
        Ok(())
    }

    fn record_path(&mut self, ordinal: usize) -> Result<Vec<usize>> {
        let path = self.xml.element_path(ordinal);
        self.path_units = self
            .path_units
            .checked_add(path.len())
            .ok_or_else(|| invalid("path units overflow; expected bounded cumulative path size"))?;
        budget(
            "path units",
            self.path_units,
            self.limits.maximum_path_units,
        )?;
        Ok(path)
    }

    fn make_reference(
        &self,
        ordinal: usize,
        identifier: &str,
        path: Vec<usize>,
        status: ComposerLinkStatus,
        candidates: Arc<[ComposerGeodeId]>,
    ) -> ComposerXmlReference {
        ComposerXmlReference {
            id: self.next_id(),
            element_path: path.into(),
            name: self.xml.name(ordinal).clone(),
            attributes: self.copied_attributes(ordinal),
            identifier: identifier.into(),
            status,
            candidates,
            element_ordinal: ordinal,
        }
    }

    fn next_id(&self) -> ComposerXmlReferenceId {
        ComposerXmlReferenceId {
            revision: self.revision.clone(),
            index: self.references.len(),
        }
    }

    fn copied_attributes(&self, ordinal: usize) -> Arc<[crate::metadata::XmlAttribute]> {
        self.xml
            .attributes(ordinal)
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }
}

fn match_identifier(
    identifier: &str,
    catalog: &GuidCatalog,
    referred: &mut BTreeSet<usize>,
) -> (ComposerLinkStatus, Arc<[ComposerGeodeId]>) {
    if identifier.is_empty() {
        return (ComposerLinkStatus::Empty, Arc::from([]));
    }
    let units = identifier.encode_utf16().collect::<Vec<_>>();
    let Some(index) = catalog.by_identifier.get(units.as_slice()) else {
        return (ComposerLinkStatus::Unmatched, Arc::from([]));
    };
    referred.insert(*index);
    let candidates = catalog.groups[*index].candidates.clone();
    let status = candidate_status(candidates.len());
    (status, candidates)
}

fn candidate_status(count: usize) -> ComposerLinkStatus {
    if count == 1 {
        return ComposerLinkStatus::Unique;
    }
    ComposerLinkStatus::Ambiguous
}

fn unreferenced(catalog: &GuidCatalog, referred: &BTreeSet<usize>) -> Arc<[ComposerGeodeId]> {
    catalog
        .groups
        .iter()
        .enumerate()
        .filter(|(index, _)| !referred.contains(index))
        .flat_map(|(_, group)| group.candidates.iter().cloned())
        .collect::<Vec<_>>()
        .into()
}
