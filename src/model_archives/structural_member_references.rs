use super::display_records::declared;
use super::geometric_references::POINT_REFERENCE_CLASSES;
use super::{ModelDecoder, reference_point_data};
use crate::Result;
use crate::archive_codec::ObjectReader;

pub(super) fn curve(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    declared(reader, map, label, reference_point_data::CURVES)
}

pub(super) fn point(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    declared(reader, map, label, POINT_REFERENCE_CLASSES)
}

pub(super) fn curves(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    for index in 0..reader.signed_count(&format!("{label}.count"), 2)? {
        curve(reader, map, &format!("{label}[{index}]"))?;
    }
    Ok(())
}
