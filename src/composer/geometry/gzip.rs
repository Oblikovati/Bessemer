use super::*;
use crate::{compression, records};
use flate2::{Compression, write::DeflateEncoder};
use std::io::Write;

pub(super) const DEFAULT_HEADER: [u8; 10] = [0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 2, 255];

pub(super) fn decode_gzip(
    bytes: &[u8],
    limits: ComposerSceneLimits,
) -> Result<([u8; 10], Vec<u8>)> {
    let header = validate_header(bytes, limits)?;
    let footer = bytes.len() - 8;
    let expected_crc = records::dword(bytes, footer)?;
    let expected_size = records::dword(bytes, footer + 4)? as usize;
    budget("decoded bytes", expected_size, limits.maximum_decoded_bytes)?;
    let decoded = super::super::payload::inflate_exact(
        &bytes[10..footer],
        expected_size,
        "Composer geometry gzip",
    )?;
    verify_crc(&decoded, expected_crc)?;
    Ok((header, decoded))
}

fn validate_header(bytes: &[u8], limits: ComposerSceneLimits) -> Result<[u8; 10]> {
    budget("encoded bytes", bytes.len(), limits.maximum_encoded_bytes)?;
    if bytes.len() < 18 || bytes[..4] != DEFAULT_HEADER[..4] {
        return Err(unsupported(format!(
            "gzip prefix {:?}, length {}; expected 1f8b0800 and at least 18 bytes",
            bytes.get(..4),
            bytes.len()
        )));
    }
    let mut header = DEFAULT_HEADER;
    header.copy_from_slice(&bytes[..10]);
    Ok(header)
}

fn verify_crc(bytes: &[u8], expected: u32) -> Result<()> {
    let actual = compression::checksum(bytes);
    if actual == expected {
        return Ok(());
    }
    Err(BessemerError::new(
        "Composer geometry stream",
        "gzip CRC",
        FailureClass::Integrity,
        format!("CRC 0x{actual:08x}; expected 0x{expected:08x}"),
    ))
}

pub(super) fn encode_gzip(
    decoded: &[u8],
    header: [u8; 10],
    limits: ComposerSceneLimits,
) -> Result<Vec<u8>> {
    let size = u32::try_from(decoded.len())
        .map_err(|_| invalid(format!("gzip size {}; expected u32", decoded.len())))?;
    let compressed = compress_bounded(decoded, limits.maximum_encoded_bytes)?;
    let mut bytes = header.to_vec();
    bytes.extend_from_slice(&compressed);
    bytes.extend_from_slice(&compression::checksum(decoded).to_le_bytes());
    bytes.extend_from_slice(&size.to_le_bytes());
    Ok(bytes)
}

fn compress_bounded(decoded: &[u8], maximum: usize) -> Result<Vec<u8>> {
    budget("minimum gzip bytes", 18, maximum)?;
    let output = super::gzip_output::GzipOutput::new(maximum - 18);
    let mut encoder = DeflateEncoder::new(output, Compression::best());
    encoder.write_all(decoded).map_err(gzip_error)?;
    Ok(encoder.finish().map_err(gzip_error)?.bytes)
}

fn gzip_error(error: std::io::Error) -> BessemerError {
    if let Some(cause) = error
        .get_ref()
        .and_then(|cause| cause.downcast_ref::<BessemerError>())
    {
        return cause.clone();
    }
    BessemerError::new(
        "Composer geometry stream",
        "gzip construction",
        FailureClass::Integrity,
        format!("compression error {error}; expected complete raw deflate output"),
    )
}
