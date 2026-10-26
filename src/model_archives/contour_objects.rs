use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists, profile_features};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn contour(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for label in ["array_20", "array_40"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    if reader.i32("profile_presence_18")? != 0 {
        declared(reader, map, "profile_10", profile_features::CLASSES)?;
    }
    reader.i32s(&["raw_60", "raw_64", "raw_68", "raw_78", "raw_7c"])
}

pub(super) fn chooser(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    model_lists::pointers(reader, map, "array_08", ModelObjectRole::Any)?;
    if reader.i32("profile_presence_48")? != 0 {
        declared(reader, map, "profile_40", profile_features::CLASSES)?;
    }
    reader.i32s(&["raw_38", "raw_3c"])?;
    arrays(reader, map)?;
    reader.i32("raw_58")?;
    Ok(())
}

fn arrays(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let mask = reader.u16("array_mask")?;
    if mask > 3 {
        return reader.checked(Err(super::invalid(format!(
            "contour chooser mask {mask}; expected 0..=3"
        ))));
    }
    for (bit, offset) in [(1, "28"), (2, "30")] {
        if mask & bit != 0 {
            declared(reader, map, &format!("array_{offset}"), &["suObArray"])?;
        }
    }
    Ok(())
}

pub(super) fn region(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "boundary_entities_08", &["suObList"])
}
