use super::archive_decode::DisplayDecoder;
use crate::{Result, archive_codec::ArchiveString};

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<std::sync::Arc<ArchiveString>> {
    let component_id = archive.text_value("comp_id_string")?;
    archive.transform("comp_xform")?;
    archive.bounds("comp_bbox")?;
    archive.object("comp_viz_prop", &["moVisualProperties_c"])?;
    archive.text("ref_configuration")?;
    archive.i32("num_user_comp_bodies")?;
    archive.u32("model_update_stamp")?;
    archive.u32("model_id")?;
    archive.text("model_name")?;
    archive.text("path_name")?;
    archive.i32("is_a_zone")?;
    integers(archive, "gobbled_up_atom_ids")?;
    display_mode(archive)?;
    archive.f64("unknown_190")?;
    reference(archive)?;
    Ok(component_id)
}

fn display_mode(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.i32("display_mode_data[0]")? == 6 {
        return Ok(());
    }
    archive.i32s(&["display_mode_data[1]", "display_mode_data[2]"])
}

fn reference(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&["visible", "suppressed", "comp_type", "ref_configuration_id"])?;
    archive.u32("file_type")?;
    archive.i32s(&[
        "has_pattern_ref",
        "unknown_1a8",
        "reference_id",
        "display_state",
        "is_virtual_comp",
    ])?;
    for label in [
        "comp_ref",
        "spool_ref",
        "comp_description",
        "config_description",
    ] {
        archive.text(label)?;
    }
    dependencies(archive)
}

fn dependencies(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    strings(archive, "depended_comp_names")?;
    let count = archive.count("equ_dep_comp_names.count", 5)?;
    for index in 0..count {
        archive.scope(&format!("equ_dep_comp_names[{index}]"), |archive| {
            archive.text("key")?;
            strings(archive, "values")
        })?;
    }
    folder(archive)
}

fn folder(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&[
        "folder_comp",
        "view_comp_type",
        "asm_feat_type",
        "belt_has_part",
        "grouped_folder_comp",
        "grouped_folder_type",
        "fixed",
    ])?;
    archive.u16("solve_status")?;
    archive.i32s(&["temp_fixed", "flexible"])?;
    if archive.i32("has_graphics_connect_ref_mgr")? != 0 {
        archive.object(
            "graphics_connect_ref_mgr_data",
            &["moGraphicsConnectRefMgrData_c"],
        )?;
    }
    geometry_and_state(archive)
}

fn geometry_and_state(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("seed_reference_id")?;
    archive.scope("ref_geom_data_collector", super::modern_reference::read)?;
    tessellation_numbers(archive)?;
    archive.text("display_state_name")?;
    archive.i32("routing_assembly")?;
    archive.u8("top_level_speed_pak")?;
    Ok(())
}

fn tessellation_numbers(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.count("viewer_comp_tess_numbers.count", 12)?;
    for index in 0..count {
        for member in 0..3 {
            archive.i32(&format!("viewer_comp_tess_numbers[{index}][{member}]"))?;
        }
    }
    Ok(())
}

pub(super) fn integers(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    let count = archive.count(&format!("{label}.count"), 4)?;
    for index in 0..count {
        archive.i32(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

fn strings(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    let count = archive.count(&format!("{label}.count"), 1)?;
    for index in 0..count {
        archive.text(&format!("{label}[{index}]"))?;
    }
    Ok(())
}
