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
    archive.i32s(&["unknown_10", "always_2d"])?;
    archive.text("view_names_3d_with_scale")
}

pub(super) fn property(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("unknown_18")?;
    point(archive, "unknown_00")?;
    archive.i32s(&["unknown_1c", "unknown_20"])
}

pub(super) fn start(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32s(&["unknown_00", "unknown_04"])?;
    let count = archive.count("edw_annot_data.count", 27)?;
    for index in 0..count {
        archive.scope(&format!("edw_annot_data[{index}]"), annotation)?;
    }
    archive.text("id_string")
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
    archive.i32("unknown_i")?;
    strings(archive, "unknown_strings2")?;
    archive.text("unknown_s3")?;
    archive.text("unknown_s4")?;
    archive.text("unknown_s5")
}

fn strings(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    let count = archive.count(&format!("{label}.count"), 1)?;
    for index in 0..count {
        archive.text(&format!("{label}[{index}]"))?;
    }
    Ok(())
}
