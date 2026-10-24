use super::{
    DisplayVersion, DisplayViewLimits, NamedSectionPlane, NamedSectionSelections,
    NamedSelectionValues, NamedView, StoredDouble, transform_codec,
    view_support::{ViewBudget, require_size},
};
use crate::{
    Result,
    archive_codec::{ArchiveCursor, ArchiveString, read_point},
};

pub(super) fn read(
    bytes: &[u8],
    version: DisplayVersion,
    limits: DisplayViewLimits,
) -> Result<Option<Vec<NamedView>>> {
    require_size(bytes.len(), limits.maximum_bytes, "named-view input bytes")?;
    let mut cursor = ArchiveCursor::new(bytes);
    let mut budget = ViewBudget::new(limits);
    let views = if version.value() > 0x1be7 {
        Some(read_views(&mut cursor, version, &mut budget)?)
    } else {
        None
    };
    cursor.finish()?;
    Ok(views)
}

fn read_views(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<Vec<NamedView>> {
    let count = cursor.nonnegative_count(70, "named views")?;
    require_size(count, budget.limits.maximum_views, "named views")?;
    (0..count)
        .map(|_| read_view(cursor, version, budget))
        .collect()
}

fn read_view(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<NamedView> {
    let view_name = ArchiveString::decode(cursor)?;
    let model_to_view = transform_codec::read_compact(cursor)?;
    let pan_zoom = transform_codec::read_compact(cursor)?;
    let unknown_string = ArchiveString::decode(cursor)?;
    let tail = read_tail(cursor, version, budget)?;
    Ok(NamedView {
        view_name,
        model_to_view,
        pan_zoom,
        unknown_string,
        section_planes: tail.section_planes,
        selections: tail.selections,
        unknown_bitset: tail.unknown_bitset,
    })
}

struct NamedTail {
    section_planes: Option<Vec<NamedSectionPlane>>,
    selections: Option<NamedSectionSelections>,
    unknown_bitset: Option<[i32; 2]>,
}

fn read_tail(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<NamedTail> {
    let section_planes = conditional_planes(cursor, version, budget)?;
    let selected = version.value() > 0x2003
        && section_planes
            .as_ref()
            .is_some_and(|planes| !planes.is_empty());
    let selections = if selected {
        Some(read_selections(cursor, budget)?)
    } else {
        None
    };
    let unknown_bitset = read_bitset(cursor, version)?;
    Ok(NamedTail {
        section_planes,
        selections,
        unknown_bitset,
    })
}

fn read_bitset(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
) -> Result<Option<[i32; 2]>> {
    if version.value() <= 0x200d {
        return Ok(None);
    }
    Ok(Some([cursor.i32()?, cursor.i32()?]))
}

fn conditional_planes(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<Option<Vec<NamedSectionPlane>>> {
    if version.value() <= 0x1bf4 {
        return Ok(None);
    }
    Ok(Some(read_planes(cursor, version, budget)?))
}

fn read_planes(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<Vec<NamedSectionPlane>> {
    let count = cursor.nonnegative_count(60, "named section planes")?;
    budget.nested(count, "named section planes")?;
    (0..count).map(|_| read_plane(cursor, version)).collect()
}

fn read_plane(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
) -> Result<NamedSectionPlane> {
    Ok(NamedSectionPlane {
        root_point: read_point(cursor)?,
        normal: read_point(cursor)?,
        unknown_double: StoredDouble::from_bits(cursor.u64()?),
        unknown_i0: cursor.i32()?,
        unknown_pair: if version.value() > 0x2001 {
            Some([cursor.i32()?, cursor.i32()?])
        } else {
            None
        },
        unknown_word: if version.value() > 0x2007 {
            Some(cursor.u32()?)
        } else {
            None
        },
    })
}

fn read_selections(
    cursor: &mut ArchiveCursor<'_>,
    budget: &mut ViewBudget,
) -> Result<NamedSectionSelections> {
    let has_selections = cursor.i32()?;
    let values = if has_selections != 0 {
        Some(read_selection_values(cursor, budget)?)
    } else {
        None
    };
    Ok(NamedSectionSelections {
        has_selections,
        values,
    })
}

fn read_selection_values(
    cursor: &mut ArchiveCursor<'_>,
    budget: &mut ViewBudget,
) -> Result<NamedSelectionValues> {
    let unknown_sel0 = cursor.i32()?;
    let is_part = cursor.i32()?;
    let count = cursor.nonnegative_count(1, "named selection names")?;
    budget.nested(count, "named selection names")?;
    let names = (0..count)
        .map(|_| ArchiveString::decode(cursor))
        .collect::<Result<Vec<_>>>()?;
    Ok(NamedSelectionValues {
        unknown_sel0,
        is_part,
        names,
    })
}
