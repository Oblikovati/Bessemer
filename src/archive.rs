use std::collections::BTreeSet;
use std::ops::Range;

use crate::records::{archive_offset, dword, equal, invalid, name_bytes, region, require, word};
use crate::{BessemerError, FailureClass, Result};

#[derive(Debug, Clone)]
pub(crate) struct Entry {
    pub name: String,
    pub local: Vec<u8>,
    pub central: Vec<u8>,
    pub stored: Range<usize>,
    pub expanded: u32,
    pub crc: u32,
    pub method: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct Directory {
    pub entries: Vec<Entry>,
    pub end_signature: Vec<u8>,
}

struct ArchiveLayout {
    start: usize,
    end: usize,
    count: u16,
    central: Vec<u8>,
    local: Vec<u8>,
}

pub(crate) fn parse(bytes: &[u8], version: u32) -> Result<Directory> {
    let mut failure = invalid(format!(
        "{} bytes; expected a nonempty SolidWorks directory",
        bytes.len()
    ));
    for end in (8..bytes.len().saturating_sub(21)).rev() {
        if !possible_end(bytes, end) {
            continue;
        }
        match candidate(bytes, version, end) {
            Ok(directory) => return Ok(directory),
            Err(error) if error.class == FailureClass::Unsupported => failure = error,
            Err(_) => {}
        }
    }
    Err(failure)
}

fn possible_end(bytes: &[u8], end: usize) -> bool {
    bytes[end + 4..end + 8] == [0; 4]
        && bytes[end + 8..end + 10] == bytes[end + 10..end + 12]
        && bytes[end + 10..end + 12] != [0; 2]
}

fn candidate(bytes: &[u8], version: u32, end: usize) -> Result<Directory> {
    let layout = archive_layout(bytes, end)?;
    let entries = read_directory(bytes, version, &layout)?;
    validate_entries(&entries, layout.start)?;
    Ok(Directory {
        entries,
        end_signature: region(bytes, end, 4)?.to_vec(),
    })
}

fn archive_layout(bytes: &[u8], end: usize) -> Result<ArchiveLayout> {
    equal(word(bytes, end + 20)?, 0, "archive comment length")?;
    let start = archive_offset(bytes, end + 16)?;
    let size = dword(bytes, end + 12)? as usize;
    require(
        start.checked_add(size) == Some(end),
        format!("directory {start}+{size}; expected end {end}"),
    )?;
    let central = region(bytes, start, 4)?.to_vec();
    let local_at = archive_offset(bytes, start + 42)?;
    let local = region(bytes, local_at, 4)?.to_vec();
    Ok(ArchiveLayout {
        start,
        end,
        count: word(bytes, end + 10)?,
        central,
        local,
    })
}

fn read_directory(bytes: &[u8], version: u32, layout: &ArchiveLayout) -> Result<Vec<Entry>> {
    let mut entries = Vec::new();
    let mut cursor = layout.start;
    for _ in 0..layout.count {
        let entry = read_entry(bytes, version, cursor, layout)?;
        cursor += entry.central.len();
        require(
            cursor <= layout.end,
            format!("central entry end {cursor}; expected <= {}", layout.end),
        )?;
        entries.push(entry);
    }
    Ok(entries)
}

fn validate_entries(entries: &[Entry], central_start: usize) -> Result<()> {
    let mut names = BTreeSet::new();
    let mut spans = Vec::new();
    for entry in entries {
        require(
            names.insert(&entry.name),
            format!("duplicate stream {:?}; expected unique names", entry.name),
        )?;
        spans.push(entry.stored.start - entry.local.len()..entry.stored.end);
    }
    validate_spans(&mut spans, central_start)
}

fn validate_spans(spans: &mut [Range<usize>], central_start: usize) -> Result<()> {
    spans.sort_by_key(|span| span.start);
    let mut previous_end = 8;
    for span in spans {
        require(
            span.start >= previous_end && span.end <= central_start,
            format!("local range {span:?}; expected disjoint records before {central_start}"),
        )?;
        previous_end = span.end;
    }
    Ok(())
}

fn read_entry(bytes: &[u8], version: u32, cursor: usize, layout: &ArchiveLayout) -> Result<Entry> {
    let central = central_record(bytes, cursor, &layout.central)?;
    let local_at = archive_offset(&central, 42)?;
    let local = local_record(bytes, local_at)?;
    let raw = region(&central, 46, word(&central, 28)? as usize)?;
    validate_headers(&central, &local, &layout.local, raw)?;
    let start = local_at + local.len();
    let stored = start..start + dword(&central, 20)? as usize;
    region(bytes, stored.start, stored.len())?;
    entry_fields(central, local, stored, version)
}

fn entry_fields(
    central: Vec<u8>,
    local: Vec<u8>,
    stored: Range<usize>,
    version: u32,
) -> Result<Entry> {
    let raw = region(&central, 46, word(&central, 28)? as usize)?;
    let name = String::from_utf8(name_bytes(raw, version))
        .map_err(|error| invalid(format!("name {raw:?}; expected UTF-8: {error}")))?;
    Ok(Entry {
        name,
        expanded: dword(&central, 24)?,
        crc: dword(&central, 16)?,
        method: word(&central, 10)?,
        local,
        central,
        stored,
    })
}

fn central_record(bytes: &[u8], cursor: usize, signature: &[u8]) -> Result<Vec<u8>> {
    let header = region(bytes, cursor, 46)?;
    equal(&header[..4], signature, "central signature")?;
    let length =
        46 + word(header, 28)? as usize + word(header, 30)? as usize + word(header, 32)? as usize;
    Ok(region(bytes, cursor, length)?.to_vec())
}

fn local_record(bytes: &[u8], offset: usize) -> Result<Vec<u8>> {
    let header = region(bytes, offset, 30)?;
    let length = 30 + word(header, 26)? as usize + word(header, 28)? as usize;
    Ok(region(bytes, offset, length)?.to_vec())
}

fn validate_headers(central: &[u8], local: &[u8], signature: &[u8], raw_name: &[u8]) -> Result<()> {
    equal(&local[..4], signature, "local signature")?;
    equal(&local[4..26], &central[6..28], "local fields")?;
    equal(
        region(local, 30, word(local, 26)? as usize)?,
        raw_name,
        "local name",
    )?;
    equal(word(central, 34)?, 0, "entry disk")?;
    validate_encoding(local)
}

fn validate_encoding(local: &[u8]) -> Result<()> {
    let flags = word(local, 6)?;
    let method = word(local, 8)?;
    let version = word(local, 4)?;
    if flags & 0x2049 == 0 && matches!(method, 0 | 8) && version == 20 {
        return Ok(());
    }
    Err(BessemerError::new(
        "parse",
        "record",
        FailureClass::Unsupported,
        format!(
            "flags {flags:#x}, method {method}, version {version}; expected unencrypted record without descriptor, method 0/8, version 20"
        ),
    ))
}
