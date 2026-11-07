use super::*;
use crate::composer::geometry::{ComposerGeometryArchive, ComposerSceneValue};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub(super) struct GuidCatalog {
    pub groups: Arc<[ComposerGuidGroup]>,
    pub by_identifier: BTreeMap<Arc<[u16]>, usize>,
    pub unkeyed: Arc<[ComposerGeodeId]>,
    pub geodes: usize,
}

struct GeodeRecord {
    id: ComposerGeodeId,
    guid: Option<Arc<[u16]>>,
}

pub(super) fn catalog(
    scene: &ComposerGeometryArchive,
    limits: ComposerLinkLimits,
) -> Result<GuidCatalog> {
    budget("geodes", scene.coverage().geodes, limits.maximum_geodes)?;
    let records = geode_records(scene)?;
    require_geode_count(records.len(), scene.coverage().geodes)?;
    let mut groups: Vec<(Arc<[u16]>, Vec<ComposerGeodeId>)> = Vec::new();
    let mut by_identifier = BTreeMap::new();
    let mut unkeyed = Vec::new();
    let geodes = records.len();
    for record in records {
        add_geode(record, &mut groups, &mut by_identifier, &mut unkeyed);
    }
    Ok(freeze_catalog(groups, by_identifier, unkeyed, geodes))
}

fn require_geode_count(actual: usize, expected: usize) -> Result<()> {
    if actual == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "canonical geode records {actual}; expected {expected} declared by the complete scene"
    )))
}

fn freeze_catalog(
    groups: Vec<(Arc<[u16]>, Vec<ComposerGeodeId>)>,
    by_identifier: BTreeMap<Arc<[u16]>, usize>,
    unkeyed: Vec<ComposerGeodeId>,
    geodes: usize,
) -> GuidCatalog {
    let groups = groups
        .into_iter()
        .map(|(identifier, candidates)| ComposerGuidGroup {
            identifier,
            candidates: candidates.into(),
        })
        .collect::<Vec<_>>();
    GuidCatalog {
        groups: groups.into(),
        by_identifier,
        unkeyed: unkeyed.into(),
        geodes,
    }
}

fn add_geode(
    record: GeodeRecord,
    groups: &mut Vec<(Arc<[u16]>, Vec<ComposerGeodeId>)>,
    lookup: &mut BTreeMap<Arc<[u16]>, usize>,
    unkeyed: &mut Vec<ComposerGeodeId>,
) {
    let Some(key) = record.guid.filter(|key| !key.is_empty()) else {
        unkeyed.push(record.id);
        return;
    };
    let index = group_slot(key, groups, lookup);
    groups[index].1.push(record.id);
}

fn group_slot(
    key: Arc<[u16]>,
    groups: &mut Vec<(Arc<[u16]>, Vec<ComposerGeodeId>)>,
    lookup: &mut BTreeMap<Arc<[u16]>, usize>,
) -> usize {
    match lookup.get(&key) {
        Some(index) => *index,
        None => {
            let index = groups.len();
            lookup.insert(key.clone(), index);
            groups.push((key, Vec::new()));
            index
        }
    }
}

fn geode_records(scene: &ComposerGeometryArchive) -> Result<Vec<GeodeRecord>> {
    let mut records: Vec<GeodeRecord> = Vec::new();
    for field in scene.fields() {
        if let Some(entry) = root_entry(&field.path, ".tag")?
            && field.value == ComposerSceneValue::Byte(b'u')
        {
            records.push(GeodeRecord {
                id: ComposerGeodeId {
                    revision: scene.revision().clone(),
                    entry,
                },
                guid: None,
            });
        }
        if let Some(entry) = root_entry(&field.path, ".userdata.guid")? {
            assign_guid(&mut records, entry, &field.value)?;
        }
    }
    Ok(records)
}

fn root_entry(path: &str, suffix: &str) -> Result<Option<usize>> {
    let Some(tail) = path.strip_prefix("entries[") else {
        return Ok(None);
    };
    let Some((digits, ending)) = tail.split_once(']') else {
        return Ok(None);
    };
    if ending != suffix {
        return Ok(None);
    }
    digits.parse().map(Some).map_err(|_| {
        invalid(format!(
            "scene entry {digits:?}; expected canonical usize entry index"
        ))
    })
}

fn assign_guid(
    records: &mut [GeodeRecord],
    entry: usize,
    value: &ComposerSceneValue,
) -> Result<()> {
    let record = active_record(records, entry)?;
    let ComposerSceneValue::Text(text) = value else {
        return Err(invalid(format!(
            "GUID value {value:?}; expected stored UTF-16 text"
        )));
    };
    record.guid = Some(text.code_units().into());
    Ok(())
}

fn active_record(records: &mut [GeodeRecord], entry: usize) -> Result<&mut GeodeRecord> {
    records
        .last_mut()
        .filter(|record| record.id.entry == entry)
        .ok_or_else(|| {
            invalid(format!(
                "GUID entry {entry}; expected the active canonical geode"
            ))
        })
}
