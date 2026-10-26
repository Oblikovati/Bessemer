use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) struct SweepControls {
    pub dimensions: bool,
    pub second_dimension: bool,
    pub angular: bool,
    pub radial: bool,
}

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<SweepControls> {
    arrays(reader, map, &["2c0", "2e0", "300"])?;
    let angular = prefix(reader, map)?;
    let (dimensions, second_dimension) = middle(reader, map)?;
    let radial = late(reader)?;
    declared(
        reader,
        map,
        "twist_298",
        &["moTwistDataForBiDirectionalSweep_c"],
    )?;
    declared(
        reader,
        map,
        "end_point_2a0",
        &["moEndPointRef_w", "moLoftSynchRef_c"],
    )?;
    reader.u8("raw_2b8")?;
    arrays(reader, map, &["320", "340"])?;
    if reader.version() > 0x27d9 {
        reader.i32("raw_3b0")?;
    }
    Ok(SweepControls {
        dimensions,
        second_dimension,
        angular,
        radial,
    })
}

fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<bool> {
    bytes(reader, &["270", "274"])?;
    let angular = matches!(reader.i32("raw_278")?, 8 | 9);
    reader.f64("raw_70")?;
    bytes(reader, &["2ac", "2b0"])?;
    let directions = [reader.i32("raw_7c")?, reader.i32("raw_80")?];
    for (value, label) in directions.into_iter().zip(["direction_88", "direction_90"]) {
        if value == 2 {
            declared(reader, map, label, &["moDirectionSpec_c"])?;
        }
    }
    Ok(angular)
}

fn middle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<(bool, bool)> {
    for label in ["list_98", "list_e0"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    reader.i32("raw_230")?;
    reader.u8("raw_78")?;
    let dimensions = reader.u8("raw_234")? != 0;
    reader.f64s(&["raw_238", "raw_240"])?;
    let second = reader.i32("raw_248")? == 3;
    reader.i32s(&["raw_24c", "raw_2b4"])?;
    Ok((dimensions, second))
}

fn late(reader: &mut ModelDecoder<'_>) -> Result<bool> {
    reader.f64s(&["raw_280", "raw_288"])?;
    reader.i32("raw_290")?;
    bytes(reader, &["27c", "2a8"])?;
    let radial = reader.i32("raw_258")? != 0;
    reader.f64("raw_260")?;
    Ok(radial)
}

pub(super) fn bytes(reader: &mut ModelDecoder<'_>, offsets: &[&str]) -> Result<()> {
    for offset in offsets {
        reader.u8(&format!("raw_{offset}"))?;
    }
    Ok(())
}

fn arrays(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offsets: &[&str]) -> Result<()> {
    for offset in offsets {
        model_lists::pointers(
            reader,
            map,
            &format!("array_{offset}"),
            ModelObjectRole::Any,
        )?;
    }
    Ok(())
}
