use super::display_records::declared;
use super::{ModelDecoder, body_features, node::ModelNodePrefix};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Select raw-body framing from the containing feature's decoded node flag.
/// Example: `flatten_bends::read(reader, map)?` with an explicit kernel context.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let node = prefix(reader, map)?;
    reader.scope("payload", |reader| payload(reader, node.flags_28))
}

fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<ModelNodePrefix> {
    let node = reader.scope("body", |reader| body_features::base_with_node(reader, map))?;
    reader.scope("bend_group", |reader| {
        super::base_flanges::bend_group(reader, map)
    })?;
    Ok(node)
}

fn payload(reader: &mut ModelDecoder<'_>, flags: u32) -> Result<()> {
    reader.i32("raw_8c8")?;
    if flags & 0x800 == 0 {
        return Ok(());
    }
    // C-287: ordinary context clears helper +834 and transfer mode, independently of saved +850.
    if reader.i32("body_presence_8c0")? != 0 {
        reader.scope("body_8c0", super::kernel_frames::read_raw_body)?;
    }
    Ok(())
}

/// Preserve the fixed-face reference and raw process option in native order.
/// Example: `flatten_bends::process(reader, map)?` for ordinary process bends.
pub(super) fn process(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    prefix(reader, map)?;
    reader.scope("payload", |reader| {
        declared(reader, map, "fixed_face_888", &["moFixedFaceInFlat_c"])?;
        reader.i32("raw_890")?;
        Ok(())
    })
}
