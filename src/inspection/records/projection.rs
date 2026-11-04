use super::{InspectionRecord, InspectionRecordId, InspectionRecordKind};
use crate::archive_codec::ArchiveRevision;
use crate::metadata::Utf8XmlDocument;
use std::sync::Arc;

pub(super) fn collect(xml: &Utf8XmlDocument, revision: &ArchiveRevision) -> Vec<InspectionRecord> {
    let mut records = Vec::new();
    let mut owners = vec![None; xml.element_count()];
    for ordinal in 1..xml.element_count() {
        let owner = xml
            .parent(ordinal)
            .and_then(|parent| owners[parent].clone());
        owners[ordinal] = owner.clone();
        if let Some(kind) = role(xml, ordinal) {
            let id = InspectionRecordId {
                revision: revision.clone(),
                ordinal,
            };
            records.push(make_record(xml, &id, kind, owner));
            owners[ordinal] = Some(id);
        }
    }
    records
}

fn make_record(
    xml: &Utf8XmlDocument,
    id: &InspectionRecordId,
    kind: InspectionRecordKind,
    owner: Option<InspectionRecordId>,
) -> InspectionRecord {
    InspectionRecord {
        id: id.clone(),
        kind,
        owner,
        attributes: xml.attributes(id.ordinal).cloned().collect::<Arc<[_]>>(),
    }
}

fn role(xml: &Utf8XmlDocument, ordinal: usize) -> Option<InspectionRecordKind> {
    RECORD_PATHS
        .iter()
        .find(|(_, path)| matches_path(xml, ordinal, path))
        .map(|(kind, _)| *kind)
}

fn matches_path(xml: &Utf8XmlDocument, mut ordinal: usize, path: &[&str]) -> bool {
    for local in path {
        let name = xml.name(ordinal);
        if name.namespace.is_some() || name.local.as_ref() != *local {
            return false;
        }
        let Some(parent) = xml.parent(ordinal) else {
            return false;
        };
        ordinal = parent;
    }
    ordinal == 0
}

use InspectionRecordKind::*;
const RECORD_PATHS: &[(InspectionRecordKind, &[&str])] = &[
    (Sheet, &["INSPECTION_SHEET"]),
    (Document, &["DOCUMENT", "DOCUMENT", "INSPECTION_SHEET"]),
    (
        Group,
        &[
            "IXGroupProxy",
            "IXCharacteristics",
            "PackageOptions",
            "INSPECTION_SHEET",
        ],
    ),
    (
        Characteristic,
        &[
            "IXCharacteristic",
            "IXGroupProxy",
            "IXCharacteristics",
            "PackageOptions",
            "INSPECTION_SHEET",
        ],
    ),
    (
        Measurement,
        &[
            "Measurement",
            "Measurements",
            "IXCharacteristic",
            "IXGroupProxy",
            "IXCharacteristics",
            "PackageOptions",
            "INSPECTION_SHEET",
        ],
    ),
    (
        CustomProperty,
        &[
            "CustomProperty",
            "CustomProperties",
            "IXCharacteristic",
            "IXGroupProxy",
            "IXCharacteristics",
            "PackageOptions",
            "INSPECTION_SHEET",
        ],
    ),
];
