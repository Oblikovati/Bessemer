use super::display_records::declared;
use super::feature_edit_values::{dimension, doubles, vectors, wrapper};
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve the bending feature's required data and independent matrix-helper pointer.
/// Example: `bending_features::read(reader, map)?` for `moBending_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.required_object(
        "data_810",
        map,
        ModelObjectRole::Declared(&["moBendingData_c"]),
    )?;
    declared(reader, map, "matrix_helper_820", &["moMatrixIndexHelper_c"])
}

/// Read the direct owned array and preserve the packed signed word without runtime migration.
/// Example: `bending_features::payload(reader, map)?` for `moBendingData_c`.
pub(super) fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    leading(reader, map)?;
    for offset in ["130", "138", "140"] {
        wrapper(reader, map, offset)?;
    }
    for (pointer, word) in [("148", "150"), ("158", "160"), ("168", "170")] {
        dimension(reader, map, pointer)?;
        reader.i32(&format!("raw_{word}"))?;
    }
    doubles(reader, &["f0", "f8"])?;
    reader.i32("raw_120").map(|_| ())
}

fn leading(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // #54 / C-397 proves direct suObArray construction and dispatch at the data object's start.
    model_lists::pointers(reader, map, "array_a0", ModelObjectRole::Any)?;
    declared(
        reader,
        map,
        "stock_128",
        &["moStock_c", "moMirrorStock_c", "moXformStock_c"],
    )?;
    double_vector(reader, "point_10")?;
    vectors(reader, &["28", "40", "58"])?;
    doubles(reader, &["70", "78"])?;
    reader.i32("packed_80_e8_ec")?;
    doubles(reader, &["88", "90"])?;
    reader.i32("raw_98").map(|_| ())
}
