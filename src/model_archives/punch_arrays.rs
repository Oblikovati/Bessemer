use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::{MaterialRead, vectors::double_vector};

struct PunchArrayLayout {
    rows: usize,
    columns: usize,
}

/// Preserve dynamic-width punch points or signed integers after validating the full extent.
/// Example: `punch_arrays::read(reader, true)?` for a point array.
pub(super) fn read(reader: &mut ModelDecoder<'_>, points: bool) -> Result<()> {
    let Some(layout) = layout(reader, if points { 24 } else { 4 })? else {
        return Ok(());
    };
    for row in 0..layout.rows {
        for column in 0..layout.columns {
            let label = format!("values[{row}][{column}]");
            if points {
                double_vector(reader, &label)?;
            } else {
                reader.i32(&label)?;
            }
        }
    }
    Ok(())
}

fn layout(reader: &mut ModelDecoder<'_>, unit: u32) -> Result<Option<PunchArrayLayout>> {
    let width = reader.u32("element_width")?;
    if width == 0 {
        return Ok(None);
    }
    require_width(reader, width, unit)?;
    reader.u32("raw_metadata_1")?;
    reader.u32("raw_metadata_2")?;
    let rows = reader.unsigned_count("count", width as usize)?;
    let columns = (width / unit) as usize;
    let count = rows.checked_mul(columns).ok_or_else(|| {
        super::invalid(format!(
            "punch array {rows} rows * {columns} values; expected a representable value count"
        ))
    });
    let count = reader.checked(count)?;
    reader.entries(count, "punch dynamic-array values")?;
    Ok(Some(PunchArrayLayout { rows, columns }))
}

fn require_width(reader: &mut ModelDecoder<'_>, width: u32, unit: u32) -> Result<()> {
    if width.is_multiple_of(unit) {
        return Ok(());
    }
    reader.checked(Err(super::invalid(format!(
        "punch array element width {width}; expected a nonzero multiple of {unit} bytes"
    ))))
}
