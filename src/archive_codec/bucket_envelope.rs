use super::bucket_reader::BucketRead;
use crate::Result;

pub(crate) fn read<R: BucketRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    super::vectors::double_vector(archive, "normal")?;
    archive.i32s(&["unknown_6f8", "persp_mode"])?;
    archive.f64("cross_hatch_scale")?;
    archive.i32("tess_number")?;
    let two_dimensional = bucket_coordinate_scope(archive)?;
    poly_lists(archive, context, two_dimensional)?;
    archive.bucket_edges(context, two_dimensional)?;
    if archive.version() > 0x253 {
        archive.bucket_items(context, "dl_bucket")?;
    }
    ghost_and_hidden(archive, context)?;
    component_map(archive)?;
    thread_maps(archive, context)?;
    tail(archive, context)
}

const BUCKET_POLY_LABELS: [&str; 13] = [
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
];

fn poly_lists<R: BucketRead>(
    archive: &mut R,
    context: &mut R::ObjectContext,
    two_dimensional: bool,
) -> Result<()> {
    for label in BUCKET_POLY_LABELS {
        if archive.bucket_file_version() < 0x224 && matches!(label, "tan_v" | "tan_h") {
            continue;
        }
        if archive.version() <= 0x11a5 && matches!(label, "weldbead_v" | "weldbead_h") {
            continue;
        }
        if legacy_list_omitted(archive.version(), label) {
            continue;
        }
        archive.bucket_polygons(context, label, two_dimensional)?;
    }
    Ok(())
}

fn component_map(archive: &mut impl BucketRead) -> Result<()> {
    if archive.version() <= 0xa59 {
        return Ok(());
    }
    let word_keys = archive.version() <= 0x23d7;
    let count = component_count(archive, if word_keys { 3 } else { 5 })?;
    archive.entries(count as usize, "bucket component strings")?;
    for index in 0..count {
        component_key(archive, index, word_keys)?;
        archive.text(&format!("component_str_map[{index}].value"))?;
    }
    Ok(())
}

fn component_count(archive: &mut impl BucketRead, minimum: usize) -> Result<u32> {
    let word = archive.u16("component_str_map.count_word")?;
    let count = if word == u16::MAX {
        archive.u32("component_str_map.count_dword")?
    } else {
        u32::from(word)
    };
    if count as usize > archive.remaining() / minimum {
        return Err(archive.invalid(format!(
            "component string map count {count}; expected entries fitting {} bytes",
            archive.remaining()
        )));
    }
    Ok(count)
}

fn component_key(archive: &mut impl BucketRead, index: u32, word_keys: bool) -> Result<()> {
    let label = format!("component_str_map[{index}].key");
    if word_keys {
        archive.u16(&label)?;
    } else {
        archive.i32(&label)?;
    }
    Ok(())
}

fn thread_maps<R: BucketRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    if archive.version() <= 0xad2 {
        return Ok(());
    }
    for index in 0..4 {
        archive.scope(&format!("cthread_maps[{index}]"), |archive| {
            thread_entries(archive, context)
        })?;
    }
    Ok(())
}

fn thread_entries<R: BucketRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    let count = super::scene_data::nonnegative_count(archive, "count", 6)?;
    for index in 0..count {
        archive.scope(&format!("entries[{index}]"), |archive| {
            thread_entry(archive, context)
        })?;
    }
    Ok(())
}

fn thread_entry<R: BucketRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    archive.bucket_object(context, "key")?;
    // gcBucket restores the enclosing 2D state before its thread maps; modern roots start in 3D.
    archive.bucket_polygons(context, "value", false)
}

fn tail<R: BucketRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    if archive.version() > 0x147d {
        archive.i32("cross_hatch_trimmed")?;
    }
    if archive.version() > 0x184c {
        archive.bucket_polygons(context, "halo_repair", false)?;
    }
    if archive.version() > 0x1c3d {
        archive.i32("surf_cut_body")?;
    }
    if archive.version() <= 0x2789 {
        return Ok(());
    }
    let count = super::scene_data::nonnegative_count(archive, "hatch_bucket_set_info.count", 28)?;
    for index in 0..count {
        hatch_info(archive, index)?;
    }
    detailing_tail(archive, context)
}

fn detailing_tail<R: BucketRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    if archive.version() > 0x2f8b {
        archive.i32("bucket_data_for_detailing_mode_saved")?;
    }
    if archive.version() > 0x2faa {
        archive.bucket_items(context, "dl_model_bucket")?;
    }
    if archive.version() >= 0x372a {
        archive.i32("needs_post_process_bucket_for_detailing_mode")?;
    }
    Ok(())
}

fn hatch_info(archive: &mut impl BucketRead, index: usize) -> Result<()> {
    for component in 0..7 {
        archive.i32(&format!("hatch_bucket_set_info[{index}][{component}]"))?;
    }
    Ok(())
}

fn legacy_list_omitted(version: u32, label: &str) -> bool {
    (version <= 0x7f0 && label == "solid_fill")
        || (version <= 0x753 && matches!(label, "explode_v" | "explode_h"))
        || (version <= 0x741 && label == "silh_h")
}

fn ghost_and_hidden<R: BucketRead>(archive: &mut R, context: &mut R::ObjectContext) -> Result<()> {
    if archive.version() > 0x4bf {
        archive.bucket_items(context, "dl_ghost_bucket")?;
    }
    if archive.version() >= 0x4d4 {
        archive.i32("show_hidden_count")?;
    }
    Ok(())
}

fn bucket_coordinate_scope(archive: &mut impl BucketRead) -> Result<bool> {
    if archive.version() <= 0x49c {
        return Ok(false);
    }
    let two_dimensional = archive.i32("has_2d_info")? != 0;
    if two_dimensional {
        archive.graph_transform("xform_2d")?;
    }
    Ok(two_dimensional)
}
