use super::ModelDecoder;
use super::component_references::FEATURE_CLASSES;
use super::display_dimension::CLASSES as DISPLAY_CLASSES;
use super::display_records::declared;
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("common_bend_800", |reader| common(reader, map))?;
    reader.i32s(&["raw_840", "raw_844", "raw_848"])?;
    reader.f64("raw_8f0")?;
    declared(reader, map, "dimension_8f8", DISPLAY_CLASSES)?;
    reader.i32("raw_900")?;
    declared(reader, map, "dimension_908", DISPLAY_CLASSES)?;
    declared(reader, map, "fixed_face_910", &["moFixedFaceInFlat_c"])?;
    reader.i32s(&["raw_930", "raw_950"])?;
    reader.text("raw_958")?;
    reader.i32s(&["raw_960", "raw_964", "raw_968", "raw_96c"])?;
    tail(reader)
}

pub(super) fn common(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "dimension_10", DISPLAY_CLASSES)?;
    declared(reader, map, "feature_18", FEATURE_CLASSES)?;
    if reader.u8("has_allowance")? != 0 {
        reader.scope("allowance_08", |reader| {
            super::sheet_allowance::read(reader, map)
        })?;
    }
    reader.i32s(&["raw_00", "raw_04", "raw_discarded_1", "raw_discarded_2"])?;
    if reader.sheet_metal_extended() {
        declared(reader, map, "sheet_feature_20", &["moCompSheetmetalFeat_c"])?;
    }
    Ok(())
}

fn tail(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if !reader.sheet_metal_extended() {
        return Ok(());
    }
    reader.i32s(&["raw_978", "raw_974"])?;
    if reader.version() >= 0x2bc5 {
        reader.i32("raw_97c")?;
    }
    Ok(())
}
