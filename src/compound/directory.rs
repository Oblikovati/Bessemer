use super::header::FREE;
use super::{CompoundEntryKind, CompoundLimits, Result, budget, invalid, require};
use crate::records::{dword, word};
use std::collections::BTreeSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(super) struct DirectoryRecord {
    pub raw: [u8; 128],
    pub kind: CompoundEntryKind,
    pub name: Arc<[u16]>,
    pub start: u32,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub(super) struct DirectoryTree {
    pub records: Vec<DirectoryRecord>,
    pub parents: Vec<Option<usize>>,
    pub children: Vec<Vec<usize>>,
}

impl DirectoryTree {
    fn from_records(records: Vec<DirectoryRecord>) -> Self {
        let count = records.len();
        Self {
            records,
            parents: vec![None; count],
            children: vec![Vec::new(); count],
        }
    }

    pub fn parse(bytes: &[u8], major: u16, limits: CompoundLimits) -> Result<Self> {
        budget(
            "directory slots",
            bytes.len() / 128,
            limits.maximum_directory_entries.min(0xffff_fffa),
        )?;
        require(
            !bytes.is_empty() && bytes.len().is_multiple_of(128),
            format!(
                "directory bytes {}; expected a nonempty multiple of 128",
                bytes.len()
            ),
        )?;
        let records = directory_records(bytes, major)?;
        validate_root(&records[0])?;
        let mut tree = Self::from_records(records);
        tree.link(limits.maximum_storage_depth)?;
        tree.unique_names()?;
        Ok(tree)
    }

    fn link(&mut self, maximum_depth: usize) -> Result<()> {
        let mut seen = vec![false; self.records.len()];
        seen[0] = true;
        let mut pending = vec![(dword(&self.records[0].raw, 76)?, 0, 1)];
        while let Some((id, parent, depth)) = pending.pop() {
            if id == FREE {
                continue;
            }
            self.attach(id as usize, parent, depth, maximum_depth, &mut seen)?;
            let record = &self.records[id as usize];
            pending.extend([
                (dword(&record.raw, 68)?, parent, depth),
                (dword(&record.raw, 72)?, parent, depth),
            ]);
            if record.kind == CompoundEntryKind::Storage {
                pending.push((dword(&record.raw, 76)?, id as usize, depth + 1));
            }
        }
        require_reachable(&self.records, &seen)?;
        Ok(())
    }

    fn attach(
        &mut self,
        id: usize,
        parent: usize,
        depth: usize,
        maximum: usize,
        seen: &mut [bool],
    ) -> Result<()> {
        budget("directory depth", depth, maximum)?;
        validate_link(&self.records, id, seen)?;
        seen[id] = true;
        self.parents[id] = Some(parent);
        self.children[parent].push(id);
        Ok(())
    }

    fn unique_names(&self) -> Result<()> {
        for (parent, children) in self.children.iter().enumerate() {
            let mut names = BTreeSet::new();
            for child in children {
                let name = &self.records[*child].name;
                require(
                    names.insert(name),
                    format!(
                        "parent {parent} repeats raw UTF-16 name {name:?}; expected distinct child names"
                    ),
                )?;
            }
        }
        Ok(())
    }
}

fn parse_record(bytes: &[u8], major: u16, index: usize) -> Result<DirectoryRecord> {
    let raw: [u8; 128] = bytes.try_into().map_err(|_| {
        invalid(format!(
            "directory slot {index} has {} bytes; expected 128",
            bytes.len()
        ))
    })?;
    let kind = record_kind(raw[66], index)?;
    if kind == CompoundEntryKind::Unused {
        return Ok(DirectoryRecord {
            raw,
            kind,
            name: Arc::from([]),
            start: 0,
            size: 0,
        });
    }
    parsed_record(raw, kind, major, index)
}

fn record_kind(value: u8, index: usize) -> Result<CompoundEntryKind> {
    match (index, value) {
        (_, 0) => Ok(CompoundEntryKind::Unused),
        (_, 1) => Ok(CompoundEntryKind::Storage),
        (_, 2) => Ok(CompoundEntryKind::Stream),
        (0, 5) => Ok(CompoundEntryKind::Root),
        _ => Err(invalid(format!(
            "directory slot {index} type {value}; expected unused 0, storage 1, stream 2 or root 5 only at slot 0"
        ))),
    }
}

fn record_name(raw: &[u8], index: usize) -> Result<Arc<[u16]>> {
    let length = word(raw, 64)? as usize;
    require(
        (2..=64).contains(&length) && length.is_multiple_of(2),
        format!("directory slot {index} name length {length}; expected even 2..=64 including NUL"),
    )?;
    require(
        word(raw, length - 2)? == 0,
        format!("directory slot {index} name lacks NUL at {}", length - 2),
    )?;
    let units: Vec<u16> = raw[..length - 2]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect();
    valid_name_units(&units, index)?;
    Ok(units.into())
}

fn validate_root(root: &DirectoryRecord) -> Result<()> {
    require(
        root.kind == CompoundEntryKind::Root
            && root.name.as_ref() == [82, 111, 111, 116, 32, 69, 110, 116, 114, 121],
        format!(
            "root kind {:?}, name {:?}; expected Root Entry at slot 0",
            root.kind, root.name
        ),
    )?;
    require(
        dword(&root.raw, 68)? == FREE && dword(&root.raw, 72)? == FREE,
        format!(
            "root sibling links {:#010x}, {:#010x}; expected two NOSTREAM values",
            dword(&root.raw, 68)?,
            dword(&root.raw, 72)?
        ),
    )
}

fn active_name(raw: &[u8], kind: CompoundEntryKind, index: usize) -> Result<Arc<[u16]>> {
    // #107: the black root in MS-CFB 3.3.1 is an example; installed CWR roots use red.
    require(
        raw[67] <= 1,
        format!(
            "directory slot {index} color {}; expected red 0 or black 1",
            raw[67]
        ),
    )?;
    if kind == CompoundEntryKind::Stream {
        require(
            dword(raw, 76)? == FREE,
            format!(
                "stream slot {index} child {}; expected NOSTREAM",
                dword(raw, 76)?
            ),
        )?;
    }
    record_name(raw, index)
}

fn record_size(raw: &[u8], major: u16, kind: CompoundEntryKind, index: usize) -> Result<usize> {
    if !matches!(kind, CompoundEntryKind::Root | CompoundEntryKind::Stream) {
        return Ok(0);
    }
    let lower = u64::from(dword(raw, 120)?);
    let size = if major == 3 {
        lower
    } else {
        lower | (u64::from(dword(raw, 124)?) << 32)
    };
    usize::try_from(size).map_err(|_| {
        invalid(format!(
            "directory slot {index} size {size}; expected a representable byte length"
        ))
    })
}

fn require_reachable(records: &[DirectoryRecord], seen: &[bool]) -> Result<()> {
    for (index, record) in records.iter().enumerate() {
        require(
            record.kind == CompoundEntryKind::Unused || seen[index],
            format!(
                "active directory slot {index} is unreachable; expected one root-owned directory hierarchy"
            ),
        )?;
    }
    Ok(())
}

fn validate_link(records: &[DirectoryRecord], id: usize, seen: &[bool]) -> Result<()> {
    let record = records.get(id).ok_or_else(|| {
        invalid(format!(
            "directory link {id}; expected 0..{}",
            records.len()
        ))
    })?;
    require(
        !seen[id]
            && matches!(
                record.kind,
                CompoundEntryKind::Storage | CompoundEntryKind::Stream
            ),
        format!(
            "directory slot {id}, kind {:?}, visited {}; expected one non-root active child",
            record.kind, seen[id]
        ),
    )
}

fn valid_name_units(units: &[u16], index: usize) -> Result<()> {
    require(
        !units.is_empty() && !units.iter().any(|unit| [0, 33, 47, 58, 92].contains(unit)),
        format!(
            "directory slot {index} name {units:?}; expected nonempty units without NUL, !, /, : or backslash"
        ),
    )
}

fn directory_records(bytes: &[u8], major: u16) -> Result<Vec<DirectoryRecord>> {
    bytes
        .as_chunks::<128>()
        .0
        .iter()
        .enumerate()
        .map(|(index, raw)| parse_record(raw, major, index))
        .collect()
}

fn parsed_record(
    raw: [u8; 128],
    kind: CompoundEntryKind,
    major: u16,
    index: usize,
) -> Result<DirectoryRecord> {
    let start = if matches!(kind, CompoundEntryKind::Root | CompoundEntryKind::Stream) {
        dword(&raw, 116)?
    } else {
        0
    };
    Ok(DirectoryRecord {
        name: active_name(&raw, kind, index)?,
        size: record_size(&raw, major, kind, index)?,
        raw,
        kind,
        start,
    })
}
