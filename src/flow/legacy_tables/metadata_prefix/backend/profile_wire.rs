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

#[derive(Clone, Debug)]
pub(super) struct OuterReferences {
    pub start: usize,
    pub keys: Arc<[FlowGuid]>,
    pub attributes_start: usize,
    pub attributes: Arc<[i32]>,
}

pub(super) fn validate_profile_options(options: FlowMediumProfileOptions) -> Result<()> {
    let bounds = [
        options.maximum_first_objects,
        options.maximum_reference_objects,
        options.maximum_references,
        options.maximum_total_wide_words,
        options.maximum_total_name_units,
        options.body.maximum_body_bytes,
        options.body.maximum_wide_words,
        options.body.maximum_name_units,
    ];
    if bounds.contains(&0) {
        return Err(metadata_invalid(format!(
            "profile options {options:?}; expected all independent bounds positive"
        )));
    }
    Ok(())
}

pub(super) fn outer_references(
    cursor: &mut ArchiveCursor<'_>,
    raw_len: usize,
    second_keys: &BTreeSet<[u8; 16]>,
    remaining: &mut usize,
    profile: &str,
) -> Result<OuterReferences> {
    let count = cursor.nonnegative_count(20, "outer references")?;
    spend(remaining, count, "total reference occurrences")?;
    let start = raw_len - cursor.remaining();
    let keys = references(cursor, count, second_keys, "fresh second table")?;
    let (attributes_start, attributes) = read_attributes(cursor, raw_len, count, profile)?;
    Ok(OuterReferences {
        start,
        keys: keys.into(),
        attributes_start,
        attributes: attributes.into(),
    })
}
fn read_attributes(
    cursor: &mut ArchiveCursor<'_>,
    raw_len: usize,
    count: usize,
    profile: &str,
) -> Result<(usize, Vec<i32>)> {
    let attributes_start = raw_len - cursor.remaining();
    let mut attributes = Vec::with_capacity(count);
    for _ in 0..count {
        attributes.push(cursor.i32()?);
    }
    final_empty(cursor, profile)?;
    final_empty(cursor, profile)?;
    Ok((attributes_start, attributes))
}
fn final_empty(cursor: &mut ArchiveCursor<'_>, profile: &str) -> Result<()> {
    let count = cursor.i32()?;
    if count != 0 {
        return Err(metadata_invalid(format!(
            "final outer collection count {count}; expected zero for supported {profile} profile"
        )));
    }
    Ok(())
}
