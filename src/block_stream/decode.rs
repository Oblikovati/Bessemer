use flate2::{Decompress, FlushDecompress, Status};

use super::{BLOCK_LIMIT, CHUNK_GUID, ChunkFraming, CompressedChunk, failure, invalid_chunk};
use crate::records::{dword, region};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn chunks(
    bytes: &[u8],
    framing: ChunkFraming,
    maximum: usize,
) -> Result<Vec<CompressedChunk>> {
    require_nonempty(bytes, "stream")?;
    let mut parsed = Vec::new();
    let (mut cursor, mut available) = (0, maximum);
    while cursor < bytes.len() {
        let (chunk, consumed) = decode_chunk(&bytes[cursor..], framing, &mut available)
            .map_err(|error| locate_failure(error, parsed.len(), cursor))?;
        parsed.push(chunk);
        cursor += consumed;
    }
    Ok(parsed)
}

fn locate_failure(mut error: BessemerError, index: usize, cursor: usize) -> BessemerError {
    error.entity = format!("chunk {index} at byte {cursor}");
    error.operation = "decode compressed-block stream";
    error
}

fn decode_chunk(
    bytes: &[u8],
    framing: ChunkFraming,
    available: &mut usize,
) -> Result<(CompressedChunk, usize)> {
    let (body, bound) = chunk_bounds(bytes, framing)?;
    validate_guid(region(bytes, body, 16)?)?;
    let (payload, end) = inflate_blocks(&bytes[..bound], body + 16, available)?;
    validate_boundary(end, bound, framing)?;
    let chunk = CompressedChunk {
        original: bytes[..end].to_vec(),
        payload,
        replacement: None,
    };
    Ok((chunk, end))
}

fn validate_guid(guid: &[u8]) -> Result<()> {
    if guid == CHUNK_GUID {
        return Ok(());
    }
    Err(invalid_chunk(format!(
        "GUID {guid:02x?}; expected {CHUNK_GUID:02x?}"
    )))
}

fn validate_boundary(end: usize, bound: usize, framing: ChunkFraming) -> Result<()> {
    if framing == ChunkFraming::GuidOnly || end == bound {
        return Ok(());
    }
    Err(invalid_chunk(format!(
        "terminator end {end}; expected chunk boundary {bound}"
    )))
}

fn chunk_bounds(bytes: &[u8], framing: ChunkFraming) -> Result<(usize, usize)> {
    if framing == ChunkFraming::GuidOnly {
        return Ok((0, bytes.len()));
    }
    let size = dword(bytes, 0)? as usize;
    region(bytes, 4, size)?;
    Ok((4, 4 + size))
}

fn inflate_blocks(
    bytes: &[u8],
    mut cursor: usize,
    available: &mut usize,
) -> Result<(Vec<u8>, usize)> {
    let mut payload = Vec::new();
    loop {
        let expanded = dword(bytes, cursor)? as usize;
        let stored = dword(bytes, cursor + 4)? as usize;
        cursor += 8;
        if expanded == 0 && stored == 0 {
            break;
        }
        validate_sizes(expanded, stored, *available)?;
        payload.extend_from_slice(&inflate_block(region(bytes, cursor, stored)?, expanded)?);
        *available -= expanded;
        cursor += stored;
    }
    require_nonempty(&payload, "chunk payload")?;
    Ok((payload, cursor))
}

fn require_nonempty(bytes: &[u8], entity: &str) -> Result<()> {
    if !bytes.is_empty() {
        return Ok(());
    }
    Err(invalid_chunk(format!(
        "{entity} length 0; expected nonempty content"
    )))
}

fn validate_sizes(expanded: usize, stored: usize, available: usize) -> Result<()> {
    if expanded == 0 || expanded > BLOCK_LIMIT || stored == 0 {
        return Err(invalid_chunk(format!(
            "block sizes {expanded}/{stored}; expected expanded 1..={BLOCK_LIMIT}, stored > 0"
        )));
    }
    if expanded > available {
        return Err(failure(
            0,
            FailureClass::Unsupported,
            format!("expanded block size {expanded}; expected <= remaining budget {available}"),
        ));
    }
    Ok(())
}

fn inflate_block(stored: &[u8], expected: usize) -> Result<Vec<u8>> {
    let mut decoder = Decompress::new(true);
    let mut expanded = vec![0; expected + 1];
    let status = decoder
        .decompress(stored, &mut expanded, FlushDecompress::Finish)
        .map_err(zlib_failure)?;
    validate_zlib_result(&decoder, status, stored.len(), expected)?;
    expanded.truncate(expected);
    Ok(expanded)
}

fn zlib_failure(error: flate2::DecompressError) -> BessemerError {
    failure(
        0,
        FailureClass::Integrity,
        format!("zlib error {error}; expected complete RFC 1950 stream"),
    )
}

fn validate_zlib_result(
    decoder: &Decompress,
    status: Status,
    stored: usize,
    expected: usize,
) -> Result<()> {
    if status == Status::StreamEnd
        && decoder.total_in() == stored as u64
        && decoder.total_out() == expected as u64
    {
        return Ok(());
    }
    Err(failure(
        0,
        FailureClass::Integrity,
        format!(
            "zlib {status:?}, input {}, output {}; expected StreamEnd, input {stored}, output {expected}",
            decoder.total_in(),
            decoder.total_out()
        ),
    ))
}
