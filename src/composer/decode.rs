use super::{ComposerCompression, ComposerLimits, failure};
use crate::records::{dword, region, require, word};
use crate::{FailureClass, Result};
use std::{ops::Range, sync::Arc};

#[derive(Debug)]
pub(super) struct ComposerRecord {
    pub name: Arc<str>,
    pub local: Range<usize>,
    pub stored: Range<usize>,
    pub central: Range<usize>,
    pub expanded: u32,
    pub crc: u32,
    pub method: ComposerCompression,
}

#[derive(Debug)]
pub(super) struct ComposerDirectory {
    pub members: Vec<ComposerRecord>,
    pub end: usize,
}

pub(super) fn decode_composer_directory(
    bytes: &[u8],
    limits: ComposerLimits,
) -> Result<ComposerDirectory> {
    require(
        bytes.len() <= limits.maximum_bytes && bytes.starts_with(b"SMG"),
        format!(
            "{} bytes, prefix {:?}; expected SMG and <= {} bytes",
            bytes.len(),
            bytes.get(..3),
            limits.maximum_bytes
        ),
    )?;
    let mut failure = failure(
        FailureClass::Invalid,
        "end record",
        format!(
            "{} bytes; expected a consistent ZIP32 end record",
            bytes.len()
        ),
    );
    for end in (3.max(bytes.len().saturating_sub(65557))..bytes.len().saturating_sub(21)).rev() {
        if bytes.get(end..end + 4) != Some(b"PK\x05\x06") {
            continue;
        }
        match candidate(bytes, end, limits) {
            Ok(directory) => return Ok(directory),
            Err(error) => failure = error,
        }
    }
    Err(failure)
}

fn candidate(bytes: &[u8], end: usize, limits: ComposerLimits) -> Result<ComposerDirectory> {
    let tail = region(bytes, end, 22)?;
    require(
        end + 22 + usize::from(word(tail, 20)?) == bytes.len(),
        "end comment length; expected physical EOF",
    )?;
    let disks = (word(tail, 4)?, word(tail, 6)?);
    let counts = (word(tail, 8)?, word(tail, 10)?);
    unsupported(
        disks == (0, 0) && counts.0 == counts.1,
        &format!("disks {disks:?}, counts {counts:?}"),
    )?;
    let count = usize::from(word(tail, 10)?);
    unsupported(
        count != 65535 && dword(tail, 12)? != u32::MAX && dword(tail, 16)? != u32::MAX,
        &format!(
            "ZIP64 count {count}, directory size {}, offset {}",
            dword(tail, 12)?,
            dword(tail, 16)?
        ),
    )?;
    let central = dword(tail, 16)? as usize + 3;
    require(
        central <= end
            && end - central == dword(tail, 12)? as usize
            && count <= (end - central) / 46
            && count <= limits.maximum_members,
        format!(
            "directory {central}..{end}, count {count}; expected exact span and <= {} bounded members",
            limits.maximum_members
        ),
    )?;
    let members = directory_records(bytes, central, end, count)?;
    partition(&members, central)?;
    Ok(ComposerDirectory { members, end })
}

fn directory_records(
    bytes: &[u8],
    start: usize,
    end: usize,
    count: usize,
) -> Result<Vec<ComposerRecord>> {
    let mut members = Vec::with_capacity(count);
    let mut position = start;
    for _ in 0..count {
        let member = central_record(bytes, position, end, start)?;
        position = member.central.end;
        members.push(member);
    }
    require(
        position == end,
        format!("directory end {position}; expected {end}"),
    )?;
    Ok(members)
}

