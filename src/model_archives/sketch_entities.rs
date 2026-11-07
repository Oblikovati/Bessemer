use super::{ModelDecoder, ModelObjectRole, sketch_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, line_style, visibility};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SketchEntityFlags {
    pub flags_54: u16,
    pub flags_56: u16,
}

pub(super) fn entity(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<SketchEntityFlags> {
    reader.require_ordinary_model()?;
    reader.i32s(&["permanent_mark", "unknown_0c"])?;
    reader.scope("visual_properties", visibility::read)?;
    reader.i32("reference_count")?;
    let minimal = reader.u16("minimal")?;
    super::part_objects::absent_payload(reader, "minimal-owner sketch entity", minimal.into())?;
    reader.u16("color")?;
    reader.u16("constraint_status")?;
    reader.u16("line_type")?;
    reader.scope("line_width", line_style::short_width)?;
    reader.object(
        "external_entity",
        map,
        ModelObjectRole::Declared(&["sgExtEnt_c"]),
    )?;
    entity_suffix(reader, map)
}

fn entity_suffix(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<SketchEntityFlags> {
    let flags_54 = reader.u16("flags_54")?;
    let flags_56 = reader.u16("flags_56")?;
    reader.i32("unknown_50")?;
    reader.object("group", map, ModelObjectRole::Declared(&["moGroup"]))?;
    if reader.u8("has_description")? != 0 {
        reader.text("description")?;
    }
    reader.f64("unknown_68")?;
    reader.u32("unknown_74")?;
    reader.u32("unknown_78")?;
    Ok(SketchEntityFlags { flags_54, flags_56 })
}

pub(super) fn slot(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("entity", |reader| entity(reader, map))?;
    reader.i32s(&["type", "length_type"])?;
    for index in 0..6 {
        reader.object(
            &format!("handles[{index}]"),
            map,
            ModelObjectRole::Declared(sketch_references::HANDLES),
        )?;
    }
    Ok(())
}
