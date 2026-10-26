use super::raw_fields::{
    RawScalarKind::{Double, Signed, Text, Word},
    groups,
};
use super::{ModelDecoder, ModelObjectRole, features};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    raw_1614: i32,
) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_14f0")?;
    reader.u8("raw_1420")?;
    reader.i32s(&["raw_14d0", "raw_1524"])?;
    mass_caches(reader)?;
    reader.i32("raw_13f8")?;
    reader.object(
        "exploded_views",
        map,
        ModelObjectRole::Declared(&["moPrtExplViewManager_c"]),
    )?;
    reader.i32s(&["raw_1690", "raw_1694"])?;
    maps_and_sheet_metal(reader, map)?;
    if raw_1614 == 0 {
        boxes(reader, map)?;
    }
    reader.i32s(&["raw_18bc", "raw_18c0", "raw_18e8"])?;
    if reader.i32("raw_1bc8")? != 0 {
        body_ids(reader)?;
    }
    Ok(())
}

fn maps_and_sheet_metal(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.i32("map_presence")? != 0 {
        maps(reader, map)?;
    }
    sheet_metal_entries(reader, map)
}

fn sheet_metal_entries(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("sheet_metal_count", 2)? {
        reader.object(
            &format!("sheet_metal_data[{index}]"),
            map,
            ModelObjectRole::Declared(&["moPerBodySheetMetalDataNew_c"]),
        )?;
    }
    Ok(())
}

fn mass_caches(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = reader.signed_count("mass_cache_count", 4)?;
    let shape = if count <= 2 {
        Ok(())
    } else {
        Err(super::invalid(format!(
            "mass cache count {count}; expected <= 2"
        )))
    };
    reader.checked(shape)?;
    for index in 0..count {
        reader.scope(&format!("mass_caches[{index}]"), mass_cache)?;
    }
    if count > 1 {
        reader.i32("raw_1630")?;
    }
    Ok(())
}

fn mass_cache(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = reader.signed_count("entry_count", 16)?;
    for index in 0..count {
        reader.scope(&format!("entries[{index}]"), |reader| {
            groups(reader, &[(Signed, "key body_data"), (Double, "value")])
        })?;
    }
    if count != 0 {
        reader.scope("mass_properties", mass_properties)?;
    }
    Ok(())
}

fn mass_properties(reader: &mut ModelDecoder<'_>) -> Result<()> {
    groups(
        reader,
        &[
            (Double, "20 28 30 10 08 18 38 40 48 50 60 58"),
            (Signed, "1c0 1c4 1c8 1cc 1e0"),
            (Double, "00"),
            (Signed, "134 138 13c 140"),
            (Text, "148 150"),
            (Signed, "1f0"),
            (
                Double,
                "c8 d0 d8 68_x 68_y 68_z 80_x 80_y 80_z 98_x 98_y 98_z",
            ),
            (Signed, "15c"),
            (Word, "128"),
        ],
    )
}

fn maps(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("object_map_count", 6)? {
        reader.scope(&format!("object_map[{index}]"), |reader| {
            reader.i32("key")?;
            reader.object("value", map, ModelObjectRole::Any)
        })?;
    }
    for index in 0..reader.signed_count("byte_map_count", 5)? {
        reader.scope(&format!("byte_map[{index}]"), |reader| {
            reader.i32("raw_key")?;
            reader.u8("raw_value")?;
            Ok(())
        })?;
    }
    Ok(())
}

fn boxes(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("cached_box_count", 4)? {
        reader.scope(&format!("cached_boxes[{index}]"), |reader| {
            if reader.i32("key")? > 0 {
                reader.object(
                    "value",
                    map,
                    ModelObjectRole::Declared(&["moCachedBoundingBoxData_c"]),
                )?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

fn body_ids(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("body_id_count", 4)? {
        reader.i32(&format!("body_ids[{index}]"))?;
    }
    Ok(())
}

pub(super) fn exploded_manager(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    features::read(reader, map)?;
    reader.object("views", map, ModelObjectRole::Declared(&["suObArray"]))?;
    reader.object("id", map, ModelObjectRole::Declared(&["moExplPrtId_c"]))?;
    reader.f64("raw_390")?;
    reader.i32s(&["raw_398", "raw_39c"])?;
    for index in 0..reader.count_with_minimum("word_map_count", 4)? {
        reader.scope(&format!("word_map[{index}]"), |reader| {
            reader.u16("key")?;
            reader.object("value", map, ModelObjectRole::Any)
        })?;
    }
    Ok(())
}

pub(super) fn exploded_id(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("name")?;
    reader.i32("raw_18")?;
    Ok(())
}

pub(super) fn cached_box(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_c0")?;
    for (index, offset) in ["08", "0c"].into_iter().enumerate() {
        reader.scope(&format!("boxes[{index}]"), |reader| {
            reader.i32(&format!("raw_{offset}"))?;
            if reader.i32("presence")? != 0 {
                reader.f64s(&[
                    "raw_low_x",
                    "raw_low_y",
                    "raw_low_z",
                    "raw_high_x",
                    "raw_high_y",
                    "raw_high_z",
                ])?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

pub(super) fn sheet_metal(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    super::model_lists::unsigned_words(reader, "raw_array_30")?;
    reader.i32s(&["raw_08", "raw_0c"])?;
    reader.object(
        "sheet_metal",
        map,
        ModelObjectRole::Declared(&["moSheetMetal_c"]),
    )?;
    reader.object(
        "flat_pattern",
        map,
        ModelObjectRole::Declared(&["moFlatPattern_c"]),
    )?;
    reader.i32s(&["raw_28", "raw_20", "raw_24", "raw_50"])
}
