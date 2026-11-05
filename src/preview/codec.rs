use super::{
    BitmapInfoHeader, DibContents, PaletteEntry, header, indexed_rgb, invalid, rle, rows,
    types::{DibEncoding, ImageShape},
};
use crate::{Result, archive_codec::ArchiveCursor};

pub(super) fn decode(bytes: &[u8], maximum_pixels: usize) -> Result<DibContents> {
    let mut cursor = ArchiveCursor::new(bytes);
    require_stream_length(&mut cursor)?;
    let header = header::read_header(&mut cursor)?;
    let shape = header::image_shape(&header, maximum_pixels)?;
    let count = header::palette_count(&header, shape)?;
    let palette = read_palette(cursor.take(palette_size(count)?)?);
    let pixel_bytes = cursor.take(image_byte_count(&header, shape)?)?.to_vec();
    cursor.finish()?;
    let contents = DibContents {
        header,
        palette,
        pixel_bytes,
    };
    validate(&contents, maximum_pixels)?;
    Ok(contents)
}

fn require_stream_length(cursor: &mut ArchiveCursor<'_>) -> Result<()> {
    let size = cursor.u32()? as usize;
    if size == cursor.remaining() {
        return Ok(());
    }
    Err(invalid(format!(
        "DIB stream size {size}; expected {} bytes after prefix",
        cursor.remaining()
    )))
}

fn palette_size(count: usize) -> Result<usize> {
    count.checked_mul(4).ok_or_else(|| {
        invalid(format!(
            "palette count {count}; expected byte length in usize"
        ))
    })
}

fn read_palette(bytes: &[u8]) -> Vec<PaletteEntry> {
    bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|entry| PaletteEntry {
            blue: entry[0],
            green: entry[1],
            red: entry[2],
            reserved: entry[3],
        })
        .collect()
}

pub(super) fn validate(contents: &DibContents, maximum_pixels: usize) -> Result<ImageShape> {
    let shape = header::image_shape(&contents.header, maximum_pixels)?;
    let palette = header::palette_count(&contents.header, shape)?;
    require_palette_length(contents, palette)?;
    require_image_byte_length(contents, image_byte_count(&contents.header, shape)?)?;
    match shape.encoding {
        DibEncoding::Rle8 => rle::walk_rle(&contents.pixel_bytes, shape, palette, None)?,
        DibEncoding::Bgr555 | DibEncoding::Bgr24 | DibEncoding::Bgrx32 => {
            require_rgb_size(contents, shape)?
        }
        DibEncoding::IndexedRgb(bits) => {
            require_rgb_size(contents, shape)?;
            indexed_rgb::validate_indices(&contents.pixel_bytes, shape, bits, palette)?;
        }
    }
    stream_size(contents)?;
    Ok(shape)
}

fn require_image_byte_length(contents: &DibContents, expected: usize) -> Result<()> {
    if contents.pixel_bytes.len() != expected {
        return Err(invalid(format!(
            "image byte length {}; expected {expected} for declared image size {}",
            contents.pixel_bytes.len(),
            contents.header.image_size
        )));
    }
    Ok(())
}

fn require_palette_length(contents: &DibContents, expected: usize) -> Result<()> {
    if contents.palette.len() == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "palette length {}; expected {expected}",
        contents.palette.len()
    )))
}

fn require_rgb_size(contents: &DibContents, shape: ImageShape) -> Result<()> {
    let expected = rows::byte_count(shape)?;
    if contents.pixel_bytes.len() == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "uncompressed image bytes {}; expected {expected}",
        contents.pixel_bytes.len()
    )))
}

fn image_byte_count(header: &BitmapInfoHeader, shape: ImageShape) -> Result<usize> {
    // BI_RGB permits an omitted size; dimensions still bound the exact stored extent (#208).
    if header.image_size == 0 && shape.encoding != DibEncoding::Rle8 {
        return rows::byte_count(shape);
    }
    Ok(header.image_size as usize)
}

fn stream_size(contents: &DibContents) -> Result<u32> {
    let size = 40u64 + 4 * contents.palette.len() as u64 + contents.pixel_bytes.len() as u64;
    u32::try_from(size).map_err(|_| invalid(format!("DIB body size {size}; expected u32")))
}

pub(super) fn encode(contents: &DibContents) -> Result<Vec<u8>> {
    let mut output = stream_size(contents)?.to_le_bytes().to_vec();
    header::write_header(&contents.header, &mut output);
    for entry in &contents.palette {
        output.extend_from_slice(&[entry.blue, entry.green, entry.red, entry.reserved]);
    }
    output.extend_from_slice(&contents.pixel_bytes);
    Ok(output)
}
