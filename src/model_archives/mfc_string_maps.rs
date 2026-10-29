use super::{ModelDecoder, configuration_io, invalid};
use crate::Result;
use crate::archive_codec::MaterialRead;

/// Preserve ordered MFC map entries, including duplicate keys and original text prefixes.
/// Example: `mfc_string_maps::read(reader)?` after the map's object tag.
pub(super) fn read(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_model()?;
    for index in 0..configuration_io::map_entries64(reader, "entries.count", 2)? {
        for member in ["key", "value"] {
            reader.scope(&format!("entries[{index}].{member}.map_text"), text)?;
        }
    }
    Ok(())
}

fn text(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let byte = reader.u8("length_byte")?;
    let (count, wide) = if byte != 255 {
        (u64::from(byte), false)
    } else {
        first_word(reader)?
    };
    let count = bounded_units(reader, count, wide)?;
    for index in 0..count {
        if wide {
            reader.u16(&format!("utf16[{index}]"))?;
        } else {
            reader.u8(&format!("latin1[{index}]"))?;
        }
    }
    Ok(())
}

fn first_word(reader: &mut ModelDecoder<'_>) -> Result<(u64, bool)> {
    let word = reader.u16("length_word")?;
    if word != 0xfffe {
        return extended_word(reader, word).map(|count| (count, false));
    }
    let byte = reader.u8("wide_length_byte")?;
    if byte != 255 {
        return Ok((u64::from(byte), true));
    }
    let word = reader.u16("wide_length_word")?;
    extended_word(reader, word).map(|count| (count, true))
}

fn extended_word(reader: &mut ModelDecoder<'_>, word: u16) -> Result<u64> {
    if word != 0xffff {
        return Ok(u64::from(word));
    }
    let dword = reader.u32("length_dword")?;
    if dword != u32::MAX {
        return Ok(u64::from(dword));
    }
    reader.u64("length_qword")
}

fn bounded_units(reader: &mut ModelDecoder<'_>, count: u64, wide: bool) -> Result<usize> {
    let bounded = i64::try_from(count).map_err(|_| {
        invalid(format!(
            "map string count {count}; expected <= i64::MAX bounded code units"
        ))
    });
    let count = reader.checked(bounded)?;
    reader.counted_entries("map string code units", count, if wide { 2 } else { 1 })
}
