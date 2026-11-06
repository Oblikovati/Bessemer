use super::*;

pub(super) fn validate_limits(limits: FlowMappedLimits) -> Result<()> {
    if limits.maximum_entries > 0 && limits.maximum_pairs > 0 && limits.maximum_numeric_words > 0 {
        return Ok(());
    }
    Err(mapped_failure(
        FailureClass::Invalid,
        format!(
            "limits {limits:?}; expected positive entry, total pair and total numeric-word bounds"
        ),
    ))
}

pub(super) fn read(
    source: &FlowCarrier,
    record: FlowRecord<'_>,
    limits: FlowMappedLimits,
    context: FlowMappedContext,
) -> Result<FlowMappedObjects> {
    let (version, entries, coverage) = read_entries(record.payload, limits, &context)?;
    Ok(FlowMappedObjects {
        source: source.clone(),
        record: record.id,
        payload: Arc::from(record.payload),
        version,
        entries: Arc::from(entries),
        context,
        limits,
        coverage,
    })
}

fn entry_count(
    cursor: &mut ArchiveCursor<'_>,
    limits: FlowMappedLimits,
    context: &FlowMappedContext,
) -> Result<usize> {
    let count = cursor.nonnegative_count(1, "entries")?;
    if count > limits.maximum_entries {
        return Err(mapped_failure(
            FailureClass::Invalid,
            format!(
                "entry count {count}; expected at most {}",
                limits.maximum_entries
            ),
        ));
    }
    validate_context_shape(context, count)?;
    Ok(count)
}

fn read_entry(
    cursor: &mut ArchiveCursor<'_>,
    index: usize,
    context: &FlowMappedContext,
    budget: &mut blocks::Budget,
) -> Result<FlowMappedEntry> {
    let start = budget.length - cursor.remaining();
    let presence = cursor.u8()?;
    let matched = match context {
        FlowMappedContext::Unavailable => None,
        FlowMappedContext::Aligned(values) => values[index],
    };
    let body = if presence == 0 {
        validate_null(index, matched)?;
        None
    } else {
        Some(body::read(cursor, matched, budget)?)
    };
    Ok(FlowMappedEntry {
        start,
        end: budget.length - cursor.remaining(),
        presence,
        body,
    })
}

fn validate_null(index: usize, matched: Option<FlowMatchedObjectCount>) -> Result<()> {
    if matched.is_none() {
        return Ok(());
    }
    Err(mapped_failure(
        FailureClass::Invalid,
        format!(
            "null entry {index} has matched count {matched:?}; expected missing count for null entry"
        ),
    ))
}

fn coverage(entries: &[FlowMappedEntry], budget: &blocks::Budget) -> FlowMappedCoverage {
    let bodies = entries.iter().filter_map(|entry| entry.body.as_ref());
    let supplied = bodies.clone().filter(|body| body.matched.is_some()).count();
    let unresolved = bodies.filter(|body| body.matched.is_none()).count();
    FlowMappedCoverage {
        mode: FlowMappedCheckMode::ExactExplicitContextFrames,
        checked_bytes: budget.length,
        checked_entries: entries.len(),
        checked_pairs: budget.pairs,
        checked_numeric_words: budget.numeric,
        supplied_matched_counts: supplied,
        unresolved_zero_factor_counts: unresolved,
        achieved_byte_error: 0,
    }
}

fn read_entries(
    payload: &[u8],
    limits: FlowMappedLimits,
    context: &FlowMappedContext,
) -> Result<(u32, Vec<FlowMappedEntry>, FlowMappedCoverage)> {
    let mut cursor = ArchiveCursor::new(payload);
    let version = blocks::version(&mut cursor, "list")?;
    let count = entry_count(&mut cursor, limits, context)?;
    let mut budget = blocks::Budget {
        numeric: 0,
        pairs: 0,
        limits,
        length: payload.len(),
    };
    let entries = (0..count)
        .map(|index| read_entry(&mut cursor, index, context, &mut budget))
        .collect::<Result<Vec<_>>>()?;
    cursor.finish()?;
    let coverage = coverage(&entries, &budget);
    Ok((version, entries, coverage))
}

fn validate_context_shape(context: &FlowMappedContext, count: usize) -> Result<()> {
    if let FlowMappedContext::Aligned(values) = context
        && values.len() != count
    {
        return Err(mapped_failure(
            FailureClass::Invalid,
            format!(
                "{} supplied context entries; expected exactly {count} aligned physical entries",
                values.len()
            ),
        ));
    }
    Ok(())
}
