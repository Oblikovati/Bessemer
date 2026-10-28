use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::{ModelDecoder, ModelFieldValue, ModelObjectRole, kernel_frames, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, StoredDouble};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_curve", |reader| reference_curve(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

pub(super) fn reference_curve(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_geometry", |reader| {
        super::reference_planes::reference_geometry(reader, map)
    })?;
    reader.scope("wire_body_710", kernel_frames::read_body)?;
    reader.i32("wire_node_id_718")?;
    model_lists::pointers(reader, map, "attributes_728", ModelObjectRole::Any)
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_778")?;
    reader.f64("raw_780")?;
    let additional = reader.scope("data_788", data)?;
    for offset in ["790", "798", "7a0"] {
        declared(reader, map, &format!("dimension_{offset}"), DIMENSIONS)?;
    }
    if additional {
        declared(reader, map, "dimension_7a8", DIMENSIONS)?;
    }
    reader.i32("raw_7c8")?;
    declared(reader, map, "dimension_7b0", DIMENSIONS)?;
    if reader.u8("dimension_table_presence")? == 1 {
        reader.scope("dimension_table_7c0", |reader| dimension_table(reader, map))?;
    }
    Ok(())
}

fn data(reader: &mut ModelDecoder<'_>) -> Result<bool> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_00", "raw_04"])?;
    reader.f64s(&["raw_10", "raw_18", "raw_20"])?;
    let additional = pitch_condition(reader)?;
    reader.f64s(&["raw_30", "raw_38"])?;
    reader.i32s(&["raw_40", "raw_44"])?;
    reader.f64s(&["raw_48", "raw_50", "raw_58"])?;
    if reader.u8("variable_pitch_presence")? != 0 {
        reader.scope("variable_pitch_60", variable_pitch)?;
    }
    Ok(additional)
}

fn pitch_condition(reader: &mut ModelDecoder<'_>) -> Result<bool> {
    let bits = reader.scalar(|cursor| cursor.u64())?;
    reader.field(
        "raw_28",
        ModelFieldValue::F64(StoredDouble::from_bits(bits)),
    )?;
    // #54 / C-212 binds the native sign-clear mask and strict double threshold.
    Ok(f64::from_bits(bits).abs() > 1e-6)
}

fn variable_pitch(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32("raw_00")?;
    for offset in ["08", "18", "28", "38", "48"] {
        reader.scope(
            &format!("array_{offset}"),
            super::display_placement::parameters,
        )?;
    }
    Ok(())
}

fn dimension_table(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let count = reader.signed_count("rows.count", 8)?;
    for row in 0..count {
        for column in 0..4 {
            declared(reader, map, &format!("rows[{row}][{column}]"), DIMENSIONS)?;
        }
    }
    if count != 0 {
        declared(reader, map, "dimension_60", DIMENSIONS)?;
    }
    Ok(())
}
