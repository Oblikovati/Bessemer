use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| super::body_features::base(reader, map))?;
    reader.scope("bend_group", |reader| bend_group(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn bend_group(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("common_bend_800", |reader| {
        super::sheet_payload::common(reader, map)
    })?;
    reader.i32s(&["raw_840", "raw_844", "raw_848"])?;
    model_lists::pointers(reader, map, "array_858", ModelObjectRole::Any)?;
    reader.u32("raw_878")?;
    Ok(())
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "profile_888", super::profile_features::CLASSES)?;
    reader.i32s(&["raw_898", "raw_8c8"])?;
    let mode = reader.i32("raw_8f8")?;
    let reference_byte = reader.u8("reference_byte")?;
    reader.i32("raw_880")?;
    super::base_flange_directions::read(reader, map, mode, reference_byte)?;
    for label in ["958", "978", "9b8", "9d8"] {
        reader.scope(&format!("handle_arrays.{label}"), |reader| {
            handles(reader, map)
        })?;
    }
    tail(reader, map)
}

fn handles(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let count = reader.signed_count("count", 2)?;
    if count == 0 {
        return Ok(());
    }
    declared(reader, map, "sketch", &["sgSketch"])?;
    for index in 0..count {
        declared(
            reader,
            map,
            &format!("handles[{index}]"),
            super::sketch_references::HANDLES,
        )?;
    }
    Ok(())
}

fn tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x3b0c {
        reader.i32("raw_8cc")?;
    }
    if reader.version() > 0x46b9 {
        declared(reader, map, "from_end_a18", &["moFromEndSpec_c"])?;
    }
    Ok(())
}
