use super::{ModelDecoder, ModelObjectRole, body_features};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve the complete modern cavity body and its two typed reference slots.
/// Example: `cavity_features::read(reader, map)?` for `moCavity_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_810")?;
    double_vector(reader, "vector_818")?;
    reader.i32s(&["raw_830", "raw_834"])?;
    reader.object("face_838", map, ModelObjectRole::Declared(&["moFaceRef_c"]))?;
    reader.object(
        "coordinate_848",
        map,
        ModelObjectRole::Declared(&["moCoordSysRef_w"]),
    )
}
