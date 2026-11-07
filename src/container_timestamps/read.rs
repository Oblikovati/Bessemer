use super::*;
use crate::records::{dword, region};
impl ZipEntryTimestamps {
    /// Captures one bounded current wire; e.g. `let times = ZipEntryTimestamps::from_bytes(&document.to_bytes()?, 16_000_000)?`.
    pub fn from_bytes(raw: &[u8], maximum_file_bytes: usize) -> Result<Self> {
        if raw.len() > maximum_file_bytes {
            return Err(failure(
                "file",
                FailureClass::Invalid,
                format!(
                    "file bytes {}; expected at most configured bound {maximum_file_bytes}",
                    raw.len()
                ),
            ));
        }
        let document = Document::from_bytes(raw)?;
        let entries = read_entries(&document)?;
        Ok(Self {
            revision: ArchiveRevision::default(),
            raw: Arc::from(document.original),
            entries: entries.into(),
            maximum: maximum_file_bytes,
        })
    }
}
fn read_entries(document: &Document) -> Result<Vec<TimestampSpan>> {
    let mut central = document.directory.central_start;
    document
        .directory
        .entries
        .iter()
        .map(|entry| {
            let span = read_span(document, entry, central)?;
            central = central.checked_add(entry.central.len()).ok_or_else(|| {
                failure(
                    &entry.name,
                    FailureClass::Invalid,
                    "directory end overflow; expected a complete header within the file",
                )
            })?;
            Ok(span)
        })
        .collect()
}
fn read_span(
    document: &Document,
    entry: &crate::archive::Entry,
    central: usize,
) -> Result<TimestampSpan> {
    let local = local_header_start(entry)?;
    let local = checked_interval(&document.original, local, 10)?;
    let central = checked_interval(&document.original, central, 12)?;
    Ok(TimestampSpan {
        name: Arc::from(entry.name.as_str()),
        local,
        central,
        packed: dword(&document.original, central.0)?,
    })
}
fn local_header_start(entry: &crate::archive::Entry) -> Result<usize> {
    entry
        .stored
        .start
        .checked_sub(entry.local.len())
        .ok_or_else(|| {
            failure(
                &entry.name,
                FailureClass::Invalid,
                format!(
                    "stored start {}; expected at least local header length {}",
                    entry.stored.start,
                    entry.local.len()
                ),
            )
        })
}
fn checked_interval(raw: &[u8], base: usize, member: usize) -> Result<(usize, usize)> {
    let start = base.checked_add(member).ok_or_else(|| {
        failure(
            "timestamp",
            FailureClass::Invalid,
            format!("header {base} member {member}; expected representable four-byte interval"),
        )
    })?;
    region(raw, start, 4)?;
    Ok((start, start + 4))
}
