use super::*;

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowLegacyWordOptions,
) -> Result<FlowLegacyWords> {
    validate_options(options)?;
    let wire = wire::read(
        source,
        id,
        405,
        options.compression,
        options.maximum_decoded_bytes,
    )?;
    let (words_start, count) = word_span(&wire, options)?;
    Ok(FlowLegacyWords {
        wire,
        options,
        words_start,
        count,
    })
}

fn count(
    cursor: &mut ArchiveCursor<'_>,
    version: i32,
    legacy: Option<FlowLegacyWordCounts>,
) -> Result<usize> {
    match version {
        0 => legacy.map(|counts| counts.total() as usize).ok_or_else(|| {
            table_failure(
                405,
                FailureClass::Missing,
                "version 0 context None; expected explicit validated G.Fluid/G.Solid counts"
                    .to_owned(),
            )
        }),
        1 => stored_count(cursor, legacy),
        _ => Err(table_failure(
            405,
            FailureClass::Unsupported,
            format!("version {version}; expected legacy word-table versions zero/one"),
        )),
    }
}

fn stored_count(
    cursor: &mut ArchiveCursor<'_>,
    legacy: Option<FlowLegacyWordCounts>,
) -> Result<usize> {
    if let Some(counts) = legacy {
        return Err(table_failure(
            405,
            FailureClass::Invalid,
            format!(
                "version 1 legacy context {counts:?}; expected None because count is physically stored"
            ),
        ));
    }
    let count = cursor.i32()?;
    if count >= 0 {
        return Ok(count as usize);
    }
    Err(table_failure(
        405,
        FailureClass::Invalid,
        format!("stored count {count}; expected nonnegative signed-i32 count"),
    ))
}

fn validate_options(options: FlowLegacyWordOptions) -> Result<()> {
    if options.maximum_words == 0 {
        return Err(table_failure(
            405,
            FailureClass::Invalid,
            "word limit 0; expected positive maximum".to_owned(),
        ));
    }
    Ok(())
}

fn word_span(wire: &TableWire, options: FlowLegacyWordOptions) -> Result<(usize, usize)> {
    let mut cursor = ArchiveCursor::new(&wire.decoded);
    cursor.take(4)?;
    let count = count(&mut cursor, wire.version, options.legacy_counts)?;
    check_word_count(&cursor, count, options.maximum_words)?;
    let words_start = wire.decoded.len() - cursor.remaining();
    cursor.take(count * 4)?;
    cursor.finish()?;
    Ok((words_start, count))
}

fn check_word_count(cursor: &ArchiveCursor<'_>, count: usize, maximum: usize) -> Result<()> {
    if count > maximum || count > cursor.remaining() / 4 {
        return Err(table_failure(
            405,
            FailureClass::Invalid,
            format!(
                "word count {count}; expected <= {} words and <= {} words in remaining payload",
                maximum,
                cursor.remaining() / 4
            ),
        ));
    }
    Ok(())
}
