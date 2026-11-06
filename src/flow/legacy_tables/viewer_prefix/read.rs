use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowViewerPrefixOptions,
) -> Result<FlowViewerPrefix> {
    validate_options(options)?;
    let wire = match options.profile {
        FlowViewerPrefixProfile::InflateMetadataQuery => wire::versioned(
            source,
            id,
            121,
            FlowLegacyTableTransport::Zlib,
            OPERATION,
            |stored, version| expand(stored, version, options.maximum_decoded_bytes),
        )?,
    };
    let layout = read_layout(&wire.decoded, options.maximum_prefix_wide_values)?;
    Ok(FlowViewerPrefix {
        wire,
        options,
        layout,
    })
}
fn validate_options(options: FlowViewerPrefixOptions) -> Result<()> {
    if options.maximum_decoded_bytes >= 72 && options.maximum_prefix_wide_values >= 6 {
        return Ok(());
    }
    Err(invalid(format!(
        "decoded limit {}, prefix-wide limit {}; expected at least 72 decoded bytes and six known wide values",
        options.maximum_decoded_bytes, options.maximum_prefix_wide_values
    )))
}
fn expand(stored: &[u8], version: i32, maximum: usize) -> Result<Vec<u8>> {
    if version < 2 {
        return Err(table_failure(
            121,
            FailureClass::Unsupported,
            format!(
                "viewer-query outer version {version}; expected at least two under the explicitly declared profile"
            ),
        ));
    }
    super::super::super::payload_codec::inflate_zlib(stored, maximum, OPERATION)
}
fn read_layout(raw: &[u8], maximum: usize) -> Result<ViewerLayout> {
    let mut cursor = ArchiveCursor::new(raw);
    cursor.take(8)?;
    let lengths = [cursor.i32()?, cursor.i32()?, cursor.i32()?];
    let wide_values = validate_lengths(lengths, maximum)?;
    let mut arrays = [ViewerSpan::EMPTY; 3];
    for (ordinal, length) in lengths.into_iter().enumerate() {
        arrays[ordinal] = read_array(&mut cursor, raw.len(), ordinal, length as usize)?;
    }
    let triplet_start = raw.len() - cursor.remaining();
    cursor.take(28)?;
    Ok(ViewerLayout {
        arrays,
        triplet_start,
        prefix_end: raw.len() - cursor.remaining(),
        wide_values,
    })
}
fn validate_lengths(lengths: [i32; 3], maximum: usize) -> Result<usize> {
    lengths.into_iter().enumerate().try_fold(3,|total,(ordinal,length)| {
        if length<=0 || length as usize>maximum-total {
            return Err(invalid(format!("array {ordinal} length {length}, accumulated {total} known wide values; expected positive length within total bound {maximum}, including the discarded triplet")));
        }
        Ok(total+length as usize)
    })
}
fn read_array(
    cursor: &mut ArchiveCursor<'_>,
    total: usize,
    ordinal: usize,
    count: usize,
) -> Result<ViewerSpan> {
    let available = cursor.remaining() / 8;
    if count > available {
        return Err(invalid(format!(
            "array {ordinal} length {count}; expected at most {available} complete raw wide values in remaining bytes"
        )));
    }
    let start = total - cursor.remaining();
    cursor.take(count * 8)?;
    Ok(ViewerSpan {
        start,
        end: total - cursor.remaining(),
    })
}
