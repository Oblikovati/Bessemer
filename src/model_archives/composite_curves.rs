use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, helix_records, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("reference_curve", |reader| {
        helix_records::reference_curve(reader, map)
    })?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // #54 / C-278: only +778 is tagged; the five following containers have direct bodies.
    reader.i32("raw_780")?;
    declared(reader, map, "tagged_array_778", &["suObArray"])?;
    model_lists::pointers(reader, map, "array_788", ModelObjectRole::Any)?;
    for offset in ["7a8", "7c8", "7e8", "808"] {
        model_lists::unsigned_words(reader, &format!("dwords_{offset}"))?;
    }
    Ok(())
}
