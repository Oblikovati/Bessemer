use super::component_references::FEATURE_CLASSES;
use super::display_records::declared;
use super::model_references::WRAPPER_CLASSES;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

// #54 / C-194: boundary-reference descendants retain the documented common base.
const BOUNDARY_REFERENCES: &[&str] = &[
    "moCompBEntRef_c",
    "moCompFace_c",
    "moCompVertex_c",
    "moCompSilhVertex_c",
    "moCompEdge_c",
    "moCompSilhEdge_c",
    "moCompLoop_c",
];

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    reader.scope("bend_group", |reader| {
        super::base_flanges::bend_group(reader, map)
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    leading_fields(reader, map)?;
    declared(reader, map, "feature_9f8", FEATURE_CLASSES)?;
    declared(reader, map, "list_a38", &["suObList"])?;
    if reader.i32("body_presence_a08")? != 0 {
        reader.scope("body_a10", super::kernel_frames::read_raw_body)?;
    }
    continuation(reader, map)
}

fn leading_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    array(reader, map, "948")?;
    for label in ["8f8", "900", "908"] {
        declared(
            reader,
            map,
            &format!("reference_{label}"),
            BOUNDARY_REFERENCES,
        )?;
    }
    reader.i32("raw_9b0")?;
    if reader.version() > 0x372b {
        reader.i32("raw_9b4")?;
    }
    array(reader, map, "968")?;
    numeric_prefix(reader)
}

fn numeric_prefix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32s(&["raw_9bc", "raw_9c0", "raw_9f0", "raw_9c4"])?;
    reader.f64("raw_9c8")?;
    reader.i32("raw_9e0")?;
    reader.f64("raw_9d8")?;
    reader.i32s(&["raw_9e4", "raw_9d0", "raw_9ec", "raw_aa8"])
}

fn continuation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_988")?;
    array(reader, map, "990")?;
    if reader.version() > 0x3364 {
        declared(reader, map, "face_920", &["moCompFace_c"])?;
    }
    if reader.version() > 0x3761 && reader.i32("raw_928")? != 0 {
        reader.text("string_930")?;
    }
    declared(reader, map, "reference_910", WRAPPER_CLASSES)?;
    reader.scope("box_918", super::flat_pattern_boxes::read)?;
    reader.i32s(&["raw_a28", "raw_a2c", "raw_a30", "raw_a34"])?;
    if reader.sheet_metal_extended() {
        reader.i32("raw_a80")?;
    }
    pairs(reader)
}

fn array(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    model_lists::pointers(reader, map, &format!("array_{label}"), ModelObjectRole::Any)
}

fn pairs(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("pairs_ac0.count", 8)? {
        reader.scope(&format!("pairs_ac0[{index}]"), |reader| {
            reader.i32s(&["first", "second"])
        })?;
    }
    Ok(())
}
