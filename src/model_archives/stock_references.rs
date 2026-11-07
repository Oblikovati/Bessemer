use super::{ModelDecoder, ModelObjectRole, model_lists, model_references, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the directly owned configuration name and signed identifier.
/// Example: `stock_references::configuration(reader)?`.
pub(super) fn configuration(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("name")?;
    reader.i32("id")?;
    Ok(())
}

/// Retain conditional kernel bodies and each separate direct attribute list.
/// Example: `stock_references::body(reader, map)?` for `moBodyRef_w`.
pub(super) fn body(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("wrapper", |reader| model_references::wrapper(reader, map))?;
    if reader.i32("raw_a8")? == 0 {
        return Ok(());
    }
    for index in 0..reader.signed_count("bodies.count", 3)? {
        reader.scope(
            &format!("bodies[{index}].body"),
            super::kernel_frames::read_body,
        )?;
        model_lists::pointers(
            reader,
            map,
            &format!("bodies[{index}].attributes"),
            ModelObjectRole::Any,
        )?;
    }
    reader.object(
        "external_object_e0",
        map,
        ModelObjectRole::Declared(&["moExtObject_c"]),
    )
}

/// Preserve component-body aliases, both compact transforms and the stored box.
/// Example: `stock_references::component(reader, map)?` for `moCompBody_c`.
pub(super) fn component(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("component_reference", |reader| {
        super::component_references::base(reader, map)
    })?;
    if reader.u8("alias_presence")? != 0 {
        reader.text("alias_08")?;
    }
    reader.i32("raw_f4")?;
    optional_transform(reader, "transform_f8")?;
    reader.u32("raw_158")?;
    reader.i32s(&["raw_15c", "raw_160", "raw_164"])?;
    sketch_arrays::doubles(reader, "box_100", 10)?;
    optional_transform(reader, "transform_168")
}

fn optional_transform(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    if reader.u8(&format!("{label}_presence"))? == 0 {
        return Ok(());
    }
    reader.transform(label)
}
