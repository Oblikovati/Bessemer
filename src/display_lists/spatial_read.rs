use super::{
    DisplayVersion, DisplayViewLimits, SpatialView, transform_codec, view_bitmap,
    view_plane::{self, PlaneGates},
    view_support::{ViewBudget, require_size},
};
use crate::{
    Result,
    archive_codec::{ArchiveCursor, ArchiveString},
};

pub(super) fn read(
    bytes: &[u8],
    version: DisplayVersion,
    limits: DisplayViewLimits,
) -> Result<Option<Vec<SpatialView>>> {
    require_size(bytes.len(), limits.maximum_bytes, "3D-view input bytes")?;
    let mut cursor = ArchiveCursor::new(bytes);
    let views = if version.value() > 0x1c18 {
        Some(read_views(&mut cursor, version, limits)?)
    } else {
        None
    };
    cursor.finish()?;
    Ok(views)
}

fn read_views(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    limits: DisplayViewLimits,
) -> Result<Vec<SpatialView>> {
    let count = cursor.nonnegative_count(74, "3D views")?;
    require_size(count, limits.maximum_views, "3D views")?;
    let mut budget = ViewBudget::new(limits);
    (0..count)
        .map(|_| read_view(cursor, version, &mut budget))
        .collect()
}

struct SpatialHeader {
    view_name: ArchiveString,
    unknown_active: Option<i32>,
    model_to_view: super::CompactTransform,
    pan_zoom: super::CompactTransform,
    unknown_string: ArchiveString,
    preview_bitmap: Option<super::ViewBitmap>,
    unknown_ids_string: Option<ArchiveString>,
}

fn read_view(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<SpatialView> {
    let header = read_header(cursor, version)?;
    let section_planes = read_planes(cursor, version, budget)?;
    let tail = read_tail(cursor, version, !section_planes.is_empty())?;
    Ok(SpatialView {
        view_name: header.view_name,
        unknown_active: header.unknown_active,
        model_to_view: header.model_to_view,
        pan_zoom: header.pan_zoom,
        unknown_string: header.unknown_string,
        preview_bitmap: header.preview_bitmap,
        unknown_ids_string: header.unknown_ids_string,
        section_planes,
        unknown_after_planes: tail.after,
        unknown_468: tail.unknown_468,
        unknown_bitset: tail.bitset,
        unknown_text_string: tail.text,
    })
}

fn read_header(cursor: &mut ArchiveCursor<'_>, version: DisplayVersion) -> Result<SpatialHeader> {
    Ok(SpatialHeader {
        view_name: ArchiveString::decode(cursor)?,
        unknown_active: integer_after(cursor, version, 0x46c9)?,
        model_to_view: transform_codec::read_compact(cursor)?,
        pan_zoom: transform_codec::read_compact(cursor)?,
        unknown_string: ArchiveString::decode(cursor)?,
        preview_bitmap: bitmap_after(cursor, version)?,
        unknown_ids_string: string_after(cursor, version, 0x1c42)?,
    })
}

fn bitmap_after(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
) -> Result<Option<super::ViewBitmap>> {
    if version.value() <= 0x1c1e {
        return Ok(None);
    }
    Ok(Some(view_bitmap::read(cursor)?))
}

fn read_planes(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    budget: &mut ViewBudget,
) -> Result<Vec<super::ViewSectionPlane>> {
    view_plane::read_list(
        cursor,
        version,
        budget,
        PlaneGates {
            pair_after: 0x2012,
            word_after: 0x2012,
            pair_label: "3D plane pair",
            word_label: "3D plane word",
        },
        "3D planes",
    )
}

struct SpatialTail {
    after: Option<i32>,
    unknown_468: Option<i32>,
    bitset: Option<[i32; 2]>,
    text: Option<ArchiveString>,
}

fn read_tail(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    populated: bool,
) -> Result<SpatialTail> {
    let after = if populated {
        integer_after(cursor, version, 0x2012)?
    } else {
        None
    };
    let unknown_468 = integer_after(cursor, version, 0x1fb5)?;
    let bitset = if version.value() > 0x1fb5 {
        Some([cursor.i32()?, cursor.i32()?])
    } else {
        None
    };
    let text = string_after(cursor, version, 0x3331)?;
    Ok(SpatialTail {
        after,
        unknown_468,
        bitset,
        text,
    })
}

fn integer_after(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    threshold: u32,
) -> Result<Option<i32>> {
    if version.value() <= threshold {
        return Ok(None);
    }
    Ok(Some(cursor.i32()?))
}

fn string_after(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    threshold: u32,
) -> Result<Option<ArchiveString>> {
    if version.value() <= threshold {
        return Ok(None);
    }
    Ok(Some(ArchiveString::decode(cursor)?))
}
