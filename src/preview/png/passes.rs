use super::{PngHeader, PngLimits, invalid};
use crate::Result;

const ADAM7: [(usize, usize, usize, usize); 7] = [
    (0, 0, 8, 8),
    (4, 0, 8, 8),
    (0, 4, 4, 8),
    (2, 0, 4, 4),
    (0, 2, 2, 4),
    (1, 0, 2, 2),
    (0, 1, 1, 2),
];

#[derive(Clone, Copy)]
pub(super) struct Pass {
    pub x: usize,
    pub y: usize,
    pub dx: usize,
    pub dy: usize,
    pub width: usize,
    pub height: usize,
}

pub(super) fn image_passes(header: PngHeader) -> Vec<Pass> {
    let patterns: &[(usize, usize, usize, usize)] = if header.interlaced {
        &ADAM7
    } else {
        &[(0, 0, 1, 1)]
    };
    patterns
        .iter()
        .copied()
        .map(|(x, y, dx, dy)| Pass {
            x,
            y,
            dx,
            dy,
            width: extent(header.width as usize, x, dx),
            height: extent(header.height as usize, y, dy),
        })
        .filter(|pass| pass.width != 0 && pass.height != 0)
        .collect()
}

fn extent(size: usize, offset: usize, step: usize) -> usize {
    size.saturating_sub(offset).div_ceil(step)
}

pub(super) fn row_bytes(header: PngHeader, width: usize) -> Result<usize> {
    width
        .checked_mul(header.color.channels())
        .and_then(|samples| samples.checked_mul(header.bit_depth as usize))
        .map(|bits| bits.div_ceil(8))
        .ok_or_else(|| {
            invalid(format!(
                "PNG row width {width} at {} bits; expected addressable row bytes",
                header.bit_depth
            ))
        })
}

pub(super) fn expanded_size(header: PngHeader, limits: PngLimits) -> Result<usize> {
    let mut total = 0usize;
    for pass in image_passes(header) {
        let size = row_bytes(header, pass.width)?
            .checked_add(1)
            .and_then(|row| row.checked_mul(pass.height));
        total = size
            .and_then(|size| total.checked_add(size))
            .ok_or_else(|| invalid("PNG scanline size overflow; expected usize"))?;
    }
    if total > limits.maximum_inflated_bytes || total >= isize::MAX as usize {
        return Err(invalid(format!(
            "PNG expanded scanlines {total}; expected at most {} and below isize::MAX",
            limits.maximum_inflated_bytes
        )));
    }
    Ok(total)
}
