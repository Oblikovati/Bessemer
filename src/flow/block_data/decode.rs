use super::*;
use entry::BodyReader;

struct DecodedBody {
    prefix: FlowBlockCountPrefix,
    bitmap_end: usize,
    blocks: Vec<BlockSpan>,
    entries: Vec<EntrySpan>,
    word_counts: [usize; 2],
}

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowBlockDataOptions,
) -> Result<FlowLegacyBlockData> {
    validate_options(options)?;
    let stored =
        super::super::payload_codec::record_payload(source, id, 406, "Flow legacy block data")?;
    let decoded = expand(stored, options)?;
    let parsed = parse_body(&decoded, options)?;
    let coverage = coverage(stored.len(), decoded.len(), &parsed, options.compression);
    Ok(FlowLegacyBlockData {
        source: source.clone(),
        record: id.clone(),
        decoded: decoded.into(),
        prefix: parsed.prefix,
        bitmap_end: parsed.bitmap_end,
        blocks: parsed.blocks.into(),
        entries: parsed.entries.into(),
        options,
        coverage,
    })
}

fn validate_options(options: FlowBlockDataOptions) -> Result<()> {
    if options.maximum_decoded_bytes >= 12
        && options.maximum_blocks > 0
        && options.maximum_entries > 0
        && options.maximum_integer_words > 0
    {
        return Ok(());
    }
    Err(invalid_block(format!(
        "options {options:?}; expected decoded byte limit >=12 and positive block, entry and integer-word limits"
    )))
}

fn expand(stored: &[u8], options: FlowBlockDataOptions) -> Result<Vec<u8>> {
    if options.compression == FlowBlockDataCompression::Zlib {
        return super::super::payload_codec::inflate_zlib(
            stored,
            options.maximum_decoded_bytes,
            "Flow legacy block data",
        );
    }
    if stored.len() <= options.maximum_decoded_bytes {
        return Ok(stored.to_vec());
    }
    Err(invalid_block(format!(
        "raw payload {} bytes; expected at most {} decoded bytes",
        stored.len(),
        options.maximum_decoded_bytes
    )))
}

fn count_prefix(cursor: &mut ArchiveCursor<'_>) -> Result<FlowBlockCountPrefix> {
    let version = legacy_version(cursor)?;
    let g_fluid = cursor.i32()?;
    let g_solid = cursor.i32()?;
    if g_fluid < 0 || g_solid < 0 {
        return Err(invalid_block(format!(
            "G.Fluid {g_fluid}, G.Solid {g_solid}; expected nonnegative signed-i32 count profile"
        )));
    }
    let mathematical_total = g_fluid as u64 + g_solid as u64;
    Ok(FlowBlockCountPrefix {
        version,
        g_fluid,
        g_solid,
        mathematical_total,
        native_total: mathematical_total as u32 as i32,
    })
}

pub(super) fn bitmap_count(prefix: FlowBlockCountPrefix) -> usize {
    if prefix.version >= 8 && prefix.native_total > 0 {
        return prefix.native_total as usize / 8 + 1;
    }
    0
}

fn parse_body(decoded: &[u8], options: FlowBlockDataOptions) -> Result<DecodedBody> {
    let mut cursor = ArchiveCursor::new(decoded);
    let prefix = count_prefix(&mut cursor)?;
    cursor.take(bitmap_count(prefix))?;
    let bitmap_end = decoded.len() - cursor.remaining();
    let mut reader = BodyReader::new(cursor, decoded.len(), prefix.version, options);
    let blocks = read_blocks(&mut reader)?;
    reader.cursor.finish()?;
    Ok(DecodedBody {
        prefix,
        bitmap_end,
        blocks,
        entries: reader.entries,
        word_counts: reader.word_counts,
    })
}

fn read_blocks(reader: &mut BodyReader<'_>) -> Result<Vec<BlockSpan>> {
    let count = reader.options.leaf_count.value() as usize;
    if count > reader.options.maximum_blocks || count > reader.cursor.remaining() / 5 {
        return Err(invalid_block(format!(
            "leaf context {count}; expected <= {} blocks and <= {} complete headers in remaining payload",
            reader.options.maximum_blocks,
            reader.cursor.remaining() / 5
        )));
    }
    let mut blocks = Vec::with_capacity(count);
    for _ in 0..count {
        blocks.push(read_block(reader)?);
    }
    Ok(blocks)
}

fn read_block(reader: &mut BodyReader<'_>) -> Result<BlockSpan> {
    let start = reader.position();
    reader.cursor.take(3)?;
    let count = (reader.cursor.u16()? as i16).max(0) as usize;
    let first_entry = reader.entries.len();
    read_entries(reader, start, count)?;
    Ok(BlockSpan {
        start,
        end: reader.position(),
        first_entry,
        entry_end: reader.entries.len(),
    })
}

fn coverage(
    stored: usize,
    decoded: usize,
    parsed: &DecodedBody,
    compression: FlowBlockDataCompression,
) -> FlowBlockDataCoverage {
    let mode = match compression {
        FlowBlockDataCompression::Raw => FlowBlockDataCheckMode::ExactRawLegacyBody,
        FlowBlockDataCompression::Zlib => FlowBlockDataCheckMode::ExactZlibLegacyBody,
    };
    FlowBlockDataCoverage {
        mode,
        stored_bytes: stored,
        decoded_bytes: decoded,
        checked_blocks: parsed.blocks.len(),
        checked_entries: parsed.entries.len(),
        first_integer_words: parsed.word_counts[0],
        second_integer_words: parsed.word_counts[1],
        achieved_error: 0,
    }
}

fn legacy_version(cursor: &mut ArchiveCursor<'_>) -> Result<i32> {
    let version = cursor.i32()?;
    if !(1..=17).contains(&version) {
        return Err(block_failure(
            FailureClass::Unsupported,
            format!(
                "body version {version}; expected legacy versions 1 through 17, without delegated body reconstruction"
            ),
        ));
    }
    Ok(version)
}

fn read_entries(reader: &mut BodyReader<'_>, start: usize, count: usize) -> Result<()> {
    let first_entry = reader.entries.len();
    let available = reader.options.maximum_entries - first_entry;
    if count > available || count > reader.cursor.remaining() {
        return Err(invalid_block(format!(
            "block at {start}, entry count {count}; expected <= {available} additional entries and <= {} remaining bytes",
            reader.cursor.remaining()
        )));
    }
    for _ in 0..count {
        let entry = reader.read_entry()?;
        reader.entries.push(entry);
    }
    Ok(())
}
