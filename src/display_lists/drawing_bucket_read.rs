use super::{DisplayArchiveContents, DisplayArchiveContext, DisplayArchiveLimits};
use super::{
    archive_decode::DisplayDecoder,
    archive_error::{invalid, unsupported},
};
use crate::Result;

pub(super) fn read(
    bytes: &[u8],
    context: &DisplayArchiveContext,
    limits: DisplayArchiveLimits,
) -> Result<DisplayArchiveContents> {
    let version = context.version.value();
    if version <= 0x27cb {
        return Err(unsupported(format!(
            "drawing VBLists model version {version}; expected modern model version >=0x27CC"
        )));
    }
    let mut archive = DisplayDecoder::new(bytes, context.clone(), limits)?;
    archive.drawing_buckets = true;
    let count = archive.count("view_bucket_count", 2)?;
    for index in 0..count {
        archive.object(&format!("view_buckets[{index}]"), &["uiViewBucket_c"])?;
    }
    if archive.i32("shaded_data_present")? != 0 {
        shaded(&mut archive, count)?;
    }
    archive.finish()
}

fn shaded(archive: &mut DisplayDecoder<'_>, views: usize) -> Result<()> {
    for label in ["shaded_model_indices", "raw_words_20"] {
        archive.entries(views, label)?;
        for index in 0..views {
            archive.u32(&format!("{label}[{index}]"))?;
        }
    }
    for label in ["string_lists_28", "string_lists_68"] {
        lists(archive, label, views, strings)?;
    }
    let count = archive.count("model_count", 7)?;
    for index in 0..count {
        archive.scope(&format!("models[{index}]"), model)?;
    }
    trailing(archive, views)
}

fn model(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let part = archive.i32("is_part_word")?;
    if part != 0 && part != 1 {
        return Err(invalid(format!(
            "drawing model kind {part}; expected 0 (assembly) or 1 (part)"
        )));
    }
    archive.text("model_reference")?;
    archive.u16("raw_30")?;
    super::modern_root::old_lights(archive)?;
    archive.object("visual_properties", &["moVisualProperties_c"])?;
    archive.scope("tessellation", |archive| {
        if part == 1 {
            return super::modern_part_tess::read(archive);
        }
        archive.object_array("components", super::modern_assembly::ASSEMBLY_CLASSES)
    })
}

fn lists(
    archive: &mut DisplayDecoder<'_>,
    label: &str,
    views: usize,
    read: fn(&mut DisplayDecoder<'_>) -> Result<()>,
) -> Result<()> {
    archive.entries(views, label)?;
    for index in 0..views {
        archive.scope(&format!("{label}[{index}]"), read)?;
    }
    Ok(())
}

fn strings(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.count("count", 1)?;
    for index in 0..count {
        archive.text(&format!("strings[{index}]"))?;
    }
    Ok(())
}

fn words(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = archive.count("count", 4)?;
    for index in 0..count {
        archive.u32(&format!("words[{index}]"))?;
    }
    Ok(())
}

fn trailing(archive: &mut DisplayDecoder<'_>, views: usize) -> Result<()> {
    for label in ["lists_f8", "lists_118", "lists_48"] {
        lists(archive, label, views, words)?;
    }
    Ok(())
}
