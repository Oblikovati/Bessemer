use super::{MaterialRead, line_style};
use crate::Result;

pub(crate) fn read(archive: &mut impl MaterialRead) -> Result<()> {
    archive.u16("unknown_0")?;
    archive.u16("unknown_2")?;
    archive.u8("unknown_4")?;
    archive.u32("unknown_8")?;
    archive.u16("layer")?;
    archive.scope("line_width", line_style::short_width)
}
