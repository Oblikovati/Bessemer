use super::display_records::declared;
use super::{ModelDecoder, sweep_data};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })?;
    // #54 / C-275: the backend selector returns zero, so the surface remains inline.
    reader.scope("surface", |reader| {
        super::planar_surface_records::surface(reader, map)
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let controls = reader.scope("data", |reader| sweep_data::read(reader, map))?;
    sweep_data::bytes(reader, &["7b4", "7b8", "7bc", "7c0", "7c4"])?;
    if controls.angular {
        for label in ["angle_bd8", "angle_be0"] {
            declared(reader, map, label, &["moDisplayAngularDim_c"])?;
        }
    }
    edge_sequence(reader, map)?;
    if controls.radial {
        declared(reader, map, "radius_be8", &["moDisplayRadialDim_c"])?;
    }
    Ok(())
}

fn edge_sequence(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // #54 / 11.233: the signed load-loop count is not ReadCount; negatives consume nothing.
    let count = reader.i32("edge_data_count")?;
    if count <= 0 {
        return Ok(());
    }
    for index in 0..reader.counted_entries("sweep-surface edge data", i64::from(count), 2)? {
        declared(
            reader,
            map,
            &format!("edge_data[{index}]"),
            &["moEdgeData_c"],
        )?;
    }
    Ok(())
}

pub(super) fn edge_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["endpoint_18", "endpoint_20", "endpoint_28"] {
        declared(reader, map, label, &["moEndPointRef_w", "moLoftSynchRef_c"])?;
    }
    Ok(())
}
