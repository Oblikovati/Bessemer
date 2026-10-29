use super::{ModelDecoder, ModelObjectRole, invalid, part_objects::absent_payload};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn array(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    if reader.i32("has_array")? == 0 {
        return Ok(());
    }
    let size = reader.u32("element_size")?;
    if size == 0 {
        return Ok(());
    }
    array_size(reader, size)?;
    reader.u32("capacity")?;
    reader.u32("growth")?;
    array_payload(reader, size as usize)
}

fn array_size(reader: &mut ModelDecoder<'_>, size: u32) -> Result<()> {
    let shape = if size.is_multiple_of(24) {
        Ok(())
    } else {
        Err(invalid(format!(
            "dynamic-array element size {size}; expected a multiple of 24"
        )))
    };
    reader.checked(shape)
}

fn array_payload(reader: &mut ModelDecoder<'_>, size: usize) -> Result<()> {
    let count = reader.u32("count")? as usize;
    let available = reader.remaining() / size;
    let shape = if count <= available {
        Ok(())
    } else {
        Err(invalid(format!(
            "dynamic-array count {count} size {size}; expected <= {available} remaining elements"
        )))
    };
    reader.checked(shape)?;
    reader.entries(count, "dynamic-array elements")?;
    for index in 0..count {
        reader.fixed_octets(&format!("elements[{index}]"), size)?;
    }
    Ok(())
}

pub(super) fn flags(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.i32("unknown_f14")?;
    let shrink = reader.i32("has_shrink_wrap_data")?;
    if reader.version() > 0x2f72 {
        reader.i32("is_defeatured_state")?;
    }
    absent_payload(reader, "shrink-wrap data", shrink)?;
    reader.i32("has_cthread")?;
    Ok(())
}

pub(super) fn collections(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for index in 0..reader.object_count("object_count")? {
        reader.object(&format!("objects[{index}]"), map, ModelObjectRole::Any)?;
    }
    reader.i32("unknown_d50")?;
    notes(reader, map)
}

fn notes(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("note_map_count", 5)? {
        reader.scope(&format!("display_flag_notes[{index}]"), |reader| {
            reader.text("name")?;
            for index in 0..reader.signed_count("note_count", 2)? {
                reader.object(
                    &format!("notes[{index}]"),
                    map,
                    ModelObjectRole::Declared(&["moDisplayFlagNoteData_c"]),
                )?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

pub(super) fn tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.fixed_octets("unknown_77c", 16)?;
    reader.i32("unknown_1180")?;
    reader.fixed_octets("unknown_79c", 16)?;
    reader.i32s(&["unknown_1184", "unknown_1188", "unknown_118c"])?;
    let imports = reader.i32("has_component_import_data")?;
    absent_payload(reader, "component-import data", imports)?;
    if reader.version() > 0x27cc {
        reader.i32("is_mixed_mode_model")?;
    }
    if reader.version() > 0x2b6c {
        reader.u32("unknown_12f8")?;
    }
    late_tail(reader, map)
}

fn late_tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x46bb {
        reader.i32("unknown_1300")?;
    }
    if reader.version() > 0x46c5 {
        for index in 0..reader.signed_count("unknown_map_1308.count", 8)? {
            reader.u32(&format!("unknown_map_1308[{index}].unknown_1c"))?;
            reader.i32(&format!("unknown_map_1308[{index}].unknown_20"))?;
        }
    }
    if reader.version() > 0x2bce {
        reader.object(
            "mbd_3d_pdf_publish_cache",
            map,
            ModelObjectRole::Declared(&["moMBD3DPDFPublishCacheData_c"]),
        )?;
    }
    Ok(())
}
