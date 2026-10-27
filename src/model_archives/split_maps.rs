use super::{ModelField, ModelFieldValue, PartResolvedFeaturesArchive};
use crate::Result;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Immutable raw split-map entries and the separately derived native insertion result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SplitIntegerMap {
    path: Arc<str>,
    count: i32,
    pairs: Vec<(i32, i32)>,
    loaded: BTreeMap<i32, i32>,
}

impl SplitIntegerMap {
    /// Identifies the physical map occurrence, without document naming policy.
    /// ```no_run
    /// # fn example(map: &bessemer::model_archives::SplitIntegerMap) {
    /// let occurrence = map.path();
    /// # }
    /// ```
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the signed stored count, including negative no-entry values.
    /// ```no_run
    /// # fn example(map: &bessemer::model_archives::SplitIntegerMap) {
    /// let stored_count = map.raw_count();
    /// # }
    /// ```
    pub fn raw_count(&self) -> i32 {
        self.count
    }

    /// Retains every consumed pair in wire order, including duplicates and negative entries.
    /// ```no_run
    /// # fn example(map: &bessemer::model_archives::SplitIntegerMap) {
    /// let stored_pairs = map.raw_pairs();
    /// # }
    /// ```
    pub fn raw_pairs(&self) -> &[(i32, i32)] {
        &self.pairs
    }

    /// Applies native nonnegative filtering and last accepted duplicate replacement.
    /// ```no_run
    /// # fn example(map: &bessemer::model_archives::SplitIntegerMap) {
    /// let ordered_native_result = map.loaded();
    /// # }
    /// ```
    pub fn loaded(&self) -> &BTreeMap<i32, i32> {
        &self.loaded
    }
}

impl PartResolvedFeaturesArchive {
    /// Derives immutable split maps without changing raw fields or stored bytes.
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartResolvedFeaturesArchive) -> bessemer::Result<()> {
    /// let maps = archive.split_integer_maps()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn split_integer_maps(&self) -> Result<Vec<SplitIntegerMap>> {
        read(self.fields())
    }
}

pub(super) fn read(fields: &[ModelField]) -> Result<Vec<SplitIntegerMap>> {
    fields
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            field
                .path
                .strip_suffix(".map_9a8_count")
                .map(|parent| parse(fields, index, parent))
        })
        .collect()
}

fn parse(fields: &[ModelField], index: usize, parent: &str) -> Result<SplitIntegerMap> {
    let count = signed(fields.get(index), &format!("{parent}.map_9a8_count"))?;
    let pairs = pairs(fields, index + 1, count.max(0) as usize, parent)?;
    let loaded = pairs
        .iter()
        .copied()
        .filter(|&(key, value)| key >= 0 && value >= 0)
        .collect();
    Ok(SplitIntegerMap {
        path: Arc::from(format!("{parent}.map_9a8")),
        count,
        pairs,
        loaded,
    })
}

fn pairs(
    fields: &[ModelField],
    start: usize,
    count: usize,
    parent: &str,
) -> Result<Vec<(i32, i32)>> {
    pair_count(count, fields.len().saturating_sub(start) / 2)?;
    (0..count)
        .map(|index| {
            pair(
                fields,
                start + 2 * index,
                &format!("{parent}.raw_map_9a8[{index}]"),
            )
        })
        .collect()
}

fn pair_count(count: usize, available: usize) -> Result<()> {
    if count <= available {
        return Ok(());
    }
    Err(super::invalid(format!(
        "split map count {count}; expected at most {available} complete i32 pairs"
    )))
}

fn pair(fields: &[ModelField], start: usize, prefix: &str) -> Result<(i32, i32)> {
    Ok((
        signed(fields.get(start), &format!("{prefix}.key"))?,
        signed(fields.get(start + 1), &format!("{prefix}.value"))?,
    ))
}

fn signed(field: Option<&ModelField>, path: &str) -> Result<i32> {
    match field {
        Some(ModelField {
            path: actual,
            value: ModelFieldValue::I32(value),
        }) if actual.as_ref() == path => Ok(*value),
        actual => Err(super::invalid(format!(
            "split map field {}; expected {path} = i32",
            super::field_mismatch::field_summary(actual)
        ))),
    }
}
