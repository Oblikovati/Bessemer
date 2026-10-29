use super::{ModelDecoder, ModelObjectRole, one_bends};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

/// Preserve the complete One Bend base before its tagged sketch-handle suffix.
/// Example: `sketch_bends::read(reader, map)?` for `moSketchBend_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let kind = reader.scope("one_bend", |reader| one_bends::read_kind(reader, map))?;
    if kind == 7 {
        return reader.checked(Err(BessemerError::new(
            "model archive", "sketch-bend kind", FailureClass::Unsupported,
            format!("sketch-bend kind {kind}; expected a non-7 kind until its direct owned-data and raw kernel-body serializers are documented"),
        )));
    }
    reader.scope("payload", |reader| continuation(reader, map))
}

fn continuation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["sketch_e00", "handle_df0", "handle_e18", "handle_e40"] {
        reader.object(label, map, ModelObjectRole::Any)?;
    }
    reader.object(
        "curve_de0",
        map,
        ModelObjectRole::Declared(super::sweep_reference_records::CURVE_REFERENCE_CLASSES),
    )?;
    reader.i32("raw_e70")?;
    Ok(())
}
