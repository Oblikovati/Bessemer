use super::super::load_reader::BinaryLoadReader;
use super::decode::{doubles, words};
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    words(reader, &["word594", "word598", "word5dc"])?;
    doubles(reader, &["double5a0", "double5a8", "double5d0"])?;
    words(reader, &["word284"])?;
    reader.text("mesh.text5f0")?;
    reader.text("mesh.text5f8")?;
    early(reader)?;
    late(reader)
}

fn early(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 800005 {
        words(reader, &["word708", "word70c"])?;
    }
    if (800078..810000).contains(&reader.version) || reader.version >= 810004 {
        words(reader, &["word37c"])?;
    }
    if reader.version >= 800035 {
        doubles(
            reader,
            &["double710", "double718", "double720", "double728"],
        )?;
    }
    if reader.version > 800037 {
        words(reader, &["word188"])?;
    }
    intermediate(reader)
}

fn intermediate(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 830018 {
        doubles(reader, &["double5b0", "double5b8"])?;
    }
    if reader.version >= 830020 {
        words(reader, &["word5e0"])?;
    }
    Ok(())
}

fn late(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version > 830033 {
        for name in ["text600", "text608", "text610", "text618"] {
            reader.text(&format!("mesh.{name}"))?;
        }
    }
    if reader.version >= 840018 {
        words(reader, &["word378"])?;
    }
    if reader.version >= 890514 {
        words(reader, &["word730", "word734"])?;
    }
    if reader.version >= 890601 {
        doubles(reader, &["double560", "double568"])?;
    }
    if reader.version >= 890605 {
        words(reader, &["word280"])?;
    }
    Ok(())
}
