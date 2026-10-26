use super::{ModelDecoder, ModelObjectRole, features, model_lists};
use crate::Result;
use crate::archive_codec::{
    MaterialRead, ObjectReader,
    light_payload::{self, LightPayloadBranch},
};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("shared_parameters", |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })?;
    reader.object(
        "environment_folder",
        map,
        ModelObjectRole::Declared(&["moEnvFolder_c"]),
    )?;
    reader.object(
        "units_9b8",
        map,
        ModelObjectRole::Declared(&["moUnitsTable_c"]),
    )?;
    reader.object("list_9d0", map, ModelObjectRole::Declared(&["suObList"]))?;
    if reader.version() < 0x3734 {
        named_planes(reader)?;
    }
    Ok(())
}

fn named_planes(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let count = reader.u32("named_plane_count")? as usize;
    let available = reader.remaining() / 145;
    let shape = if count <= available {
        Ok(())
    } else {
        Err(super::invalid(format!(
            "named-plane count {count}; expected <= {available} entries of at least 145 bytes"
        )))
    };
    reader.checked(shape)?;
    reader.entries(count, "named planes")?;
    for index in 0..count {
        reader.scope(&format!("named_planes[{index}]"), plane_entry)?;
    }
    Ok(())
}

fn plane_entry(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.text("name")?;
    for index in 0..3 {
        for label in ["root_point", "normal"] {
            crate::archive_codec::vectors::double_vector(
                reader,
                &format!("planes[{index}].{label}"),
            )?;
        }
    }
    Ok(())
}

pub(super) fn light(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("model_feature", |reader| {
        features::model_feature(reader, map)
    })?;
    match name {
        "moLightFeature_c" | "moAmbientLight_c" => {
            light_payload::ambient(reader, LightPayloadBranch::ModelFeature)
        }
        "moDirectionLight_c" => {
            light_payload::directional(reader, LightPayloadBranch::ModelFeature)
        }
        "moSpotLight_c" => light_payload::spot(reader, LightPayloadBranch::ModelFeature),
        "moPointLight_c" => light_payload::point(reader, LightPayloadBranch::ModelFeature),
        _ => Err(super::invalid(format!(
            "light class {name:?}; expected a documented model light class"
        ))),
    }
}
