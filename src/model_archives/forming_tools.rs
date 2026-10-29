use super::{
    ModelDecoder, ModelObjectRole, body_features, component_references, configuration_io, fillets,
    kernel_frames, model_lists, profile_features,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Retain the complete ordinary forming-tool instance, including its inline part dependency.
/// Example: `forming_tools::read(reader, map)?` for `moFormToolInstance_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    prefix(reader, map)?;
    words_and_strings(reader, map)?;
    geometry(reader, map)?;
    configuration_io::strings(reader, "strings_9e0")?;
    reader.scope("keeper_9b0", |reader| fillets::keeper(reader, map))?;
    reader.object(
        "component_930",
        map,
        ModelObjectRole::Declared(component_references::FEATURE_CLASSES),
    )?;
    for label in ["bodies_8d0", "bodies_8f0"] {
        reader.scope(label, |reader| attributed_bodies(reader, map))?;
    }
    transform_map(reader, map)?;
    reader.i32s(&["raw_9c8", "raw_9cc", "raw_9d8"])
}

fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.required_object(
        "profile_8f8",
        map,
        ModelObjectRole::Declared(profile_features::CLASSES),
    )?;
    reader.i32("raw_940")?;
    reader.transform("transform_948")?;
    reader.object(
        "tool_908",
        map,
        ModelObjectRole::Declared(&["moFormTool_c"]),
    )?;
    reader.object(
        "temporary_list",
        map,
        ModelObjectRole::Declared(&["suObList"]),
    )
}

fn words_and_strings(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.text("string_950")?;
    reader.i32("raw_958")?;
    reader.object(
        "external_960",
        map,
        ModelObjectRole::Declared(&["moExtObject_c"]),
    )?;
    reader.text("string_968")?;
    reader.i32s(&[
        "raw_970", "raw_974", "raw_978", "raw_97c", "raw_6c8", "raw_980", "raw_984",
    ])
}

fn geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..3 {
        reader.f64(&format!("point_988[{index}]"))?;
    }
    reader.f64("raw_9a0")?;
    reader.i32("raw_9a8")?;
    reader.object(
        "profile_900",
        map,
        ModelObjectRole::Declared(profile_features::CLASSES),
    )?;
    reader.i32s(&["raw_9b8", "raw_9dc"])
}

fn attributed_bodies(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..configuration_io::positive_count(reader, "count", 3)? {
        reader.scope(&format!("items[{index}]"), |reader| {
            reader.scope("body", kernel_frames::read_body)?;
            model_lists::pointers(reader, map, "attributes", ModelObjectRole::Any)
        })?;
    }
    Ok(())
}

fn transform_map(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..configuration_io::positive_count(reader, "transforms_858.count", 36)? {
        reader.scope(&format!("transforms_858[{index}]"), |reader| {
            reader.object(
                "component",
                map,
                ModelObjectRole::Declared(component_references::FEATURE_CLASSES),
            )?;
            reader.transform("transform")
        })?;
    }
    Ok(())
}
