use super::{ModelDecoder, ModelObjectRole, component_references, dimensions};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord};
const VIEW_TAG: ModelObjectRole<'_> = ModelObjectRole::Declared(&["moDrViewTag_c"]);
const EXTERNAL: ModelObjectRole<'_> = ModelObjectRole::Declared(&["moGFR_c"]);
const DIMENSION: ModelObjectRole<'_> = ModelObjectRole::Declared(dimensions::CLASSES);

pub(super) const CLASSES: &[&str] = &[
    "moDimHandle_c",
    "moFeatureDimHandle_c",
    "moRefDimHandle_c",
    "moSkDimHandleRadial_c",
    "moSkDimHandleOffset_c",
    "moSkDimHandleValG2_c",
    "moSkDimHandleValG3_c",
];

fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<bool> {
    component_references::base(reader, map)?;
    optional_transform(reader, "120")?;
    for index in 0..4 {
        reader.object(&format!("drawing_view_tags[{index}]"), map, VIEW_TAG)?;
    }
    reader.u8("is_external_feature_dimension")?;
    let external = references(reader, map)?;
    reader.i32("reference_id")?;
    optional_transform(reader, "128")?;
    reader.i32("unknown_118")?;
    Ok(external)
}

fn references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<bool> {
    let external = reader.object_record("external_reference", map, EXTERNAL)?;
    let reference = reader.object_record("reference_dimension", map, DIMENSION)?;
    if !matches!(reference, ObjectRecord::Null) {
        reader.transform("transform_108")?;
        if reader.i32("has_string_110")? != 0 {
            reader.text("unknown_110")?;
        }
    }
    Ok(!matches!(external, ObjectRecord::Null))
}

fn optional_transform(reader: &mut ModelDecoder<'_>, offset: &str) -> Result<()> {
    if reader.u16(&format!("has_transform_{offset}"))? != 0 {
        reader.transform(&format!("transform_{offset}"))?;
    }
    Ok(())
}

pub(super) fn model(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if !base(reader, map)? {
        reader.required_object("dimension", map, DIMENSION)?;
    }
    Ok(())
}

pub(super) fn sketch(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if base(reader, map)? {
        reader.u16("external_sketch_dimension_type")?;
    } else {
        reader.required_object("sketch", map, ModelObjectRole::Declared(&["sgSketch"]))?;
    }
    reader.i32("sketch_dimension_id")?;
    Ok(())
}
