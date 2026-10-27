use super::{ModelDecoder, ModelObjectRole, model_lists, structural_contours};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
        "moWeldMemberFeat_c" => super::structural_member_records::read(reader, map),
        "moProfileRef_w" => profile(reader, map),
        "moWeldmentContours_c" => structural_contours::contours(reader, map),
        "moAdvCusPropList_c" | "moWeldSegSkIndices_c" => packed(reader, map),
        "moWeldSegSkIndex_c" => index(reader),
        "moContourOrientation_c" => structural_contours::orientation(reader, map),
        "moWeldSingleContour_c" => structural_contours::single(reader, map),
        "moWeldSegment_c" => structural_contours::segment(reader, map),
        _ => return None,
    };
    Some(result)
}

fn profile(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::model_references::wrapper(reader, map)?;
    reader.u32("raw_a8")?;
    Ok(())
}

fn packed(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
}

fn index(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_0c", "raw_10"])
}
