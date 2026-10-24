use super::{
    archive_decode::DisplayDecoder,
    archive_error::{invalid, unsupported},
};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
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
