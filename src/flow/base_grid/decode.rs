use super::*;
use crate::archive_codec::ArchiveCursor;

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowGridOptions,
) -> Result<FlowBaseGrid> {
    validate_options(options)?;
    let stored = super::super::payload_codec::record_payload(source, id, 101, "Flow base grid")?;
    let decoded = expand(stored, options)?;
    let dimensions = read_dimensions(&decoded)?;
    let words: u64 = dimensions.into_iter().map(|n| n as u64 + 1).sum();
    validate_extent(words, decoded.len(), options)?;
    let coverage = checked_coverage(stored.len(), decoded.len(), words as usize, options);
    Ok(FlowBaseGrid {
        source: source.clone(),
        record: id.clone(),
        decoded: decoded.into(),
        dimensions,
        spans: boundary_spans(dimensions),
        options,
        coverage,
    })
}

fn checked_coverage(
    stored: usize,
    decoded: usize,
    words: usize,
    options: FlowGridOptions,
) -> FlowGridCoverage {
    let mode = match options.compression {
        FlowGridCompression::Raw => FlowGridCheckMode::ExactRawBoundaryArrays,
        FlowGridCompression::Zlib => FlowGridCheckMode::ExactZlibBoundaryArrays,
    };
    FlowGridCoverage {
        mode,
        stored_bytes: stored,
        decoded_bytes: decoded,
        boundary_words: words,
        achieved_error: 0,
    }
}

fn validate_options(options: FlowGridOptions) -> Result<()> {
    if options.maximum_decoded_bytes >= 36 && options.maximum_boundary_words >= 3 {
        return Ok(());
    }
    Err(grid_failure(format!(
        "options {options:?}; expected decoded byte limit >=36 and total boundary-word limit >=3"
    )))
}

fn expand(stored: &[u8], options: FlowGridOptions) -> Result<Vec<u8>> {
    if options.compression == FlowGridCompression::Zlib {
        return super::super::payload_codec::inflate_zlib(
            stored,
            options.maximum_decoded_bytes,
            "Flow base grid",
        );
    }
    if stored.len() <= options.maximum_decoded_bytes {
        return Ok(stored.to_vec());
    }
    Err(grid_failure(format!(
        "raw payload {} bytes; expected at most {} decoded bytes",
        stored.len(),
        options.maximum_decoded_bytes
    )))
}

fn read_dimensions(decoded: &[u8]) -> Result<[i32; 3]> {
    let mut cursor = ArchiveCursor::new(decoded);
    let dimensions = [cursor.i32()?, cursor.i32()?, cursor.i32()?];
    if dimensions.into_iter().all(|n| n >= 0) {
        return Ok(dimensions);
    }
    Err(grid_failure(format!(
        "dimensions {dimensions:?}; expected three nonnegative signed i32 values"
    )))
}

fn validate_extent(words: u64, actual: usize, options: FlowGridOptions) -> Result<()> {
    let expected = 12 + words * 8;
    if words <= options.maximum_boundary_words as u64 && expected == actual as u64 {
        return Ok(());
    }
    Err(grid_failure(format!(
        "{words} boundary words, {actual} decoded bytes; expected at most {} words and exactly {expected} bytes through EOF",
        options.maximum_boundary_words
    )))
}

fn boundary_spans(dimensions: [i32; 3]) -> [(usize, usize); 3] {
    let mut offset = 12;
    dimensions.map(|n| {
        let start = offset;
        offset += (n as usize + 1) * 8;
        (start, offset)
    })
}
