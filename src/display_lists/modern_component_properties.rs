use super::{archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn property(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let version = archive.context.version.value();
    let fonts = if version < 0x12cf {
        25
    } else if version < 0x13ef {
        26
    } else if version < 0x23b3 {
        30
    } else {
        31
    };
    archive.entries(fonts, "component property fonts")?;
    for index in 0..fonts {
        archive.scope(&format!("line_fonts[{index}]"), super::modern_ui::font)?;
    }
    archive.scope("ent_vis_prop", visibility)?;
    archive.u8("all_views")?;
    property_tail(archive)
}

fn property_tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let version = archive.context.version.value();
    if version >= 0x147c {
        archive.i32("added_for_envelop")?;
    }
    if version > 0x3738 {
        archive.i32("empty_prop")?;
    }
    Ok(())
}

fn visibility(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.u16("unknown_0")?;
    archive.u16("unknown_2")?;
    archive.u8("unknown_4")?;
    archive.u32("unknown_8")?;
    archive.u16("layer")?;
    archive.scope("line_width", super::modern_item_style::width)
}

pub(super) fn holder(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.cobject("unknown_8")?;
    archive.text("component_name")?;
    archive.object("property", &["moComponentProperty"])?;
    occurrences(archive)?;
    archive.i32s(&["unknown_40", "from_sub_assembly"])
}

fn occurrences(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.su_count("occurrences")?;
    if count > archive.cursor.remaining() / 4 {
        return Err(invalid(format!(
            "component occurrence count {count}; expected unsigned words fitting {} bytes",
            archive.cursor.remaining()
        )));
    }
    for index in 0..count {
        archive.u32(&format!("occurrences[{index}]"))?;
    }
    Ok(())
}
