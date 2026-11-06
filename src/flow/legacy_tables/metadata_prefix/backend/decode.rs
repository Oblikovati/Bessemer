use super::*;
pub(super) fn read(prefix: &FlowMetadataPrefix) -> Result<FlowMediumBackendPrefix> {
    require_active(prefix)?;
    let raw = prefix.opaque_tail();
    let mut cursor = ArchiveCursor::new(raw);
    zero(&mut cursor)?;
    let mut tables = Vec::with_capacity(2);
    for ordinal in 0..2 {
        let table = table(&mut cursor, raw.len(), ordinal)?;
        let populated = table.count > 0;
        tables.push(table);
        if populated {
            break;
        }
    }
    Ok(FlowMediumBackendPrefix {
        prefix: prefix.clone(),
        tables: tables.into(),
        end: raw.len() - cursor.remaining(),
    })
}
fn zero(cursor: &mut ArchiveCursor<'_>) -> Result<()> {
    let version = cursor.u8()?;
    if version != 0 {
        return Err(metadata_invalid(format!(
            "backend/table version {version}; expected zero"
        )));
    }
    Ok(())
}
fn table(
    cursor: &mut ArchiveCursor<'_>,
    length: usize,
    ordinal: usize,
) -> Result<BackendTableSpan> {
    let start = length - cursor.remaining();
    zero(cursor)?;
    let count = cursor.i32()?;
    if count < 0 {
        return Err(metadata_invalid(format!(
            "backend table {ordinal} count {count}; expected 0..=2147483647"
        )));
    }
    let kind = if count > 0 { Some(cursor.i32()?) } else { None };
    let key = ordinal == 0 && kind == Some(1);
    if key {
        cursor.take(16)?;
    }
    Ok(BackendTableSpan {
        start,
        end: length - cursor.remaining(),
        count,
        kind,
        key,
    })
}

fn require_active(prefix: &FlowMetadataPrefix) -> Result<()> {
    if !prefix.body_active() {
        return Err(metadata_invalid(format!(
            "signed outer version {}; expected active version 2..=127 for backend framing",
            prefix.signed_version()
        )));
    }
    Ok(())
}
