use super::display_records::declared;
use super::{ModelDecoder, linear_patterns, reference_planes};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("pattern", |reader| linear_patterns::common(reader, map))?;
    if reader.version() > 0x3745 {
        declared(reader, map, "plane_11a8", &["moPlaneRef_w"])?;
    }
    reader.scope("direct_plane_1138", |reader| {
        reference_planes::plane_reference(reader, map)
    })
}
