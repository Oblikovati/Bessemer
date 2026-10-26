use super::{ModelDecoder, ModelObjectRole, sketch_geometry::DirectReader};
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn word_count(reader: &mut ModelDecoder<'_>, label: &str) -> Result<usize> {
    let count = reader.u16(label)?;
    reader.counted_entries(label, i64::from(count), 2)
}

pub(super) fn direct(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    labels: (&str, &str),
    body: DirectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..word_count(reader, labels.0)? {
        reader.scope(&format!("{}[{index}]", labels.1), |reader| {
            body(reader, map)
        })?;
    }
    Ok(())
}

pub(super) fn tagged(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    labels: (&str, &str),
    role: ModelObjectRole<'_>,
) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..word_count(reader, labels.0)? {
        reader.required_object(&format!("{}[{index}]", labels.1), map, role)?;
    }
    Ok(())
}
