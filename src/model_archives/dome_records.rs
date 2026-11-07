use super::display_records::declared;
use super::{ModelDecoder, body_features};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["raw_858", "raw_85c"])?;
    declared(reader, map, "dimension_850", &["moDisplayDistanceDim_c"])?;
    declared(reader, map, "faces_838", &["suObList"])?;
    for label in ["reference_840", "reference_848"] {
        declared(reader, map, label, super::model_references::WRAPPER_CLASSES)?;
    }
    reader.i32("raw_860")?;
    Ok(())
}
