use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.i32s(&["raw_af8", "raw_d4c"])?;
    if reader.i32("print_settings_presence")? != 0 {
        reader.scope("print_settings", |reader| print_settings(reader, map))?;
    }
    reader.object("list_a48", map, ModelObjectRole::Declared(&["suObList"]))?;
    reader.i32("raw_eb0")?;
    if reader.version() > 0x27d3 {
        reader.text("raw_758")?;
        reader.text("raw_760")?;
    }
    formatting_tail(reader, map)
}

fn formatting_tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x27de {
        reader.object(
            "thread_manager",
            map,
            ModelObjectRole::Declared(&["moCThreadRefMgr_c"]),
        )?;
    }
    if reader.version() > 0x2b65 {
        reader.object(
            "bend_format",
            map,
            ModelObjectRole::Declared(&["moBendNoteFormat_c"]),
        )?;
    }
    final_formatting(reader)
}

fn final_formatting(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x3b05 {
        pair_map(reader)?;
    }
    if reader.version() > 0x3ef1 && reader.version() < 0x3ef4 {
        reader.u16("geometric_tolerance_decimal_separator")?;
    }
    Ok(())
}

fn print_settings(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.f64s(&["raw_08", "raw_10", "raw_18", "raw_20"])?;
    reader.i32("raw_28")?;
    reader.object(
        "component_reference",
        map,
        ModelObjectRole::Declared(&["moCompRef_c"]),
    )
}

pub(super) fn thread_manager(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for index in 0..reader.signed_count("group_count", 8)? {
        reader.scope(&format!("groups[{index}]"), |reader| {
            reader.i32("key")?;
            for index in 0..reader.signed_count("holder_count", 2)? {
                reader.object(
                    &format!("holders[{index}]"),
                    map,
                    ModelObjectRole::Declared(&["moCThreadRefEntHolder_c"]),
                )?;
            }
            Ok(())
        })?;
    }
    if reader.version() > 0x3755 {
        reader.i32("raw_1a0")?;
    }
    Ok(())
}

pub(super) fn bend_format(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.text("format")?;
    reader.object(
        "substitutions",
        map,
        ModelObjectRole::Declared(&["su_CMapStringToString"]),
    )
}

fn pair_map(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("string_pair_count", 6)? {
        reader.scope(&format!("string_pair_map[{index}]"), pair_entry)?;
    }
    Ok(())
}

fn pair_entry(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.text("first")?;
    reader.text("second")?;
    for index in 0..reader.signed_count("value_count", 8)? {
        reader.u32(&format!("values[{index}][0]"))?;
        reader.u32(&format!("values[{index}][1]"))?;
    }
    Ok(())
}
