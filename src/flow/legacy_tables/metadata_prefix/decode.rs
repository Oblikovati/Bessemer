use super::*;

pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    options: FlowMetadataPrefixOptions,
) -> Result<FlowMetadataPrefix> {
    check_options(options)?;
    let stored =
        super::super::super::payload_codec::record_payload(source, id, 424, "Flow legacy tables")?;
    let (vectors, objects, end) = parse(stored, options)?;
    Ok(FlowMetadataPrefix {
        source: source.clone(),
        record: id.clone(),
        raw: stored.into(),
        options,
        vectors: vectors.into(),
        objects: objects.into(),
        end,
    })
}
fn parse(
    stored: &[u8],
    options: FlowMetadataPrefixOptions,
) -> Result<(Vec<VectorSpan>, Vec<ObjectSpan>, usize)> {
    if stored.len() > options.maximum_payload_bytes {
        return Err(metadata_invalid(format!(
            "payload {} bytes; expected at most {}",
            stored.len(),
            options.maximum_payload_bytes
        )));
    }
    let mut cursor = ArchiveCursor::new(stored);
    if (cursor.u8()? as i8) <= 1 {
        return Ok((Vec::new(), Vec::new(), 1));
    }
    let vectors = vectors(&mut cursor, stored.len(), options)?;
    let objects = objects(&mut cursor, stored, options)?;
    Ok((vectors, objects, stored.len() - cursor.remaining()))
}
fn check_options(options: FlowMetadataPrefixOptions) -> Result<()> {
    let bounds = [
        options.maximum_payload_bytes,
        options.maximum_vectors,
        options.maximum_objects,
        options.maximum_values,
        options.maximum_string_units,
    ];
    if bounds.contains(&0) {
        return Err(metadata_invalid(format!(
            "options {options:?}; expected every independent bound positive"
        )));
    }
    Ok(())
}
fn bounded_count(
    cursor: &mut ArchiveCursor<'_>,
    minimum: usize,
    maximum: usize,
    label: &str,
) -> Result<usize> {
    let count = cursor.nonnegative_count(minimum, label)?;
    if count > maximum {
        return Err(metadata_invalid(format!(
            "{label} count {count}; expected at most {maximum}"
        )));
    }
    Ok(count)
}
fn vectors(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    options: FlowMetadataPrefixOptions,
) -> Result<Vec<VectorSpan>> {
    let count = bounded_count(cursor, 4, options.maximum_vectors, "metadata vectors")?;
    let mut result = Vec::with_capacity(count);
    let mut remaining = options.maximum_values;
    for _ in 0..count {
        let start = length - cursor.remaining();
        let values = bounded_count(cursor, 4, remaining, "metadata vector values")?;
        cursor.take(values * 4)?;
        remaining -= values;
        result.push(VectorSpan {
            start,
            end: length - cursor.remaining(),
            count: values,
        });
    }
    Ok(result)
}
fn objects(
    cursor: &mut ArchiveCursor<'_>,
    raw: &[u8],
    options: FlowMetadataPrefixOptions,
) -> Result<Vec<ObjectSpan>> {
    let count = bounded_count(cursor, 17, options.maximum_objects, "metadata objects")?;
    let mut result = Vec::with_capacity(count);
    let mut remaining = options.maximum_string_units;
    for _ in 0..count {
        result.push(object(cursor, raw, &mut remaining)?);
    }
    Ok(result)
}
fn object(cursor: &mut ArchiveCursor<'_>, raw: &[u8], remaining: &mut usize) -> Result<ObjectSpan> {
    let start = raw.len() - cursor.remaining();
    let first = text(cursor, raw, remaining)?;
    let second = text(cursor, raw, remaining)?;
    let state = raw.len() - cursor.remaining();
    cursor.u8()?;
    cursor.i32()?;
    let third = text(cursor, raw, remaining)?;
    let fourth = if raw[0] >= 3 {
        Some(text(cursor, raw, remaining)?)
    } else {
        None
    };
    Ok(ObjectSpan {
        start,
        end: raw.len() - cursor.remaining(),
        strings: [Some(first), Some(second), Some(third), fourth],
        state,
    })
}
pub(super) fn text(
    cursor: &mut ArchiveCursor<'_>,
    raw: &[u8],
    remaining: &mut usize,
) -> Result<TextSpan> {
    let start = raw.len() - cursor.remaining();
    let count = bounded_count(cursor, 2, *remaining, "metadata UTF-16 units")?;
    let bytes = cursor.take(count * 2)?;
    let native_units = wire::utf16_text_prefix(424, start, bytes)?;
    *remaining -= count;
    Ok(TextSpan {
        start,
        end: raw.len() - cursor.remaining(),
        native_end: start + 4 + native_units * 2,
    })
}
