use super::{PngChunk, PngColor, PngContents, PngHeader, PngLimits, invalid, unsupported};
use crate::Result;

pub(super) fn parse_header(contents: &PngContents, limits: PngLimits) -> Result<PngHeader> {
    let first = contents
        .chunks
        .first()
        .ok_or_else(|| invalid("chunk count 0; expected IHDR, IDAT and IEND"))?;
    let header = parse_ihdr(first)?;
    require_header(header, limits)?;
    Ok(header)
}

fn parse_ihdr(first: &PngChunk) -> Result<PngHeader> {
    if first.kind.bytes() != *b"IHDR" || first.payload.len() != 13 {
        return Err(invalid(format!(
            "first chunk {:?} length {}; expected IHDR length 13",
            first.kind.bytes(),
            first.payload.len()
        )));
    }
    let bytes = &first.payload;
    let header = PngHeader {
        width: be32(&bytes[..4]),
        height: be32(&bytes[4..8]),
        bit_depth: bytes[8],
        color: PngColor::from_wire(bytes[9])?,
        interlaced: bytes[12] == 1,
    };
    require_methods(bytes)?;
    Ok(header)
}

fn require_methods(bytes: &[u8]) -> Result<()> {
    if bytes[10] != 0 || bytes[11] != 0 || bytes[12] > 1 {
        return Err(unsupported(format!(
            "PNG methods {:?}; expected compression 0, filter 0, interlace 0 or 1",
            &bytes[10..]
        )));
    }
    Ok(())
}

pub(super) fn require_header(header: PngHeader, limits: PngLimits) -> Result<()> {
    if header.width == 0
        || header.height == 0
        || header.width > i32::MAX as u32
        || header.height > i32::MAX as u32
    {
        return Err(invalid(format!(
            "PNG dimensions {}x{}; expected each in 1..=2147483647",
            header.width, header.height
        )));
    }
    require_depth(header)?;
    require_pixel_budget(header, limits)?;
    sample_count(header)?;
    Ok(())
}

fn require_depth(header: PngHeader) -> Result<()> {
    let valid = match header.color {
        PngColor::Grayscale => [1, 2, 4, 8, 16].contains(&header.bit_depth),
        PngColor::Indexed => [1, 2, 4, 8].contains(&header.bit_depth),
        _ => [8, 16].contains(&header.bit_depth),
    };
    if !valid {
        return Err(invalid(format!(
            "PNG depth {} for {:?}; expected a PNG color/depth combination",
            header.bit_depth, header.color
        )));
    }
    Ok(())
}

fn require_pixel_budget(header: PngHeader, limits: PngLimits) -> Result<()> {
    let count = pixel_count(header)?;
    if count > isize::MAX as usize / 8 {
        return Err(invalid(format!(
            "PNG pixel count {count}; expected addressable RGBA16 allocation below {}",
            isize::MAX as usize / 8
        )));
    }
    if count > limits.maximum_pixels {
        return Err(invalid(format!(
            "PNG pixel count {count}; expected at most {}",
            limits.maximum_pixels
        )));
    }
    Ok(())
}

pub(super) fn pixel_count(header: PngHeader) -> Result<usize> {
    (header.width as usize)
        .checked_mul(header.height as usize)
        .ok_or_else(|| {
            invalid(format!(
                "PNG dimensions {}x{}; expected pixel count in usize",
                header.width, header.height
            ))
        })
}

pub(super) fn sample_count(header: PngHeader) -> Result<usize> {
    let count = pixel_count(header)?
        .checked_mul(header.color.channels())
        .filter(|count| *count <= isize::MAX as usize / 2);
    count.ok_or_else(|| {
        invalid(format!(
            "PNG sample allocation for {}x{} {:?}; expected addressable u16 samples",
            header.width, header.height, header.color
        ))
    })
}

pub(super) fn be32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
pub(super) fn be16(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}
