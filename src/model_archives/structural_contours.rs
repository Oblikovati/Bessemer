use super::display_records::declared;
use super::{
    ModelDecoder, ModelObjectRole, model_lists, structural_member_references as references,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn contours(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.signed_count("contours.count", 2)? {
        declared(
            reader,
            map,
            &format!("contours[{index}]"),
            &["moWeldSingleContour_c"],
        )?;
    }
    reader.f64("raw_f0")?;
    reader.f64("raw_f8")?;
    Ok(())
}

pub(super) fn orientation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    references::curve(reader, map, "curve_08")?;
    references::point(reader, map, "point_10")?;
    declared(reader, map, "direction_18", &["moDirectionSpec_c"])?;
    reader.i32s(&["raw_20", "raw_24"])?;
    reader.f64("raw_28")?;
    references::point(reader, map, "point_108")?;
    reader.i32s(&["raw_114", "raw_118", "raw_11c"])?;
    reader.f64("raw_120")?;
    reader.i32s(&["raw_30", "raw_110"])
}

pub(super) fn single(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "orientation_20", &["moContourOrientation_c"])?;
    model_lists::pointers(reader, map, "array_28", ModelObjectRole::Any)?;
    reader.f64("raw_50")?;
    reader.f64("raw_58")?;
    reader.i32s(&["raw_60", "raw_64", "raw_6c", "raw_68"])
}

pub(super) fn segment(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    references::curve(reader, map, "curve_08")?;
    reader.i32s(&["raw_10", "raw_14", "raw_18"])?;
    for label in ["20", "28", "30", "38"] {
        reader.f64(&format!("raw_{label}"))?;
    }
    model_lists::pointers(reader, map, "array_50", ModelObjectRole::Any)?;
    reader.i32s(&["raw_40", "raw_44", "raw_48", "raw_cc", "raw_d0"])
}
