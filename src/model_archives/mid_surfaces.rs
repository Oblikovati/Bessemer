use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Retain the full feature and raw numeric/list members; e.g. `read(reader, map)?`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| {
        reader.f64("raw_800")?;
        reader.u8("raw_808")?;
        declared(reader, map, "list_848", &["suObList"])?;
        declared(reader, map, "list_850", &["suObList"])
    })
}

/// Preserve the base's external sheet framing; e.g. `base_reference(reader, map)?`.
pub(super) fn base_reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference_geometry(reader, map)?;
    reader.scope("surface", |reader| {
        super::planar_surface_records::external_surface(reader, map)
    })
}

/// Retain the inline surface, direct keeper and face arrays; e.g. `reference(reader, map)?`.
pub(super) fn reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference_geometry(reader, map)?;
    reader.scope("surface", |reader| {
        super::planar_surface_records::surface(reader, map)
    })?;
    reader.scope("payload", |reader| reference_payload(reader, map))
}

fn reference_geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })
}

fn reference_payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.i32("body_presence_7b0")? == 1 {
        reader.scope("body_7b0", super::kernel_frames::read_body)?;
    }
    declared(reader, map, "data_808", &["moMidSurfData_c"])?;
    reader.scope("keeper_800", |reader| keeper(reader, map))?;
    reader.i32("raw_7c8")?;
    for offset in ["7e8", "7f8"] {
        reader.scope(&format!("faces_{offset}"), |reader| faces(reader, map))?;
    }
    Ok(())
}

fn faces(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..super::configuration_io::positive_count(reader, "declared_count", 2)? {
        declared(
            reader,
            map,
            &format!("items[{index}]"),
            &["moSingleFaceRef_w"],
        )?;
    }
    Ok(())
}

/// Retain inherited IDs and both direct arrays; e.g. `keeper(reader, map)?`.
fn keeper(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::fillets::keeper(reader, map)?;
    for offset in ["128", "148"] {
        model_lists::pointers(
            reader,
            map,
            &format!("array_{offset}"),
            ModelObjectRole::Any,
        )?;
    }
    Ok(())
}

/// Preserve nullable presence and the final word; e.g. `data(reader, map)?`.
pub(super) fn data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("array_08", |reader| {
        body_features::nullable_array(reader, map)
    })?;
    reader.i32("raw_10").map(|_| ())
}
