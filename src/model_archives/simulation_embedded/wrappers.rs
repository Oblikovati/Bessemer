use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::model_archives::{ModelDecoder, ModelObjectRole, model_references};

pub(super) fn common_wrapper(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object("component_reference", map, ModelObjectRole::Any)?;
    model_references::wrapper_fields(reader, map)?;
    reader.i32s(&["unknown_4c", "unknown_68"])?;
    if reader.archive_version().value() > 0x1fd1 {
        reader.i32s(&["unknown_34", "unknown_30"])?;
    }
    model_references::final_words(reader)
}

pub(super) fn component_transform(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    if reader.archive_version().value() < 4307 {
        return Err(super::unsupported(
            "moCompXFormWrapper_c",
            format!(
                "version {}; expected embedded component transform version >= 4307",
                reader.archive_version().value(),
            ),
        ));
    }
    common_wrapper(reader, map)?;
    reader.transform("transform_a8")?;
    reader.transform("transform_130")?;
    Ok(())
}

pub(super) fn doubles(reader: &mut ModelDecoder<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        reader.f64(&format!("{label}[{index}]"))?;
    }
    Ok(())
}
