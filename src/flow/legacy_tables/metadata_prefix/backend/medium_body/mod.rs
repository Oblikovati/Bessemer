//! Shared exact medium-body prefix, curve framing and bounded borrowed numeric arrays.
use super::*;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CurveSpan {
    pub start: usize,
    pub end: usize,
    pub scalar: usize,
    pub count: usize,
    pub flag: usize,
    pub arrays: [(usize, usize); 4],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PrefixSpan {
    pub name: TextSpan,
    pub curves: Arc<[CurveSpan]>,
    pub arrays: [(usize, usize); 6],
}
pub(super) fn prefix(
    cursor: &mut ArchiveCursor<'_>,
    raw: &[u8],
    maximum_units: usize,
    remaining: &mut usize,
    label: &str,
) -> Result<PrefixSpan> {
    let name = header(cursor, raw, maximum_units, label)?;
    let curves = curves(cursor, raw.len(), 7, remaining, label)?;
    let arrays = shared_arrays(cursor, raw.len(), remaining)?;
    Ok(PrefixSpan {
        name,
        curves,
        arrays,
    })
}
pub(super) fn curves(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    count: usize,
    remaining: &mut usize,
    label: &str,
) -> Result<Arc<[CurveSpan]>> {
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(curve(cursor, length, remaining, label)?);
    }
    Ok(result.into())
}
pub(super) fn curve(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    remaining: &mut usize,
    label: &str,
) -> Result<CurveSpan> {
    let start = length - cursor.remaining();
    zero(cursor, label)?;
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
pub(super) fn words(
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
pub(super) fn zero(cursor: &mut ArchiveCursor<'_>, label: &str) -> Result<()> {
    let version = cursor.u8()?;
    if version != 0 {
        return Err(metadata_invalid(format!(
            "{label} version {version}; expected zero"
        )));
    }
    Ok(())
}
fn header(
    cursor: &mut ArchiveCursor<'_>,
    raw: &[u8],
    maximum_units: usize,
    label: &str,
) -> Result<TextSpan> {
    zero(cursor, label)?;
    cursor.u8()?;
    cursor.take(16)?;
    let mut remaining = maximum_units;
    super::super::decode::text(cursor, raw, &mut remaining)
}

pub(super) fn byte_position(cursor: &mut ArchiveCursor<'_>, length: usize) -> Result<usize> {
    let start = length - cursor.remaining();
    cursor.u8()?;
    Ok(start)
}

pub(super) fn curve_array(
    raw: &[u8],
    curve: &CurveSpan,
    role: FlowMediumCurveArray,
) -> Option<(usize, usize)> {
    let index = match role {
        FlowMediumCurveArray::First => 0,
        FlowMediumCurveArray::Second => 1,
        FlowMediumCurveArray::ExtraFirst => 2,
        FlowMediumCurveArray::ExtraSecond => 3,
    };
    if index >= 2 && raw[curve.flag] == 0 {
        return None;
    }
    Some(curve.arrays[index])
}
