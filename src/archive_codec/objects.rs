use std::collections::BTreeMap;

use super::{ArchiveCursor, malformed};
use crate::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ArchiveMapIndex(u32);

impl ArchiveMapIndex {
    pub(crate) fn nonzero(value: u32) -> Result<Self> {
        if (1..=0x7fff_ffff).contains(&value) {
            return Ok(Self(value));
        }
        Err(malformed(format!(
            "map identity {value}; expected 1..=0x7fffffff"
        )))
    }

    pub(crate) fn value(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ArchiveClass {
    pub name: String,
    pub schema: u16,
}

#[derive(Debug, Clone)]
enum MapSlot {
    Null,
    Class(ArchiveClass),
    Object(ArchiveClass),
}

pub(crate) enum ObjectRecord {
    Null,
    New {
        index: ArchiveMapIndex,
        class: ArchiveClass,
    },
    Reference {
        index: ArchiveMapIndex,
        class: ArchiveClass,
    },
}

pub(crate) struct ObjectReader {
    slots: Vec<MapSlot>,
}

impl ObjectReader {
    pub(crate) fn new() -> Self {
        Self {
            slots: vec![MapSlot::Null],
        }
    }

    pub(crate) fn object(&mut self, cursor: &mut ArchiveCursor<'_>) -> Result<ObjectRecord> {
        let tag = cursor.u16()?;
        if tag == 0 {
            return Ok(ObjectRecord::Null);
        }
        if tag == 0xffff {
            let class = read_class(cursor)?;
            self.insert(MapSlot::Class(class.clone()))?;
            return self.new_object(class);
        }
        let (index, is_class) = read_index(cursor, tag)?;
        self.mapped_record(index, is_class)
    }

    fn mapped_record(&mut self, raw: u32, is_class: bool) -> Result<ObjectRecord> {
        let slot = self.slots.get(raw as usize).ok_or_else(|| {
            malformed(format!("map index {raw}; expected < {}", self.slots.len()))
        })?;
        match (slot, is_class) {
            (MapSlot::Null, false) => Ok(ObjectRecord::Null),
            (MapSlot::Class(class), true) => self.new_object(class.clone()),
            (MapSlot::Object(class), false) => Ok(ObjectRecord::Reference {
                index: ArchiveMapIndex::nonzero(raw)?,
                class: class.clone(),
            }),
            _ => Err(malformed(format!(
                "map index {raw} names {slot:?}; expected {}",
                if is_class { "class" } else { "object" }
            ))),
        }
    }

    fn new_object(&mut self, class: ArchiveClass) -> Result<ObjectRecord> {
        let index = self.insert(MapSlot::Object(class.clone()))?;
        Ok(ObjectRecord::New { index, class })
    }

    fn insert(&mut self, slot: MapSlot) -> Result<ArchiveMapIndex> {
        let value = u32::try_from(self.slots.len()).map_err(|_| {
            malformed(format!(
                "map length {}; expected <= 0x7fffffff",
                self.slots.len()
            ))
        })?;
        let index = ArchiveMapIndex::nonzero(value)?;
        self.slots.push(slot);
        Ok(index)
    }
}

fn read_class(cursor: &mut ArchiveCursor<'_>) -> Result<ArchiveClass> {
    let schema = cursor.u16()?;
    let length = cursor.u16()? as usize;
    let raw = cursor.take(length)?;
    if raw.is_empty() || !raw.is_ascii() {
        return Err(malformed(format!(
            "class name {raw:?}; expected nonempty ASCII"
        )));
    }
    let name = raw.iter().map(|byte| char::from(*byte)).collect();
    Ok(ArchiveClass { name, schema })
}

fn read_index(cursor: &mut ArchiveCursor<'_>, tag: u16) -> Result<(u32, bool)> {
    if tag != 0x7fff {
        return Ok((u32::from(tag & 0x7fff), tag & 0x8000 != 0));
    }
    let wide = cursor.u32()?;
    Ok((wide & 0x7fff_ffff, wide & 0x8000_0000 != 0))
}

pub(crate) struct ObjectWriter {
    classes: BTreeMap<ArchiveClass, ArchiveMapIndex>,
    next: u32,
}

impl ObjectWriter {
    pub(crate) fn new() -> Self {
        Self {
            classes: BTreeMap::new(),
            next: 1,
        }
    }

    pub(crate) fn object(
        &mut self,
        class: &ArchiveClass,
        output: &mut Vec<u8>,
    ) -> Result<ArchiveMapIndex> {
        if let Some(index) = self.classes.get(class) {
            write_tag(output, *index, true);
            return self.reserve();
        }
        write_class(output, class)?;
        let index = self.reserve()?;
        self.classes.insert(class.clone(), index);
        self.reserve()
    }

    pub(crate) fn reference(index: ArchiveMapIndex, output: &mut Vec<u8>) {
        write_tag(output, index, false);
    }

    fn reserve(&mut self) -> Result<ArchiveMapIndex> {
        let index = ArchiveMapIndex::nonzero(self.next)?;
        self.next += 1;
        Ok(index)
    }
}

fn write_class(output: &mut Vec<u8>, class: &ArchiveClass) -> Result<()> {
    let length = u16::try_from(class.name.len()).map_err(|_| {
        malformed(format!(
            "class name length {}; expected <= u16::MAX",
            class.name.len()
        ))
    })?;
    output.extend_from_slice(&0xffff_u16.to_le_bytes());
    output.extend_from_slice(&class.schema.to_le_bytes());
    output.extend_from_slice(&length.to_le_bytes());
    output.extend_from_slice(class.name.as_bytes());
    Ok(())
}

fn write_tag(output: &mut Vec<u8>, index: ArchiveMapIndex, is_class: bool) {
    if index.value() < 0x7fff {
        let tag = index.value() as u16 | if is_class { 0x8000 } else { 0 };
        output.extend_from_slice(&tag.to_le_bytes());
        return;
    }
    output.extend_from_slice(&0x7fff_u16.to_le_bytes());
    let tag = index.value() | if is_class { 0x8000_0000 } else { 0 };
    output.extend_from_slice(&tag.to_le_bytes());
}
