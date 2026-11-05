use super::{ModelDecoder, ModelObjectRole, one_bends};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the complete One Bend base before its tagged sketch-handle suffix.
/// Example: `sketch_bends::read(reader, map, 1)?` for `moSketchBend_c`.
pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    schema: u16,
) -> Result<()> {
    let kind = reader.scope("one_bend", |reader| one_bends::read_kind(reader, map))?;
    if kind == 7 {
        reader.require_sketch_bend_owned(schema)?;
        reader.scope("owned_be8", |reader| {
            super::sketch_bend_owned::read_face(reader, map)
        })?;
    }
    reader.scope("payload", |reader| continuation(reader, map))?;
    if kind == 7 {
        reader.scope("body_bf0", super::kernel_frames::read_raw_body)?;
    }
    Ok(())
}

fn continuation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["sketch_e00", "handle_df0", "handle_e18", "handle_e40"] {
        reader.object(label, map, ModelObjectRole::Any)?;
    }
    reader.object(
        "curve_de0",
        map,
        ModelObjectRole::Declared(super::reference_point_data::CURVES),
    )?;
    reader.i32("raw_e70")?;
    Ok(())
}
