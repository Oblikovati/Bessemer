//! Shared serialized profile framing; callers select the accepted entry dialect.
use super::*;
use std::collections::BTreeSet;
pub(super) fn references(
    cursor: &mut ArchiveCursor<'_>,
    count: usize,
    keys: &BTreeSet<[u8; 16]>,
    label: &str,
) -> Result<Vec<FlowGuid>> {
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        let key = guid(cursor)?;
        if !keys.contains(key.as_bytes()) {
            return Err(metadata_invalid(format!(
                "reference key {key}; expected membership in {label}"
            )));
        }
        result.push(key);
    }
    Ok(result)
}
pub(super) fn guid(cursor: &mut ArchiveCursor<'_>) -> Result<FlowGuid> {
    let mut key = [0; 16];
    key.copy_from_slice(cursor.take(16)?);
    Ok(FlowGuid::from_bytes(key))
}
pub(super) fn zero(cursor: &mut ArchiveCursor<'_>) -> Result<()> {
    let version = cursor.u8()?;
    if version != 0 {
        return Err(metadata_invalid(format!(
            "profile table/reference version {version}; expected zero"
        )));
    }
    Ok(())
}
pub(super) fn table_count(
    cursor: &mut ArchiveCursor<'_>,
    minimum: usize,
    maximum: usize,
    label: &str,
) -> Result<usize> {
    zero(cursor)?;
    let count = cursor.nonnegative_count(minimum, label)?;
    if count > maximum {
        return Err(metadata_invalid(format!(
            "{label} count {count}; expected at most {maximum}"
        )));
    }
    Ok(count)
}
pub(super) fn spend(remaining: &mut usize, count: usize, label: &str) -> Result<()> {
    if count > *remaining {
        return Err(metadata_invalid(format!(
            "{label} count {count}; expected at most {} remaining",
            *remaining
        )));
    }
    *remaining -= count;
    Ok(())
}
