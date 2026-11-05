use crate::archive_codec::ArchiveRevision;
use crate::metadata::XmlAttribute;
use std::sync::Arc;

/// Literal record scope; container elements are not document records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionRecordKind {
    /// Direct unqualified inspection sheet.
    Sheet,
    /// Nested DOCUMENT record inside its DOCUMENT container.
    Document,
    /// IXGroupProxy in the sheet's characteristics collection.
    Group,
    /// Direct IXCharacteristic inside a group.
    Characteristic,
    /// Named Measurement inside a characteristic's Measurements container.
    Measurement,
    /// Named CustomProperty inside a characteristic's CustomProperties container.
    CustomProperty,
}

/// Physical record occurrence scoped to one immutable project revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectionRecordId {
    pub(super) revision: ArchiveRevision,
    pub(super) ordinal: usize,
}

/// Ordered lexical record; unknown attributes and duplicate names remain distinct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectionRecord {
    /// Revision-bound occurrence, independent of stored Uid/Name strings.
    pub id: InspectionRecordId,
    /// Exact literal role selected by its complete element ancestry.
    pub kind: InspectionRecordKind,
    /// Nearest containing named record; XML wrapper elements remain in source bytes.
    pub owner: Option<InspectionRecordId>,
    /// Every ordinary attribute, in original order and expanded namespace form.
    pub attributes: Arc<[XmlAttribute]>,
}

impl InspectionRecord {
    /// Read an exact unqualified attribute without coercion; e.g. `record.attribute("Value")`.
    pub fn attribute(&self, local: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|attribute| {
                attribute.name.namespace.is_none() && attribute.name.local.as_ref() == local
            })
            .map(|attribute| attribute.value.as_str())
    }
}

/// Completion of a fully checked lexical edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionRecordCompletion {
    /// Source bytes and identities were retained.
    Unchanged,
    /// A new immutable project revision was produced.
    Changed,
}

/// Declared check scope for record operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionRecordCheckMode {
    /// Entire UTF-8 XML and exact named ancestry, without numerical semantics.
    ExactLexicalXml,
}

/// Complete syntax coverage with explicit semantic limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InspectionRecordCoverage {
    /// Full parse scope, separate from native application acceptance.
    pub mode: InspectionRecordCheckMode,
    /// Entire plaintext XML length, including its BOM if present.
    pub xml_bytes: usize,
    /// All parsed elements, including unknown extensions.
    pub elements: usize,
    /// All selected named record occurrences.
    pub records: usize,
    /// XML-decoded selected-value disagreement after reparse; zero on success.
    pub achieved_value_error: usize,
}

/// Explicit exclusions from successful lexical operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionRecordDiagnostic {
    /// Stored values were not evaluated, converted to numbers or assigned units.
    ValueSemanticsNotEvaluated,
    /// Stored paths were not opened and cross-record links were not resolved.
    ExternalSourcesNotResolved,
    /// Native application reload was not executed.
    ApplicationReloadNotChecked,
}

/// Relation between a result record and every source occurrence that produced it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectionRecordHistory {
    /// All source record parents, retained even for unchanged duplicates.
    pub parents: Arc<[InspectionRecordId]>,
    /// Result occurrence identity.
    pub result: InspectionRecordId,
    /// Whether this occurrence owns the selected attribute replacement.
    pub relation: InspectionRecordRelation,
}

/// Record-level lexical edit provenance; containment alone does not imply mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionRecordRelation {
    /// This record's own attributes were copied without lexical changes.
    Copied,
    /// This record's selected attribute was derived from its source value.
    Derived,
}
