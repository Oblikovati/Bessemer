use super::arrays::DynamicArray;
use super::codec::{FacetCodec, read_optional, write_optional};
use super::context::FacetState;
use super::output::FacetOutput;
use super::{ArchiveString, StoredFloat, invalid};
use crate::Result;
use crate::archive_codec::ArchiveCursor;

/// A raw i32 presence word and its conditional value; Send/Sync follows `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flagged<T> {
    /// Every nonzero value selects the payload, without normalizing the stored word.
    pub flag: i32,
    /// Present exactly when the flag is nonzero.
    pub value: Option<T>,
}

impl<T: FacetCodec> FacetCodec for Flagged<T> {
    const MINIMUM: usize = 4;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let flag = cursor.i32()?;
        let value = read_optional(cursor, context, flag != 0)?;
        Ok(Self { flag, value })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        output.i32(self.flag);
        write_optional(
            &self.value,
            output,
            context,
            self.flag != 0,
            "flagged payload",
        )
    }
}

/// Two consecutive values; Send/Sync follows `T`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayPair<T> {
    /// First stored value.
    pub first: T,
    /// Second stored value.
    pub second: T,
}

impl<T: FacetCodec> FacetCodec for ArrayPair<T> {
    const MINIMUM: usize = T::MINIMUM * 2;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        Ok(Self {
            first: T::read(cursor, context)?,
            second: T::read(cursor, context)?,
        })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        self.first.write(output, context)?;
        self.second.write(output, context)
    }
}

/// Byte-width presence framing for auxiliary texture coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteFlaggedArray {
    /// Raw byte; nonzero selects the array.
    pub flag: u8,
    /// The complete nullable dynamic-array value when selected.
    pub value: Option<DynamicArray>,
}

impl FacetCodec for ByteFlaggedArray {
    const MINIMUM: usize = 1;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let flag = cursor.u8()?;
        let value = read_optional(cursor, context, flag != 0)?;
        Ok(Self { flag, value })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        output.u8(self.flag);
        write_optional(
            &self.value,
            output,
            context,
            self.flag != 0,
            "auxiliary texture array",
        )
    }
}

/// Legacy array or modern curvature flag and conditional colours.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FacetCurvature {
    /// Versions through 0x330 store a dynamic array.
    Legacy(DynamicArray),
    /// Later versions store a raw flag and optional direct f32 array.
    Calculated(Flagged<Vec<StoredFloat>>),
}

impl FacetCodec for FacetCurvature {
    const MINIMUM: usize = 4;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        if context.version <= 0x330 {
            context.curvature = false;
            return DynamicArray::read(cursor, context).map(Self::Legacy);
        }
        let value = Flagged::<Vec<StoredFloat>>::read(cursor, context)?;
        context.curvature = value.flag != 0;
        Ok(Self::Calculated(value))
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        context.curvature = false;
        match self {
            Self::Legacy(value) if context.version <= 0x330 => value.write(output, context),
            Self::Calculated(value) if context.version > 0x330 => {
                context.curvature = value.flag != 0;
                value.write(output, context)
            }
            _ => Err(invalid(format!(
                "curvature representation at version {:#x}; expected legacy through 0x330, calculated afterward",
                context.version
            ))),
        }
    }
}

/// Display-colour flag with colours present only when curvature is also selected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacetDisplayColor {
    /// Raw display-colour word.
    pub flag: i32,
    /// Direct colours, selected by this flag and the preceding curvature flag together.
    pub colors: Option<Vec<i32>>,
}

impl FacetCodec for FacetDisplayColor {
    const MINIMUM: usize = 4;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let flag = cursor.i32()?;
        let enabled = flag != 0 && context.curvature;
        let colors = read_optional(cursor, context, enabled)?;
        Ok(Self { flag, colors })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        output.i32(self.flag);
        let enabled = self.flag != 0 && context.curvature;
        write_optional(&self.colors, output, context, enabled, "display colours")
    }
}

/// Inverse presence flag: zero selects a CString, nonzero stores no string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacetString {
    /// Raw empty-string word; no normalization is applied.
    pub empty_flag: i32,
    /// Present exactly for a zero word, including a stored empty CString.
    pub text: Option<ArchiveString>,
}

impl FacetCodec for FacetString {
    const MINIMUM: usize = 4;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let empty_flag = cursor.i32()?;
        let text = read_optional(cursor, context, empty_flag == 0)?;
        Ok(Self { empty_flag, text })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        output.i32(self.empty_flag);
        write_optional(
            &self.text,
            output,
            context,
            self.empty_flag == 0,
            "face string",
        )
    }
}
