use super::{
    AnnotationSectionPlane, AnnotationView, AnnotationViewContents, AnnotationViewId,
    DisplayVersion, DisplayViewLimits, StoredDouble, transform_codec,
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
) -> Result<Option<AnnotationViewContents>> {
    require_size(
        bytes.len(),
        limits.maximum_bytes,
        "annotation-view input bytes",
    )?;
    let mut cursor = ArchiveCursor::new(bytes);
    let contents = if version.value() > 0x1c39 {
        Some(read_contents(&mut cursor, version, limits)?)
    } else {
        None
    };
    cursor.finish()?;
    Ok(contents)
}

fn read_contents(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    limits: DisplayViewLimits,
) -> Result<AnnotationViewContents> {
    let count = cursor.nonnegative_count(87, "annotation views")?;
    require_size(count, limits.maximum_views, "annotation views")?;
    let mut budget = ViewBudget::new(limits);
    let views = (0..count)
        .map(|_| read_view(cursor, version, &mut budget))
        .collect::<Result<Vec<_>>>()?;
    let unknown_bitset = if version.value() > 0x1fa8 {
        Some([cursor.i32()?, cursor.i32()?])
    } else {
        None
    };
    Ok(AnnotationViewContents {
        views,
        unknown_bitset,
    })
}

fn read_view(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<AnnotationView> {
    Ok(AnnotationView {
        view_name: ArchiveString::decode(cursor)?,
        view_id: AnnotationViewId::new(cursor.i32()?),
        is_shown: cursor.i32()?,
        is_current: cursor.i32()?,
        unknown_flag: read_flag(cursor, version)?,
        unknown_name2: ArchiveString::decode(cursor)?,
        model_to_view: transform_codec::read_compact(cursor)?,
        pan_zoom: transform_codec::read_compact(cursor)?,
        unknown_string: ArchiveString::decode(cursor)?,
        section_planes: read_planes(cursor, budget)?,
    })
}

fn read_flag(cursor: &mut ArchiveCursor<'_>, version: DisplayVersion) -> Result<Option<i32>> {
    if version.value() <= 0x1c40 {
        return Ok(None);
    }
    Ok(Some(cursor.i32()?))
}

fn read_planes(
    cursor: &mut ArchiveCursor<'_>,
    budget: &mut ViewBudget,
) -> Result<Vec<AnnotationSectionPlane>> {
    let count = cursor.nonnegative_count(60, "annotation planes")?;
    budget.nested(count, "annotation planes")?;
    (0..count).map(|_| read_plane(cursor)).collect()
}

fn read_plane(cursor: &mut ArchiveCursor<'_>) -> Result<AnnotationSectionPlane> {
    Ok(AnnotationSectionPlane {
        root_point: read_point(cursor)?,
        normal: read_point(cursor)?,
        unknown_double: StoredDouble::from_bits(cursor.u64()?),
        unknown_integer: cursor.i32()?,
    })
}
