use super::{ModelDecoder, ModelObjectRole, configuration_io as io};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.unsigned_count("map_5f8.count", 6)? {
        reader.i32(&format!("map_5f8[{index}].key"))?;
        reader.object(
            &format!("map_5f8[{index}].material"),
            map,
            ModelObjectRole::Declared(&["moCustomDefinedMaterialProperty_c"]),
        )?;
    }
    string_values(reader)?;
    reader.i32("persisted_state_650")?;
    for index in 0..reader.unsigned_count("map_640.count", 6)? {
        reader.i32(&format!("map_640[{index}].key"))?;
        reader.object(
            &format!("map_640[{index}].pair"),
            map,
            ModelObjectRole::Declared(&["moSMAndFPFeatIdPair"]),
        )?;
    }
    property_values(reader, map)?;
    reader.object("list_6c8", map, ModelObjectRole::Declared(&["suObList"]))?;
    owned_maps(reader, map)?;
    io::pointers(
        reader,
        map,
        "conversion_ids_700",
        ModelObjectRole::Declared(&["moConvertFeatIds_c"]),
    )?;
    io::strings(reader, "strings_728")?;
    reader.i32("raw_750")?;
    reader.u32("raw_754")?;
    final_maps(reader)
}

fn string_values(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..io::positive_count(reader, "map_610.count", 8)? {
        reader.i32(&format!("map_610[{index}].key"))?;
        for item in 0..io::positive_count(reader, &format!("map_610[{index}].count"), 1)? {
            reader.text(&format!("map_610[{index}].strings[{item}]"))?;
        }
    }
    Ok(())
}

fn property_values(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.unsigned_count("map_6a8.count", 3)? {
        reader.text(&format!("map_6a8[{index}].key"))?;
        reader.object(
            &format!("map_6a8[{index}].properties"),
            map,
            ModelObjectRole::Declared(&["moCustomPropertyList"]),
        )?;
    }
    Ok(())
}

fn owned_maps(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    io::empty_signed(reader, "owned_6d8.objects_count", 6)?;
    io::empty_signed(reader, "owned_6d8.associations_count", 8)?;
    super::configuration_scenes::owned_map(reader, map, "owned_6e0")
}

fn final_maps(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = reader.unsigned_count("plane_map_7f0.count", 1)?;
    io::empty(reader, "plane_map_7f0", count)?;
    reader.text("string_808")?;
    reader.i32s(&["raw_810", "raw_814"])?;
    for index in 0..io::positive_count(reader, "pairs_818.count", 2)? {
        reader.text(&format!("pairs_818[{index}].key"))?;
        reader.text(&format!("pairs_818[{index}].value"))?;
    }
    reader.u32("time_828")?;
    set_values(reader)?;
    reader.i32("raw_858")?;
    let presence = reader.u32("display_maps_presence")?;
    io::empty(reader, "display_maps_898", presence as usize)
}

fn set_values(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..io::positive_count(reader, "map_6b8.count", 8)? {
        reader.i32(&format!("map_6b8[{index}].key"))?;
        for item in 0..reader.unsigned_count(&format!("map_6b8[{index}].count"), 4)? {
            reader.i32(&format!("map_6b8[{index}].values[{item}]"))?;
        }
    }
    Ok(())
}
