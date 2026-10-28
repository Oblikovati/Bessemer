use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })?;
    // #54 / C-271: the native +AB0 selector returns zero; the surface base is inline.
    reader.scope("surface", |reader| {
        super::planar_surface_records::surface(reader, map)
    })?;
    model_lists::pointers(reader, map, "array_7b8", ModelObjectRole::Any)?;
    reader.scope("direct_data_7b0", |reader| data(reader, map))
}

pub(super) fn data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.required_object(
        "owner",
        map,
        ModelObjectRole::Declared(&["moFeature_c", "moRuledSrfFromEdge_c"]),
    )?;
    model_lists::pointers(reader, map, "array_30", ModelObjectRole::Any)?;
    reader.scope("words_f0", super::polyline_records::word_array)?;
    model_lists::pointers(reader, map, "array_50", ModelObjectRole::Any)?;
    declared(reader, map, "distance_18", &["moDisplayDistanceDim_c"])?;
    declared(reader, map, "angle_20", &["moDisplayAngularDim_c"])?;
    reader.f64("raw_90")?;
    reader.i32s(&["raw_98", "raw_9c", "raw_a0", "raw_a8"])?;
    declared(reader, map, "direction_e8", &["moDirectionSpec_c"])?;
    suffix(reader)
}

fn suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.f64("raw_b0")?;
    reader.i32("raw_b8")?;
    reader.f64s(&["raw_vector_c0[0]", "raw_vector_c0[1]", "raw_vector_c0[2]"])?;
    reader.i32("raw_d8")?;
    Ok(())
}
