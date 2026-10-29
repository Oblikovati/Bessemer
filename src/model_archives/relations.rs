use super::{ModelDecoder, ModelObjectRole, component_references, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn relation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.text("unknown_10")?;
    reader.u16("unknown_18")?;
    reader.i32("unknown_48")?;
    sides(reader, map)?;
    reader.i32s(&["unknown_74", "relation_id"])?;
    reader.text("unknown_38")?;
    reader.i32s(&["unknown_88", "unknown_6c", "unknown_8c"])
}

fn sides(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["left", "right"] {
        reader.object(
            label,
            map,
            ModelObjectRole::Declared(&["moRelEquationSide_c"]),
        )?;
    }
    Ok(())
}

pub(super) fn equation_side(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    model_lists::pointers(reader, map, "elements", ModelObjectRole::Any)?;
    reader.text("unknown_50")?;
    reader.i32s(&["unknown_58", "is_left"])?;
    reader.object(
        "relation",
        map,
        ModelObjectRole::Declared(&["moRelation_c"]),
    )?;
    Ok(())
}

pub(super) fn element(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.text("unknown_08")?;
    reader.i32s(&["unknown_10", "unknown_14", "unknown_18", "unknown_1c"])
}

pub(super) fn dimension(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    model_lists::pointers(reader, map, "objects_30", ModelObjectRole::Any)?;
    reader.object(
        "component_reference",
        map,
        ModelObjectRole::Declared(&["moCompRef_c"]),
    )?;
    reader.object(
        "dimension_reference",
        map,
        ModelObjectRole::Declared(&["moDimRefWrapper_c"]),
    )?;
    element(reader)
}

pub(super) fn global_variable(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.object(
        "component_transform",
        map,
        ModelObjectRole::Declared(&["moCompXFormWrapper_c"]),
    )?;
    reader.object(
        "global_variable_reference",
        map,
        ModelObjectRole::Declared(&["moGlobalVarRefWrapper_c"]),
    )?;
    element(reader)
}

pub(super) fn component_variable(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    component_references::base(reader, map)?;
    reader.i32("unknown_e8")?;
    Ok(())
}
