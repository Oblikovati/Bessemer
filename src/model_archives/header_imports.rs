use std::sync::Arc;

use super::{ModelDecoder, header_order, invalid};
use crate::archive_codec::{ArchiveMapIndex, ObjectReader};
use crate::header::{ArchiveVersion, DocumentHeader, HeaderReference};
use crate::{BessemerError, FailureClass, Result};

pub(super) struct ModelHeaderLoadPlan {
    header: Arc<DocumentHeader>,
    references: Vec<HeaderReference>,
}

pub(super) struct ImportedHeaderMap {
    header: Arc<DocumentHeader>,
    slots: Vec<(ArchiveMapIndex, HeaderReference)>,
}

impl ModelHeaderLoadPlan {
    pub fn new(
        header: Arc<DocumentHeader>,
        version: ArchiveVersion,
        maximum: usize,
    ) -> Result<Self> {
        require_import_context(&header, version)?;
        let references = header_order::ordered_imports(header.contents(), maximum)?;
        Ok(Self { header, references })
    }

    pub fn append(
        self,
        decoder: &mut ModelDecoder<'_>,
        map: &mut ObjectReader,
    ) -> Result<ImportedHeaderMap> {
        let context = require_import_context(&self.header, decoder.archive_version());
        decoder.checked(context)?;
        decoder.entries(self.references.len(), "header imports")?;
        let mut slots = Vec::with_capacity(self.references.len());
        for reference in self.references {
            slots.push(append_reference(decoder, map, &self.header, reference)?);
        }
        Ok(ImportedHeaderMap {
            header: self.header,
            slots,
        })
    }
}

fn append_reference(
    decoder: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    header: &DocumentHeader,
    reference: HeaderReference,
) -> Result<(ArchiveMapIndex, HeaderReference)> {
    let class = header_order::imported_class(header.contents(), reference);
    let class = decoder.checked(class)?;
    let index = map
        .append_loaded_object(class)
        .map_err(super::model_failure);
    Ok((decoder.checked(index)?, reference))
}

impl ImportedHeaderMap {
    pub fn origin(&self, index: ArchiveMapIndex) -> Option<HeaderReference> {
        let position = self
            .slots
            .binary_search_by_key(&index, |(slot, _)| *slot)
            .ok()?;
        self.slots.get(position).map(|(_, reference)| *reference)
    }

    pub fn header(&self) -> &DocumentHeader {
        &self.header
    }

    pub fn slots(&self) -> &[(ArchiveMapIndex, HeaderReference)] {
        &self.slots
    }
}

fn require_import_context(header: &DocumentHeader, version: ArchiveVersion) -> Result<()> {
    require_modern_import_version(version)?;
    if header.archive_version() != version {
        return Err(invalid(format!(
            "header version {} with model version {}; expected equal outer versions",
            header.archive_version().value(),
            version.value()
        )));
    }
    Ok(())
}

fn require_modern_import_version(version: ArchiveVersion) -> Result<()> {
    if version.value() > 0x27bc {
        return Ok(());
    }
    Err(BessemerError::new(
        "model archive",
        "header imports",
        FailureClass::Unsupported,
        format!(
            "model version {}; expected > 0x27BC for modern header imports",
            version.value()
        ),
    ))
}
