use super::*;
use crate::archive_codec::ArchiveCursor;

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowCountPrefixOptions,
) -> Result<FlowContextCounts> {
    validate_options(options)?;
    let stored =
        super::super::payload_codec::record_payload(source, id, 406, "Flow context counts")?;
    let decoded = expand(stored, options)?;
    let (version, counts) = prefix(&decoded)?;
    let coverage = coverage(stored.len(), decoded.len(), options.compression);
    Ok(FlowContextCounts {
        source: source.clone(),
        record: id.clone(),
        decoded: decoded.into(),
        version,
        counts,
        options,
        coverage,
    })
}

fn expand(stored: &[u8], options: FlowCountPrefixOptions) -> Result<Vec<u8>> {
    if options.compression == FlowCountTransport::Zlib {
        return super::super::payload_codec::inflate_zlib(
            stored,
            options.maximum_decoded_bytes,
            "Flow context counts",
        );
    }
    if stored.len() <= options.maximum_decoded_bytes {
        return Ok(stored.to_vec());
    }
    Err(count_failure(
        FailureClass::Invalid,
        format!(
            "raw payload {} bytes; expected at most {} decoded bytes",
            stored.len(),
            options.maximum_decoded_bytes
        ),
    ))
}

fn prefix(decoded: &[u8]) -> Result<(i32, GlobalCountPair)> {
    let mut cursor = prefix_cursor(decoded)?;
    let version = cursor.i32()?;
    if version < 1 {
        return Err(count_failure(
            FailureClass::Unsupported,
            format!("version {version}; expected positive signed-i32 count-prefix version"),
        ));
    }
    Ok((
        version,
        GlobalCountPair::read(&mut cursor, "Flow context counts")?,
    ))
}

fn coverage(
    stored: usize,
    decoded: usize,
    compression: FlowCountTransport,
) -> FlowCountPrefixCoverage {
    let mode = match compression {
        FlowCountTransport::Raw => FlowCountPrefixCheckMode::ExactRawPrefixOpaqueBody,
        FlowCountTransport::Zlib => FlowCountPrefixCheckMode::ExactZlibPrefixOpaqueBody,
    };
    FlowCountPrefixCoverage {
        mode,
        stored_bytes: stored,
        decoded_bytes: decoded,
        known_prefix_bytes: 12,
        opaque_body_bytes: decoded - 12,
        achieved_prefix_error: 0,
    }
}

fn validate_options(options: FlowCountPrefixOptions) -> Result<()> {
    if options.maximum_decoded_bytes < 12 {
        return Err(count_failure(
            FailureClass::Invalid,
            format!(
                "decoded byte limit {}; expected at least twelve",
                options.maximum_decoded_bytes
            ),
        ));
    }
    Ok(())
}

fn prefix_cursor(decoded: &[u8]) -> Result<ArchiveCursor<'_>> {
    if decoded.len() < 12 {
        return Err(count_failure(
            FailureClass::Invalid,
            format!(
                "decoded payload {} bytes; expected at least twelve count-prefix bytes",
                decoded.len()
            ),
        ));
    }
    Ok(ArchiveCursor::new(decoded))
}
