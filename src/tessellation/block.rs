use super::codec::FacetCodec;
use super::context::{CollectionClass, FacetState};
use super::directory::read_high_words;
use super::facets::FacetContainer;
use super::output::FacetOutput;
use crate::Result;
use crate::archive_codec::ArchiveCursor;

/// A block entry's identity and complete ordered face list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacetBlockComponent {
    /// Stored position identifier matched against directory entries.
    pub index_in_block: i32,
    /// Component hash key, high word first on the wire.
    pub hash_key: u64,
    /// All archived faces in stored order.
    pub faces: Vec<FacetContainer>,
}

impl FacetCodec for FacetBlockComponent {
    const MINIMUM: usize = 16;
    const CLASS: CollectionClass = CollectionClass::Component;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        context.begin(Self::CLASS)?;
        Ok(Self {
            index_in_block: cursor.i32()?,
            hash_key: read_high_words(cursor)?,
            faces: Vec::<FacetContainer>::read(cursor, context)?,
        })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        context.begin(Self::CLASS)?;
        output.i32(self.index_in_block);
        output.high_words(self.hash_key);
        self.faces.write(output, context)
    }
}

/// Owned contents of one `FaceTessellations/NNN-NNN-NNN` stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaceTessellationBlockContents {
    /// Entries extend to file end; an empty stream is a complete empty block.
    pub entries: Vec<FacetBlockComponent>,
}

impl FacetCodec for FaceTessellationBlockContents {
    const MINIMUM: usize = 0;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let mut entries = Vec::new();
        while cursor.remaining() != 0 {
            entries.push(FacetBlockComponent::read(cursor, context)?);
        }
        Ok(Self { entries })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        for entry in &self.entries {
            entry.write(output, context)?;
        }
        Ok(())
    }
}
