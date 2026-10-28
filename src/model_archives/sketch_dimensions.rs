use super::{ModelDecoder, ModelObjectRole, dimensions, sketch_collections, sketch_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

// #54 / C-385: primary slots and native ancestry bind these exact derived bodies.
pub(super) const G1_CLASSES: &[&str] = &["sgValG1Dim", "sgCircleDim", "sgSplineTanMagnitudeDim"];
pub(super) const G2_CLASSES: &[&str] = &[
    "sgValG2Dim",
    "sgLLDist",
    "sgPntPntDist",
    "sgPntLineDist",
    "sgPntPntHorDist",
    "sgPntPntVertDist",
    "sgAnglDim",
    "sgPoint3DPlaneDist",
    "sg3DPlane3DPlaneDist",
];
pub(super) const G3_CLASSES: &[&str] = &["sgValG3Dim", "sgArcLenDist"];
pub(super) const OFFSET_CLASSES: &[&str] = &["sgOffsetDim", "sgSkOffsetDim"];

pub(super) fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    dimensions::base(reader, map)?;
    reader.i32("dcm_type")?;
    reader.u16("constraint_status")?;
    reader.u16("invalidated")?;
    reader.u16("dangling")?;
    Ok(())
}

fn valued(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    if reader.u16("has_plane_transform")? != 0 {
        // #86 / C-395: this call site selects the fixed archive operator, not restore().
        super::fixed_transform_records::read(reader, "plane_transform.words", "valued dimension")?;
    }
    Ok(())
}

pub(super) fn valued_one(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    valued(reader, map)?;
    handles(reader, map, &["entity0", "entity1"])?;
    reader.i32s(&["arc_dimension_type0", "arc_dimension_type1"])?;
    double_vector(reader, "vector_188")
}

pub(super) fn valued_two(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    valued(reader, map)?;
    handles(reader, map, &["entities[0]", "entities[1]"])?;
    for offset in ["180", "184", "1b8", "1bc"] {
        reader.u16(&format!("word_{offset}"))?;
    }
    reader.object(
        "plane_handle",
        map,
        ModelObjectRole::Declared(&["sg3DPlaneHandle"]),
    )?;
    double_vector(reader, "vector_1c8")?;
    if reader.version() > 0x2f79 && reader.version() < 0x2fa0 {
        reader.u16("legacy_word")?;
    }
    Ok(())
}

pub(super) fn valued_three(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    valued(reader, map)?;
    handles(reader, map, &["entities[0]", "entities[1]", "entities[2]"])?;
    reader.u16("quadrant")?;
    Ok(())
}

pub(super) fn offset(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    valued(reader, map)?;
    reader.u16("offset_index")?;
    reader.u16("offset_ok")?;
    Ok(())
}

pub(super) fn entity_help(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.f64s(&["value_08", "value_10"])?;
    reader.u16("word_18")?;
    Ok(())
}

pub(super) fn handles(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    labels: &[&str],
) -> Result<()> {
    for label in labels {
        reader.object(
            label,
            map,
            ModelObjectRole::Declared(sketch_references::HANDLES),
        )?;
    }
    Ok(())
}

pub(super) fn valued_collections(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    let groups = [
        (("valued_g1_count", "valued_g1"), G1_CLASSES),
        (("valued_g2_count", "valued_g2"), G2_CLASSES),
        (("valued_g3_count", "valued_g3"), G3_CLASSES),
        (("offset_count", "offsets"), OFFSET_CLASSES),
    ];
    for (labels, classes) in groups {
        sketch_collections::tagged(reader, map, labels, ModelObjectRole::Declared(classes))?;
    }
    Ok(())
}
