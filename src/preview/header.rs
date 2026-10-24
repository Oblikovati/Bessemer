use super::types::{DibEncoding, ImageShape};
use super::{BitmapInfoHeader, invalid, unsupported};
use crate::{Result, archive_codec::ArchiveCursor};

pub(super) fn read_header(cursor: &mut ArchiveCursor<'_>) -> Result<BitmapInfoHeader> {
    Ok(BitmapInfoHeader {
        header_size: cursor.u32()?,
        width: cursor.i32()?,
        height: cursor.i32()?,
        planes: cursor.u16()?,
        bit_count: cursor.u16()?,
        compression: cursor.u32()?,
        image_size: cursor.u32()?,
        x_pixels_per_metre: cursor.i32()?,
        y_pixels_per_metre: cursor.i32()?,
        colors_used: cursor.u32()?,
        colors_important: cursor.u32()?,
    })
}

pub(super) fn write_header(header: &BitmapInfoHeader, output: &mut Vec<u8>) {
    for word in [
        header.header_size,
        header.width as u32,
        header.height as u32,
    ] {
        output.extend_from_slice(&word.to_le_bytes());
    }
    output.extend_from_slice(&header.planes.to_le_bytes());
    output.extend_from_slice(&header.bit_count.to_le_bytes());
    for word in [
        header.compression,
        header.image_size,
        header.x_pixels_per_metre as u32,
        header.y_pixels_per_metre as u32,
        header.colors_used,
        header.colors_important,
    ] {
        output.extend_from_slice(&word.to_le_bytes());
    }
}

pub(super) fn image_shape(header: &BitmapInfoHeader, maximum_pixels: usize) -> Result<ImageShape> {
    let encoding = encoding(header)?;
    require_dimensions(header, encoding)?;
    let width = header.width as usize;
    let height = header.height.unsigned_abs() as usize;
    let count = pixel_count(width, height, maximum_pixels)?;
    Ok(ImageShape {
        width,
        height,
        count,
        top_down: header.height < 0,
        encoding,
    })
}

fn require_dimensions(header: &BitmapInfoHeader, encoding: DibEncoding) -> Result<()> {
    if header.width <= 0 || header.height == 0 || header.height == i32::MIN {
        return Err(invalid(format!(
            "dimensions {}x{}; expected positive width and nonzero representable height",
            header.width, header.height
        )));
    }
    if encoding == DibEncoding::Rle8 && header.height < 0 {
        return Err(invalid(format!(
            "RLE8 height {}; expected positive bottom-up height",
            header.height
        )));
    }
    Ok(())
}

fn pixel_count(width: usize, height: usize, maximum_pixels: usize) -> Result<usize> {
    let count = width.checked_mul(height).ok_or_else(|| {
        invalid(format!(
            "dimensions {width}x{height}; expected usize pixel count"
        ))
    })?;
    if count > maximum_pixels {
        return Err(invalid(format!(
            "pixel count {count}; expected at most {maximum_pixels}"
        )));
    }
    Ok(count)
}

fn encoding(header: &BitmapInfoHeader) -> Result<DibEncoding> {
    if header.header_size != 40 {
        return Err(unsupported(format!(
            "DIB header size {}; expected BITMAPINFOHEADER size 40",
            header.header_size
        )));
    }
    if header.planes != 1 {
        return Err(invalid(format!("DIB planes {}; expected 1", header.planes)));
    }
    match (header.bit_count, header.compression) {
        (8, 1) => Ok(DibEncoding::Rle8),
        (32, 0) => Ok(DibEncoding::Bgrx32),
        other => Err(unsupported(format!(
            "DIB bit count/compression {other:?}; expected (8, BI_RLE8=1) or (32, BI_RGB=0)"
        ))),
    }
}

pub(super) fn palette_count(header: &BitmapInfoHeader, shape: ImageShape) -> Result<usize> {
    if shape.encoding == DibEncoding::Bgrx32 {
        return Ok(header.colors_used as usize);
    }
    if header.colors_used > 256 {
        return Err(invalid(format!(
            "RLE8 palette count {}; expected 0..=256",
            header.colors_used
        )));
    }
    Ok(if header.colors_used == 0 {
        256
    } else {
        header.colors_used as usize
    })
}
