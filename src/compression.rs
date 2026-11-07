use std::io::{Read, Write};

use flate2::{Compression, Crc, read::DeflateDecoder, write::DeflateEncoder};

use crate::archive::Entry;
use crate::{BessemerError, FailureClass, Result};

pub(crate) fn expand(stored: &[u8], entry: &Entry) -> Result<Vec<u8>> {
    let expanded = match entry.method {
        0 => copy_stored(stored, entry)?,
        _ => inflate(stored, entry)?,
    };
    let crc = checksum(&expanded);
    if expanded.len() != entry.expanded as usize || crc != entry.crc {
        return Err(codec_error(
            "extract",
            &entry.name,
            format!(
                "size {}, CRC {crc:08x}; expected size {}, CRC {:08x}",
                expanded.len(),
                entry.expanded,
                entry.crc
            ),
        ));
    }
    Ok(expanded)
}

fn copy_stored(stored: &[u8], entry: &Entry) -> Result<Vec<u8>> {
    if stored.len() == entry.expanded as usize {
        return Ok(stored.to_vec());
    }
    Err(codec_error(
        "extract",
        &entry.name,
        format!("stored size {}; expected {}", stored.len(), entry.expanded),
    ))
}

fn inflate(stored: &[u8], entry: &Entry) -> Result<Vec<u8>> {
    let mut decoder = DeflateDecoder::new(stored);
    let mut expanded = Vec::new();
    decoder
        .by_ref()
        .take(u64::from(entry.expanded) + 1)
        .read_to_end(&mut expanded)
        .map_err(|error| {
            codec_error(
                "extract",
                &entry.name,
                format!("deflate error {error}; expected valid raw deflate"),
            )
        })?;
    if decoder.total_in() != stored.len() as u64 {
        return Err(codec_error(
            "extract",
            &entry.name,
            format!(
                "consumed {} of {} bytes; expected complete deflate consumption",
                decoder.total_in(),
                stored.len()
            ),
        ));
    }
    Ok(expanded)
}

pub(crate) fn deflate(name: &str, replacement: &[u8]) -> Result<Vec<u8>> {
    let failure = |error: std::io::Error| {
        codec_error(
            "compress",
            name,
            format!("compression error {error}; expected encodable raw deflate"),
        )
    };
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(replacement).map_err(failure)?;
    encoder.finish().map_err(failure)
}

pub(crate) fn checksum(bytes: &[u8]) -> u32 {
    let mut crc = Crc::new();
    crc.update(bytes);
    crc.sum()
}

fn codec_error(operation: &'static str, name: &str, cause: String) -> BessemerError {
    BessemerError::new(operation, name, FailureClass::Integrity, cause)
}
