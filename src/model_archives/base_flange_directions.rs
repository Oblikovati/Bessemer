use super::ModelDecoder;
use super::display_records::declared;
use super::model_references::WRAPPER_CLASSES;
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    mode: i32,
    reference_byte: u8,
) -> Result<()> {
    // C-183: equality preserves out-of-range modes without overflowing mode - 1.
    for index in 0..2 {
        if mode == 0 || mode == index + 1 {
            reader.scope(&format!("directions[{index}]"), |reader| {
                direction(reader, map, reference_byte)
            })?;
        }
    }
    Ok(())
}

fn direction(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    reference_byte: u8,
) -> Result<()> {
    let kind = reader.i32("type")?;
    selected_payload(reader, map, kind, reference_byte)?;
    declared(reader, map, "dimension", super::display_dimension::CLASSES)?;
    reader.f64("raw_extra")?;
    Ok(())
}

fn selected_payload(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    kind: i32,
    reference_byte: u8,
) -> Result<()> {
    match kind {
        0 | 6 => {
            reader.f64("raw_value")?;
        }
        3 => return reader.object("reference", map, super::ModelObjectRole::Any),
        4 | 5 if reference_byte != 0 => {
            return declared(reader, map, "reference", WRAPPER_CLASSES);
        }
        _ => {}
    }
    Ok(())
}
