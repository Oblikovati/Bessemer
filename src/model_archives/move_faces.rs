use super::display_records::declared;
use super::feature_edit_values::{dimension, doubles, vectors, wrapper};
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Retain the complete move-face body, compact transform and ordered reference slots.
/// Example: `move_faces::read(reader, map)?` for `moMoveFace_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    leading(reader, map)?;
    geometry(reader, map)?;
    reader.transform("transform_928")?;
    vectors(reader, &["9b0"])?;
    reader.i32s(&["raw_9c8", "raw_a98", "raw_a94"])?;
    model_lists::pointers(reader, map, "array_aa0", ModelObjectRole::Any)?;
    trailing(reader, map)
}

fn leading(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    doubles(reader, &["828", "820"])?;
    reader.i32("raw_800")?;
    declared(reader, map, "direction_818", &["moDirectionSpec_c"])?;
    reader.i32("raw_838")?;
    reader.scope("array_810", |reader| {
        body_features::nullable_array(reader, map)
    })?;
    declared(
        reader,
        map,
        "line_830",
        super::geometric_references::LINE_REFERENCE_CLASSES,
    )?;
    declared(reader, map, "list_808", &["suObList"])?;
    dimension(reader, map, "840")?;
    dimension(reader, map, "848")
}

fn geometry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["raw_880", "raw_884"])?;
    double_vector(reader, "point_888")?;
    vectors(reader, &["8a0", "8b8", "8d0"])?;
    wrapper(reader, map, "8e8")?;
    wrapper(reader, map, "8f0")?;
    vectors(reader, &["8f8", "910"])
}

fn trailing(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_ac4")?;
    doubles(reader, &["ac8"])?;
    wrapper(reader, map, "ad0")?;
    declared(
        reader,
        map,
        "point_reference_ad8",
        super::geometric_references::POINT_REFERENCE_CLASSES,
    )?;
    wrapper(reader, map, "ae0")?;
    reader.i32("raw_ae8")?;
    wrapper(reader, map, "af0")?;
    declared(reader, map, "direction_af8", &["moDirectionSpec_c"])?;
    reader.i32("raw_b30").map(|_| ())
}
