use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve the stored corner options before native zero-to-two normalization.
/// Example: `corner_hems::corner(reader, map)?` for a complete corner feature.
pub(super) fn corner(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    bend_base(reader, map)?;
    reader.scope("payload", |reader| corner_payload(reader, map))
}

fn bend_base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("bend_group", |reader| {
        super::base_flanges::bend_group(reader, map)
    })
}

fn corner_payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["890", "898"] {
        declared(reader, map, &format!("list_{offset}"), &["suObList"])?;
    }
    reader.i32s(&["raw_8a0", "raw_9a0"])?;
    reader.f64s(&["raw_948", "raw_950"])?;
    reader.i32s(&["raw_958", "raw_940", "raw_95c"])?;
    model_lists::pointers(reader, map, "direct_list_8b0", ModelObjectRole::Any)
}

/// Decode the shared fixed face-geometry body without adding a derived suffix.
/// Example: `corner_hems::face_geometry(reader)?` for base or planar geometry.
pub(super) fn face_geometry(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    double_vector(reader, "point_08")?;
    double_vector(reader, "vector_20")
}

/// Retain the directly owned hem data between independently counted arrays.
/// Example: `corner_hems::hem(reader, map)?` for a complete hem feature.
pub(super) fn hem(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    bend_base(reader, map)?;
    reader.scope("payload", |reader| {
        model_lists::pointers(reader, map, "array_9a8", ModelObjectRole::Any)?;
        reader.scope("data", |reader| hem_data(reader, map))?;
        reader.i32("raw_9c8")?;
        Ok(())
    })
}

/// Decode direct or tagged hem data with its required feature-owner reference.
/// Example: `corner_hems::hem_data(reader, map)?` using the saved helper state.
pub(super) fn hem_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    hem_owner(reader, map)?;
    dimensions(reader, map, &["40", "48"])?;
    reader.f64("raw_60")?;
    reader.i32s(&["raw_80", "raw_14", "raw_10"])?;
    dimensions(reader, map, &["78"])?;
    reader.i32("raw_d0")?;
    declared(reader, map, "relief_d8", &["moReliefData_c"])?;
    model_lists::pointers(reader, map, "array_90", ModelObjectRole::Any)
}

fn hem_owner(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.required_object(
        "owner_08",
        map,
        ModelObjectRole::Declared(body_features::CLASSES),
    )?;
    model_lists::pointers(reader, map, "array_20", ModelObjectRole::Any)?;
    if !reader.sheet_metal_extended() {
        declared(
            reader,
            map,
            "sheet_metal_18",
            &["moSheetMetal_c", "moTemplateSheetMetal_c"],
        )?;
    }
    Ok(())
}

fn dimensions(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    offsets: &[&str],
) -> Result<()> {
    for offset in offsets {
        declared(
            reader,
            map,
            &format!("dimension_{offset}"),
            &["moDisplayDistanceDim_c"],
        )?;
    }
    Ok(())
}