fn central_record(
    bytes: &[u8],
    start: usize,
    end: usize,
    central: usize,
) -> Result<ComposerRecord> {
    let fixed = region(bytes, start, 46)?;
    require(
        &fixed[..4] == b"PK\x01\x02",
        format!("central signature {:?}; expected PK0102", &fixed[..4]),
    )?;
    supported_fields(fixed)?;
    let name_length = usize::from(word(fixed, 28)?);
    let extra_length = usize::from(word(fixed, 30)?);
    let stop = start + 46 + name_length + extra_length + usize::from(word(fixed, 32)?);
    require(
        name_length != 0 && stop <= end,
        format!(
            "central variable span {start}..{stop}, name {name_length}; expected nonempty name within {end}"
        ),
    )?;
    let raw_name = region(bytes, start + 46, name_length)?;
    extra_fields(region(bytes, start + 46 + name_length, extra_length)?)?;
    let local = local_record(bytes, fixed, raw_name, central)?;
    let method = if word(fixed, 10)? == 0 {
        ComposerCompression::Stored
    } else {
        ComposerCompression::Deflated
    };
    Ok(ComposerRecord {
        name: super::names::decode_name(raw_name, word(fixed, 8)?)?.into(),
        local: local.0,
        stored: local.1,
        central: start..stop,
        expanded: dword(fixed, 24)?,
        crc: dword(fixed, 16)?,
        method,
    })
}

fn supported_fields(fixed: &[u8]) -> Result<()> {
    let disk = word(fixed, 34)?;
    let version = word(fixed, 6)?;
    unsupported(
        disk == 0 && version <= 20,
        &format!("disk {disk}, extraction version {version}"),
    )?;
    unsupported(
        [20, 24, 42]
            .into_iter()
            .all(|offset| dword(fixed, offset).is_ok_and(|v| v != u32::MAX)),
        "ZIP64 member fields",
    )?;
    let flags = word(fixed, 8)?;
    let method = word(fixed, 10)?;
    unsupported(
        flags & !0x806 == 0 && matches!(method, 0 | 8),
        &format!("flags {flags:#x}, method {method}"),
    )
}

fn local_record(
    bytes: &[u8],
    central: &[u8],
    name: &[u8],
    boundary: usize,
) -> Result<(Range<usize>, Range<usize>)> {
    let start = dword(central, 42)? as usize + 3;
    let fixed = region(bytes, start, 30)?;
    require(
        &fixed[..4] == b"PK\x03\x04" && fixed[4..26] == central[6..28],
        format!("local header at {start}; expected matching ZIP32 signature and central metadata"),
    )?;
    let name_length = usize::from(word(fixed, 26)?);
    let extra_length = usize::from(word(fixed, 28)?);
    let payload = start + 30 + name_length + extra_length;
    let stop = payload + dword(central, 20)? as usize;
    require(
        stop <= boundary && region(bytes, start + 30, name_length)? == name,
        format!(
            "local member {start}..{stop}; expected matching name and payload before {boundary}"
        ),
    )?;
    extra_fields(region(bytes, start + 30 + name_length, extra_length)?)?;
    Ok((start..payload, payload..stop))
}

fn extra_fields(bytes: &[u8]) -> Result<()> {
    let mut position = 0;
    while position < bytes.len() {
        let tag = word(bytes, position)?;
        let length = usize::from(word(bytes, position + 2)?);
        unsupported(tag != 1, "ZIP64 extra-field tag 1")?;
        region(bytes, position + 4, length)?;
        position += 4 + length;
    }
    Ok(())
}

fn partition(members: &[ComposerRecord], boundary: usize) -> Result<()> {
    let mut ordered: Vec<_> = members.iter().collect();
    ordered.sort_by_key(|member| member.local.start);
    let mut position = 3;
    for member in ordered {
        require(
            member.local.start == position,
            format!(
                "local start {}, previous end {position}; expected no gap or overlap",
                member.local.start
            ),
        )?;
        position = member.stored.end;
    }
    require(
        position == boundary,
        format!("local end {position}; expected directory start {boundary}"),
    )
}

fn unsupported(supported: bool, observed: &str) -> Result<()> {
    if supported {
        return Ok(());
    }
    Err(failure(
        FailureClass::Unsupported,
        "ZIP32 profile",
        format!("{observed}; expected single-disk, unencrypted ZIP32 without descriptors"),
    ))
}
