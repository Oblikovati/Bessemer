use std::io::Write;

use flate2::{Compression, write::ZlibEncoder};

use super::{BLOCK_LIMIT, CHUNK_GUID, ChunkFraming, CompressedChunk, failure};
use crate::{FailureClass, Result};

pub(super) fn append(
    output: &mut Vec<u8>,
    chunk: &CompressedChunk,
    framing: ChunkFraming,
    index: usize,
) -> Result<()> {
    let Some(replacement) = &chunk.replacement else {
        output.extend_from_slice(&chunk.original);
        return Ok(());
    };
    let mut body = CHUNK_GUID.to_vec();
    for block in replacement.chunks(BLOCK_LIMIT) {
        append_block(&mut body, block, index)?;
    }
    body.extend_from_slice(&[0; 8]);
    if framing == ChunkFraming::LengthPrefixed {
        let length = u32::try_from(body.len()).map_err(|_| {
            failure(
                index,
                FailureClass::Unsupported,
                format!("encoded size {}; expected <= u32::MAX", body.len()),
            )
        })?;
        output.extend_from_slice(&length.to_le_bytes());
    }
    output.extend_from_slice(&body);
    Ok(())
}

fn append_block(output: &mut Vec<u8>, block: &[u8], index: usize) -> Result<()> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    let failure = |error: std::io::Error| {
        failure(
            index,
            FailureClass::Integrity,
            format!("zlib encoding error {error}; expected encodable block"),
        )
    };
    encoder.write_all(block).map_err(failure)?;
    let encoded = encoder.finish().map_err(failure)?;
    output.extend_from_slice(&(block.len() as u32).to_le_bytes());
    output.extend_from_slice(&(encoded.len() as u32).to_le_bytes());
    output.extend_from_slice(&encoded);
    Ok(())
}
