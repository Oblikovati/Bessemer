use super::display_records::declared;
use super::{ModelDecoder, reference_planes};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    let result = match name {
        "moConnectionPoint_c" => connection(reader, map),
        "moRoutePoint_c" => route(reader, map),
        "moConfigurableParam_c" => configurable(reader),
        "ThreeDRadialOrDiameterDim_c" => radial_dimension(reader, map),
        _ => return None,
    };
    Some(result)
}

fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reference_planes::reference_geometry(reader, map)?;
    declared(
        reader,
        map,
        "point_reference_6f0",
        &["moPointRef_w", "moEndPointRef_w", "moLoftSynchRef_c"],
    )
}

fn connection(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    reader.scope("payload", |reader| {
        super::connection_payload::read(reader, map)
    })
}

fn route(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    declared(
        reader,
        map,
        "line_reference_710",
        super::geometric_references::LINE_REFERENCE_CLASSES,
    )?;
    double_vector(reader, "vector_6f8")
}

pub(super) fn configurable(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("raw_10")?;
    reader.text("raw_18")
}

fn radial_dimension(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    super::dimensions::geometric(reader, map)?;
    reader.i32("raw_f8")?;
    Ok(())
}
