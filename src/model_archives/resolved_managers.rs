use super::{ModelDecoder, ModelObjectRole, invalid, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn atom_manager(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.i32("unknown_260")?;
    for index in 0..reader.signed_count("atom_count", 2)? {
        reader.required_object(
            &format!("atoms[{index}]"),
            map,
            ModelObjectRole::Declared(&["moAtom_c"]),
        )?;
    }
    reader.u32("unknown_a8")?;
    Ok(())
}

pub(super) fn ghost_manager(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.i32("unknown_80")?;
    let packed = reader.u32("packed_count")?;
    let count = ghost_count(reader, packed)?;
    for index in 0..count {
        reader.scope(&format!("objects[{index}]"), |reader| {
            ghost_body(reader, map)
        })?;
    }
    Ok(())
}

fn ghost_count(reader: &mut ModelDecoder<'_>, packed: u32) -> Result<usize> {
    let count = (packed & 0x0fffffff) as usize;
    let available = reader.remaining() / 28;
    let shape = if count <= available {
        Ok(())
    } else {
        Err(invalid(format!(
            "ghost record count {count} from {packed:#010x}; expected <= {available} remaining records"
        )))
    };
    reader.checked(shape)?;
    reader.entries(count, "ghost records")?;
    Ok(count)
}

fn ghost_body(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["unknown_08", "unknown_0c", "unknown_10", "unknown_c0"])?;
    for index in 0..reader.signed_count("integer_count", 4)? {
        reader.i32(&format!("unknown_a0[{index}]"))?;
    }
    if reader.i32("has_geometric_data")? != 0 {
        reader.object(
            "geometric_data",
            map,
            ModelObjectRole::Declared(&["moGhostGeometricData_c"]),
        )?;
    }
    for index in 0..reader.signed_count("point_count", 24)? {
        for coordinate in 0..3 {
            reader.f64(&format!("points[{index}][{coordinate}]"))?;
        }
    }
    Ok(())
}

pub(super) fn relation_manager(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_model()?;
    model_lists::pointers(reader, map, "relations", ModelObjectRole::Any)?;
    cache(reader)?;
    reader.text("unknown_170")?;
    reader.i32("unknown_178")?;
    reader.u32("time")?;
    reader.i32s(&["unknown_150", "unknown_f4"])?;
    model_lists::pointers(reader, map, "objects_a0", ModelObjectRole::Any)?;
    relation_parameters(reader, map)
}

fn cache(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.count_with_minimum("cache_count", 9)? {
        reader.scope(&format!("cached_dimension_values[{index}]"), |reader| {
            reader.text("name")?;
            reader.f64("value")
        })?;
    }
    Ok(())
}

fn relation_parameters(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() <= 0x336a {
        return Ok(());
    }
    reader.i32("unknown_1b0")?;
    for index in 0..reader.signed_count("feature_parameter_count", 2)? {
        reader.object(
            &format!("feature_parameters[{index}]"),
            map,
            ModelObjectRole::Declared(&["moFeatureParameter_c"]),
        )?;
    }
    Ok(())
}
