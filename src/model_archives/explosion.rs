use super::{ModelDecoder, ModelObjectRole, features, model_lists, node};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

const ARRAY: ModelObjectRole<'_> = ModelObjectRole::Declared(&["suObArray"]);
const SOLID: ModelObjectRole<'_> = ModelObjectRole::Declared(&["moCompSolidBody_c"]);
const STEP: ModelObjectRole<'_> = ModelObjectRole::Declared(&[
    "moPrtExplodeStep_c",
    "moBodyExplodeStep_c",
    "moPrtChainExplodeStep_c",
]);

pub(super) fn exploder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::model_feature(reader, map)?;
    reader.i32s(&["raw_360", "raw_364"])?;
    reader.object("array_368", map, ARRAY)?;
    reader.object("array_370", map, ARRAY)?;
    reader.f64("raw_378")?;
    reader.object("array_388", map, ARRAY)?;
    for index in 0..10 {
        reader.f64(&format!("raw_bbox_390[{index}]"))?;
    }
    reader.scope("list_398", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })
}

pub(super) fn step(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::read(reader, map)?;
    reader.object(
        "direction",
        map,
        ModelObjectRole::Declared(&["moDirectionSpec_c"]),
    )?;
    reader.f64("raw_368")?;
    reader.object("array_370", map, ARRAY)?;
    reader.i32s(&["raw_378", "raw_37c"])?;
    double_vector(reader, "raw_vector_3c8")?;
    reader.transform("transform_3e0")?;
    model_lists::unsigned_words(reader, "raw_array_3a0")
}

pub(super) fn body_step(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    step(reader, map)?;
    reader.required_object("body_478", map, SOLID)?;
    reader.object("array_480", map, ARRAY)
}

pub(super) fn chain_step(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    step(reader, map)?;
    reader.scope("array_478", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })?;
    reader.i32("raw_498")?;
    Ok(())
}

pub(super) fn body_transform(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.transform("transform")?;
    reader.required_object("body", map, SOLID)
}

pub(super) fn line(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.scope("node", |reader| node::lightweight(reader, map).map(|_| ()))?;
    let role = if reader.assembly_kind().is_some() {
        ModelObjectRole::Declared(&[
            "moPrtExplodeStep_c",
            "moBodyExplodeStep_c",
            "moPrtChainExplodeStep_c",
            "moCompExplodeStep_c",
        ])
    } else {
        STEP
    };
    reader.required_object("step", map, role)?;
    double_vector(reader, "raw_point_48")?;
    double_vector(reader, "raw_point_60")?;
    reader.f64s(&["raw_78", "raw_80"])?;
    reader.i32s(&["raw_88", "raw_8c", "raw_90"])?;
    for label in ["attachment_a0", "attachment_a8"] {
        reader.object(label, map, ModelObjectRole::Declared(&["moAttachPoint_c"]))?;
    }
    Ok(())
}

pub(super) fn direction(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_model()?;
    reader.scope("array_08", |reader| {
        if reader.i32("presence")? != 0 {
            reader.scope("array", |reader| {
                model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
            })?;
        }
        Ok(())
    })?;
    reader.i32("word_10")?;
    if reader.u8("has_vector")? != 0 {
        direction_vectors(reader)?;
    }
    reader.i32("word_14")?;
    Ok(())
}

fn direction_vectors(reader: &mut ModelDecoder<'_>) -> Result<()> {
    double_vector(reader, "vector_18")?;
    if reader.u8("has_vector_20")? != 0 {
        double_vector(reader, "vector_20")?;
    }
    if reader.u8("has_point_28")? != 0 {
        double_vector(reader, "point_28")?;
    }
    Ok(())
}
