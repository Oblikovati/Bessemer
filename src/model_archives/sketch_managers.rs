use super::{ModelDecoder, ModelObjectRole, sketch_arrays, sketch_dimensions};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn blocks(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for index in 0..reader.signed_count("blocks.count", 6)? {
        reader.scope(&format!("blocks[{index}]"), |reader| {
            reader.i32("key")?;
            reader.object("block", map, ModelObjectRole::Declared(&["sgBlock"]))
        })?;
    }
    block_values(reader)?;
    reader.scope("handle_data", |reader| handle_data(reader, map))
}

fn block_values(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("values.count", 12)? {
        reader.scope(&format!("values[{index}]"), |reader| {
            reader.i32("block_id")?;
            reader.u32("key")?;
            reader.u32("value")?;
            Ok(())
        })?;
    }
    Ok(())
}

pub(super) fn handle_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for index in 0..reader.signed_count("entries.count", 4)? {
        reader.scope(&format!("entries[{index}]"), |reader| {
            sketch_dimensions::handles(reader, map, &["handle"])?;
            reader.object(
                "data",
                map,
                ModelObjectRole::Declared(&["sgEntHandleData_c"]),
            )
        })?;
    }
    Ok(())
}

pub(super) fn loops(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    for index in 0..reader.signed_count("slots.count", 4)? {
        reader.scope(&format!("slots[{index}]"), |reader| loop_slot(reader, map))?;
    }
    Ok(())
}

fn loop_slot(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.i32("marker")? == 0 {
        return Ok(());
    }
    reader.i32("id")?;
    integer_pairs(reader, "geometry")?;
    sketch_arrays::integers(reader, "words_110_11c", 4)?;
    reader.i32("word_130")?;
    reader.f64("value_138")?;
    reader.object(
        "dimension",
        map,
        ModelObjectRole::Declared(super::display_dimension::CLASSES),
    )
}

pub(super) fn integer_pairs(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 8)? {
        reader.i32(&format!("{label}[{index}][0]"))?;
        reader.i32(&format!("{label}[{index}][1]"))?;
    }
    Ok(())
}

pub(super) fn slots(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.i32("word_10")?;
    for index in 0..reader.signed_count("slots.count", 6)? {
        reader.scope(&format!("slots[{index}]"), |reader| {
            reader.i32("index")?;
            reader.object("slot", map, ModelObjectRole::Declared(&["sgSlot_c"]))
        })?;
    }
    Ok(())
}
