use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists, model_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Retain the complete move/copy feature and its independently versioned dimensions.
/// Example: `move_copy_bodies::read(reader, map)?` for `moMoveCopyBody_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| outer(reader, map))
}

fn outer(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "data_9c8", &["moMoveCopyBodyData_c"])?;
    declared(reader, map, "matrix_helper_9d8", &["moMatrixIndexHelper_c"])?;
    reader.i32("raw_9f0")?;
    if reader.version() > 0x3b0b {
        dimensions(reader, map)?;
    }
    if reader.version() > 0x42cc {
        late_dimensions(reader, map)?;
    }
    Ok(())
}

fn dimensions(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["808", "810", "818", "828", "830", "838"] {
        dimension(reader, map, offset)?;
    }
    reader.i32s(&[
        "raw_8b8", "raw_8bc", "raw_8c0", "raw_8c4", "raw_8c8", "raw_8cc",
    ])
}

fn dimension(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offset: &str) -> Result<()> {
    declared(
        reader,
        map,
        &format!("dimension_{offset}"),
        super::display_dimension::CLASSES,
    )
}

fn late_dimensions(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for (pointer, word) in [("800", "raw_8d0"), ("820", "raw_8d4")] {
        dimension(reader, map, pointer)?;
        reader.i32(word)?;
    }
    Ok(())
}

/// Decode the complete documented nested payload without native ownership fixups.
/// Example: `move_copy_bodies::payload(reader, map)?` for `moMoveCopyBodyData_c`.
pub(super) fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    leading(reader, map)?;
    stock_and_vectors(reader, map)?;
    trailing_references(reader, map)?;
    reader.scope("mesh_tolerance_338", super::move_copy_deformation::mesh)?;
    if reader.version() > 0x2f69 {
        reader.scope("conformal_90", super::move_copy_deformation::conformal)?;
    }
    late_words(reader)
}

fn leading(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for offset in ["3d8", "3e0", "3e8"] {
        wrapper(reader, map, offset)?;
    }
    reader.f64("raw_328")?;
    model_lists::pointers(reader, map, "array_370", ModelObjectRole::Any)?;
    super::fixed_transform_records::read(reader, "transform_08", "move/copy")?;
    double_vector(reader, "point_2d8")?;
    reader.i32("raw_320")?;
    vectors(reader, &["2f0", "308"])?;
    reader.i32s(&["raw_368", "raw_36c"])
}

fn stock_and_vectors(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // C-246: ordinary suPtrArray::Serialize contributes no bytes or object-map slot.
    declared(reader, map, "stock_3d0", &["moStock_c"])?;
    reader.i32("raw_410")?;
    vectors(reader, &["418", "430", "448"])
}

fn trailing_references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    wrapper(reader, map, "478")?;
    reader.i32("raw_4c0")?;
    declared(reader, map, "plane_4c8", &["moPlaneRef_w"])?;
    reader.i32s(&["raw_4d0", "raw_4d4", "raw_330"])
}

fn wrapper(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, offset: &str) -> Result<()> {
    declared(
        reader,
        map,
        &format!("reference_{offset}"),
        model_references::WRAPPER_CLASSES,
    )
}

fn vectors(reader: &mut ModelDecoder<'_>, offsets: &[&str]) -> Result<()> {
    for offset in offsets {
        double_vector(reader, &format!("vector_{offset}"))?;
    }
    Ok(())
}

fn late_words(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.version() > 0x3b0b {
        reader.i32s(&[
            "raw_484", "raw_488", "raw_48c", "raw_494", "raw_498", "raw_49c", "raw_4a4",
        ])?;
    }
    if reader.version() > 0x42cc {
        reader.i32s(&["raw_490", "raw_4a0"])?;
    }
    Ok(())
}
