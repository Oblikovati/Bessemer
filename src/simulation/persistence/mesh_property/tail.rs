use super::super::load_reader::BinaryLoadReader;
use super::decode::{raw_fields, words};
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 810006 {
        duration(reader)?;
    }
    if reader.version >= 840013 {
        words(reader, &["word9b0"])?;
    }
    if reader.version >= 850001 {
        arrays(reader)?;
    }
    late(reader)
}

fn late(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 890104 {
        words(reader, &["temporary_word"])?;
    }
    if reader.version >= 890111 {
        words(reader, &["word2ac"])?;
    }
    if reader.version >= 890512 {
        words(reader, &["word89c", "word8a0"])?;
    }
    if reader.version >= 890703 {
        keyed(reader)?;
    }
    newest(reader)
}

fn newest(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    if reader.version >= 890907 {
        let count = reader.count("mesh.double_array.count", 10000, 8)?;
        reader.doubles("mesh.double_array", count)?;
    }
    if reader.version >= 890909 {
        words(reader, &["word73c"])?;
    }
    if reader.version >= 891103 {
        let count = reader.count("mesh.newest_set.count", 10000, 4)?;
        reader.words("mesh.newest_set.keys", count)?;
    }
    Ok(())
}

fn duration(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    let count = if reader.version >= 840017 {
        reader.count("mesh.duration_count", 10000, 4)?
    } else {
        1
    };
    reader.words("mesh.duration_words", count)
}

fn arrays(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    let count = reader.count("mesh.array_objects.count", 10000, 28)?;
    for index in 0..count {
        let prefix = format!("mesh.array_object{index}");
        reader.words(&format!("{prefix}.five_words"), 5)?;
        let first = reader.count(&format!("{prefix}.count0"), 10000, 4)?;
        let second = reader.count(&format!("{prefix}.count1"), 10000, 4)?;
        reader.words(&format!("{prefix}.array0"), first)?;
        reader.words(&format!("{prefix}.array1"), second)?;
    }
    Ok(())
}

fn keyed(reader: &mut BinaryLoadReader<'_>) -> Result<()> {
    let count = reader.count("mesh.map.count", 10000, 48)?;
    for index in 0..count {
        keyed_record(reader, &format!("mesh.map{index}"))?;
    }
    Ok(())
}

fn keyed_record(reader: &mut BinaryLoadReader<'_>, prefix: &str) -> Result<()> {
    reader.signed(&format!("{prefix}.key"))?;
    reader.words(&format!("{prefix}.words"), 3)?;
    reader.doubles(&format!("{prefix}.doubles"), 3)?;
    reader.signed(&format!("{prefix}.word"))?;
    let count = reader.count(&format!("{prefix}.child_count"), 10000, 40)?;
    raw_fields(reader, &format!("{prefix}.children"), count, 40)
}
