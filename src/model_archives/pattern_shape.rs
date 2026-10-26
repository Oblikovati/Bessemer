use super::ModelDecoder;
use super::display_records::declared;
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    let kind = reader.i32("type")?;
    if kind == 0 {
        return Ok(());
    }
    dimensions(reader, map, kind)?;
    if reader.i32("point_mode_98")? != 0 {
        double_vector(reader, "point_90")?;
    } else {
        declared(reader, map, "reference_68", &["moRefWrapper_c"])?;
    }
    double_vector(reader, "vector_18")?;
    trailer(reader, map, kind)
}

fn trailer(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, kind: i32) -> Result<()> {
    if kind != 1 {
        if ![2, 3].contains(&kind) {
            reader.i32("word_80")?;
        }
        declared(reader, map, "dimension_60", &["moDisplayAngularDim_c"])?;
    }
    reader.i32("word_9c")?;
    Ok(())
}

fn dimensions(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, kind: i32) -> Result<()> {
    if kind == 4 {
        declared(reader, map, "dimension_40", &["moDisplayRadialDim_c"])?;
    }
    if [1, 4].contains(&kind) {
        declared(reader, map, "dimension_48", &["moDisplayRadialDim_c"])?;
    }
    if kind == 3 {
        declared(reader, map, "dimension_58", &["moDisplayDistanceDim_c"])?;
    }
    if [2, 3].contains(&kind) {
        declared(reader, map, "dimension_50", &["moDisplayDistanceDim_c"])?;
    }
    Ok(())
}
