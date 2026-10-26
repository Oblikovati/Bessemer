use super::display_records::declared;
use super::{ModelDecoder, body_features, display_dimension};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "list_820", &["suObList"])?;
    declared(reader, map, "dimension_810", display_dimension::CLASSES)?;
    reader.u8("raw_818")?;
    for label in ["list_828", "list_830"] {
        declared(reader, map, label, &["suObList"])?;
    }
    declared(
        reader,
        map,
        "edge_840",
        &["moCompEdge_c", "moCompSilhEdge_c"],
    )?;
    declared(reader, map, "solid_body_848", &["moCompSolidBody_c"])
}
