use super::{
    DibContents, PaletteEntry, header, invalid, rle,
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
    let pixel_bytes = cursor.take(header.image_size as usize)?.to_vec();
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
    if contents.pixel_bytes.len() != contents.header.image_size as usize {
        return Err(invalid(format!(
            "image byte length {}; expected declared {}",
            contents.pixel_bytes.len(),
            contents.header.image_size
        )));
    }
    match shape.encoding {
        DibEncoding::Rle8 => rle::walk_rle(&contents.pixel_bytes, shape, palette, None)?,
        DibEncoding::Bgrx32 => require_bgrx_size(contents, shape)?,
    }
    stream_size(contents)?;
    Ok(shape)
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

fn require_bgrx_size(contents: &DibContents, shape: ImageShape) -> Result<()> {
    let expected = shape.count.checked_mul(4).ok_or_else(|| {
        invalid(format!(
            "BGRX count {}; expected byte size in usize",
            shape.count
        ))
    })?;
    if contents.pixel_bytes.len() == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "BGRX bytes {}; expected {expected}",
        contents.pixel_bytes.len()
    )))
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
