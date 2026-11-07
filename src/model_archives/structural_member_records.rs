use super::display_records::declared;
use super::{
    ModelDecoder, ModelObjectRole, model_lists, structural_member_references as references,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

const ADVANCED_PROPERTIES: &[&str] = &["moAdvCusPropList_c"];

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "profile_820", &["moProfileRef_w"])?;
    declared(reader, map, "plane_828", &["moPlaneRef_w"])?;
    reader.i32s(&["raw_910", "raw_914"])?;
    references::curves(reader, map, "curves_890")?;
    points(reader, map)?;
    reader.object("object_918", map, ModelObjectRole::Any)?;
    reader.f64("raw_920")?;
    reader.text("string_928")?;
    references::point(reader, map, "point_950")?;
    declared(reader, map, "properties_990", &["moCustomPropertyList"])?;
    reader.i32s(&["raw_938", "raw_93c", "raw_940", "raw_944", "raw_948"])?;
    references::curves(reader, map, "curves_8b0")?;
    continuation(reader, map)
}

fn points(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("points_830.count", 10)? {
        reader.scope(&format!("points_830[{index}]"), |reader| {
            references::point(reader, map, "point")?;
            reader.i32s(&["raw_8f0", "raw_8d0"])
        })?;
    }
    Ok(())
}

fn continuation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.f64("raw_930")?;
    if reader.i32("raw_9c8")? > 0 {
        declared(reader, map, "contours_9c0", &["moWeldmentContours_c"])?;
    }
    model_lists::pointers(reader, map, "array_9f0", ModelObjectRole::Any)?;
    reader.text("string_960")?;
    reader.text("string_968")?;
    reader.i32("raw_958")?;
    reader.text("string_970")?;
    reader.i32s(&["raw_a14", "raw_a18", "raw_978"])?;
    reader.text("string_980")?;
    reader.i32("raw_988")?;
    declared(reader, map, "advanced_properties_9a8", ADVANCED_PROPERTIES)
}
