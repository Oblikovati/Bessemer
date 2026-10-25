use super::archive_decode::DisplayDecoder;
use crate::{Result, archive_codec::ArchiveString};

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<std::sync::Arc<ArchiveString>> {
    let component_id = identity(archive)?;
    archive.text("ref_configuration")?;
    archive.i32("num_user_comp_bodies")?;
    archive.u32("model_update_stamp")?;
    archive.u32("model_id")?;
    archive.text("model_name")?;
    archive.text("path_name")?;
    component_zone_and_atoms(archive)?;
    display_mode(archive)?;
    if archive.context.version.value() >= 0xb9b {
        archive.f64("unknown_190")?;
    }
    reference(archive)?;
    Ok(component_id)
}

pub(super) fn identity(archive: &mut DisplayDecoder<'_>) -> Result<std::sync::Arc<ArchiveString>> {
    let component_id = archive.text_value("comp_id_string")?;
    archive.transform("comp_xform")?;
    archive.bounds("comp_bbox")?;
    archive.object("comp_viz_prop", &["moVisualProperties_c"])?;
    Ok(component_id)
}

fn display_mode(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0xaa4 || archive.i32("display_mode_data[0]")? == 6 {
        return Ok(());
    }
    archive.i32s(&["display_mode_data[1]", "display_mode_data[2]"])
}

fn reference(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0xdca {
        archive.i32("visible")?;
    }
    for (threshold, label) in [(0x12d0, "suppressed"), (0x12d3, "comp_type")] {
        if archive.context.version.value() > threshold {
            archive.i32(label)?;
        }
    }
    if archive.context.version.value() > 0xdde {
        archive.i32("ref_configuration_id")?;
        archive.u32("file_type")?;
    }
    reference_state(archive)
}

fn reference_state(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0xde2 {
        archive.i32s(&["has_pattern_ref", "unknown_1a8"])?;
    }
    if archive.context.version.value() > 0xe11 {
        archive.i32("reference_id")?;
    }
    if archive.context.version.value() > 0x10ae {
        archive.i32("display_state")?;
    }
    if archive.context.version.value() > 0x110d {
        archive.i32("is_virtual_comp")?;
    }
    reference_labels(archive)
}

fn reference_labels(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x12c3 {
        for label in ["comp_ref", "spool_ref", "comp_description"] {
            archive.text(label)?;
        }
    }
    if archive.context.version.value() > 0x279c {
        archive.text("config_description")?;
    }
    dependencies(archive)
}

fn dependencies(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x12df {
        strings(archive, "depended_comp_names")?;
    }
    if archive.context.version.value() > 0x12e2 {
        equation_dependencies(archive)?;
    }
    folder(archive)
}

fn equation_dependencies(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.count("equ_dep_comp_names.count", 5)?;
    for index in 0..count {
        archive.scope(&format!("equ_dep_comp_names[{index}]"), |archive| {
            archive.text("key")?;
            strings(archive, "values")
        })?;
    }
    Ok(())
}

fn folder(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x12e8 {
        archive.i32("folder_comp")?;
    }
    historical_component_state(archive)?;
    if archive.context.version.value() > 0x23e4 {
        archive.i32s(&["grouped_folder_comp", "grouped_folder_type"])?;
    }
    fixed_and_flexible(archive)
}

fn historical_component_state(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    for (threshold, label) in [
        (0x17f3, "view_comp_type"),
        (0x17f5, "asm_feat_type"),
        (0x183d, "belt_has_part"),
    ] {
        if archive.context.version.value() > threshold {
            archive.i32(label)?;
        }
    }
    Ok(())
}

fn fixed_and_flexible(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0x279d {
        return geometry_and_state(archive);
    }
    archive.i32("fixed")?;
    archive.u16("solve_status")?;
    if archive.context.version.value() > 0x27b1 {
        archive.i32("temp_fixed")?;
    }
    transient_viewer(archive)?;
    flexible_state(archive)
}

fn transient_viewer(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if (0x27b7..0x27e3).contains(&archive.context.version.value()) {
        archive.object("unknown_obj", &["moViewerComponentData_c"])?;
    }
    Ok(())
}

fn flexible_state(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x27c1 {
        archive.i32("flexible")?;
    }
    if archive.context.version.value() > 0x2b88 && archive.i32("has_graphics_connect_ref_mgr")? != 0
    {
        archive.object(
            "graphics_connect_ref_mgr_data",
            &["moGraphicsConnectRefMgrData_c"],
        )?;
    }
    geometry_and_state(archive)
}

fn geometry_and_state(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0x2b9b {
        return Ok(());
    }
    archive.text("seed_reference_id")?;
    if archive.context.version.value() > 0x2f44 {
        archive.scope("ref_geom_data_collector", super::modern_reference::read)?;
    }
    tessellation_numbers(archive)?;
    display_state(archive)
}

fn display_state(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x3b1c {
        archive.text("display_state_name")?;
    }
    if archive.context.version.value() > 0x42e7 {
        archive.i32("routing_assembly")?;
    }
    if archive.context.version.value() >= 0x42f3 {
        archive.u8("top_level_speed_pak")?;
    }
    Ok(())
}

fn tessellation_numbers(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0x3b1b {
        return Ok(());
    }
    if archive.context.version.value() < 0x3b23 {
        return fixed_tessellation_numbers(archive);
    }
    let count = archive.count("viewer_comp_tess_numbers.count", 12)?;
    for index in 0..count {
        for member in 0..3 {
            archive.i32(&format!("viewer_comp_tess_numbers[{index}][{member}]"))?;
        }
    }
    Ok(())
}

fn fixed_tessellation_numbers(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    // #28: the old ID is -1 in memory; only the other two tuple words are archived.
    archive.entries(1, "fixed viewer tessellation numbers")?;
    archive.i32s(&[
        "viewer_comp_tess_numbers[0][1]",
        "viewer_comp_tess_numbers[0][2]",
    ])
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

fn component_zone_and_atoms(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x768 {
        archive.i32("is_a_zone")?;
    }
    if archive.context.version.value() > 0x864 {
        integers(archive, "gobbled_up_atom_ids")?;
    }
    Ok(())
}
