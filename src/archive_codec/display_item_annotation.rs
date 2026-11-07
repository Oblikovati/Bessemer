use super::display_items::{point, points, words};
use super::item_reader::ItemRead;
use crate::Result;

pub(crate) fn table(archive: &mut impl ItemRead) -> Result<()> {
    if archive.version() > 0x10b2 {
        archive.i32s(&["unknown_0c", "unknown_08"])?;
    }
    if archive.version() > 0x1115 {
        table_ole_numbers(archive)?;
    }
    archive.text("table_name")?;
    if archive.version() > 0x1c56 {
        archive.i32("unknown_10")?;
    }
    if archive.version() > 0x238c {
        archive.i32("always_2d")?;
    }
    if archive.version() > 0x3332 {
        archive.text("view_names_3d_with_scale")?;
    }
    Ok(())
}

fn table_ole_numbers(archive: &mut impl ItemRead) -> Result<()> {
    if archive.i32("ole_list_absent")? != 0 {
        return Ok(());
    }
    let count = archive.u32("ole_object_num_list.count")?;
    words(archive, "ole_object_num_list", count as usize)
}

pub(crate) fn property(archive: &mut impl ItemRead) -> Result<()> {
    archive.i32("unknown_18")?;
    point(archive, "unknown_00")?;
    archive.i32s(&["unknown_1c", "unknown_20"])
}

pub(crate) fn start(archive: &mut impl ItemRead) -> Result<()> {
    archive.i32s(&["unknown_00", "unknown_04"])?;
    annotation_old_ids(archive)?;
    if archive.version() <= 0x1c49 {
        return Ok(());
    }
    let minimum = annotation_minimum(archive.version());
    let count = archive.item_count("edw_annot_data.count", minimum)?;
    for index in 0..count {
        archive.scope(&format!("edw_annot_data[{index}]"), annotation)?;
    }
    if archive.version() >= 0x35e9 {
        archive.text("id_string")?;
    }
    Ok(())
}

fn annotation_old_ids(archive: &mut impl ItemRead) -> Result<()> {
    if !(0x1c16..0x1c4a).contains(&archive.version()) {
        return Ok(());
    }
    let count = archive.item_count("unknown_old_ids.count", 4)?;
    for index in 0..count {
        archive.i32(&format!("unknown_old_ids[{index}]"))?;
    }
    Ok(())
}

fn annotation(archive: &mut impl ItemRead) -> Result<()> {
    archive.i32("unknown_0")?;
    let count = archive.item_count("unknown_ints.count", 4)?;
    for index in 0..count {
        archive.i32(&format!("unknown_ints[{index}]"))?;
    }
    let count = archive.item_count("unknown_points.count", 24)?;
    points(archive, "unknown_points", count)?;
    if archive.version() > 0x1c4b {
        strings(archive, "unknown_strings1")?;
    }
    annotation_suffix(archive)
}

fn annotation_suffix(archive: &mut impl ItemRead) -> Result<()> {
    if archive.version() > 0x1fa8 {
        archive.i32("unknown_i")?;
    }
    if archive.version() > 0x1fd4 {
        strings(archive, "unknown_strings2")?;
    }
    if archive.version() > 0x23fe {
        archive.text("unknown_s3")?;
    }
    if archive.version() > 0x332b {
        archive.text("unknown_s4")?;
        archive.text("unknown_s5")?;
    }
    Ok(())
}

fn annotation_minimum(version: u32) -> usize {
    12 + 4 * usize::from(version > 0x1c4b)
        + 4 * usize::from(version > 0x1fa8)
        + 4 * usize::from(version > 0x1fd4)
        + usize::from(version > 0x23fe)
        + 2 * usize::from(version > 0x332b)
}

fn strings(archive: &mut impl ItemRead, label: &str) -> Result<()> {
    let count = archive.item_count(&format!("{label}.count"), 1)?;
    for index in 0..count {
        archive.text(&format!("{label}[{index}]"))?;
    }
    Ok(())
}
