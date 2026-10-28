use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve the complete bend group and its directly owned edge-flange data.
/// Example: `edge_flange_records::read(reader, map)?` for an edge-flange feature.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("bend_group", |reader| {
        super::base_flanges::bend_group(reader, map)
    })?;
    reader.scope("data", |reader| super::edge_flange_data::read(reader, map))?;
    model_lists::pointers(reader, map, "array_af8", ModelObjectRole::Any)?;
    if reader.version() >= 0x372f {
        reader.i32("raw_b38")?;
    }
    Ok(())
}

/// Decode the fixed geometry body shared by base and linear edge geometry.
/// Example: `edge_flange_records::geometry(reader)?` for either native binding.
pub(super) fn geometry(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_08")?;
    for offset in ["10", "28", "40", "58", "70"] {
        let prefix = if matches!(offset, "10" | "28") {
            "vector"
        } else {
            "point"
        };
        double_vector(reader, &format!("{prefix}_{offset}"))?;
    }
    Ok(())
}

/// Retain each per-edge record and its nonzero late dimension flag.
/// Example: `edge_flange_records::per_edge(reader, map)?` for a per-edge record.
pub(super) fn per_edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "profile_08", super::profile_features::CLASSES)?;
    declared(
        reader,
        map,
        "edge_10",
        &["moCompEdge_c", "moCompSilhEdge_c"],
    )?;
    reader.i32("raw_20")?;
    declared(
        reader,
        map,
        "geometry_70",
        &["moEdgeGeom_c", "moLinearEdgeGeom_c"],
    )?;
    per_edge_dimension(reader, map)
}

fn per_edge_dimension(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() >= 0x42cc && reader.i32("raw_c8")? != 0 {
        declared(reader, map, "dimension_d0", &["moDisplayDistanceDim_c"])?;
    }
    Ok(())
}
