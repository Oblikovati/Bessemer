use super::archive_decode::DisplayDecoder;
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.object_array("bodies", &["uoTempBodyTessData_c"])?;
    model_visual_properties(archive)?;
    archive.u32("ddata_time_stamp")?;
    paired_arrays(archive)?;
    super::modern_decals::part(archive)?;
    part_configuration(archive)
}

fn model_visual_properties(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0x10ae {
        return super::modern_body_tess::single_visual_property(
            archive,
            "model_viz_props[0].value",
        );
    }
    let count = archive.count("model_viz_props.count", 6)?;
    for index in 0..count {
        archive.scope(&format!("model_viz_props[{index}]"), |archive| {
            archive.i32("key")?;
            archive.object("value", &["moVisualProperties_c"])
        })?;
    }
    Ok(())
}

fn part_configuration(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() >= 0x10f0 {
        archive.i32("unknown_168")?;
    }
    if archive.context.version.value() > 0x12fc {
        archive.i32("is_tessed_for_exploded_view")?;
    }
    if archive.context.version.value() > 0x2b96 {
        archive.i32("config_id")?;
        archive.text("config_name")?;
    }
    part_graphics(archive)
}

fn part_graphics(archive: &mut DisplayDecoder<'_>) -> Result<()> {
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
    if archive.context.version.value() > 0x2f5d {
        archive.scope(
            "ref_geom_feature_data_collector",
            super::modern_reference::read,
        )?;
    }
    if archive.context.version.value() > 0x3751 {
        archive.u8("show_c_thread")?;
    }
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
