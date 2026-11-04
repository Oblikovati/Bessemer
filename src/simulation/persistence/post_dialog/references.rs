use super::super::load_reader::BinaryLoadReader;
use super::helpers::{count, counted_bytes, raw_words};
use crate::{Result, archive_codec::ArchiveCursor, simulation::invalid};

pub(super) fn read(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    let count = count(reader, label)?;
    for index in 0..count {
        record(reader, &format!("{label}.record[{index}]"))?;
    }
    raw_words(reader, &format!("{label}.two_trailing_words"), 2)
}

fn record(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    if reader.version >= 840002 {
        modern(reader, label)?;
    } else {
        raw_words(reader, &format!("{label}.legacy_header"), 7)?;
        persist_text(reader, &format!("{label}.legacy_persist_text"))?;
        counted_bytes(reader, &format!("{label}.blob"), 1000000)?;
    }
    if reader.version >= 800075 {
        raw_words(reader, &format!("{label}.words58_5c"), 2)?;
    }
    if reader.version >= 890021 {
        raw_words(reader, &format!("{label}.word60_raw64_68.words"), 1)?;
        reader.blob(&format!("{label}.word60_raw64_68.raw64"), 8)?;
    }
    Ok(())
}

fn modern(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    raw_words(reader, &format!("{label}.id_header"), 2)?;
    counted_bytes(reader, &format!("{label}.blob"), 1000000)?;
    for index in 0..2 {
        reader.embedded(&format!("{label}.archive[{index}]"))?;
    }
    raw_words(reader, &format!("{label}.words38_48"), 5)?;
    persist_text(reader, &format!("{label}.persist_text"))
}

fn persist_text(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    let count = ArchiveCursor::new(&reader.bytes[reader.position()..]).u32()?;
    if count > 0x100000 {
        return Err(invalid(
            label,
            format!("{count} payload bytes; expected <= imported PersistText safety cap 1048576"),
        ));
    }
    reader.text(label)
}
