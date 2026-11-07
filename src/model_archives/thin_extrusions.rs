use super::body_features::{self, BodyFeatureKind};
use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn extrusion(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    body_features::read(reader, map, BodyFeatureKind::Extrusion)?;
    required_specification(reader, map, "thin_specification_860")?;
    bend_records(reader)
}

pub(super) fn revolution(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    body_features::read(reader, map, BodyFeatureKind::Revolution)?;
    required_specification(reader, map, "thin_specification_8e8")
}

fn required_specification(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    reader.required_object(label, map, ModelObjectRole::Declared(&["moThinSpec_c"]))
}

pub(super) fn specification(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["dimension_08", "dimension_10"] {
        declared(reader, map, label, super::display_dimension::CLASSES)?;
    }
    reader.i32("raw_2c")?;
    reader.f64s(&["raw_30", "raw_38"])?;
    reader.i32s(&["raw_28", "raw_40"])?;
    declared(
        reader,
        map,
        "radial_dimension_48",
        &["moDisplayRadialDim_c"],
    )?;
    reader.i32s(&["raw_50", "raw_54"])
}

pub(super) fn bend_records(reader: &mut ModelDecoder<'_>) -> Result<()> {
    // #54 / C-219: 0xffff is 65,535 triples, never a packed-count escape.
    let count = reader.u16("bend_records_858.count")?;
    for index in 0..reader.counted_entries("thin bend records", i64::from(count), 12)? {
        for coordinate in 0..3 {
            reader.u32(&format!("bend_records_858[{index}][{coordinate}]"))?;
        }
    }
    Ok(())
}
