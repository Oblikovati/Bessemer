mod parallel;
use super::*;
pub(super) fn parallel(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowParallelTableOptions,
) -> Result<FlowParallelTable> {
    parallel::read(source, id, options)
}

pub(super) fn load(
    source: &FlowCarrier,
    id: &FlowRecordId,
    identifier: u32,
    compression: FlowLegacyTableTransport,
    maximum: usize,
) -> Result<TableWire> {
    let wire = wire::read(source, id, identifier, compression, maximum)?;
    if wire.version >= 0 {
        return Ok(wire);
    }
    Err(simple_invalid(
        identifier,
        format!(
            "version {}; expected nonnegative native signed version",
            wire.version
        ),
    ))
}

pub(super) fn leaf_pairs(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowLeafPairOptions,
) -> Result<FlowLeafPairs> {
    positive_limit(437, "wire rows", options.maximum_wire_rows)?;
    let wire = load(
        source,
        id,
        437,
        options.compression,
        options.maximum_decoded_bytes,
    )?;
    if wire.version != 0 {
        return Err(restored(437, wire.version, "per-leaf pairs"));
    }
    let mut cursor = payload_cursor(&wire)?;
    let rows = options.leaf_count.value() as usize;
    consume_rows(&mut cursor, 437, rows, 16, options.maximum_wire_rows)?;
    cursor.finish()?;
    Ok(FlowLeafPairs {
        wire,
        options,
        rows,
    })
}

pub(super) fn five_words(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowFiveWordTableOptions,
) -> Result<FlowFiveWordTable> {
    positive_limit(411, "wire rows", options.maximum_wire_rows)?;
    let maximum = options.maximum_decoded_bytes;
    let wire = load(source, id, 411, options.compression, maximum)?;
    let mut cursor = payload_cursor(&wire)?;
    let count = five_count(&mut cursor, wire.version, options.legacy_count)?;
    let rows = five_rows(wire.version, count)?;
    let start = position(&wire, &cursor);
    consume_rows(&mut cursor, 411, rows, 20, options.maximum_wire_rows)?;
    cursor.finish()?;
    Ok(FlowFiveWordTable {
        wire,
        options,
        count,
        start,
        rows,
    })
}

pub(super) fn bytes(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowRawByteTableOptions,
) -> Result<FlowRawByteTable> {
    positive_limit(413, "raw bytes", options.maximum_raw_bytes)?;
    let maximum = options.maximum_decoded_bytes;
    let wire = load(source, id, 413, options.compression, maximum)?;
    let mut cursor = payload_cursor(&wire)?;
    let count = byte_count(&mut cursor, wire.version, options.legacy_count)?;
    let start = position(&wire, &cursor);
    consume_rows(
        &mut cursor,
        413,
        count.value.value() as usize,
        1,
        options.maximum_raw_bytes,
    )?;
    cursor.finish()?;
    Ok(FlowRawByteTable {
        wire,
        options,
        count,
        start,
    })
}

fn five_rows(version: i32, count: Option<SimpleDeclaredCount>) -> Result<usize> {
    if (3..=4).contains(&version) {
        return count
            .map(|count| count.value.value() as usize)
            .ok_or_else(|| missing_count(411));
    }
    if version >= 5 && count.is_some_and(|count| count.value.value() != 0) {
        return Err(restored(411, version, "populated five-word rows"));
    }
    Ok(0)
}

fn declared(
    cursor: &mut ArchiveCursor<'_>,
    identifier: u32,
    stored: bool,
    legacy: Option<FlowSimpleLegacyCount>,
    required: bool,
) -> Result<Option<SimpleDeclaredCount>> {
    if stored {
        return stored_count(cursor, identifier, legacy).map(Some);
    }
    if let Some(value) = legacy {
        return Ok(Some(SimpleDeclaredCount {
            value,
            stored: false,
        }));
    }
    if required {
        return Err(missing_count(identifier));
    }
    Ok(None)
}

fn stored_count(
    cursor: &mut ArchiveCursor<'_>,
    identifier: u32,
    legacy: Option<FlowSimpleLegacyCount>,
) -> Result<SimpleDeclaredCount> {
    if let Some(value) = legacy {
        return Err(simple_invalid(
            identifier,
            format!("legacy context {value:?}; expected None because count is physically stored"),
        ));
    }
    let value = nonnegative_count(cursor, identifier)?;
    Ok(SimpleDeclaredCount {
        value,
        stored: true,
    })
}

pub(super) fn nonnegative_count(
    cursor: &mut ArchiveCursor<'_>,
    identifier: u32,
) -> Result<FlowSimpleLegacyCount> {
    let value = cursor.i32()?;
    if value >= 0 {
        return Ok(FlowSimpleLegacyCount(value as u32));
    }
    Err(simple_invalid(
        identifier,
        format!("count {value}; expected nonnegative signed-i32 count"),
    ))
}

pub(super) fn payload_cursor(wire: &TableWire) -> Result<ArchiveCursor<'_>> {
    let mut cursor = ArchiveCursor::new(&wire.decoded);
    cursor.take(4)?;
    Ok(cursor)
}

pub(super) fn position(wire: &TableWire, cursor: &ArchiveCursor<'_>) -> usize {
    wire.decoded.len() - cursor.remaining()
}

fn consume_rows(
    cursor: &mut ArchiveCursor<'_>,
    identifier: u32,
    count: usize,
    width: usize,
    maximum: usize,
) -> Result<()> {
    if count > maximum || count > cursor.remaining() / width {
        return Err(simple_invalid(
            identifier,
            format!(
                "wire count {count}, width {width}; expected <= {maximum} values and <= {} complete values in remaining bytes",
                cursor.remaining() / width
            ),
        ));
    }
    cursor.take(count * width)?;
    Ok(())
}

pub(super) fn positive_limit(identifier: u32, label: &str, value: usize) -> Result<()> {
    if value > 0 {
        return Ok(());
    }
    Err(simple_invalid(
        identifier,
        format!("{label} limit {value}; expected positive maximum"),
    ))
}

pub(super) fn restored(identifier: u32, version: i32, field: &str) -> BessemerError {
    table_failure(
        identifier,
        FailureClass::Unsupported,
        format!(
            "version {version} {field}; expected a supported raw or inactive empty layout, not populated restored arrays"
        ),
    )
}

fn missing_count(identifier: u32) -> BessemerError {
    table_failure(
        identifier,
        FailureClass::Missing,
        "legacy context None; expected an explicit validated member count".to_owned(),
    )
}

fn five_count(
    cursor: &mut ArchiveCursor<'_>,
    version: i32,
    legacy: Option<FlowSimpleLegacyCount>,
) -> Result<Option<SimpleDeclaredCount>> {
    declared(cursor, 411, version >= 4, legacy, version == 3)
}

fn byte_count(
    cursor: &mut ArchiveCursor<'_>,
    version: i32,
    legacy: Option<FlowSimpleLegacyCount>,
) -> Result<SimpleDeclaredCount> {
    let count =
        declared(cursor, 413, version > 0, legacy, true)?.ok_or_else(|| missing_count(413))?;
    if version >= 2 && count.value.value() > 0 {
        return Err(restored(413, version, "populated bytes"));
    }
    Ok(count)
}
