use super::{ModelDecoder, ModelObjectRole, configuration_io as io};
use crate::archive_codec::{
    MaterialRead, ObjectReader, ObjectRecord, scene_data, scene_floor, scene_walls, vectors,
};
use crate::{BessemerError, FailureClass, Result};

const OLDER_SCENE_CLASSES: &[&str] = &[
    "swCubicalScene_c",
    "swSphericalScene_c",
    "pwSceneData_c",
    "moNodeName_c",
    "moVisualProperties_c",
];
const SCENE_CLASSES: &[&str] = &["swScene_c", "swCubicalScene_c", "swSphericalScene_c"];

pub(super) fn owned_map(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    reader.configuration_scene(|reader| {
        if reader.manager_dialect() == Some(super::manager_dialect::ManagerDialect::Owned11000) {
            return reader.scoped_fresh_objects(
                OLDER_SCENE_CLASSES,
                "version-11000 scene",
                |reader| owned_entries(reader, map, label),
            );
        }
        owned_entries(reader, map, label)
    })
}

fn owned_entries(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    for index in 0..io::positive_count(reader, &format!("{label}.objects_count"), 6)? {
        reader.object(
            &format!("{label}.objects[{index}].scene"),
            map,
            ModelObjectRole::Declared(SCENE_CLASSES),
        )?;
        reader.i32(&format!("{label}.objects[{index}].raw_value"))?;
    }
    for index in 0..io::positive_count(reader, &format!("{label}.associations_count"), 6)? {
        reader.i32(&format!("{label}.associations[{index}].raw_key"))?;
        reader.object(
            &format!("{label}.associations[{index}].scene"),
            map,
            ModelObjectRole::Declared(SCENE_CLASSES),
        )?;
    }
    Ok(())
}

pub(super) fn scene(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_configuration_scene()?;
    reader.scope("node", |reader| super::node::lightweight(reader, map))?;
    if reader.i32("merged_presence")? != 0 {
        reader.scope("merged_list", |reader| node_list(reader, map))?;
    }
    scene_visual(reader, map)?;
    reader.scope("feature_list", |reader| node_list(reader, map))?;
    reader.object(
        "scene_data",
        map,
        ModelObjectRole::Declared(&["pwSceneData_c"]),
    )?;
    for index in 0..scene_data::nonnegative_count(reader, "walls.count", 8)? {
        reader.scope(&format!("walls[{index}]"), |reader| {
            scene_walls::read(reader, map)
        })?;
    }
    bounds(reader)?;
    reader.i32("floor_orient")?;
    reader.transform("required_xform")?;
    mapped_pointer(reader, map, "floor_selection")?;
    scene_floor::read(reader)
}

pub(super) fn pw_data(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_configuration_scene()?;
    scene_data::read(reader)
}

fn scene_visual(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.manager_dialect() == Some(super::manager_dialect::ManagerDialect::Owned11000) {
        return reader.object(
            "visual_properties",
            map,
            ModelObjectRole::Declared(&["moVisualProperties_c"]),
        );
    }
    mapped_pointer(reader, map, "visual_properties")
}

fn node_list(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.object_count("nodes.count")? {
        mapped_pointer(reader, map, &format!("nodes[{index}]"))?;
    }
    reader.i32("raw_tail")?;
    Ok(())
}

fn mapped_pointer(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    let record = reader.object_tag(label, map, ModelObjectRole::Any)?;
    if let ObjectRecord::New { class, .. } = record {
        return reader.checked(Err(BessemerError::new(
            "configuration scene",
            label,
            FailureClass::Unsupported,
            format!(
                "fresh class {:?}; expected null or an already loaded object",
                class.name
            ),
        )));
    }
    Ok(())
}

/// Preserve ten native bounding-box scalars; e.g. `bounds(reader)?` for direct scene or section members.
pub(super) fn bounds(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for label in ["bbox.centre", "bbox.high", "bbox.low"] {
        vectors::double_vector(reader, label)?;
    }
    reader.f64("bbox.radius")
}
