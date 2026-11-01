use super::super::load_reader::BinaryLoadReader;
use crate::{Result, archive_codec::ArchiveCursor};

pub(super) fn raw_words(
    reader: &mut BinaryLoadReader<'_>,
    label: &str,
    count: usize,
) -> Result<()> {
    for index in 0..count {
        reader.unsigned(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn offsets(
    reader: &mut BinaryLoadReader<'_>,
    prefix: &str,
    offsets: &[u32],
) -> Result<()> {
    for offset in offsets {
        reader.unsigned(&format!("{prefix}.word_{offset:x}"))?;
    }
    Ok(())
}

pub(super) fn raw64(reader: &mut BinaryLoadReader<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        reader.blob(&format!("{label}[{index}]"), 8)?;
    }
    Ok(())
}

pub(super) fn counted_bytes(
    reader: &mut BinaryLoadReader<'_>,
    label: &str,
    maximum: usize,
) -> Result<()> {
    let count = ArchiveCursor::new(&reader.bytes[reader.position()..]).i32()?;
    let count = reader.bounded(count, label, maximum, 1)?;
    reader.blob(label, count + 4)
}

pub(super) fn count(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<usize> {
    reader.count(&format!("{label}.count"), 10000, 1)
}

pub(super) fn color(reader: &mut BinaryLoadReader<'_>, label: &str) -> Result<()> {
    for component in 0..4 {
        reader.unsigned(&format!("{label}.component_{component}"))?;
    }
    Ok(())
}
