use super::{invalid, pixels::stored_row, types::ImageShape};
use crate::Result;

pub(super) fn byte_count(shape: ImageShape, bits: u8) -> Result<usize> {
    let stride = row_stride(shape.width, bits)?;
    stride.checked_mul(shape.height).ok_or_else(|| {
        invalid(format!(
            "indexed RGB stride {stride} and height {}; expected byte size in usize",
            shape.height
        ))
    })
}

fn row_stride(width: usize, bits: u8) -> Result<usize> {
    width
        .checked_mul(bits as usize)
        .and_then(|count| count.checked_add(31))
        .map(|count| (count & !31) / 8)
        .ok_or_else(|| {
            invalid(format!(
                "indexed RGB width {width} at {bits} bits; expected aligned row size in usize"
            ))
        })
}

pub(super) fn validate_indices(
    bytes: &[u8],
    shape: ImageShape,
    bits: u8,
    palette: usize,
) -> Result<()> {
    let stride = row_stride(shape.width, bits)?;
    if let Some(index) = (0..shape.count)
        .map(|position| pixel_index(bytes, shape, bits, stride, position))
        .find(|index| *index as usize >= palette)
    {
        return Err(invalid(format!(
            "stored index {index}; expected below palette length {palette}"
        )));
    }
    Ok(())
}

pub(super) fn decode(bytes: &[u8], shape: ImageShape, bits: u8) -> Result<Vec<Option<u8>>> {
    let stride = row_stride(shape.width, bits)?;
    Ok((0..shape.count)
        .map(|position| Some(pixel_index(bytes, shape, bits, stride, position)))
        .collect())
}

pub(super) fn replace(
    source: &[u8],
    shape: ImageShape,
    bits: u8,
    indices: &[u8],
) -> Result<Vec<u8>> {
    let stride = row_stride(shape.width, bits)?;
    let mut bytes = source.to_vec();
    for (position, &value) in indices.iter().enumerate() {
        let (offset, shift) = location(shape, bits, stride, position);
        let mask = (u8::MAX >> (8 - bits)) << shift;
        bytes[offset] = (bytes[offset] & !mask) | (value << shift);
    }
    Ok(bytes)
}

fn pixel_index(bytes: &[u8], shape: ImageShape, bits: u8, stride: usize, position: usize) -> u8 {
    let (offset, shift) = location(shape, bits, stride, position);
    (bytes[offset] >> shift) & (u8::MAX >> (8 - bits))
}

fn location(shape: ImageShape, bits: u8, stride: usize, position: usize) -> (usize, u8) {
    let row = position / shape.width;
    let bit = (position % shape.width) * bits as usize;
    let offset = stored_row(row, shape) * stride + bit / 8;
    (offset, 8 - bits - (bit % 8) as u8)
}
