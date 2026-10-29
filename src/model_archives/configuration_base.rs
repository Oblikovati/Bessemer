use super::raw_fields::RawScalarKind::{Octet, Signed, Text, Unsigned};
use super::{ModelDecoder, ModelObjectRole, configuration_io as io, features, raw_fields};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::configuration_manager::require_manager_scope(reader)?;
    reader.scope("feature", |reader| features::without_children(reader, map))?;
    identity_fields(reader)?;
    relationship_fields(reader, map)?;
    initial_maps(reader, map)?;
    common_maps(reader, map)?;
    late_maps(reader, map)?;
    super::configuration_suffix::read(reader, map)
}

fn identity_fields(reader: &mut ModelDecoder<'_>) -> Result<()> {
    raw_fields::groups(
        reader,
        &[
            (Text, "4a0"),
            (Signed, "4a8"),
            (Unsigned, "4bc"),
            (Text, "490"),
            (Unsigned, "854 858 668"),
        ],
    )?;
    reader.scope("mass_properties_660", super::part_tail::mass_properties)?;
    reader.u32("time_670")?;
    Ok(())
}

fn relationship_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object(
        "parent_698",
        map,
        ModelObjectRole::Declared(&["moPartConfiguration_c"]),
    )?;
    io::list(reader, map, "children_6a0")?;
    raw_fields::groups(reader, &[(Signed, "6f0"), (Octet, "680 960")])?;
    io::list(reader, map, "list_520")?;
    raw_fields::groups(reader, &[(Unsigned, "9a0"), (Octet, "964")])?;
    let presence = reader.u8("has_data_608")?;
    io::empty(reader, "optional_data_608", usize::from(presence))?;
    reader.object(
        "toolbox_658",
        map,
        ModelObjectRole::Declared(&["moToolboxPartSpecArray_c"]),
    )?;
    io::list(reader, map, "list_a90")?;
    reader.i32s(&["raw_ae8", "raw_aec"])
}

fn initial_maps(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..io::positive_count(reader, "map_610.count", 10)? {
        reader.u32(&format!("map_610[{index}].key_low"))?;
        reader.u32(&format!("map_610[{index}].key_high"))?;
        reader.object(
            &format!("map_610[{index}].object"),
            map,
            ModelObjectRole::Any,
        )?;
    }
    reader.i32s(&["raw_77c", "raw_780"])?;
    reader.f32("raw_784")?;
    reader.f32("raw_788")?;
    io::pointers(
        reader,
        map,
        "array_350",
        ModelObjectRole::Declared(&["moSingleFaceRef_w"]),
    )?;
    io::pointers(
        reader,
        map,
        "array_378",
        ModelObjectRole::Declared(&["moRefWrapper_c"]),
    )?;
    raw_fields::fields(reader, Signed, "b28 820 824 828 82c 830 834 838 83c")?;
    io::pointers(
        reader,
        map,
        "array_3c8",
        ModelObjectRole::Declared(&["moRefWrapper_c"]),
    )?;
    raw_fields::fields(reader, Signed, "748 bcc 848")
}

fn common_maps(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.unsigned_count("map_418.count", 6)? {
        reader.i32(&format!("map_418[{index}].key"))?;
        reader.object(
            &format!("map_418[{index}].visual"),
            map,
            ModelObjectRole::Declared(&["moVisualProperties_c"]),
        )?;
    }
    for index in 0..io::positive_count(reader, "map_428.count", 8)? {
        reader.i32(&format!("map_428[{index}].key"))?;
        io::integer_pairs(reader, &format!("map_428[{index}].values"), false)?;
    }
    io::integer_pairs(reader, "map_af8_08", false)?;
    reader.i32("raw_b00")?;
    for index in 0..io::positive_count(reader, "map_458.count", 8)? {
        reader.i32(&format!("map_458[{index}].key"))?;
        reader.i32(&format!("map_458[{index}].value"))?;
    }
    io::integer_pairs(reader, "map_b08", false)
}

fn late_maps(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    io::integer_pairs(reader, "map_a48", true)?;
    io::empty_string_int_map(reader, "strings_5f8")?;
    reader.i32("raw_b2c")?;
    io::empty_signed(reader, "visual_count_b50", 4)?;
    super::custom_properties::list(reader, "properties_b60")?;
    reader.object(
        "abbreviated_views_b78",
        map,
        ModelObjectRole::Declared(&["moAbbrViewManager_c"]),
    )?;
    reader.i32("raw_744")?;
    for index in 0..io::positive_count(reader, "set_a68.count", 4)? {
        reader.u32(&format!("set_a68[{index}]"))?;
    }
    super::configuration_atoms::linked_map(reader, map, "map_448")?;
    let presence = reader.i32("print_settings_presence")?;
    io::absent(reader, "print_settings_b80", presence)?;
    for index in 0..reader.unsigned_count("property_map_b68.count", 3)? {
        reader.text(&format!("property_map_b68[{index}].key"))?;
        reader.object(
            &format!("property_map_b68[{index}].properties"),
            map,
            ModelObjectRole::Declared(&["moCustomPropertyList"]),
        )?;
    }
    final_base_fields(reader, map)
}

fn final_base_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_bc8")?;
    io::integer_pairs(reader, "map_b90", false)?;
    raw_fields::fields(reader, Signed, "4c0 4c4 a78")?;
    reader.object(
        "envelope_bd0",
        map,
        ModelObjectRole::Declared(&["moEnvelopePublisher_c"]),
    )?;
    for index in 0..reader.unsigned_count("final_vector_bf0.count", 4)? {
        reader.i32(&format!("final_vector_bf0[{index}]"))?;
    }
    Ok(())
}
