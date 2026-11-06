use super::*;

pub(super) struct Budget {
    pub numeric: usize,
    pub pairs: usize,
    pub limits: FlowMappedLimits,
    pub length: usize,
}

pub(super) fn words(
    cursor: &mut ArchiveCursor<'_>,
    count: usize,
    budget: &mut Budget,
) -> Result<FlowMappedNumericBlock> {
    let maximum =
        (cursor.remaining() / 8).min(budget.limits.maximum_numeric_words - budget.numeric);
    if count > maximum {
        return Err(mapped_failure(
            FailureClass::Invalid,
            format!(
                "numeric count {count}; expected at most {maximum} remaining wire/budget words"
            ),
        ));
    }
    let offset = budget.length - cursor.remaining();
    let bits = read_word_bits(cursor, count)?;
    budget.numeric += count;
    Ok(FlowMappedNumericBlock { offset, bits })
}

pub(super) fn counted_words(
    cursor: &mut ArchiveCursor<'_>,
    budget: &mut Budget,
) -> Result<FlowMappedNumericBlock> {
    let count = cursor.nonnegative_count(8, "numeric array")?;
    words(cursor, count, budget)
}

pub(super) fn product(factors: &[usize], maximum: usize) -> Result<usize> {
    if factors.contains(&0) {
        return Ok(0);
    }
    let mut value = 1_usize;
    for factor in factors {
        value = value.checked_mul(*factor).filter(|value| *value <= maximum)
            .ok_or_else(|| mapped_failure(FailureClass::Invalid, format!("product {factors:?}; expected at most {maximum} remaining numeric words without usize overflow")))?;
    }
    Ok(value)
}

pub(super) fn product_words(
    cursor: &mut ArchiveCursor<'_>,
    factors: &[usize],
    budget: &mut Budget,
) -> Result<FlowMappedNumericBlock> {
    let maximum =
        (cursor.remaining() / 8).min(budget.limits.maximum_numeric_words - budget.numeric);
    let count = product(factors, maximum)?;
    words(cursor, count, budget)
}

pub(super) fn version(cursor: &mut ArchiveCursor<'_>, label: &str) -> Result<u32> {
    let value = cursor.u32()?;
    if value <= 6 {
        return Ok(value);
    }
    Err(mapped_failure(
        FailureClass::Unsupported,
        format!("{label} version {value}; expected strict profile 0 through 6"),
    ))
}

fn read_word_bits(cursor: &mut ArchiveCursor<'_>, count: usize) -> Result<Arc<[u64]>> {
    let bits: Vec<u64> = cursor
        .take(count * 8)?
        .as_chunks::<8>()
        .0
        .iter()
        .map(|bytes| u64::from_le_bytes(*bytes))
        .collect();
    Ok(Arc::from(bits))
}
