use super::{integrity, invalid};
use crate::Result;
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

pub(super) fn inflate_exact(bytes: &[u8], expected: usize) -> Result<Vec<u8>> {
    crate::block_stream::decode::inflate_block(bytes, expected)
        .map_err(|error| integrity(error.cause))
}

pub(super) fn inflate_bounded(bytes: &[u8], maximum: usize) -> Result<Vec<u8>> {
    crate::bounded_zlib::inflate(bytes, maximum).map_err(ancillary_inflate_error)
}

fn ancillary_inflate_error(error: crate::bounded_zlib::BoundedZlibError) -> crate::BessemerError {
    let over_budget = matches!(
        error,
        crate::bounded_zlib::BoundedZlibError::LimitExceeded { .. }
    );
    let cause = ancillary_zlib_cause(error);
    if over_budget {
        return invalid(cause);
    }
    integrity(cause)
}

fn ancillary_zlib_cause(error: crate::bounded_zlib::BoundedZlibError) -> String {
    use crate::bounded_zlib::BoundedZlibError::*;
    match error {
        LimitExceeded { maximum } => {
            format!("ancillary expansion exceeds {maximum}; expected within remaining budget")
        }
        Codec { cause } => {
            format!("PNG ancillary zlib error {cause}; expected complete RFC 1950 stream")
        }
        NoProgress {
            status,
            consumed,
            size,
        } => format!("ancillary zlib {status:?} consumed {consumed}/{size}; expected StreamEnd"),
        TrailingInput { consumed, size } => {
            format!("ancillary zlib consumed {consumed} bytes; expected exactly {size}")
        }
    }
}

pub(super) fn deflate(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(bytes).map_err(|error| {
        integrity(format!(
            "PNG zlib write {error}; expected complete image encoding"
        ))
    })?;
    encoder
        .finish()
        .map_err(|error| integrity(format!("PNG zlib finish {error}; expected complete stream")))
}
