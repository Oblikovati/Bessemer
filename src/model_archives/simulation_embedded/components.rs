use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord};
use crate::model_archives::{
    ModelDecoder, ModelObjectRole, component_references, part_objects::absent_payload,
};

pub(super) fn null_owner(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let owner = reader.object_record("owner", map, ModelObjectRole::Any)?;
    if owner != ObjectRecord::Null {
        return Err(super::unsupported(
            "owner",
            format!("owner {owner:?}; expected null in the Simulation embedded helper context",),
        ));
    }
    Ok(())
}

pub(super) fn unit(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    null_owner(reader, map)?;
    let external = reader.u8("external")?;
    if external != 0 {
        return Err(super::unsupported(
            "external unit model",
            format!(
                "external flag {external}; expected 0 in the Simulation embedded helper context",
            ),
        ));
    }
    Ok(())
}

pub(super) fn holder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.archive_version().value() < 4307 {
        return Err(super::unsupported(
            "moCompHolder_c",
            format!(
                "version {}; expected embedded holder version >= 4307",
                reader.archive_version().value(),
            ),
        ));
    }
    reference_base(reader, map)
}

pub(super) fn reference_base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object("component", map, ModelObjectRole::Any)?;
    component_references::base_head(reader)?;
    reader.i32s(&["unknown_a0", "unknown_34"])?;
    historical_words(reader)?;
    historical_strings(reader, map)?;
    guarded_suffix(reader)
}

fn historical_words(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let version = reader.archive_version().value();
    if version > 0x11ab {
        reader.i32("unknown_48")?;
    }
    if version > 0x279f && version < 0x27a8 {
        reader.i32("unknown_94_legacy")?;
    }
    if version > 0x27a7 {
        reader.i32s(&["unknown_90", "unknown_94", "unknown_98", "unknown_9c"])?;
    }
    Ok(())
}

fn historical_strings(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let version = reader.archive_version().value();
    if version > 0x27b1 && reader.i32("has_faceted_data")? != 0 {
        reader.object("faceted_data", map, ModelObjectRole::Any)?;
    }
    if version > 0x27ca && version < 0x27cc {
        reader.text("unknown_b8_legacy")?;
    }
    if version > 0x27cb {
        optional_text(reader, "has_string_b8", "unknown_b8")?;
    }
    Ok(())
}

fn guarded_suffix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let version = reader.archive_version().value();
    for (threshold, label) in [(0x2b60, "has_reference_info"), (0x2bb8, "has_transform_c8")] {
        if version > threshold {
            let present = reader.i32(label)?;
            absent_payload(reader, label, present)?;
        }
    }
    if version > 0x2f55 {
        optional_text(reader, "has_string_d0", "unknown_d0")?;
    }
    if version >= 0x3ef6 {
        reader.i32("unknown_4c")?;
    }
    Ok(())
}

fn optional_text(reader: &mut ModelDecoder<'_>, presence: &str, label: &str) -> Result<()> {
    if reader.i32(presence)? != 0 {
        reader.text(label)?;
    }
    Ok(())
}
