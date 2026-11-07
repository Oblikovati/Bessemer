use super::{ChunkType, PngChunk, PngContents, PngLimits, integrity, invalid};
use crate::{Result, archive_codec::ArchiveCursor, compression::checksum};

pub(super) const SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

pub(super) fn decode(bytes: &[u8], limits: PngLimits) -> Result<PngContents> {
    let mut cursor = ArchiveCursor::new(bytes);
    if cursor.take(8)? != SIGNATURE {
        return Err(invalid(
            "PNG signature mismatch; expected 89 50 4e 47 0d 0a 1a 0a",
        ));
    }
    let mut chunks = Vec::new();
    while cursor.remaining() != 0 {
        if chunks.len() == limits.maximum_chunks {
            return Err(invalid(format!(
                "chunk count {}; expected at most {}",
                chunks.len() + 1,
                limits.maximum_chunks
            )));
        }
        chunks.push(read_chunk(&mut cursor)?);
    }
    Ok(PngContents { chunks })
}

fn read_chunk(cursor: &mut ArchiveCursor<'_>) -> Result<PngChunk> {
    let length = read_be(cursor)?;
    if length > i32::MAX as u32 {
        return Err(invalid(format!(
            "chunk length {length}; expected 0..=2147483647"
        )));
    }
    let framed = cursor.take(length as usize + 4)?;
    let kind = ChunkType::new([framed[0], framed[1], framed[2], framed[3]])?;
    let crc = read_be(cursor)?;
    let expected = checksum(framed);
    if crc != expected {
        return Err(integrity(format!(
            "chunk {:?} CRC {crc:#x}; expected {expected:#x}",
            kind.bytes()
        )));
    }
    Ok(PngChunk {
        kind,
        payload: framed[4..].to_vec(),
    })
}

fn read_be(cursor: &mut ArchiveCursor<'_>) -> Result<u32> {
    let bytes = cursor.take(4)?;
    Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub(super) fn encode(contents: &PngContents) -> Result<Vec<u8>> {
    let mut output = SIGNATURE.to_vec();
    for chunk in &contents.chunks {
        append_chunk(&mut output, chunk)?;
    }
    Ok(output)
}

fn append_chunk(output: &mut Vec<u8>, chunk: &PngChunk) -> Result<()> {
    let length = u32::try_from(chunk.payload.len()).map_err(|_| {
        invalid(format!(
            "chunk length {}; expected u32",
            chunk.payload.len()
        ))
    })?;
    output.extend_from_slice(&length.to_be_bytes());
    let start = output.len();
    output.extend_from_slice(&chunk.kind.bytes());
    output.extend_from_slice(&chunk.payload);
    let crc = checksum(&output[start..]);
    output.extend_from_slice(&crc.to_be_bytes());
    Ok(())
}
