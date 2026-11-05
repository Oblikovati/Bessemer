//! One bounded RFC 1950 stream, with caller-specific diagnostics kept at the boundary.
use flate2::{Decompress, FlushDecompress, Status};

#[derive(Debug)]
pub(crate) enum BoundedZlibError {
    LimitExceeded {
        maximum: usize,
    },
    Codec {
        cause: String,
    },
    NoProgress {
        status: Status,
        consumed: u64,
        size: usize,
    },
    TrailingInput {
        consumed: u64,
        size: usize,
    },
}

type InflateResult<T> = std::result::Result<T, BoundedZlibError>;

pub(crate) fn inflate(bytes: &[u8], maximum: usize) -> InflateResult<Vec<u8>> {
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
) -> InflateResult<bool> {
    let before = decoder.total_in();
    let available = maximum.saturating_sub(output.len());
    let mut buffer = [0; 8192];
    let size = buffer.len().min(available.saturating_add(1));
    let (status, produced) =
        decompress_window(decoder, &bytes[before as usize..], &mut buffer[..size])?;
    if produced > available {
        return Err(BoundedZlibError::LimitExceeded { maximum });
    }
    output.extend_from_slice(&buffer[..produced]);
    require_progress(decoder, status, before, produced, bytes.len())
}

fn decompress_window(
    decoder: &mut Decompress,
    bytes: &[u8],
    output: &mut [u8],
) -> InflateResult<(Status, usize)> {
    let before = decoder.total_out();
    let status = decoder
        .decompress(bytes, output, FlushDecompress::None)
        .map_err(|error| BoundedZlibError::Codec {
            cause: error.to_string(),
        })?;
    Ok((status, (decoder.total_out() - before) as usize))
}

fn require_progress(
    decoder: &Decompress,
    status: Status,
    before: u64,
    produced: usize,
    size: usize,
) -> InflateResult<bool> {
    if status == Status::StreamEnd {
        require_complete(decoder, size)?;
        return Ok(true);
    }
    if produced == 0 && before == decoder.total_in() {
        return Err(BoundedZlibError::NoProgress {
            status,
            consumed: before,
            size,
        });
    }
    Ok(false)
}

fn require_complete(decoder: &Decompress, size: usize) -> InflateResult<()> {
    if decoder.total_in() == size as u64 {
        return Ok(());
    }
    Err(BoundedZlibError::TrailingInput {
        consumed: decoder.total_in(),
        size,
    })
}
