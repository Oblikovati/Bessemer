use super::{
    ComposerArchive, ComposerCompression, ComposerLimits, ComposerMemberInput, failure,
    payload::require_budget,
};
use crate::{FailureClass, Result, compression};
use flate2::{Compression, write::DeflateEncoder};
use std::io::Write;

pub(super) fn replace_composer_member(
    archive: &ComposerArchive,
    selected: usize,
    replacement: &[u8],
) -> Result<Vec<u8>> {
    require_budget(
        "replacement",
        replacement.len(),
        archive.limits.maximum_member_bytes,
    )?;
    let total = archive.coverage().expanded_bytes - archive.expanded[selected].len();
    let total = total.checked_add(replacement.len()).ok_or_else(|| {
        invalid("replacement total overflow; expected representable decoded size")
    })?;
    require_budget(
        "decoded replacement archive",
        total,
        archive.limits.maximum_expanded_bytes,
    )?;
    serialize_composer(archive, Some((selected, replacement)))
}

pub(super) fn serialize_composer(
    archive: &ComposerArchive,
    replacement: Option<(usize, &[u8])>,
) -> Result<Vec<u8>> {
    let mut output = b"SMG".to_vec();
    let mut offsets = vec![0; archive.directory.members.len()];
    append_local_members(archive, replacement, &mut output, &mut offsets)?;
    let central_start = output.len();
    append_central_members(archive, replacement, &mut output, &offsets)?;
    let mut end = archive.original[archive.directory.end..archive.directory.end + 22].to_vec();
    put_dword(&mut end, 12, wire_size(output.len() - central_start)?);
    put_dword(&mut end, 16, archive_offset(central_start)?);
    append(&mut output, &end, archive.limits)?;
    append(&mut output, archive.comment(), archive.limits)?;
    Ok(output)
}

fn append_local_members(
    archive: &ComposerArchive,
    replacement: Option<(usize, &[u8])>,
    output: &mut Vec<u8>,
    offsets: &mut [u32],
) -> Result<()> {
    let mut ordered: Vec<_> = archive.directory.members.iter().enumerate().collect();
    ordered.sort_by_key(|(_, member)| member.local.start);
    for (index, member) in ordered {
        offsets[index] = archive_offset(output.len())?;
        let mut header = archive.original[member.local.clone()].to_vec();
        let stored =
            if let Some((_, contents)) = replacement.filter(|(selected, _)| *selected == index) {
                let stored = encoded_payload(member.method, contents)?;
                payload_fields(&mut header, 14, contents, stored.len())?;
                stored
            } else {
                archive.original[member.stored.clone()].to_vec()
            };
        append(output, &header, archive.limits)?;
        append(output, &stored, archive.limits)?;
    }
    Ok(())
}

fn append_central_members(
    archive: &ComposerArchive,
    replacement: Option<(usize, &[u8])>,
    output: &mut Vec<u8>,
    offsets: &[u32],
) -> Result<()> {
    for (index, member) in archive.directory.members.iter().enumerate() {
        let mut header = archive.original[member.central.clone()].to_vec();
        if let Some((_, contents)) = replacement.filter(|(selected, _)| *selected == index) {
            let local_offset = offsets[index] as usize + 3;
            let compressed = crate::records::dword(output, local_offset + 18)? as usize;
            payload_fields(&mut header, 16, contents, compressed)?;
        }
        put_dword(&mut header, 42, offsets[index]);
        append(output, &header, archive.limits)?;
    }
    Ok(())
}

pub(super) fn construct_composer(
    members: Vec<ComposerMemberInput>,
    comment: &[u8],
    limits: ComposerLimits,
) -> Result<Vec<u8>> {
    require_budget(
        "member count",
        members.len(),
        limits.maximum_members.min(65534),
    )?;
    require_budget("archive comment", comment.len(), 65535)?;
    let mut output = b"SMG".to_vec();
    let mut central = Vec::new();
    let mut total = 0usize;
    for member in &members {
        total = total
            .checked_add(member.contents.len())
            .ok_or_else(|| invalid("decoded size overflow; expected bounded members"))?;
        require_budget(
            "aggregate decoded bytes",
            total,
            limits.maximum_expanded_bytes,
        )?;
        append_new_member(member, &mut output, &mut central, limits)?;
    }
    let central_start = output.len();
    append(&mut output, &central, limits)?;
    let end = new_end(members.len(), central.len(), central_start, comment.len())?;
    append(&mut output, &end, limits)?;
    append(&mut output, comment, limits)?;
    Ok(output)
}

