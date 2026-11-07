use super::{
    invalid,
    types::{DibEncoding, ImageShape},
};
use crate::Result;

pub(super) fn row_stride(shape: ImageShape) -> Result<usize> {
    let (units, alignment, divisor) = match shape.encoding {
        DibEncoding::IndexedRgb(bits) => (bits as usize, 32, 8),
        DibEncoding::Bgr555 => (2, 4, 1),
        DibEncoding::Bgr24 => (3, 4, 1),
        DibEncoding::Bgrx32 => (4, 4, 1),
        DibEncoding::Rle8 => return Err(invalid("Rle8 row stride; expected uncompressed RGB")),
    };
    shape
        .width
        .checked_mul(units)
        .and_then(|count| count.checked_add(alignment - 1))
        .map(|count| (count & !(alignment - 1)) / divisor)
        .ok_or_else(|| {
            invalid(format!(
                "RGB width {} with encoding {:?}; expected aligned row byte size in usize",
                shape.width, shape.encoding
            ))
        })
}

pub(super) fn byte_count(shape: ImageShape) -> Result<usize> {
    let stride = row_stride(shape)?;
    stride.checked_mul(shape.height).ok_or_else(|| {
        invalid(format!(
            "RGB stride {stride} and height {}; expected stored byte size in usize",
            shape.height
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const FORMATS: &[(DibEncoding, u128)] = &[
        (DibEncoding::IndexedRgb(1), 1),
        (DibEncoding::IndexedRgb(4), 4),
        (DibEncoding::IndexedRgb(8), 8),
        (DibEncoding::Bgr555, 16),
        (DibEncoding::Bgr24, 24),
        (DibEncoding::Bgrx32, 32),
    ];

    fn shape(width: usize, height: usize, encoding: DibEncoding) -> ImageShape {
        ImageShape {
            width,
            height,
            count: width.checked_mul(height).unwrap_or(0),
            top_down: false,
            encoding,
        }
    }

    #[test]
    fn existing_rgb_strides_match_independent_closed_forms_at_every_boundary() {
        for &(encoding, bits) in FORMATS {
            for width in [1, 2, 3, 4, 7, 8, 9, 31, 32, 33, 255, 256, 257] {
                let expected = (width as u128 * bits).div_ceil(32) * 4;
                assert_eq!(
                    row_stride(shape(width, 2, encoding)).unwrap() as u128,
                    expected
                );
                assert_eq!(
                    byte_count(shape(width, 2, encoding)).unwrap() as u128,
                    expected * 2
                );
            }
        }
    }

    #[test]
    fn shared_layout_rejects_overflow_and_compressed_rows() {
        for &(encoding, _) in FORMATS {
            assert!(row_stride(shape(usize::MAX, 1, encoding)).is_err());
            assert!(byte_count(shape(1, usize::MAX, encoding)).is_err());
        }
        assert!(row_stride(shape(1, 1, DibEncoding::Rle8)).is_err());
        assert!(byte_count(shape(1, 1, DibEncoding::Rle8)).is_err());
    }
}
