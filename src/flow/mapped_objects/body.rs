use super::*;
use blocks::Budget;

pub(super) fn read(
    cursor: &mut ArchiveCursor<'_>,
    matched: Option<FlowMatchedObjectCount>,
    budget: &mut Budget,
) -> Result<FlowMappedBody> {
    let version = blocks::version(cursor, "body")?;
    let state = cursor.u8()?;
    let numeric = blocks::words(cursor, 9, budget)?;
    let (pair_offset, pairs) = read_pairs(cursor, budget)?;
    let factor = cursor.i32()?;
    let factors = mapped_factors(factor, matched)?;
    let mapped = blocks::product_words(cursor, &factors, budget)?;
    let legacy = optional_legacy(cursor, version, budget)?;
    Ok(FlowMappedBody {
        version,
        state,
        numeric,
        pairs: Arc::from(pairs),
        pair_offset,
        factor: factor as u32,
        matched,
        mapped,
        legacy,
    })
}

fn mapped_factors(factor: i32, matched: Option<FlowMatchedObjectCount>) -> Result<[usize; 2]> {
    if factor < 0 {
        return Err(mapped_failure(
            FailureClass::Invalid,
            format!("stored factor {factor}; expected nonnegative signed-i32 count"),
        ));
    }
    if factor == 0 {
        return Ok([0, 0]);
    }
    let count = matched.ok_or_else(|| mapped_failure(FailureClass::Missing,
        format!("stored factor {factor} with no matched count; expected explicit runtime-object cardinality, independent of pair count")))?;
    Ok([factor as usize, count.0 as usize])
}

fn read_pairs(
    cursor: &mut ArchiveCursor<'_>,
    budget: &mut Budget,
) -> Result<(usize, Vec<FlowMappedIdentifierPair>)> {
    let count = cursor.nonnegative_count(8, "identifier pairs")?;
    let maximum = budget.limits.maximum_pairs - budget.pairs;
    if count > maximum {
        return Err(mapped_failure(
            FailureClass::Invalid,
            format!("pair count {count}; expected at most {maximum} remaining total pairs"),
        ));
    }
    let offset = budget.length - cursor.remaining();
    let pairs = (0..count)
        .map(|_| read_pair(cursor))
        .collect::<Result<Vec<_>>>()?;
    budget.pairs += count;
    Ok((offset, pairs))
}

fn read_pair(cursor: &mut ArchiveCursor<'_>) -> Result<FlowMappedIdentifierPair> {
    Ok(FlowMappedIdentifierPair {
        source: cursor.i32()?,
        associated: cursor.i32()?,
    })
}

fn read_legacy(cursor: &mut ArchiveCursor<'_>, budget: &mut Budget) -> Result<FlowMappedLegacy> {
    let prefix = blocks::words(cursor, 16, budget)?;
    let arrays = [
        blocks::counted_words(cursor, budget)?,
        blocks::counted_words(cursor, budget)?,
        blocks::counted_words(cursor, budget)?,
    ];
    let factors = [
        3,
        arrays[0].bits.len(),
        arrays[1].bits.len(),
        arrays[2].bits.len(),
    ];
    let tripled = blocks::product_words(cursor, &factors, budget)?;
    Ok(FlowMappedLegacy {
        prefix,
        arrays,
        tripled,
    })
}

fn optional_legacy(
    cursor: &mut ArchiveCursor<'_>,
    version: u32,
    budget: &mut Budget,
) -> Result<Option<FlowMappedLegacy>> {
    if version == 5 {
        return read_legacy(cursor, budget).map(Some);
    }
    Ok(None)
}
