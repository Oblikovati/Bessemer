use super::{
    archive_decode::DisplayDecoder,
    archive_error::{invalid, unsupported},
};
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
    archive.i32s(&["unknown_168", "is_tessed_for_exploded_view", "config_id"])?;
    archive.text("config_name")?;
    if archive.i32("has_graphics_connect_ref_mgr")? != 0 {
        archive.object(
            "graphics_connect_ref_mgr_data",
            &["moGraphicsConnectRefMgrData_c"],
        )?;
    }
    archive.bounds("bbox")?;
    archive.scope("ref_geom_feature_data_collector", reference_geometry)?;
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

pub(super) fn reference_geometry(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let high = archive.u32("count_high")?;
    let low = archive.u32("count_low")?;
    let count = (u64::from(high) << 32) | u64::from(low);
    if count > archive.cursor.remaining() as u64 / 4 {
        return Err(invalid(format!(
            "reference geometry count {count}; expected entries fitting remaining bytes"
        )));
    }
    archive.entries(count as usize, "reference geometry")?;
    for index in 0..count {
        archive.scope(&format!("entries[{index}]"), reference_item)?;
    }
    Ok(())
}

fn reference_item(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let kind = archive.i32("type")?;
    if kind != 0 && kind != 3 {
        return Err(unsupported(format!(
            "reference geometry kind {kind}; expected implemented origin or plane body"
        )));
    }
    archive.text("unknown_8")?;
    archive.i32s(&["unknown_10", "unknown_14", "unknown_18"])?;
    if kind == 0 {
        return Ok(());
    }
    reference_plane(archive)
}

fn reference_plane(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    for label in ["unknown_38", "unknown_50"] {
        for index in 0..3 {
            archive.f64(&format!("{label}[{index}]"))?;
        }
    }
    archive.fixed_transform("unknown_68")?;
    for index in 0..4 {
        archive.f64(&format!("unknown_f0[{index}]"))?;
    }
    archive.i32s(&["unknown_110", "unknown_114"])
}
