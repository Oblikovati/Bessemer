use super::*;
pub(super) fn read(
    backend: &FlowMediumBackendPrefix,
    options: FlowMediumKindTwoOptions,
) -> Result<FlowMediumKindTwo> {
    validate_options(options)?;
    require_kind(backend)?;
    let prefix = backend.metadata_prefix();
    let key_start = prefix.coverage().known_prefix_bytes + backend.end;
    let raw = prefix.payload_bytes();
    let mut cursor = ArchiveCursor::new(&raw[key_start..]);
    cursor.take(16)?;
    read_at(backend, raw.len() - cursor.remaining(), options)
}
pub(super) fn read_at(
    backend: &FlowMediumBackendPrefix,
    start: usize,
    options: FlowMediumKindTwoOptions,
) -> Result<FlowMediumKindTwo> {
    validate_options(options)?;
    let raw = backend.metadata_prefix().payload_bytes();
    let suffix = raw.get(start..).ok_or_else(|| {
        metadata_invalid(format!(
            "kind-two start {start}; expected within payload {} bytes",
            raw.len()
        ))
    })?;
    let mut cursor = ArchiveCursor::new(suffix);
    let span = body(&mut cursor, raw, start, options)?;
    check_body_bound(&span, options)?;
    Ok(FlowMediumKindTwo {
        backend: backend.clone(),
        options,
        span,
    })
}
fn body(
    cursor: &mut ArchiveCursor<'_>,
    raw: &[u8],
    start: usize,
    options: FlowMediumKindTwoOptions,
) -> Result<KindTwoSpan> {
    let mut remaining = options.body.maximum_wide_words;
    let prefix = prefix(cursor, raw, options, &mut remaining)?;
    let scalar = medium_body::words(cursor, raw.len(), 1, &mut remaining)?;
    let model = read_model(
        cursor,
        raw.len(),
        &mut remaining,
        options.maximum_word_patterns,
    )?;
    let state = medium_body::byte_position(cursor, raw.len())?;
    Ok(KindTwoSpan {
        start,
        end: raw.len() - cursor.remaining(),
        prefix,
        scalar,
        model,
        state,
        words: options.body.maximum_wide_words - remaining,
    })
}
fn validate_options(options: FlowMediumKindTwoOptions) -> Result<()> {
    let bounds = [
        options.body.maximum_body_bytes,
        options.body.maximum_wide_words,
        options.body.maximum_name_units,
        options.maximum_word_patterns,
    ];
    if bounds.contains(&0) {
        return Err(metadata_invalid(format!(
            "kind-two options {options:?}; expected all independent bounds positive"
        )));
    }
    Ok(())
}
fn require_kind(backend: &FlowMediumBackendPrefix) -> Result<()> {
    if backend.stop() == FlowMediumBackendStop::FirstTableObjectBody
        && backend.table(0).and_then(|table| table.first_kind()) == Some(2)
    {
        return Ok(());
    }
    Err(metadata_invalid(format!(
        "backend stop {:?}, first kind {:?}; expected first-table kind two",
        backend.stop(),
        backend.table(0).and_then(|table| table.first_kind())
    )))
}

fn check_body_bound(span: &KindTwoSpan, options: FlowMediumKindTwoOptions) -> Result<()> {
    let start = span.start;
    if span.end - start > options.body.maximum_body_bytes {
        return Err(metadata_invalid(format!(
            "kind-two body {} bytes; expected at most {}",
            span.end - start,
            options.body.maximum_body_bytes
        )));
    }
    Ok(())
}

fn prefix(
    cursor: &mut ArchiveCursor<'_>,
    raw: &[u8],
    options: FlowMediumKindTwoOptions,
    remaining: &mut usize,
) -> Result<PrefixSpan> {
    medium_body::prefix(
        cursor,
        raw,
        options.body.maximum_name_units,
        remaining,
        "kind-two body/curve",
    )
}

fn read_model(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    remaining: &mut usize,
    maximum_words: usize,
) -> Result<ModelSpan> {
    let kind = FlowMediumModelKind::try_from(cursor.i32()?)?;
    model::read(cursor, length, kind, remaining, maximum_words)
}
