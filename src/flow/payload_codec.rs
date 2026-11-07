//! Shared transport keeps schema-specific operation labels and exact carrier identities.
use super::{FlowCarrier, FlowRecordId};
use crate::{BessemerError, FailureClass, Result};
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

pub(super) fn record_payload<'a>(
    source: &'a FlowCarrier,
    id: &FlowRecordId,
    identifier: u32,
    operation: &'static str,
) -> Result<&'a [u8]> {
    let span = super::edit::selected_span(source, id)?;
    if span.identifier != identifier {
        return Err(BessemerError::new(
            operation,
            "record",
            FailureClass::Invalid,
            format!("identifier {}; expected {identifier}", span.identifier),
        ));
    }
    Ok(&source.as_bytes()[span.start + 12..span.end])
}

pub(super) fn inflate_zlib(
    bytes: &[u8],
    maximum: usize,
    operation: &'static str,
) -> Result<Vec<u8>> {
    crate::bounded_zlib::inflate(bytes, maximum).map_err(|error| inflate_failure(error, operation))
}

pub(super) fn encode_zlib(bytes: &[u8], operation: &'static str) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder
        .write_all(bytes)
        .map_err(|error| encode_failure(error, operation))?;
    encoder
        .finish()
        .map_err(|error| encode_failure(error, operation))
}

fn inflate_failure(
    error: crate::bounded_zlib::BoundedZlibError,
    operation: &'static str,
) -> BessemerError {
    let class = if matches!(
        error,
        crate::bounded_zlib::BoundedZlibError::LimitExceeded { .. }
    ) {
        FailureClass::Invalid
    } else {
        FailureClass::Integrity
    };
    BessemerError::new(
        operation,
        "zlib stream",
        class,
        format!(
            "{error:?}; expected one complete checksum-validated RFC 1950 stream within the configured output bound"
        ),
    )
}

fn encode_failure(error: std::io::Error, operation: &'static str) -> BessemerError {
    BessemerError::new(
        operation,
        "zlib encoding",
        FailureClass::Integrity,
        format!("{error}; expected complete RFC 1950 encoding"),
    )
}
