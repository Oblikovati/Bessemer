use super::component_references::FEATURE_CLASSES;
use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve the complete 3D-bend group and its unnormalized direct list.
/// Example: `sheet_folds::bend(reader, map)?` for `moSM3dBend_c`.
pub(super) fn bend(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("bend_group", |reader| {
        super::base_flanges::bend_group(reader, map)
    })?;
    reader.scope("payload", |reader| bend_payload(reader, map))
}

fn bend_payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "profile_8a0", super::profile_features::CLASSES)?;
    reader.f64("raw_898")?;
    reader.i32("raw_884")?;
    double_vector(reader, "point_8a8")?;
    model_lists::pointers(reader, map, "list_908", ModelObjectRole::Any)?;
    boundary(reader, map, "boundary_8d0")?;
    declared(reader, map, "line_8d8", &["sgLineHandle"])?;
    reader.i32s(&["raw_8e0", "raw_8c0", "raw_970"])?;
    declared(reader, map, "component_8c8", FEATURE_CLASSES)
}

/// Retain the shared fold body without adding a bend-group layer.
/// Example: `sheet_folds::fold(reader, map)?` for the native fold leaf.
pub(super) fn fold(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| fold_payload(reader, map))
}

fn fold_payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    model_lists::pointers(reader, map, "array_800", ModelObjectRole::Any)?;
    boundary(reader, map, "boundary_820")?;
    reader.i32("raw_838")?;
    declared(reader, map, "component_840", FEATURE_CLASSES)?;
    declared(
        reader,
        map,
        "transform_data_858",
        &["moFoldUpDownFeatXformData_c"],
    )
}

/// Decode unfold maps and raw kernel bodies before its fixed transform.
/// Example: `sheet_folds::unfold(reader, map)?` with an explicit kernel context.
pub(super) fn unfold(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    fold(reader, map)?;
    reader.scope("unfold_payload", |reader| unfold_payload(reader, map))
}

fn unfold_payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("object_int_map_890", |reader| {
        object_integer_map(reader, map)
    })?;
    raw_bodies(reader)?;
    super::fixed_transform_records::read(reader, "transform_8c0.words", "unfold")?;
    declared(
        reader,
        map,
        "edge_948",
        &["moCompEdge_c", "moCompSilhEdge_c"],
    )?;
    reader.f64("raw_950")?;
    Ok(())
}

fn raw_bodies(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = super::configuration_io::positive_count(reader, "declared_body_count", 4)?;
    for index in 0..count {
        reader.scope(
            &format!("raw_bodies_888[{index}]"),
            super::kernel_frames::read_raw_body,
        )?;
    }
    Ok(())
}

/// Preserve ordered map associations, including duplicate object keys.
/// Example: `sheet_folds::object_integer_map(reader, map)?` for a direct map.
pub(super) fn object_integer_map(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    let count = reader.count_with_minimum("entries.count", 6)?;
    for index in 0..count {
        reader.object(&format!("entries[{index}].key"), map, ModelObjectRole::Any)?;
        reader.i32(&format!("entries[{index}].value"))?;
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
