use super::{ModelDecoder, body_features, display_records::declared};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the complete body-feature parent before the native end-cap suffix.
/// Example: `end_caps::read(reader, map)?` for the exact end-cap class.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "list_800", &["suObList"])?;
    reader.i32s(&["raw_808", "raw_80c", "raw_828", "raw_82c"])?;
    // #54 / C-390: +840 precedes +838 in the native serializer.
    for offset in ["840", "838", "848", "850"] {
        declared(
            reader,
            map,
            &format!("dimension_{offset}"),
            &["moDisplayDistanceDim_c"],
        )?;
    }
    reader.f64("raw_820")?;
    reader.i32("raw_830")?;
    declared(reader, map, "dimension_858", &["moDisplayArcLengthDim_c"])
}
