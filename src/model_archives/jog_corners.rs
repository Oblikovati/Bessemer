use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Retain stored jog options and coordinates before native load/store fixups.
/// Example: `jog_corners::jog(reader, map)?` for a complete jog feature.
pub(super) fn jog(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    prefix(reader, map)?;
    reader.scope("payload", |reader| {
        jog_leading(reader, map)?;
        jog_references(reader, map)?;
        jog_tail(reader, map)
    })
}

fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("bend_group", |reader| {
        super::base_flanges::bend_group(reader, map)
    })
}

fn jog_leading(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "profile_8d0", super::profile_features::CLASSES)?;
    reader.f64("raw_8e8")?;
    reader.i32("raw_958")?;
    reader.f64("raw_950")?;
    reader.i32("raw_8f4")?;
    boundary(reader, map, "boundary_920")?;
    reader.i32("raw_8f0")?;
    dimensions(reader, map, &["978", "980"])?;
    reader.i32("raw_95c")?;
    Ok(())
}

fn jog_references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.u8("reference_presence")? == 0 {
        return Ok(());
    }
    declared(
        reader,
        map,
        "point_970",
        super::geometric_references::POINT_REFERENCE_CLASSES,
    )?;
    declared(
        reader,
        map,
        "reference_960",
        super::model_references::WRAPPER_CLASSES,
    )
}

fn jog_tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    double_vector(reader, "point_900")?;
    model_lists::pointers(reader, map, "direct_list_880", ModelObjectRole::Any)?;
    reader.i32s(&["raw_8f8", "raw_988", "raw_8e4"])?;
    declared(
        reader,
        map,
        "component_9a0",
        super::component_references::FEATURE_CLASSES,
    )?;
    reader.i32("raw_930")?;
    double_vector(reader, "vector_938")?;
    if reader.version() >= 0x3afe {
        boundary(reader, map, "boundary_928")?;
    }
    Ok(())
}

/// Preserve the direct owned keeper and the distinct tagged array framing.
/// Example: `jog_corners::break_corner(reader, map)?` for an ordinary break corner.
pub(super) fn break_corner(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    prefix(reader, map)?;
    reader.scope("payload", |reader| {
        break_leading(reader, map)?;
        reader.scope("keeper_888", |reader| super::fillets::keeper(reader, map))?;
        break_middle(reader, map)?;
        break_tail(reader, map)
    })
}

fn break_leading(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["8b0", "8b8"] {
        declared(reader, map, &format!("list_{offset}"), &["suObList"])?;
    }
    reader.i32("raw_8a0")?;
    reader.f64("raw_8a8")?;
    dimensions(reader, map, &["890"])
}

fn break_middle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(
        reader,
        map,
        "edge_8c0",
        &["moCompEdge_c", "moCompSilhEdge_c"],
    )?;
    dimensions(reader, map, &["898"])?;
    reader.i32("raw_8c8")?;
    declared(reader, map, "list_8d0", &["suObList"])?;
    reader.i32("raw_8d8")?;
    reader.f64("raw_8e0")?;
    dimensions(reader, map, &["8f8"])
}

fn break_tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["raw_8e8", "raw_908"])?;
    reader.f64("raw_910")?;
    declared(reader, map, "array_900", &["suObArray"])?;
    if reader.version() >= 0x46cb {
        reader.i32("raw_918")?;
    }
    Ok(())
}

fn boundary(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    declared(
        reader,
        map,
        label,
        super::flat_patterns::BOUNDARY_REFERENCES,
    )
}

fn dimensions(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    offsets: &[&str],
) -> Result<()> {
    for offset in offsets {
        declared(reader, map, &format!("dimension_{offset}"), DIMENSIONS)?;
    }
    Ok(())
}
