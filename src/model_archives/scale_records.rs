use super::{ModelDecoder, ModelObjectRole, body_features, model_lists, model_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.i32("raw_80c")?;
    double_vector(reader, "vector_810")?;
    reader.i32("raw_808")?;
    reader.object(
        "coordinate_system_828",
        map,
        ModelObjectRole::Declared(&["moCoordSysRef_w"]),
    )?;
    model_lists::pointers(reader, map, "array_830", ModelObjectRole::Any)
}

pub(super) fn coordinate_system(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    model_references::wrapper(reader, map)?;
    reader.transform("transform_a8")
}
