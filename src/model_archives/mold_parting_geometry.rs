use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the complete inline surface and direct settings; e.g. `read_geometry(reader, map)?`.
pub(super) fn read_geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })?;
    // #217: primary +AB0 selects 33 C0 C3; backend markers do not externalize sheets.
    reader.scope("surface", |reader| {
        super::planar_surface_records::surface(reader, map)
    })?;
    model_lists::pointers(reader, map, "edges_7b0", ModelObjectRole::Any)?;
    let method = reader.i32("method_7d0")?;
    reader.scope("settings_7d8", |reader| {
        read_settings(reader, map, method == 1)
    })
}

fn read_settings(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    child_allocated: bool,
) -> Result<()> {
    declared(reader, map, "owner_08", body_features::CLASSES)?;
    model_lists::pointers(reader, map, "edges_18", ModelObjectRole::Any)?;
    declared(reader, map, "dimension_10", &["moDisplayDistanceDim_c"])?;
    scalar_prefix(reader)?;
    component_fields(reader, map)?;
    sketch_arrays::doubles(reader, "raw_c8", 10)?;
    reader.i32s(&["raw_38", "raw_b0", "raw_3c"])?;
    declared(reader, map, "direction_a8", &["moDirectionSpec_c"])?;
    settings_suffix(reader, map, child_allocated)
}

fn settings_suffix(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    child_allocated: bool,
) -> Result<()> {
    reader.i32s(&["raw_124", "raw_144"])?;
    let selector = reader.i32("selector_170")?;
    if child_allocated && selector == 1 {
        reader.scope("child_178", |reader| {
            super::mold_parting_child::read_child(reader, map)
        })?;
    }
    Ok(())
}

fn scalar_prefix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for offset in ["88", "90", "98", "a0"] {
        reader.f64(&format!("raw_{offset}"))?;
    }
    reader.i32("raw_b4")?;
    reader.f64("raw_b8")?;
    reader.i32("raw_c0")?;
    reader.f64("raw_118")?;
    Ok(())
}

fn component_fields(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["78", "48"] {
        if reader.i32(&format!("component_{offset}_presence"))? != 0 {
            declared(
                reader,
                map,
                &format!("component_{offset}"),
                &["moCompFeature_c"],
            )?;
        }
    }
    let count = super::configuration_io::positive_count(reader, "integers_58.count", 4)?;
    sketch_arrays::integers(reader, "integers_58", count)
}
