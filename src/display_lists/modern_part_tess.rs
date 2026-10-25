use super::{archive_decode::DisplayDecoder, archive_error::unsupported};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.object_array("bodies", &["uoTempBodyTessData_c"])?;
    let count = archive.count("model_viz_props.count", 6)?;
    for index in 0..count {
        archive.scope(&format!("model_viz_props[{index}]"), |archive| {
            archive.i32("key")?;
            archive.object("value", &["moVisualProperties_c"])
        })?;
    }
    archive.u32("ddata_time_stamp")?;
    paired_arrays(archive)?;
    empty_decals(archive)?;
    part_configuration(archive)
}

fn part_configuration(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&["unknown_168", "is_tessed_for_exploded_view"])?;
    if archive.context.version.value() > 0x2b96 {
        archive.i32("config_id")?;
        archive.text("config_name")?;
    }
    if archive.context.version.value() > 0x2b9d && archive.i32("has_graphics_connect_ref_mgr")? != 0
    {
        archive.object(
            "graphics_connect_ref_mgr_data",
            &["moGraphicsConnectRefMgrData_c"],
        )?;
    }
    part_geometry(archive)
}

fn part_geometry(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0x2ba3 {
        return Ok(());
    }
    archive.bounds("bbox")?;
    archive.scope(
        "ref_geom_feature_data_collector",
        super::modern_reference::read,
    )?;
    archive.u8("show_c_thread")?;
    Ok(())
}

fn paired_arrays(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.count("unknown_30.count", 8)?;
    archive.entries(count, "unknown_48")?;
    for label in ["unknown_30", "unknown_48"] {
        for index in 0..count {
            archive.i32(&format!("{label}[{index}]"))?;
        }
    }
    Ok(())
}

fn empty_decals(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.u32("decals_map.count")?;
    archive.entries(count as usize, "decal maps")?;
    if count != 0 {
        return Err(unsupported(format!(
            "decal map count {count}; populated decal bodies are pending implementation"
        )));
    }
    Ok(())
}
