use super::display_records::declared;
use super::{ModelDecoder, model_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Retain a reviewed wrapper subtype at its native offset; e.g. `wrapper(reader, map, "130")?`.
pub(super) fn wrapper(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    offset: &str,
) -> Result<()> {
    declared(
        reader,
        map,
        &format!("reference_{offset}"),
        model_references::WRAPPER_CLASSES,
    )
}

/// Retain a shared display dimension; e.g. `dimension(reader, map, "148")?`.
pub(super) fn dimension(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    offset: &str,
) -> Result<()> {
    declared(
        reader,
        map,
        &format!("dimension_{offset}"),
        super::display_dimension::CLASSES,
    )
}

/// Preserve stored vector bits in wire order; e.g. `vectors(reader, &["28", "40"])?`.
pub(super) fn vectors(reader: &mut ModelDecoder<'_>, offsets: &[&str]) -> Result<()> {
    for offset in offsets {
        double_vector(reader, &format!("vector_{offset}"))?;
    }
    Ok(())
}

/// Preserve raw scalar bits in wire order; e.g. `doubles(reader, &["70", "78"])?`.
pub(super) fn doubles(reader: &mut ModelDecoder<'_>, offsets: &[&str]) -> Result<()> {
    for offset in offsets {
        reader.f64(&format!("raw_{offset}"))?;
    }
    Ok(())
}
