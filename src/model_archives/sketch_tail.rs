use super::{
    ModelDecoder, ModelObjectRole, model_lists, sketch_collections, sketch_entities,
    sketch_managers,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, line_style, vectors::double_vector};

const CREATION_ORDER: &[&str] = &[
    "08", "0c", "10", "14", "18", "1c", "20", "24", "28", "2c", "30", "34", "38", "40", "3c",
];

// #46: the documented indicator extends the declared sketch-user-object role.
// 11.164 / C-192 and 11.209 / C-237 additionally prove the controls' ancestry.
const USER_OBJECT_CLASSES: &[&str] = &[
    "moSketchToolsUserObject_c",
    "moExtensionLineIndicator_c",
    "moSplineTangencyControl_c",
    "moSplineControlPolygonControl_c",
    "moSplineCurvatureControl_c",
];

pub(super) fn state(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.u16("word_12c")?;
    reader.u16("word_128")?;
    reader.i32("word_130")?;
    for offset in CREATION_ORDER {
        reader.i32(&format!("creation_ids.{offset}"))?;
    }
    for offset in ["37c", "188", "18c", "190"] {
        reader.u16(&format!("word_{offset}"))?;
    }
    Ok(())
}

fn line_properties(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.text("style_name")?;
    reader.scope("width", line_style::short_width)?;
    reader.i32s(&["color", "visibility"])
}

pub(super) fn visual(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("line_properties", line_properties)?;
    reader.i32("word_568")?;
    double_vector(reader, "point_570")?;
    reader.u32("word_56c")?;
    reader.i32("word_578")?;
    for index in 0..reader.signed_count("user_objects.count", 2)? {
        reader.object(
            &format!("user_objects[{index}]"),
            map,
            ModelObjectRole::Declared(USER_OBJECT_CLASSES),
        )?;
    }
    Ok(())
}

pub(super) fn extension_indicator(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("sketch", map, ModelObjectRole::Declared(&["sgSketch"]))?;
    reader.u16("word_18")?;
    Ok(())
}

fn local_manager(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("objects", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })?;
    for index in 0..reader.count_with_minimum("equation_values.count", 9)? {
        reader.scope(&format!("equation_values[{index}]"), |reader| {
            reader.text("name")?;
            reader.f64("value")?;
            Ok(())
        })?;
    }
    Ok(())
}

pub(super) fn local_relations(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("manager", |reader| local_manager(reader, map))?;
    for index in 0..reader.signed_count("entities.count", 2)? {
        reader.scope(&format!("entities[{index}]"), |reader| {
            sketch_entities::entity(reader, map)
        })?;
    }
    Ok(())
}

pub(super) fn coincidences(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.entries(4, "sketch coincidence groups")?;
    reader.scope("coincidences", |reader| {
        for index in 0..4 {
            sketch_managers::integer_pairs(reader, &format!("groups[{index}]"))?;
        }
        Ok(())
    })?;
    reader.object("regions", map, ModelObjectRole::Declared(&["suObList"]))
}

pub(super) fn silhouette(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    let marker = reader.u16("marker")?;
    if marker != 0 {
        return Ok(());
    }
    reader.checked(Err(crate::BessemerError::new(
        "model archive", "sketch silhouette", crate::FailureClass::Unsupported,
        format!("silhouette marker {marker}; expected nonzero until its complete zero-marker body is documented"),
    )))
}

pub(super) fn silhouettes(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    if reader.version() <= 0x2f50 {
        return Ok(());
    }
    for index in 0..sketch_collections::word_count(reader, "items.count")? {
        reader.scope(&format!("items[{index}]"), silhouette)?;
    }
    Ok(())
}

pub(super) fn final_ordinary(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    // The ordinary helper's constructor fixes profile-feature mode to false (11.75).
    reader.i32("word_134")?;
    Ok(())
}
