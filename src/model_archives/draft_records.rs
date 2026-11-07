use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::{ModelDecoder, body_features, draft_keepers};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    references(reader, map)?;
    if reader.i32("raw_870")? != 0 {
        super::sheet_arrays::count(reader, "dynamic_doubles_878")?;
    }
    reader.i32("raw_880")?;
    if reader.i32("face_groups_presence")? != 0 {
        face_groups(reader, map)?;
    }
    for index in 0..reader.signed_count("points_8c8.count", 24)? {
        double_vector(reader, &format!("points_8c8[{index}]"))?;
    }
    late_words(reader, map)?;
    reader.scope("keeper_838", |reader| draft_keepers::read(reader, map))
}

fn references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "dimension_800", DIMENSIONS)?;
    reader.i32("raw_830")?;
    declared(reader, map, "direction_868", &["moDirectionSpec_c"])?;
    declared(reader, map, "word_array_828", &["su_CWordArray"])?;
    reader.scope("array_818", |reader| {
        body_features::nullable_array(reader, map)
    })?;
    reader.i32s(&["raw_860", "raw_840", "raw_844"])?;
    reader.scope("array_820", |reader| {
        body_features::nullable_array(reader, map)
    })?;
    reader.i32("raw_848")?;
    declared(reader, map, "surface_body_850", &["moCompSurfaceBody_c"])?;
    declared(reader, map, "plane_858", &["moPlaneRef_w"])
}

fn face_groups(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_72c")?;
    for index in 0..reader.signed_count("face_groups_890.count", 4)? {
        reader.scope(&format!("face_groups_890[{index}]"), |reader| {
            face_group(reader, map)
        })?;
    }
    Ok(())
}

fn face_group(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("faces.count", 2)? {
        declared(reader, map, &format!("faces[{index}]"), &["moCompFace_c"])?;
    }
    Ok(())
}

fn late_words(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() <= 0x3746 {
        return Ok(());
    }
    declared(reader, map, "dimension_808", DIMENSIONS)?;
    reader.i32s(&["raw_810", "raw_814"])
}
