use super::*;
pub(super) fn read(
    backend: &FlowMediumBackendPrefix,
    options: FlowMediumBodyOptions,
) -> Result<FlowMediumKindOne> {
    let start = backend.metadata_prefix().end + backend.end;
    read_at(backend, start, options)
}
pub(super) fn read_at(
    backend: &FlowMediumBackendPrefix,
    start: usize,
    options: FlowMediumBodyOptions,
) -> Result<FlowMediumKindOne> {
    required_kind(backend, options)?;
    let prefix = backend.metadata_prefix();
    let raw = prefix.payload_bytes();
    let suffix = raw.get(start..).ok_or_else(|| {
        metadata_invalid(format!(
            "kind-one start {start}; expected within payload {} bytes",
            raw.len()
        ))
    })?;
    let mut cursor = ArchiveCursor::new(suffix);
    let span = body(&mut cursor, raw, start, options)?;
    check_body_bound(&span, options)?;
    Ok(FlowMediumKindOne {
        backend: backend.clone(),
        options,
        span,
    })
}
fn required_kind(backend: &FlowMediumBackendPrefix, options: FlowMediumBodyOptions) -> Result<()> {
    if [
        options.maximum_body_bytes,
        options.maximum_wide_words,
        options.maximum_name_units,
    ]
    .contains(&0)
    {
        return Err(metadata_invalid(format!(
            "kind-one options {options:?}; expected all independent bounds positive"
        )));
    }
    if backend.stop() != FlowMediumBackendStop::FirstTableObjectBody
        || backend.table(0).and_then(|table| table.first_kind()) != Some(1)
    {
        return Err(kind_error(backend));
    }
    Ok(())
}
fn body(
    cursor: &mut ArchiveCursor<'_>,
    raw: &[u8],
    start: usize,
    options: FlowMediumBodyOptions,
) -> Result<BodySpan> {
    let mut remaining = options.maximum_wide_words;
    let prefix = medium_body::prefix(
        cursor,
        raw,
        options.maximum_name_units,
        &mut remaining,
        "kind-one body/curve",
    )?;
    let scalars = medium_body::words(cursor, raw.len(), 4, &mut remaining)?;
    let state = medium_body::byte_position(cursor, raw.len())?;
    Ok(BodySpan {
        start,
        end: raw.len() - cursor.remaining(),
        name: prefix.name,
        curves: prefix.curves,
        arrays: prefix.arrays,
        scalars,
        state,
        words: options.maximum_wide_words - remaining,
    })
}

fn kind_error(backend: &FlowMediumBackendPrefix) -> BessemerError {
    metadata_invalid(format!(
        "backend stop {:?}, first kind {:?}; expected first-table kind one",
        backend.stop(),
        backend.table(0).and_then(|table| table.first_kind())
    ))
}

fn check_body_bound(span: &BodySpan, options: FlowMediumBodyOptions) -> Result<()> {
    let start = span.start;
    if span.end - start > options.maximum_body_bytes {
        return Err(metadata_invalid(format!(
            "kind-one body {} bytes; expected at most {}",
            span.end - start,
            options.maximum_body_bytes
        )));
    }
    Ok(())
}
