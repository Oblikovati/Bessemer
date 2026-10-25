use super::{
    archive_decode::DisplayDecoder,
    modern_items::{point, points, words},
};
use crate::Result;

pub(super) fn table(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&["unknown_0c", "unknown_08"])?;
    if archive.i32("ole_list_absent")? == 0 {
        let count = archive.u32("ole_object_num_list.count")?;
        words(archive, "ole_object_num_list", count as usize)?;
    }
    archive.text("table_name")?;
    if archive.context.version.value() > 0x1c56 {
        archive.i32("unknown_10")?;
    }
    if archive.context.version.value() > 0x238c {
        archive.i32("always_2d")?;
    }
    if archive.context.version.value() > 0x3332 {
        archive.text("view_names_3d_with_scale")?;
    }
    Ok(())
}

pub(super) fn property(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("unknown_18")?;
    point(archive, "unknown_00")?;
    archive.i32s(&["unknown_1c", "unknown_20"])
}

pub(super) fn start(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&["unknown_00", "unknown_04"])?;
    if archive.context.version.value() <= 0x1c49 {
        return Ok(());
    }
    let minimum = annotation_minimum(archive.context.version.value());
    let count = archive.count("edw_annot_data.count", minimum)?;
    for index in 0..count {
        archive.scope(&format!("edw_annot_data[{index}]"), annotation)?;
    }
    if archive.context.version.value() >= 0x35e9 {
        archive.text("id_string")?;
    }
    Ok(())
}

fn annotation(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("unknown_0")?;
    let count = archive.count("unknown_ints.count", 4)?;
    for index in 0..count {
        archive.i32(&format!("unknown_ints[{index}]"))?;
    }
    let count = archive.count("unknown_points.count", 24)?;
    points(archive, "unknown_points", count)?;
    strings(archive, "unknown_strings1")?;
    annotation_suffix(archive)
}

fn annotation_suffix(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() > 0x1fa8 {
        archive.i32("unknown_i")?;
    }
    if archive.context.version.value() > 0x1fd4 {
        strings(archive, "unknown_strings2")?;
    }
    if archive.context.version.value() > 0x23fe {
        archive.text("unknown_s3")?;
    }
    if archive.context.version.value() > 0x332b {
        archive.text("unknown_s4")?;
        archive.text("unknown_s5")?;
    }
    Ok(())
}

fn annotation_minimum(version: u32) -> usize {
    16 + 4 * usize::from(version > 0x1fa8)
        + 4 * usize::from(version > 0x1fd4)
        + usize::from(version > 0x23fe)
        + 2 * usize::from(version > 0x332b)
}

fn strings(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    let count = archive.count(&format!("{label}.count"), 1)?;
    for index in 0..count {
        archive.text(&format!("{label}[{index}]"))?;
    }
    Ok(())
}
