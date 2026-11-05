use super::ModelDecoder;
use crate::archive_codec::{MaterialRead, vectors::double_vector};
use crate::{BessemerError, FailureClass, Result};

/// Retain the ordinary 72-byte body without runtime byte normalization; e.g. `read(reader, 1)?`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, schema: u16) -> Result<()> {
    reader.require_ordinary_part()?;
    conic_schema(reader, schema)?;
    double_vector(reader, "point_08")?;
    double_vector(reader, "vector_20")?;
    reader.f64s(&["raw_38", "raw_40"])?;
    // #218 / C-3579: native byte-backed members occupy separate full signed wire words.
    reader.i32s(&["raw_48", "raw_49"])
}

fn conic_schema(reader: &mut ModelDecoder<'_>, schema: u16) -> Result<()> {
    if schema == 1 {
        return Ok(());
    }
    reader.checked(Err(BessemerError::new(
        "model archive",
        "moConicBendData_c",
        FailureClass::Unsupported,
        format!("moConicBendData_c schema {schema}; expected documented schema 1"),
    )))
}
