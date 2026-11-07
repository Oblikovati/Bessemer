use super::{ModelDecoder, ModelObjectRole, configuration_io as io};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("config_object_4c8", |reader| config_object(reader, map))?;
    reader.f64("raw_c48")?;
    reader.object(
        "crosshatch_c40",
        map,
        ModelObjectRole::Declared(&["gcXhatch_c"]),
    )?;
    reader.i32("raw_c58")?;
    body_states(reader, map)?;
    reader.object(
        "exploded_part_cb0",
        map,
        ModelObjectRole::Declared(&["moExplPrtId_c"]),
    )?;
    reader.object(
        "exploded_array_cb8",
        map,
        ModelObjectRole::Declared(&["suObArray"]),
    )?;
    reader.u32("raw_cc0")?;
    reader.i32("raw_cc4")?;
    for index in 0..io::positive_count(reader, "map_ce0.count", 8)? {
        reader.i32(&format!("map_ce0[{index}].key"))?;
        reader.u32(&format!("map_ce0[{index}].value"))?;
    }
    io::integer_pairs(reader, "map_d18", false)?;
    reader.i32s(&["raw_c5c", "raw_ccc"])?;
    if reader.manager_dialect() == Some(super::manager_dialect::ManagerDialect::Owned11000) {
        return Ok(());
    }
    reader.i32("raw_cc8")?;
    final_set(reader)
}

/// Preserve the directly owned record with its original version gates.
/// Example: `configuration_suffix::config_object(reader, map)?` at +4C8.
pub(super) fn config_object(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::configuration_manager::require_manager_scope(reader)?;
    reader.i32s(&["raw_0c", "raw_08"])?;
    reader.object(
        "referenced_b0",
        map,
        ModelObjectRole::Declared(&["moReferencedConfiguration_c"]),
    )?;
    reader.object(
        "visual_b8",
        map,
        ModelObjectRole::Declared(&["moVisualProperties_c"]),
    )?;
    for offset in ["10", "60", "100", "80", "68", "58", "78"] {
        let label = format!("list_{offset}");
        if matches!(offset, "58" | "78") {
            let count = reader.object_count(&format!("{label}.count"))?;
            io::empty(reader, &label, count)?;
        } else {
            io::list(reader, map, &label)?;
        }
    }
    optional_fields(reader, map)
}

fn optional_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["c8", "d0", "e0", "e8", "f0"] {
        let label = format!("presence_{offset}");
        let presence = reader.u16(&label)?;
        io::empty(reader, &label, usize::from(presence))?;
    }
    io::list(reader, map, "list_70")?;
    reader.i32s(&["raw_d8", "raw_f8"])?;
    if reader.u16("presence_98")? != 0 {
        io::list(reader, map, "list_98")?;
    }
    reader.object(
        "pattern_a0",
        map,
        ModelObjectRole::Declared(&["moPatternItemId_c"]),
    )?;
    if reader.version() > 0x2b8a {
        reader.object("list_88", map, ModelObjectRole::Declared(&["suObList"]))?;
    }
    if reader.version() > 0x4972 {
        reader.object("object_a8", map, ModelObjectRole::Any)?;
    }
    Ok(())
}

pub(super) fn body_states(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..io::positive_count(reader, "body_states_ca0.count", 8)? {
        reader.scope(&format!("body_states_ca0[{index}]"), |reader| {
            reader.object(
                "body",
                map,
                // #57: the persisted pointer also targets the two implemented body subclasses.
                ModelObjectRole::Declared(super::lightweight_part::BODY_CLASSES),
            )?;
            reader.i32("raw_state")?;
            reader.object(
                "visual",
                map,
                ModelObjectRole::Declared(&["moVisualProperties_c"]),
            )
        })?;
    }
    Ok(())
}

fn final_set(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let high = reader.u32("set_dc0.count_high")?;
    let low = reader.u32("set_dc0.count_low")?;
    let count = (u64::from(high) << 32) | u64::from(low);
    let count = i64::try_from(count)
        .map_err(|_| super::invalid(format!("part-set count {count}; expected <= i64::MAX")));
    let count = reader.checked(count)?;
    for index in 0..reader.counted_entries("set_dc0", count, 4)? {
        reader.i32(&format!("set_dc0[{index}]"))?;
    }
    Ok(())
}
