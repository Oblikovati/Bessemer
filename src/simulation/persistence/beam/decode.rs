use super::super::{load_base, load_reader::BinaryLoadReader};
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    load_base::references(reader, "record0")?;
    reader.words("record0.beam.four_words", 4)?;
    reader.blob("record0.beam.shorte2", 2)?;
    reader.blob("record0.beam.shorte0", 2)?;
    reader.text("record0.beam.textd0")?;
    reader.text("record0.beam.textc8")?;
    section(reader)?;
    if reader.version >= 820007 {
        let count = reader.count("record0.beam.point_count", 10000, 24)?;
        reader.doubles("record0.beam.points", count * 3)?;
    }
    reader.signed("record0.beam.link_word")?;
    embedded_words(reader)?;
    if reader.version >= 810029 {
        reader.words("record0.beam.two_longs", 2)?;
    }
    if reader.version >= 850036 {
        reader.signed("record0.beam.worda8")?;
    }
    Ok(())
}

fn section(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version < 840014 {
        return Ok(());
    }
    reader.doubles("record0.beam.section.four_doubles", 4)?;
    if reader.version >= 850035 {
        reader.doubles("record0.beam.section.three_doubles", 3)?;
    }
    if reader.version >= 840024 {
        reader.signed("record0.beam.section.word80")?;
    }
    Ok(())
}

fn embedded_words(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    reader.signed("record0.beam.embedded.word08")?;
    reader.signed("record0.beam.embedded.word0c")?;
    reader.unsigned("record0.beam.embedded.word10")?;
    reader.signed("record0.beam.embedded.word14")?;
    reader.unsigned("record0.beam.embedded.word18")
}
