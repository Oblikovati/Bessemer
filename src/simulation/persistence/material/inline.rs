use super::super::load_reader::BinaryLoadReader;
use crate::Result;

pub(in crate::simulation::persistence) fn read_inline(
    reader: &mut BinaryLoadReader<'_>,
    prefix: &str,
) -> Result<()> {
    let first = reader.fields.len();
    super::decode::read(reader).map_err(|mut error| {
        error.entity = format!("{prefix}.{}", error.entity);
        error
    })?;
    for field in &mut reader.fields[first..] {
        field.path = format!("{prefix}.{}", field.path).into();
    }
    Ok(())
}
