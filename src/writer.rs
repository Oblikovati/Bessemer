use crate::archive::Entry;
use crate::records::invalid;
use crate::{Document, Result, compression};

pub(crate) fn serialize(document: &Document) -> Result<Vec<u8>> {
    let mut output = document.original[..8].to_vec();
    let mut central = Vec::new();
    let mut entries: Vec<&Entry> = document.directory.entries.iter().collect();
    entries.sort_by_key(|entry| entry.stored.start);
    for entry in entries {
        append_entry(document, entry, &mut output, &mut central)?;
    }
    let offset = archive_position(output.len())?;
    let size = archive_position(central.len() + 8)?;
    output.extend_from_slice(&central);
    append_end(
        &mut output,
        &document.directory.end_signature,
        document.directory.entries.len() as u16,
        size,
        offset,
    );
    Ok(output)
}

fn append_entry(
    document: &Document,
    entry: &Entry,
    output: &mut Vec<u8>,
    directory: &mut Vec<u8>,
) -> Result<()> {
    let mut local = entry.local.clone();
    let mut central = entry.central.clone();
    let stored = match document.replacements.get(&entry.name) {
        Some(replacement) => recompress(entry, replacement, &mut local, &mut central)?,
        None => document.original[entry.stored.clone()].to_vec(),
    };
    set_dword(&mut central, 42, archive_position(output.len())?);
    output.extend_from_slice(&local);
    output.extend_from_slice(&stored);
    directory.extend_from_slice(&central);
    Ok(())
}

fn recompress(
    entry: &Entry,
    replacement: &[u8],
    local: &mut [u8],
    central: &mut [u8],
) -> Result<Vec<u8>> {
    let compressed = compression::deflate(&entry.name, replacement)?;
    let compressed_size = u32::try_from(compressed.len()).map_err(|_| {
        invalid(format!(
            "compressed length {}; expected <= u32::MAX",
            compressed.len()
        ))
    })?;
    for (offset, value) in [
        (14, compression::checksum(replacement)),
        (18, compressed_size),
        (22, replacement.len() as u32),
    ] {
        set_dword(local, offset, value);
        set_dword(central, offset + 2, value);
    }
    local[6..10].copy_from_slice(&[6, 0, 8, 0]);
    central[8..12].copy_from_slice(&[6, 0, 8, 0]);
    Ok(compressed)
}

fn archive_position(position: usize) -> Result<u32> {
    position
        .checked_sub(8)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| {
            invalid(format!(
                "file position {position}; expected an archive offset in 0..=u32::MAX"
            ))
        })
}

fn set_dword(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn append_end(output: &mut Vec<u8>, signature: &[u8], count: u16, size: u32, offset: u32) {
    output.extend_from_slice(signature);
    output.extend_from_slice(&[0; 4]);
    output.extend_from_slice(&count.to_le_bytes());
    output.extend_from_slice(&count.to_le_bytes());
    output.extend_from_slice(&size.to_le_bytes());
    output.extend_from_slice(&offset.to_le_bytes());
    output.extend_from_slice(&[0; 2]);
}
