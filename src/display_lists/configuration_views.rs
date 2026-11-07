use super::{DisplayDocumentKind, archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>, count: usize, views: usize) -> Result<()> {
    let (minimum, visibility_minimum) = boundaries(archive.context.kind);
    if archive.context.version.value() <= minimum {
        return Ok(());
    }
    archive.scope("view_objects", |archive| {
        matrix(archive, count, views, true)
    })?;
    archive.scope("view_display_lists", |archive| {
        matrix(archive, count, views, false)
    })?;
    if archive.context.version.value() > visibility_minimum {
        archive.scope("annotation_view_visibility", visibility)?;
    }
    Ok(())
}

fn boundaries(kind: DisplayDocumentKind) -> (u32, u32) {
    match kind {
        DisplayDocumentKind::Part => (0x2b61, 0x2b7f),
        DisplayDocumentKind::Assembly => (0x2b6f, 0x2b77),
    }
}

fn matrix(
    archive: &mut DisplayDecoder<'_>,
    count: usize,
    views: usize,
    objects: bool,
) -> Result<()> {
    matrix_budget(archive, count, views)?;
    for index in 0..count {
        archive.scope(&format!("entries[{index}]"), |archive| {
            row(archive, views, objects)
        })?;
    }
    Ok(())
}

fn row(archive: &mut DisplayDecoder<'_>, views: usize, objects: bool) -> Result<()> {
    archive.i32("config_id")?;
    for view in 0..views {
        let label = format!("views[{view}]");
        if objects {
            archive.cobject(&label)?;
        } else {
            archive.scope(&label, super::modern_items::list)?;
        }
    }
    Ok(())
}

fn matrix_budget(archive: &mut DisplayDecoder<'_>, count: usize, views: usize) -> Result<()> {
    let cells = count.checked_mul(views).ok_or_else(|| {
        invalid(format!(
            "configuration view dimensions {count} x {views}; expected usize product"
        ))
    })?;
    let available = archive.cursor.remaining();
    if count > available / 4 || cells > (available - count * 4) / 2 {
        return Err(invalid(format!(
            "configuration view dimensions {count} x {views}; expected IDs and cells fitting {available} bytes"
        )));
    }
    archive.entries(count, "configuration view records")?;
    archive.entries(cells, "configuration view cells")
}

fn visibility(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() < 0x2b7a {
        archive.i32("unknown_legacy")?;
    }
    let count = archive.count("count", 2)?;
    for index in 0..count {
        archive.text(&format!("entries[{index}].view_name"))?;
        archive.text(&format!("entries[{index}].config_name"))?;
    }
    Ok(())
}
