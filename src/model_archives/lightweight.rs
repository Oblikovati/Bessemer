use super::header_imports::{ImportedHeaderMap, ModelHeaderLoadPlan};
use super::{
    ModelDecoder, ModelObjectRole, invalid, lightweight_part, lightweight_post, lightweight_shared,
    node,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, ObjectRecord};

pub(super) struct ModelLightweightState {
    pub imports: ImportedHeaderMap,
    pub node: node::ModelNodePrefix,
    pub raw_1614: i32,
}

pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    header: ModelHeaderLoadPlan,
) -> Result<ModelLightweightState> {
    reader.require_ordinary_part()?;
    root_tag(reader, map)?;
    let size = reader.section_size("lightweight_size")?;
    reader.bounded(size, |reader| body(reader, map, header))
}

fn body(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    header: ModelHeaderLoadPlan,
) -> Result<ModelLightweightState> {
    let imports = header.append(reader, map)?;
    reader.i32("unknown_a8")?;
    reader.u32("unknown_ac")?;
    let node = reader.scope("node", |reader| node::lightweight(reader, map))?;
    concept(reader, map)?;
    let raw_1614 = continuation(reader, map)?;
    Ok(ModelLightweightState {
        imports,
        node,
        raw_1614,
    })
}

fn root_tag(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let root = reader.object_tag("root", map, ModelObjectRole::Declared(&["moPart_c"]))?;
    let shape = if matches!(root, ObjectRecord::New { .. }) {
        Ok(())
    } else {
        Err(invalid(format!(
            "root {root:?}; expected a new moPart_c object"
        )))
    };
    reader.checked(shape)
}

fn concept(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    merged_features(reader, map)?;
    reader.object(
        "visual_properties",
        map,
        ModelObjectRole::Declared(&["moVisualProperties_c"]),
    )?;
    reader.object("units", map, ModelObjectRole::Declared(&["moUnitsTable_c"]))?;
    reader.required_object(
        "unit_component",
        map,
        ModelObjectRole::Declared(&["moUnitComponent_c"]),
    )
}

fn merged_features(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let merged = reader.i32("has_merged_feature_list")?;
    if merged != 0 {
        for index in 0..reader.object_count("merged_feature_count")? {
            reader.object(
                &format!("merged_features[{index}]"),
                map,
                ModelObjectRole::Any,
            )?;
        }
        reader.i32("merged_feature_unknown_tail")?;
    }
    Ok(())
}

fn continuation(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<i32> {
    reader.scope("face_properties", |reader| {
        lightweight_shared::face_appearances(reader, map)
    })?;
    reader.scope("spatial", |reader| lightweight_shared::spatial(reader, map))?;
    reader.scope("metadata", |reader| {
        lightweight_shared::metadata(reader, map)
    })?;
    reader.scope("manager", |reader| lightweight_shared::manager(reader, map))?;
    reader.scope("postmanager", lightweight_post::array)?;
    reader.scope("model_flags", lightweight_post::flags)?;
    reader.scope("collections", |reader| {
        lightweight_post::collections(reader, map)
    })?;
    reader.scope("model_tail", |reader| lightweight_post::tail(reader, map))?;
    reader.scope("part_appearances", |reader| {
        lightweight_part::appearances(reader, map)
    })?;
    reader.scope("part_tail", lightweight_part::tail)
}
