use super::{archive_decode::DisplayDecoder, modern_scene::wall_count};
use crate::Result;

pub(super) fn view(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.scope("bucket", bucket)?;
    if archive.i32("has_xform")? != 0 {
        archive.transform("xform")?;
    }
    if archive.context.version.value() > 0x509 && archive.i32("has_ghost_dl_xform")? != 0 {
        archive.transform("ghost_dl_xform")?;
    }
    view_flags(archive)?;
    view_sub_buckets(archive)?;
    super::legacy_bucket_visibility::view_triples(archive)?;
    if archive.context.version.value() > 0xc90 {
        archive.object("visual_properties", &["moVisualProperties_c"])?;
    }
    view_tail(archive)
}

fn view_sub_buckets(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = wall_count(archive, "sub_buckets.count", 2)?;
    for index in 0..count {
        archive.object(&format!("sub_buckets[{index}]"), &["uiViewBucket_c"])?;
    }
    Ok(())
}

fn view_tail(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0x3341 {
        return Ok(());
    }
    if archive.i32("has_original_view_bucket_in_detailing_mode")? != 0 {
        archive.scope("original_view_bucket_in_detailing_mode", bucket)?;
    }
    if archive.context.version.value() >= 0x374b {
        archive.i32("unknown_178")?;
    }
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
    crate::archive_codec::bucket_envelope::read(archive, &mut ())
}
