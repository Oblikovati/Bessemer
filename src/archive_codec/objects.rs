use std::collections::BTreeMap;

use super::{ArchiveCursor, malformed};
use crate::Result;

#[path = "object_snapshot.rs"]
mod snapshot;
pub(crate) use snapshot::ObjectMapSnapshot;

/// Nonzero persisted identity in a native MFC object map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArchiveMapIndex(u32);

impl ArchiveMapIndex {
    /// Validates a persisted object-map identity, excluding null and the class flag.
    ///
    /// ```
    /// assert_eq!(bessemer::model_archives::ModelObjectId::new(2)?.value(), 2);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn new(value: u32) -> Result<Self> {
        Self::nonzero(value)
    }

    pub(crate) fn nonzero(value: u32) -> Result<Self> {
        if (1..=0x7fff_ffff).contains(&value) {
            return Ok(Self(value));
        }
        Err(malformed(format!(
            "map identity {value}; expected 1..=0x7fffffff"
        )))
    }

    /// Returns the persisted index without implying a snapshot's ownership.
    ///
    /// ```
    /// assert_eq!(bessemer::model_archives::ModelObjectId::new(3)?.value(), 3);
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Stored MFC class name and schema, independent of the map index.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArchiveClass {
    /// Exact native runtime class name.
    pub name: String,
    /// Stored class schema.
    pub schema: u16,
}

#[derive(Debug, Clone)]
enum MapSlot {
    Null,
    Class(ArchiveClass),
    Object(ArchiveClass),
}

/// Resolved meaning of a native MFC pointer tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectRecord {
    /// Null pointer, distinct from a missing version-gated field.
    Null,
    /// New object installed in the shared map before reading its body.
    New {
        /// Assigned native object-map index.
        index: ArchiveMapIndex,
        /// Resolved native runtime class.
        class: ArchiveClass,
    },
    /// Alias to an earlier loaded object.
    Reference {
        /// Earlier object's native map index.
        index: ArchiveMapIndex,
        /// Resolved class of that earlier object.
        class: ArchiveClass,
    },
}

/// Exact short or extended MFC pointer framing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectFraming {
    /// Native tag word, including null, new-class and extended markers.
    pub tag: u16,
    /// Extension present only for tag 0x7FFF.
    pub extended: Option<u32>,
}

/// Pointer framing together with its validated shared-map interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramedObjectRecord {
    /// Exact pointer framing retained for reconstruction.
    pub framing: ObjectFraming,
    /// Resolved target, checked again when constructing a complete archive.
    pub record: ObjectRecord,
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
        self.framed_object(cursor).map(|framed| framed.record)
    }

    pub(crate) fn framed_object(
        &mut self,
        cursor: &mut ArchiveCursor<'_>,
    ) -> Result<FramedObjectRecord> {
        let tag = cursor.u16()?;
        let extended = if tag == 0x7fff {
            Some(cursor.u32()?)
        } else {
            None
        };
        let record = self.record_for_tag(cursor, tag, extended)?;
        Ok(FramedObjectRecord {
            framing: ObjectFraming { tag, extended },
            record,
        })
    }

    fn record_for_tag(
        &mut self,
        cursor: &mut ArchiveCursor<'_>,
        tag: u16,
        extended: Option<u32>,
    ) -> Result<ObjectRecord> {
        if tag == 0 {
            return Ok(ObjectRecord::Null);
        }
        if tag == 0xffff {
            let class = read_class(cursor)?;
            self.insert(MapSlot::Class(class.clone()))?;
            return self.new_object(class);
        }
        let (index, is_class) = map_tag_index(tag, extended);
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
        let index = self.append_loaded_object(Some(class.clone()))?;
        Ok(ObjectRecord::New { index, class })
    }

    /// Imports a loaded object without a class slot or stream bytes.
    /// Model header callers use `append_loaded_object(Some(class))` in native list order.
    pub(crate) fn append_loaded_object(
        &mut self,
        class: Option<ArchiveClass>,
    ) -> Result<ArchiveMapIndex> {
        let slot = class.map(MapSlot::Object).unwrap_or(MapSlot::Null);
        self.insert(slot)
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

fn map_tag_index(tag: u16, extended: Option<u32>) -> (u32, bool) {
    match extended {
        Some(wide) => (wide & 0x7fff_ffff, wide & 0x8000_0000 != 0),
        None => (u32::from(tag & 0x7fff), tag & 0x8000 != 0),
    }
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

#[cfg(test)]
#[path = "object_import_tests.rs"]
mod object_import_tests;

#[cfg(test)]
#[path = "../../tests/model_archives/header_imports.rs"]
mod native_header_import_tests;
