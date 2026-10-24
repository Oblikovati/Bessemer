use super::{integrity, invalid};
use crate::Result;
use flate2::{Compression, Decompress, FlushDecompress, Status, write::ZlibEncoder};
use std::io::Write;

pub(super) fn inflate_exact(bytes: &[u8], expected: usize) -> Result<Vec<u8>> {
    crate::block_stream::decode::inflate_block(bytes, expected)
        .map_err(|error| integrity(error.cause))
}

pub(super) fn inflate_bounded(bytes: &[u8], maximum: usize) -> Result<Vec<u8>> {
    let mut decoder = Decompress::new(true);
    let mut output = Vec::new();
    while !inflate_step(&mut decoder, bytes, maximum, &mut output)? {}
    Ok(output)
}

fn inflate_step(
    decoder: &mut Decompress,
    bytes: &[u8],
    maximum: usize,
    output: &mut Vec<u8>,
) -> Result<bool> {
    let before = decoder.total_in();
    let available = maximum.saturating_sub(output.len());
    let mut buffer = [0; 8192];
    let size = buffer.len().min(available.saturating_add(1));
    let (status, produced) =
        decompress_window(decoder, &bytes[before as usize..], &mut buffer[..size])?;
    if produced > available {
        return Err(invalid(format!(
            "ancillary expansion exceeds {maximum}; expected within remaining budget"
        )));
    }
    output.extend_from_slice(&buffer[..produced]);
    require_progress(decoder, status, before, produced, bytes.len())
}

fn decompress_window(
    decoder: &mut Decompress,
    bytes: &[u8],
    output: &mut [u8],
) -> Result<(Status, usize)> {
    let before = decoder.total_out();
    let status = decoder
        .decompress(bytes, output, FlushDecompress::None)
        .map_err(|error| {
            integrity(format!(
                "PNG ancillary zlib error {error}; expected complete RFC 1950 stream"
            ))
        })?;
    Ok((status, (decoder.total_out() - before) as usize))
}

fn require_progress(
    decoder: &Decompress,
    status: Status,
    before: u64,
    produced: usize,
    size: usize,
) -> Result<bool> {
    if status == Status::StreamEnd {
        require_complete(decoder, size)?;
        return Ok(true);
    }
    if produced == 0 && before == decoder.total_in() {
        return Err(integrity(format!(
            "ancillary zlib {status:?} consumed {before}/{size}; expected StreamEnd"
        )));
    }
    Ok(false)
}

fn require_complete(decoder: &Decompress, size: usize) -> Result<()> {
    if decoder.total_in() == size as u64 {
        return Ok(());
    }
    Err(integrity(format!(
        "ancillary zlib consumed {} bytes; expected exactly {size}",
        decoder.total_in()
    )))
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
