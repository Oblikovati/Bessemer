use super::{ModelDecoder, configuration_io};
use crate::Result;
use crate::archive_codec::MaterialRead;

/// Preserve ordered, untagged visual-property values in a configuration map.
/// Example: `configuration_visual_maps::read(reader)?` at the +B50 count.
pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..configuration_io::positive_count(reader, "visual_count_b50", 4)? {
        reader.scope(&format!("visual_map_b50[{index}]"), visual_entry)?;
    }
    Ok(())
}

fn visual_entry(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("key")?;
    reader.scope("visual", crate::archive_codec::material::visual)
}
