use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body_feature", |reader| {
        super::body_features::base(reader, map)
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["800", "820"] {
        model_lists::pointers(reader, map, &format!("array_{label}"), ModelObjectRole::Any)?;
    }
    reader.i32("raw_8c8")?;
    reader.object("reference_8c0", map, ModelObjectRole::Any)?;
    model_lists::pointers(reader, map, "array_840", ModelObjectRole::Any)?;
    reader.i32s(&["raw_8cc", "raw_8d0", "raw_8d4"])?;
    declared(
        reader,
        map,
        "point_reference_860",
        super::geometric_references::POINT_REFERENCE_CLASSES,
    )?;
    if reader.version() > 0x334d {
        reader.i32("raw_868")?;
    }
    Ok(())
}

pub(super) fn body_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "array_108", ModelObjectRole::Any)?;
    reader.i32s(&["raw_78", "raw_7c"])
}

pub(super) fn trimmed(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    body_data(reader, map)?;
    reader.scope("dwords_148", |reader| {
        model_lists::unsigned_words(reader, "items")
    })
}

pub(super) fn tool(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    body_data(reader, map)?;
    // #54 / C-283: native derives flag +158 from these values; no flag byte is stored.
    reader.f64s(&["raw_148", "raw_150"])
}
