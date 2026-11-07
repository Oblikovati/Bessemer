use super::arrays::DynamicArray;
use super::codec::{FacetCodec, read_optional, write_optional};
use super::context::FacetState;
use super::output::FacetOutput;
use crate::Result;
use crate::archive_codec::ArchiveCursor;

/// Version-dependent triangle-index flag and nullable array.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacetTriangleIndices {
    /// Stored only after version 0x1C3A.
    pub save_triangle_indices: Option<i32>,
    /// Present at 0x1C24..=0x1C3A, or afterward when the save flag is nonzero.
    pub indices: Option<DynamicArray>,
}

impl FacetCodec for FacetTriangleIndices {
    const MINIMUM: usize = 0;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        let present_flag = context.version > 0x1c3a;
        let save_triangle_indices = read_optional(cursor, context, present_flag)?;
        let enabled = enabled_indices(context.version, save_triangle_indices);
        let indices = read_optional(cursor, context, enabled)?;
        Ok(Self {
            save_triangle_indices,
            indices,
        })
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        let present_flag = context.version > 0x1c3a;
        write_optional(
            &self.save_triangle_indices,
            output,
            context,
            present_flag,
            "save triangle indices",
        )?;
        let enabled = enabled_indices(context.version, self.save_triangle_indices);
        write_optional(
            &self.indices,
            output,
            context,
            enabled,
            "triangle index array",
        )
    }
}

fn enabled_indices(version: u32, flag: Option<i32>) -> bool {
    version > 0x1c23 && (version < 0x1c3b || flag.unwrap_or(0) != 0)
}
