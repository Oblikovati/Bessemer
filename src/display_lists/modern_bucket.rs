use super::{archive_decode::DisplayDecoder, modern_items::point, modern_scene::wall_count};
use crate::Result;

pub(super) fn view(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.scope("bucket", bucket)?;
    if archive.i32("has_xform")? != 0 {
        archive.transform("xform")?;
    }
    if archive.i32("has_ghost_dl_xform")? != 0 {
        archive.transform("ghost_dl_xform")?;
    }
    view_flags(archive)?;
    let count = wall_count(archive, "sub_buckets.count", 2)?;
    for index in 0..count {
        archive.object(&format!("sub_buckets[{index}]"), &["uiViewBucket_c"])?;
    }
    archive.object("visual_properties", &["moVisualProperties_c"])?;
    if archive.i32("has_original_view_bucket_in_detailing_mode")? != 0 {
        archive.scope("original_view_bucket_in_detailing_mode", bucket)?;
    }
    archive.i32("unknown_178")?;
    Ok(())
}

fn view_flags(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("display_mode")?;
    for index in 0..4 {
        archive.i32(&format!("colors[{index}]"))?;
    }
    archive.i32s(&["unknown_68", "flags"])
}

fn bucket(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    point(archive, "normal")?;
    archive.i32s(&["unknown_6f8", "persp_mode"])?;
    archive.f64("cross_hatch_scale")?;
    archive.i32("tess_number")?;
    let two_dimensional = archive.i32("has_2d_info")? != 0;
    if two_dimensional {
        archive.transform("xform_2d")?;
    }
    poly_lists(archive, two_dimensional)?;
    archive.scope("dl_bucket", super::modern_items::list)?;
    archive.scope("dl_ghost_bucket", super::modern_items::list)?;
    archive.i32("show_hidden_count")?;
    component_map(archive)?;
    thread_maps(archive)?;
    tail(archive)
}

fn poly_lists(archive: &mut DisplayDecoder<'_>, two_dimensional: bool) -> Result<()> {
    for label in [
        "silh",
        "silh_h",
        "wire",
        "hidden",
        "hatch",
        "aux",
        "tan_v",
        "tan_h",
        "explode_v",
        "explode_h",
        "weldbead_v",
        "weldbead_h",
        "solid_fill",
    ] {
        poly_list(archive, label, two_dimensional)?;
    }
    Ok(())
}

fn poly_list(archive: &mut DisplayDecoder<'_>, label: &str, two_dimensional: bool) -> Result<()> {
    archive.scope(label, |archive| {
        super::modern_bucket_poly::list(archive, two_dimensional)
    })
}

fn component_map(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let word = archive.u16("component_str_map.count_word")?;
    let count = if word == u16::MAX {
        archive.u32("component_str_map.count_dword")?
    } else {
        u32::from(word)
    };
    if count as usize > archive.cursor.remaining() / 5 {
        return Err(super::archive_error::invalid(format!(
            "component string map count {count}; expected entries fitting {} bytes",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count as usize, "bucket component strings")?;
    for index in 0..count {
        archive.i32(&format!("component_str_map[{index}].key"))?;
        archive.text(&format!("component_str_map[{index}].value"))?;
    }
    Ok(())
}

fn thread_maps(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    for index in 0..4 {
        archive.scope(&format!("cthread_maps[{index}]"), thread_entries)?;
    }
    Ok(())
}

fn thread_entries(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = wall_count(archive, "count", 6)?;
    for index in 0..count {
        archive.scope(&format!("entries[{index}]"), thread_entry)?;
    }
    Ok(())
}

fn thread_entry(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.cobject("key")?;
    // gcBucket restores the enclosing 2D state before its thread maps; modern roots start in 3D.
    poly_list(archive, "value", false)
}

fn tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("cross_hatch_trimmed")?;
    poly_list(archive, "halo_repair", false)?;
    archive.i32("surf_cut_body")?;
    let count = wall_count(archive, "hatch_bucket_set_info.count", 28)?;
    for index in 0..count {
        hatch_info(archive, index)?;
    }
    archive.i32("bucket_data_for_detailing_mode_saved")?;
    archive.scope("dl_model_bucket", super::modern_items::list)?;
    archive.i32("needs_post_process_bucket_for_detailing_mode")?;
    Ok(())
}

fn hatch_info(archive: &mut DisplayDecoder<'_>, index: usize) -> Result<()> {
    for component in 0..7 {
        archive.i32(&format!("hatch_bucket_set_info[{index}][{component}]"))?;
    }
    Ok(())
}
