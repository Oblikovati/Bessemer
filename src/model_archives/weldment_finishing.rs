use super::display_records::declared;
use super::{ModelDecoder, body_features, cosmetic_weld_tree, model_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Retain the complete extrusion serializer before the gusset continuation.
/// Example: `weldment_finishing::gusset(reader, map)?` for the concrete gusset.
pub(super) fn gusset(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("extrusion", |reader| {
        body_features::read(reader, map, body_features::BodyFeatureKind::Extrusion)
    })?;
    reader.scope("payload", |reader| {
        gusset_prefix(reader, map)?;
        gusset_suffix(reader, map)
    })
}

fn gusset_prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    typed_members(reader, map, "list", "suObList", &["858", "860"])?;
    reader.i32s(&["raw_868", "raw_86c", "raw_870"])?;
    distance_members(reader, map, &["8d8"])?;
    reader.f64("raw_878")?;
    reader.i32("raw_880")?;
    reader.u32("packed_890_88c")?;
    distance_members(reader, map, &["898", "8a0", "8a8"])?;
    declared(reader, map, "dimension_8b0", &["moDisplayAngularDim_c"])
}

fn gusset_suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // #54 / C-391: preserve both packed words' upper bits, not just runtime bits 0/1.
    reader.u32("packed_884_888")?;
    distance_members(reader, map, &["8b8"])?;
    reader.i32("raw_8e0")?;
    distance_members(reader, map, &["8c0", "8c8"])?;
    declared(reader, map, "dimension_8d0", &["moDisplayAngularDim_c"])?;
    reader.i32s(&["raw_8e4", "raw_8f8", "raw_8fc", "raw_900"])?;
    declared(
        reader,
        map,
        "reference_908",
        model_references::WRAPPER_CLASSES,
    )
}

fn distance_members(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    offsets: &[&str],
) -> Result<()> {
    typed_members(reader, map, "dimension", "moDisplayDistanceDim_c", offsets)
}

fn typed_members(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    prefix: &str,
    class: &str,
    offsets: &[&str],
) -> Result<()> {
    for offset in offsets {
        declared(reader, map, &format!("{prefix}_{offset}"), &[class])?;
    }
    Ok(())
}

/// Decode the ordinary current-runtime weld feature with helper +834 clear.
/// Example: `weldment_finishing::bead(reader, map)?` in the declared ordinary dialect.
pub(super) fn bead(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| bead_payload(reader, map))
}

fn bead_payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    bead_prefix(reader, map)?;
    typed_members(reader, map, "side", "weldBeadSideData_c", &["828", "830"])?;
    declared(reader, map, "list_838", &["suObList"])?;
    distance_members(reader, map, &["800", "808", "810", "818", "820"])?;
    reader.i32s(&["raw_948", "raw_94c"])
}

fn bead_prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.text("string_858")?;
    declared(
        reader,
        map,
        "display_850",
        cosmetic_weld_tree::DISPLAY_ITEMS,
    )?;
    // #54 / C-391: ordinary loading serializes the cabinet; helper overrides are another dialect.
    declared(reader, map, "cabinet_860", &["moDetailCabinet_c"])?;
    reader.transform("transform_868")?;
    reader.u32("packed_840_844")?;
    Ok(())
}

/// Preserve the independent side-data body and shared tagged list identities.
/// Example: `weldment_finishing::side(reader, map)?` for the exact side-data class.
pub(super) fn side(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    typed_members(reader, map, "list", "suObList", &["30", "38", "40"])?;
    reader.i32s(&["raw_50", "raw_74"])?;
    reader.f64("raw_78")?;
    Ok(())
}
