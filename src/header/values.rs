use crate::Result;
use crate::archive_codec::{ArchiveCursor, ArchiveString};

use super::fields::{FieldValue, FieldView, WireKind};
use super::{HeaderBounds, HeaderBox, SharedFileName, StoredDouble, invalid};

pub(super) fn read_scalar(cursor: &mut ArchiveCursor<'_>, kind: WireKind) -> Result<FieldValue> {
    match kind {
        WireKind::U8 => Ok(FieldValue::U8(cursor.u8()?)),
        WireKind::U16 => Ok(FieldValue::U16(cursor.u16()?)),
        WireKind::U32 => Ok(FieldValue::U32(cursor.u32()?)),
        WireKind::I32 => Ok(FieldValue::I32(cursor.i32()?)),
        WireKind::Text => Ok(FieldValue::Text(ArchiveString::decode(cursor)?)),
        WireKind::Texts => Ok(FieldValue::Texts(read_texts(cursor)?)),
        WireKind::I32x3 => Ok(FieldValue::I32x3(read_integers(cursor)?)),
        WireKind::I32x6 => Ok(FieldValue::I32x6(read_integers(cursor)?)),
        WireKind::Bounds => Ok(FieldValue::Bounds(read_bounds(cursor)?)),
        other => Err(invalid(format!(
            "scalar kind {other:?}; expected a non-reference field"
        ))),
    }
}

fn read_texts(cursor: &mut ArchiveCursor<'_>) -> Result<Vec<ArchiveString>> {
    let count = cursor.collection_count(1)?;
    (0..count).map(|_| ArchiveString::decode(cursor)).collect()
}

fn read_integers<const N: usize>(cursor: &mut ArchiveCursor<'_>) -> Result<[i32; N]> {
    let mut result = [0; N];
    for value in &mut result {
        *value = cursor.i32()?;
    }
    Ok(result)
}

fn read_point(cursor: &mut ArchiveCursor<'_>) -> Result<[StoredDouble; 3]> {
    let mut point = [StoredDouble::from_bits(0); 3];
    for coordinate in &mut point {
        *coordinate = StoredDouble::from_bits(cursor.u64()?);
    }
    Ok(point)
}

fn read_bounds(cursor: &mut ArchiveCursor<'_>) -> Result<HeaderBounds> {
    let has_box = cursor.i32()?;
    if has_box == 0 {
        return Ok(HeaderBounds {
            has_box,
            bounds: None,
        });
    }
    let bounds = HeaderBox {
        centre: read_point(cursor)?,
        high: read_point(cursor)?,
        low: read_point(cursor)?,
        radius: StoredDouble::from_bits(cursor.u64()?),
    };
    Ok(HeaderBounds {
        has_box,
        bounds: Some(bounds),
    })
}

pub(super) fn write_count(count: usize, output: &mut Vec<u8>) -> Result<()> {
    let count = u32::try_from(count).map_err(|_| {
        invalid(format!(
            "collection length {count}; expected at most {}",
            u32::MAX
        ))
    })?;
    if count < u32::from(u16::MAX) {
        output.extend_from_slice(&(count as u16).to_le_bytes());
        return Ok(());
    }
    output.extend_from_slice(&u16::MAX.to_le_bytes());
    output.extend_from_slice(&count.to_le_bytes());
    Ok(())
}

pub(super) fn write_scalar(value: FieldView<'_>, output: &mut Vec<u8>) -> Result<()> {
    match value {
        FieldView::U8(value) => output.push(value),
        FieldView::U16(value) => output.extend_from_slice(&value.to_le_bytes()),
        FieldView::U32(value) => output.extend_from_slice(&value.to_le_bytes()),
        FieldView::I32(value) => output.extend_from_slice(&value.to_le_bytes()),
        FieldView::Text(text) => text.encode(output)?,
        FieldView::Texts(texts) => write_texts(texts, output)?,
        FieldView::I32x3(values) => write_integers(values, output),
        FieldView::I32x6(values) => write_integers(values, output),
        FieldView::Bounds(bounds) => write_bounds(bounds, output)?,
        FieldView::SharedName(SharedFileName::Text(text)) => text.encode(output)?,
        _ => return Err(invalid("reference field; expected a scalar encoding task")),
    }
    Ok(())
}

fn write_texts(texts: &[ArchiveString], output: &mut Vec<u8>) -> Result<()> {
    write_count(texts.len(), output)?;
    for text in texts {
        text.encode(output)?;
    }
    Ok(())
}

fn write_integers(values: &[i32], output: &mut Vec<u8>) {
    for value in values {
        output.extend_from_slice(&value.to_le_bytes());
    }
}

pub(super) fn require_bounds(bounds: &HeaderBounds) -> Result<()> {
    if (bounds.has_box != 0) != bounds.bounds.is_some() {
        return Err(invalid(format!(
            "box flag {} and body {:?}; expected body exactly for nonzero flag",
            bounds.has_box, bounds.bounds
        )));
    }
    Ok(())
}

fn write_bounds(bounds: &HeaderBounds, output: &mut Vec<u8>) -> Result<()> {
    require_bounds(bounds)?;
    output.extend_from_slice(&bounds.has_box.to_le_bytes());
    let Some(bounds) = &bounds.bounds else {
        return Ok(());
    };
    for point in [&bounds.centre, &bounds.high, &bounds.low] {
        for coordinate in point {
            output.extend_from_slice(&coordinate.bits().to_le_bytes());
        }
    }
    output.extend_from_slice(&bounds.radius.bits().to_le_bytes());
    Ok(())
}