fn append_new_member(
    member: &ComposerMemberInput,
    output: &mut Vec<u8>,
    central: &mut Vec<u8>,
    limits: ComposerLimits,
) -> Result<()> {
    require_budget(
        &member.name,
        member.contents.len(),
        limits.maximum_member_bytes,
    )?;
    require_budget("encoded member name", member.name.len(), 65535)?;
    if member.name.is_empty() {
        return Err(invalid(
            "member name length 0; expected 1..=65535 UTF-8 bytes",
        ));
    }
    let offset = archive_offset(output.len())?;
    let stored = encoded_payload(member.compression, &member.contents)?;
    let (local, mut directory) = new_headers(member, stored.len())?;
    put_dword(&mut directory, 42, offset);
    append(output, &local, limits)?;
    append(output, member.name.as_bytes(), limits)?;
    append(output, &stored, limits)?;
    append(central, &directory, limits)?;
    append(central, member.name.as_bytes(), limits)
}

fn new_headers(member: &ComposerMemberInput, stored: usize) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut local = vec![0; 30];
    local[..4].copy_from_slice(b"PK\x03\x04");
    local[4..6].copy_from_slice(&20u16.to_le_bytes());
    local[6..8].copy_from_slice(&0x800u16.to_le_bytes());
    let method = if member.compression == ComposerCompression::Stored {
        0u16
    } else {
        8
    };
    local[8..10].copy_from_slice(&method.to_le_bytes());
    local[12..14].copy_from_slice(&33u16.to_le_bytes());
    payload_fields(&mut local, 14, &member.contents, stored)?;
    local[26..28].copy_from_slice(&(member.name.len() as u16).to_le_bytes());
    let mut central = vec![0; 46];
    central[..4].copy_from_slice(b"PK\x01\x02");
    central[4..6].copy_from_slice(&20u16.to_le_bytes());
    central[6..28].copy_from_slice(&local[4..26]);
    central[28..30].copy_from_slice(&local[26..28]);
    Ok((local, central))
}

fn new_end(count: usize, size: usize, offset: usize, comment: usize) -> Result<Vec<u8>> {
    let mut end = vec![0; 22];
    end[..4].copy_from_slice(b"PK\x05\x06");
    end[8..10].copy_from_slice(&(count as u16).to_le_bytes());
    end[10..12].copy_from_slice(&(count as u16).to_le_bytes());
    put_dword(&mut end, 12, wire_size(size)?);
    put_dword(&mut end, 16, archive_offset(offset)?);
    end[20..22].copy_from_slice(&(comment as u16).to_le_bytes());
    Ok(end)
}

fn encoded_payload(method: ComposerCompression, contents: &[u8]) -> Result<Vec<u8>> {
    if method == ComposerCompression::Stored {
        return Ok(contents.to_vec());
    }
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(contents).map_err(|error| {
        invalid(format!(
            "raw deflate write {error}; expected encodable payload"
        ))
    })?;
    encoder.finish().map_err(|error| {
        invalid(format!(
            "raw deflate finish {error}; expected a terminated payload"
        ))
    })
}

fn payload_fields(header: &mut [u8], offset: usize, contents: &[u8], stored: usize) -> Result<()> {
    put_dword(header, offset, compression::checksum(contents));
    put_dword(header, offset + 4, wire_size(stored)?);
    put_dword(header, offset + 8, wire_size(contents.len())?);
    Ok(())
}

fn append(output: &mut Vec<u8>, bytes: &[u8], limits: ComposerLimits) -> Result<()> {
    let length = output
        .len()
        .checked_add(bytes.len())
        .ok_or_else(|| invalid("encoded size overflow; expected bounded output"))?;
    require_budget("encoded archive bytes", length, limits.maximum_bytes)?;
    output.extend_from_slice(bytes);
    Ok(())
}

fn wire_size(value: usize) -> Result<u32> {
    u32::try_from(value)
        .ok()
        .filter(|value| *value != u32::MAX)
        .ok_or_else(|| invalid(format!("ZIP32 size/offset {value}; expected < u32::MAX")))
}

fn archive_offset(value: usize) -> Result<u32> {
    value
        .checked_sub(3)
        .ok_or_else(|| invalid(format!("physical offset {value}; expected >= 3")))
        .and_then(wire_size)
}

fn put_dword(header: &mut [u8], offset: usize, value: u32) {
    header[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}
fn invalid(cause: impl Into<String>) -> crate::BessemerError {
    failure(FailureClass::Invalid, "ZIP32 construction", cause)
}
