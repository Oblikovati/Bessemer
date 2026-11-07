use super::super::load_reader::BinaryLoadReader;
use super::decode::raw_fields;
use crate::Result;

pub(super) fn read(reader: &mut BinaryLoadReader<'_>, prefix: &str) -> Result<()> {
    reader.signed(&format!("{prefix}.word00"))?;
    raw_fields(reader, &format!("{prefix}.fixed_eight_byte_fields"), 33, 8)?;
    if reader.version >= 850029 {
        array(reader, &format!("{prefix}.array0"))?;
        array(reader, &format!("{prefix}.array1"))?;
    }
    if reader.version >= 850035 {
        raw_fields(reader, &format!("{prefix}.eleven_raw64_tail"), 11, 8)?;
    }
    if reader.version >= 890502 {
        raw_fields(reader, &format!("{prefix}.two_raw64_tail"), 2, 8)?;
    }
    Ok(())
}

fn array(reader: &mut BinaryLoadReader<'_>, prefix: &str) -> Result<()> {
    raw_fields(reader, &format!("{prefix}.nine_raw64"), 9, 8)?;
    reader.signed(&format!("{prefix}.word"))?;
    let count = reader.count(&format!("{prefix}.count"), 100000, 8)?;
    raw_fields(reader, &format!("{prefix}.raw64_values"), count, 8)
}
