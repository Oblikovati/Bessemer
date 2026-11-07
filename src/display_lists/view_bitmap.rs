use super::{ViewBitmap, output::DisplayOutput, view_support::invalid};
use crate::{Result, archive_codec::ArchiveCursor};

pub(super) fn byte_count(height: i32, row_bytes: i32) -> Result<usize> {
    let length = i64::from(height) * i64::from(row_bytes);
    if !(0..=64 * 1024 * 1024).contains(&length) {
        return Err(invalid(format!(
            "3D bitmap height {height} times row bytes {row_bytes} = {length}; expected 0..=67108864 bytes"
        )));
    }
    Ok(length as usize)
}

pub(super) fn read(cursor: &mut ArchiveCursor<'_>) -> Result<ViewBitmap> {
    let width = cursor.i32()?;
    let height = cursor.i32()?;
    let row_bytes = cursor.i32()?;
    let planes = cursor.u16()?;
    let bits_per_pixel = cursor.u16()?;
    let length = byte_count(height, row_bytes)?;
    let bits = cursor.take(length)?.to_vec();
    Ok(ViewBitmap {
        width,
        height,
        row_bytes,
        planes,
        bits_per_pixel,
        bits,
    })
}

pub(super) fn write(output: &mut DisplayOutput, bitmap: &ViewBitmap) -> Result<()> {
    let expected = byte_count(bitmap.height, bitmap.row_bytes)?;
    if bitmap.bits.len() != expected {
        return Err(invalid(format!(
            "3D bitmap length {}; expected {expected} from height {} and row bytes {}",
            bitmap.bits.len(),
            bitmap.height,
            bitmap.row_bytes
        )));
    }
    output.i32(bitmap.width)?;
    output.i32(bitmap.height)?;
    output.i32(bitmap.row_bytes)?;
    output.u16(bitmap.planes)?;
    output.u16(bitmap.bits_per_pixel)?;
    output.bytes(&bitmap.bits)
}
