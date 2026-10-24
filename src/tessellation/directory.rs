use super::codec::FacetCodec;
use super::context::{CollectionClass, FacetState};
use super::output::FacetOutput;
use super::record::archive_record;
use super::{ArchiveString, invalid, unsupported};
use crate::Result;
use crate::archive_codec::ArchiveCursor;

archive_record! {
    /// One ordered key/first/second record; duplicate keys remain separate records.
    TessellationNumber(context, 12, Element) {
        /// Stored map key.
        key: required i32 => true;
        /// First stored value.
        first: required i32 => true;
        /// Second stored value.
        second: required i32 => true;
    }
}

/// Version-dependent component tessellation-number storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentTessellationNumbers {
    /// No field before version 0x3B1C.
    Absent,
    /// One pair at 0x3B1C..=0x3B22, with implicit runtime key -1.
    LegacyPair([i32; 2]),
    /// Direct count and ordered records from 0x3B23, including duplicate keys.
    Map(Vec<TessellationNumber>),
}

impl FacetCodec for ComponentTessellationNumbers {
    const MINIMUM: usize = 0;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        if context.version >= 0x3b23 {
            return Vec::<TessellationNumber>::read(cursor, context).map(Self::Map);
        }
        if context.version >= 0x3b1c {
            return <[i32; 2]>::read(cursor, context).map(Self::LegacyPair);
        }
        Ok(Self::Absent)
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        match self {
            Self::Absent if context.version < 0x3b1c => Ok(()),
            Self::LegacyPair(pair) if (0x3b1c..0x3b23).contains(&context.version) => {
                pair.write(output, context)
            }
            Self::Map(records) if context.version >= 0x3b23 => records.write(output, context),
            _ => Err(invalid(format!(
                "component tessellation-number representation at version {:#x}; expected absent before 0x3B1C, one pair through 0x3B22, map afterward",
                context.version
            ))),
        }
    }
}

/// One complete `partCompInfo_c` directory entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartComponentInfo {
    /// Stored assembly tessellation-array index.
    pub tess_array_index: i32,
    /// Complete component name/path CString.
    pub component_id: ArchiveString,
    /// Stored component model update stamp.
    pub model_update_stamp: u32,
    /// Stored FILETIME, high word first on the wire.
    pub user_component_time: u64,
    /// Shared entry key, high word first.
    pub shared_part_component_info: u64,
    /// Component hash key, high word first.
    pub hash_key: u64,
    /// Complete block-name CString.
    pub block_name: ArchiveString,
    /// Stored position identifier in the block.
    pub index_in_block: i32,
    /// All version-dependent tessellation-number records.
    pub tessellation_numbers: ComponentTessellationNumbers,
}

impl FacetCodec for PartComponentInfo {
    const MINIMUM: usize = 38;
    const CLASS: CollectionClass = CollectionClass::Component;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        context.begin(Self::CLASS)?;
        Ok(Self {
            tess_array_index: cursor.i32()?,
            component_id: ArchiveString::read(cursor, context)?,
            model_update_stamp: cursor.u32()?,
            user_component_time: read_high_words(cursor)?,
            shared_part_component_info: read_high_words(cursor)?,
            hash_key: read_high_words(cursor)?,
            block_name: ArchiveString::read(cursor, context)?,
            index_in_block: cursor.i32()?,
            tessellation_numbers: ComponentTessellationNumbers::read(cursor, context)?,
        })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        context.begin(Self::CLASS)?;
        output.i32(self.tess_array_index);
        self.component_id.write(output, context)?;
        output.u32(self.model_update_stamp);
        output.high_words(self.user_component_time);
        output.high_words(self.shared_part_component_info);
        output.high_words(self.hash_key);
        self.block_name.write(output, context)?;
        output.i32(self.index_in_block);
        self.tessellation_numbers.write(output, context)
    }
}

/// Owned contents of `FaceTessellations/Directory`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TessellationDirectoryContents {
    /// Supported directory layout version, currently 2.
    pub directory_version: i32,
    /// Last block number written, retained without renumbering streams.
    pub last_block_number: i32,
    /// Component entries in their original order.
    pub entries: Vec<PartComponentInfo>,
}

impl FacetCodec for TessellationDirectoryContents {
    const MINIMUM: usize = 12;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let directory_version = cursor.i32()?;
        require_directory_version(directory_version)?;
        Ok(Self {
            directory_version,
            last_block_number: cursor.i32()?,
            entries: Vec::<PartComponentInfo>::read(cursor, context)?,
        })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        require_directory_version(self.directory_version)?;
        output.i32(self.directory_version);
        output.i32(self.last_block_number);
        self.entries.write(output, context)
    }
}

pub(super) fn read_high_words(cursor: &mut ArchiveCursor<'_>) -> Result<u64> {
    let high = u64::from(cursor.u32()?);
    let low = u64::from(cursor.u32()?);
    Ok(high << 32 | low)
}

fn require_directory_version(version: i32) -> Result<()> {
    if version == 2 {
        return Ok(());
    }
    Err(unsupported(format!(
        "tessellation directory version {version}; expected 2"
    )))
}
