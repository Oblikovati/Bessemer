use super::{ModelDecoder, ModelObjectRole, body_features, configuration_io, features};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

const ARRAY: ModelObjectRole<'_> = ModelObjectRole::Declared(&["suObArray"]);
const COMPONENT: ModelObjectRole<'_> = ModelObjectRole::Declared(&[
    "moComponent_c",
    "moReference_c",
    "moReferencePattern_c",
    "moUnitComponent_c",
    "moChainComponent_c",
]);

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    class: &str,
) -> Option<Result<()>> {
    let body = match class {
        "moAsmExploder_c" => exploder,
        "moCompXform_c" => component_transform,
        "moRefXform_c" => reference_transform,
        "moCompExplodeStep_c" => step,
        "moMirrorAssemblyData_c" => mirror,
        "moExplodeLineProfileFeature_c" => profile,
        "moExplodeLine_c" => super::explosion::line,
        _ => return None,
    };
    Some(
        reader
            .require_ordinary_assembly()
            .and_then(|()| body(reader, map)),
    )
}

fn profile(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("profile", |reader| {
        super::profile_features::read(reader, map)
    })?;
    for index in 0..reader.signed_count("automatic_elements.count", 2)? {
        reader.object(
            &format!("automatic_elements[{index}]"),
            map,
            ModelObjectRole::Declared(&["moAutoExpldeLineElementData_c"]),
        )?;
    }
    Ok(())
}

fn exploder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    body_features::model_feature(reader, map)?;
    reader.i32s(&["word_360", "word_364"])?;
    reader.object("array_368", map, ARRAY)?;
    reader.object("discarded_array_378", map, ARRAY)?;
    reader.f64("double_380")?;
    reader.i32("word_388")?;
    reader.object("array_390", map, ARRAY)?;
    for index in 0..10 {
        reader.f64(&format!("box_3a8[{index}]"))?;
    }
    configuration_io::list(reader, map, "list_3b0")
}

fn component_transform(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.transform("transform")?;
    reader.required_object("component", map, COMPONENT)
}

fn reference_transform(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    component_transform(reader, map)?;
    reader.transform("reference_transform")
}

fn step(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::read(reader, map)?;
    for (direction, value) in [("368", "370"), ("378", "380")] {
        reader.object(
            &format!("direction_{direction}"),
            map,
            ModelObjectRole::Declared(&["moDirectionSpec_c"]),
        )?;
        reader.f64(&format!("double_{value}"))?;
    }
    reader.i32("word_388")?;
    reader.object("array_390", map, ARRAY)?;
    reader.i32s(&["word_398", "word_39c"])?;
    double_vector(reader, "vector_3c0")?;
    reader.transform("transform_3d8")?;
    step_geometry(reader)?;
    reader.required_object("component_608", map, COMPONENT)?;
    reader.object("array_610", map, ARRAY)?;
    reader.object("array_618", map, ARRAY)
}

fn step_geometry(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for offset in [
        "460", "478", "510", "528", "540", "558", "570", "588", "5a0", "5b8",
    ] {
        double_vector(reader, &format!("geometry_{offset}"))?;
    }
    Ok(())
}

fn mirror(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object(
        "external_object",
        map,
        ModelObjectRole::Declared(&["moExtObject_c"]),
    )?;
    for offset in ["20", "a8"] {
        reader.transform(&format!("transform_{offset}"))?;
    }
    double_vector(reader, "plane_point")?;
    double_vector(reader, "plane_normal")
}
