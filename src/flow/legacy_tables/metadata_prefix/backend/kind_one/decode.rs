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
fn zero(cursor: &mut ArchiveCursor<'_>) -> Result<()> {
    let version = cursor.u8()?;
    if version != 0 {
        return Err(metadata_invalid(format!(
            "kind-one body/curve version {version}; expected zero"
        )));
    }
    Ok(())
}
fn body(
    cursor: &mut ArchiveCursor<'_>,
    raw: &[u8],
    start: usize,
    options: FlowMediumBodyOptions,
) -> Result<BodySpan> {
    let name = header(cursor, raw, options.maximum_name_units)?;
    let mut remaining = options.maximum_wide_words;
    let mut curves = Vec::with_capacity(7);
    for _ in 0..7 {
        curves.push(curve(cursor, raw.len(), &mut remaining)?);
    }
    let arrays = shared_arrays(cursor, raw.len(), &mut remaining)?;
    let scalars = words(cursor, raw.len(), 4, &mut remaining)?;
    let state = raw.len() - cursor.remaining();
    cursor.u8()?;
    Ok(BodySpan {
        start,
        end: raw.len() - cursor.remaining(),
        name,
        curves: curves.into(),
        arrays,
        scalars,
        state,
        words: options.maximum_wide_words - remaining,
    })
}
fn curve(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    remaining: &mut usize,
) -> Result<CurveSpan> {
    let start = length - cursor.remaining();
    zero(cursor)?;
    let scalar = words(cursor, length, 1, remaining)?.0;
    let count = cursor.nonnegative_count(16, "medium curve values")?;
    let first = words(cursor, length, count, remaining)?;
    let second = words(cursor, length, count, remaining)?;
    let flag = length - cursor.remaining();
    let active = cursor.u8()? != 0;
    let extra_first = words(cursor, length, if active { count } else { 0 }, remaining)?;
    let extra_second = words(cursor, length, if active { count } else { 0 }, remaining)?;
    Ok(CurveSpan {
        start,
        end: length - cursor.remaining(),
        scalar,
        count,
        flag,
        arrays: [first, second, extra_first, extra_second],
    })
}
fn shared_arrays(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    remaining: &mut usize,
) -> Result<[(usize, usize); 6]> {
    let first = cursor.nonnegative_count(16, "medium first shared arrays")?;
    let a = words(cursor, length, first, remaining)?;
    let b = words(cursor, length, first, remaining)?;
    let second = cursor.nonnegative_count(32, "medium second shared arrays")?;
    Ok([
        a,
        b,
        words(cursor, length, second, remaining)?,
        words(cursor, length, second, remaining)?,
        words(cursor, length, second, remaining)?,
        words(cursor, length, second, remaining)?,
    ])
}
fn words(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    count: usize,
    remaining: &mut usize,
) -> Result<(usize, usize)> {
    if count > *remaining || count > cursor.remaining() / 8 {
        return Err(metadata_invalid(format!(
            "medium wide count {count}; expected at most {} budgeted and {} physical words",
            *remaining,
            cursor.remaining() / 8
        )));
    }
    let start = length - cursor.remaining();
    cursor.take(count * 8)?;
    *remaining -= count;
    Ok((start, length - cursor.remaining()))
}

fn kind_error(backend: &FlowMediumBackendPrefix) -> BessemerError {
    metadata_invalid(format!(
        "backend stop {:?}, first kind {:?}; expected first-table kind one",
        backend.stop(),
        backend.table(0).and_then(|table| table.first_kind())
    ))
}

fn header(cursor: &mut ArchiveCursor<'_>, raw: &[u8], maximum_units: usize) -> Result<TextSpan> {
    zero(cursor)?;
    cursor.u8()?;
    cursor.take(16)?;
    let mut remaining = maximum_units;
    metadata_decode::text(cursor, raw, &mut remaining)
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
