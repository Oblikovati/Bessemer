use super::*;

pub(super) fn validate_options(options: FlowFeatureOptions) -> Result<()> {
    if options.maximum_objects > 0
        && options.maximum_text_units > 0
        && options.maximum_tail_values > 0
    {
        return Ok(());
    }
    Err(feature_failure(
        FailureClass::Invalid,
        format!(
            "options {options:?}; expected positive object, total text-unit and stored tail bounds"
        ),
    ))
}

pub(super) fn read(
    source: &FlowCarrier,
    record: FlowRecord<'_>,
    options: FlowFeatureOptions,
) -> Result<FlowFeatureIdentities> {
    let mut cursor = ArchiveCursor::new(record.payload);
    let version = list_version(&mut cursor)?;
    let count = bounded_count(&mut cursor, 17, options.maximum_objects, "objects")?;
    let (objects, total_text_units) =
        read_objects(&mut cursor, record.payload.len(), count, options)?;
    let tail = read_tail(&mut cursor, version, options)?;
    cursor.finish()?;
    Ok(FlowFeatureIdentities {
        source: source.clone(),
        record: record.id,
        payload: Arc::from(record.payload),
        version,
        objects: Arc::from(objects),
        tail,
        total_text_units,
        options,
    })
}

fn list_version(cursor: &mut ArchiveCursor<'_>) -> Result<u32> {
    let version = cursor.u32()?;
    if version <= 6 {
        return Ok(version);
    }
    Err(feature_failure(
        FailureClass::Unsupported,
        format!("list version {version}; expected strict profile 0 through 6"),
    ))
}

fn bounded_count(
    cursor: &mut ArchiveCursor<'_>,
    width: usize,
    maximum: usize,
    label: &str,
) -> Result<usize> {
    let count = cursor.nonnegative_count(width, label)?;
    if count <= maximum {
        return Ok(count);
    }
    Err(feature_failure(
        FailureClass::Invalid,
        format!("{label} count {count}; expected at most {maximum}"),
    ))
}

fn read_objects(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    count: usize,
    options: FlowFeatureOptions,
) -> Result<(Vec<FlowFeatureIdentity>, usize)> {
    let mut objects = Vec::with_capacity(count);
    let mut total = 0;
    for _ in 0..count {
        let object = read_object(cursor, length, options.maximum_text_units - total)?;
        total += object.text.len();
        objects.push(object);
    }
    Ok((objects, total))
}

fn read_object(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    maximum_units: usize,
) -> Result<FlowFeatureIdentity> {
    let start = length - cursor.remaining();
    let header_version = cursor.u32()?;
    let type_code = cursor.u32()?;
    let body_version = cursor.u32()?;
    let text = read_text(cursor, maximum_units)?;
    let state = cursor.u8()?;
    Ok(FlowFeatureIdentity {
        start,
        end: length - cursor.remaining(),
        header_version,
        type_code,
        body_version,
        text: Arc::from(text),
        state,
    })
}

pub(super) fn validate_text(units: &[u16]) -> Result<()> {
    if units.is_empty() || units.last() == Some(&0) {
        return Ok(());
    }
    Err(feature_failure(
        FailureClass::Invalid,
        format!(
            "UTF-16 length {} ends with {:?}; expected zero units or a terminal NUL code unit",
            units.len(),
            units.last()
        ),
    ))
}

fn read_tail(
    cursor: &mut ArchiveCursor<'_>,
    version: u32,
    options: FlowFeatureOptions,
) -> Result<FlowFeatureTail> {
    if version < 4 {
        return Ok(match options.legacy_tail_count {
            None => FlowFeatureTail::RequiresRuntimeCount,
            Some(runtime_count) => FlowFeatureTail::DerivedRuntime {
                runtime_count,
                count: runtime_count.max(0) as u32,
                fill_value: 2,
            },
        });
    }
    let count = bounded_count(cursor, 4, options.maximum_tail_values, "tail values")?;
    let values: Vec<i32> = (0..count).map(|_| cursor.i32()).collect::<Result<_>>()?;
    Ok(FlowFeatureTail::Stored(Arc::from(values)))
}

fn read_text(cursor: &mut ArchiveCursor<'_>, maximum_units: usize) -> Result<Vec<u16>> {
    let count = bounded_count(cursor, 2, maximum_units, "UTF-16 units")?;
    let text: Vec<u16> = cursor
        .take(count * 2)?
        .as_chunks::<2>()
        .0
        .iter()
        .map(|bytes| u16::from_le_bytes(*bytes))
        .collect();
    validate_text(&text)?;
    Ok(text)
}
