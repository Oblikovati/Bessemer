use super::component_references::FEATURE_CLASSES;
use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

const PROPERTY_CLASSES: &[&str] = &["moFoldUpDownSketchXformPropertyData_c"];

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
        "moFoldUpDownSketchFolder_c" => folder(reader, map),
        "moFoldUpDownSketchXformData_c" => sketch(reader, map),
        "moFoldUpDownFeatXformData_c" => feature(reader, map),
        "moFoldUpDownSketchXformPropertyData_c" => property(reader),
        _ => return None,
    };
    Some(result)
}

fn folder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::folder(reader, map)?;
    declared(
        reader,
        map,
        "sketch_transform_568",
        &["moFoldUpDownSketchXformData_c"],
    )?;
    declared(
        reader,
        map,
        "feature_transform_570",
        &["moFoldUpDownFeatXformData_c"],
    )
}

fn sketch(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    direct_list(reader, map, "58")?;
    direct_list(reader, map, "a0")?;
    transforms(reader, "e8")?;
    properties(reader, map)?;
    for index in 0..reader.count_with_minimum("bytes_108.count", 1)? {
        reader.u8(&format!("bytes_108[{index}]"))?;
    }
    direct_list(reader, map, "10")
}

fn feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    direct_list(reader, map, "58")?;
    transforms(reader, "a0")?;
    direct_list(reader, map, "10")
}

fn transforms(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    let count = reader.signed_count(&format!("transforms_{label}.count"), 34)?;
    for index in 0..count {
        reader.transform(&format!("transforms_{label}[{index}]"))?;
    }
    Ok(())
}

fn properties(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("properties_158.count", 4)? {
        reader.scope(&format!("properties_158[{index}]"), |reader| {
            declared(reader, map, "feature", FEATURE_CLASSES)?;
            declared(reader, map, "property", PROPERTY_CLASSES)
        })?;
    }
    Ok(())
}

fn direct_list(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    model_lists::pointers(reader, map, &format!("list_{label}"), ModelObjectRole::Any)
}

fn property(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_08", "raw_0c", "raw_10"])?;
    reader.u32("raw_14")?;
    reader.text("string_18")?;
    reader.text("string_20")?;
    Ok(())
}
